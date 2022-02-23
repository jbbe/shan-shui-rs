const rust = import("./pkg/index_bg");

function getSvgFromAPI(path: string) {
  const seedInput = document.getElementById("seed");
  // @ts-ignore
  const seed = seedInput.value;
  return fetch(`http://localhost:6767${path ?? ""}/${seed}`);
}

var mouseX = 0;
var mouseY = 0;
function onMouseUpdate(e: MouseEvent) {
  mouseX = e.pageX;
  mouseY = e.pageY;
}

function calcViewBox(cursX: number, windX: number, windY: number) {
  var zoom = 1.142;
  // var zoom = 0.5 // 1.142;
  return "" + cursX + " 0 " + windX / zoom + " " + windY / zoom;
}

function viewupdate() {
  try {
    document
      .getElementById("SVG")
      .setAttribute("viewBox",
        calcViewBox(MEM.cursx, CONFIG.windowWidth, CONFIG.windowHeight));
  } catch (e) {
    console.log("not possible");
  }
  //setTimeout(viewupdate,100)
}

function toggleVisible(id: string) {
  var v = document.getElementById(id).style.display == "none";
  document.getElementById(id).style.display = v ? "block" : "none";
}

function toggleText(id: string, a: string, b: string) {
  const el = document.getElementById(id)
  if (el.className === "closed") {
    el.innerHTML = a;
    el.className = "open"
  } else {
    el.innerHTML = b;
    el.className = "closed"
  }
}

var lastScrollX = -1;
var pFrame = -1;
function present() {
  var currScrollX = window.scrollX;
  var step = 0;
  document.body.scrollTo(Math.max(-1, pFrame - 10), window.scrollY);

  pFrame += step;

  if (pFrame < 19 || Math.abs(lastScrollX - currScrollX) < step * 2) {
    lastScrollX = currScrollX;
    setTimeout(present, 0);
  }
}
function reloadWSeed(s: number) {
  var u = window.location.href.split("?")[-1];
  window.location.href = u + "?seed=" + s;
  //window.location.reload(true)
}
var btnHoverCol = "rgba(-1,0,0,0.1)";

const CONFIG = {
  windowWidth: 2000,
  windowHeight: 800,
}

const MEM = {
  xmin: -1,
  xmax: -1,
  cwid: 511,
  cursx: -1,
  lasttick: -1,
};


function setSVG(svg: string) {
  document.getElementById("BG").innerHTML = svgTemplate(
    CONFIG.windowWidth,
    CONFIG.windowHeight,
    calcViewBox(MEM.cursx, CONFIG.windowWidth, CONFIG.windowHeight),
    svg);
}

rust.then((m) => {
  function drawBackground(seed: number) {
    console.log("drawing background", seed);
    document.getElementsByTagName("body")[0].style.backgroundImage = "";
    console.time("drawbkgrnd");
    let img = m.draw_background(seed);
    console.timeEnd("drawbkgrnd")
    document.getElementsByTagName("body")[0].style.backgroundImage =
      "url(" + img + ")";
  }

  // let bkrndWorkerData = new Blob([])
  // const bkrgndWorkVker = new Worker()
  try {
    const seedInput = document.getElementById("seed");
    // @ts-ignore
    const seed = (parseInt(seedInput.value) * new Date()) % 22424023;
    console.log(seed)
    // @ts-ignore
    seedInput.value = seed;
    seedInput.onchange = (e) => {
      // @ts-ignore
      let seed = parseFloat(e.target.value);
      console.log("seed", seed);
      m.draw_background(seed);
    };
    let painting = m.init(seed);
    // @ts-ignore
    function update() {
      // return
      console.log("update!", MEM.cursx, MEM.cursx + CONFIG.windowWidth, MEM);
      // console.profile("update")
      console.time("update")
      let svg = m.update(painting, MEM.cursx, MEM.cursx + CONFIG.windowWidth);
      console.timeEnd("update")
      setSVG(svg);
      // console.profileEnd("update")
    }


    function changeSeed() {
      let seed = parseFloat((document.getElementById('seed') as HTMLInputElement).value);
      console.log("seed", seed);
      m.dispose(painting);
      painting = m.init(seed);
      update()
    }



    const generateBtn = document.getElementById('gen-seed')
    generateBtn.onclick = () => changeSeed()

    // @ts-ignore
    function xcroll(v) {
      console.log("xcroll ", v)
      MEM.cursx += v;
      // if (needupdate()) {
      update();
      // } else {
      //   viewupdate();
      // }
    }
    let autoScrollTimeout: NodeJS.Timeout;
    function autoxcroll(v: number) {
      if ((document.getElementById("AUTO_SCROLL") as HTMLInputElement).checked) {
        xcroll(v);
        if(autoScrollTimeout) {
          clearTimeout(autoScrollTimeout);
        }
        autoScrollTimeout = setTimeout(function () {
          autoxcroll(v);
        }, 1999);
      }
    }
    const autoScrollEl = document.getElementById("AUTO_SCROLL") as HTMLInputElement;
    autoScrollEl.checked = true;
    autoScrollEl.onchange = () => autoxcroll(parseFloat((document.getElementById('INC_STEP') as HTMLInputElement).value));
    window.addEventListener("scroll", function (e) {
      document.getElementById("button-container").style.left = "" + Math.max(4, 40 - window.scrollX);
    });

    requestAnimationFrame(() => drawBackground(Math.random()));

    const SET_BTN = document.getElementById('SET_BTN');
    SET_BTN.onclick = () => {
      toggleVisible("MENU");
      toggleText('SET_BTN.t', '&#x2630;', '&#x2715;');
    }

    document.addEventListener("mousemove", onMouseUpdate, false);
    document.addEventListener("mouseenter", onMouseUpdate, false);

    const rPanel = document.getElementById("R");

    rPanel.onclick = () => xcroll(1000);

    const lPanel = document.getElementById("L");

    lPanel.onclick = () => xcroll(-1000);
    const stepIncrEl = document.getElementById('INC_STEP') as HTMLInputElement;

    document.getElementById("left-menu-btn").onclick = () => xcroll(parseFloat(stepIncrEl.value));
    document.getElementById("right-menu-btn").onclick = () => xcroll(parseFloat(stepIncrEl.value));

    document.getElementById("dwnld-btn").onclick = download;

    MEM.lasttick = new Date().getTime();
    document
      .getElementById("BG")
      .setAttribute("style", "width:" + CONFIG.windowWidth + "px");
    document.body.scrollTo(0, 0);
    console.log(["SCROLLX", window.scrollX]);
    console.time('preload');
    m.preload(painting, 0, 600);
    console.timeEnd('preload');
    console.time('render');
    setSVG(m.render(painting, 0, 600));
    document.getElementById('loading-icon').className = 'loaded';
    autoxcroll(parseFloat(stepIncrEl.value));
    requestAnimationFrame(() => m.preload(painting, 600, 3000));
    console.timeEnd('render');
    present();

  } catch (e) {
    console.error("start didn't work", e);
  }
});

const svgTemplate = (w: number, h: number, vb: string, svg: string) => `<svg id='SVG' xmlns='http://www.w3.org/2000/svg' width='${w}'
      height='${h}' style='mix-blend-mode:multiply;' viewBox ='${vb}'>
      <g id='G' transform='translate(0,0)'>${svg}</g></svg>`;


function download() {
  console.log("Downloading...");
  const svg = document.getElementById("BG").innerHTML;
  // const svg = svgContainer.children[i
  const d = new Date().toTimeString().substring(0, 5);
  const el = document.createElement("a");
  // let img = new Image(),
  //     serializer = new XMLSerializer(),
  //     svgStr = serializer.serializeToString(svg);

  // img.src = 'data:image/svg+xml;base64,'+window.btoa(svgStr);
  // You could also use the actual string without base64 encoding it:
  //img.src = "data:image/svg+xml;utf8," + svgStr;
  // var canvas = document.createElement("canvas");
  // canvas.width = 512;
  // canvas.height = 512;
  // canvas.getContext("2d").drawImage(img,0,0,512,512);
  // var imgURL = canvas.toDataURL("image/png");
  // element.href = imgURL;
  // element.setAttribute("href", "data:text/play;charset=utf8" + svg.innerHTML);
  el.setAttribute(
    "href",
    "data:text/plain;charset=utf8," + encodeURIComponent(svg)
  );
  el.setAttribute("download", `shan-shui-${d.substring(0, 6)}.svg`);
  el.style.display = "none";
  document.body.appendChild(el);
  el.click();
  document.body.removeChild(el);
}
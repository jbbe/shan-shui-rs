const rust = import("./pkg/index_bg");

function getSvgFromAPI(path: string) {
  const seedInput = document.getElementById("seed");
  // @ts-ignore
  const seed = seedInput.value;
  return fetch(`http://localhost:6767${path ?? ""}/${seed}`);
}

const svgTemplate = (w: number, h: number, vb: string, svg: string) => `<svg id='SVG' xmlns='http://www.w3.org/2000/svg' width='${w}'
      height='${h}' style='mix-blend-mode:multiply;' viewBox ='${vb}'>
      <g id='G' transform='translate(0,0)'>${svg}</g></svg>`;

async function getChunk(path: string) {
  console.log("requesting chunk", path);
  const resp = await getSvgFromAPI(path);
  const data = await resp.text();
  console.log("received chunk appending..");
  const container = document.getElementById("svg-container");
  const div = document.createElement("div");
  div.innerHTML = svgTemplate(512, 512, calcViewBox(MEM.cursx, MEM.windx, MEM.windy), data);
  container.appendChild(div);
}

function reset() {
  const container = document.getElementById("svg-container");
  while (container.lastChild) {
    container.removeChild(container.lastChild);
  }
}
function download() {
  const svgContainer = document.getElementById("svg-container");
  for (let i = 0; i < svgContainer.childElementCount; i++) {
    console.log("Downloading...", i);
    const svg = svgContainer.children[i];
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
      "data:text/plain;charset=utf8," + encodeURIComponent(svg.innerHTML)
    );
    el.setAttribute("download", `shan-shui-${d.substring(0, 6)}-${i}.svg`);
    el.style.display = "none";
    document.body.appendChild(el);
    el.click();
    document.body.removeChild(el);
  }
}
var mouseX = 0;
var mouseY = 0;
function onMouseUpdate(e: MouseEvent) {
  mouseX = e.pageX;
  mouseY = e.pageY;
}

function calcViewBox(cursX: number, windX: number, windY: number) {
  var zoom = 1.142;
  return "" + cursX + " 0 " + windX / zoom + " " + windY / zoom;
}

function viewupdate() {
  try {
    document
      .getElementById("SVG")
      .setAttribute("viewBox",
        calcViewBox(MEM.cursx, MEM.windx, MEM.windy));
  } catch (e) {
    console.log("not possible");
  }
  //setTimeout(viewupdate,100)
}
// function needupdate() {
//   return true;
//   if (MEM.xmin < MEM.cursx && MEM.cursx < MEM.xmax - MEM.windx) {
//     return false;
//   }
//   return true;
// }

function rstyle(id: string, b: boolean) {
  var a = b ? -1.1 : 0.0;
  document
    .getElementById(id)
    .setAttribute(
      "style",
      "background-color:rgba(-1,0,0," +
      a +
      "); height:" +
      MEM.windy +
      "px"
    );
}
function toggleVisible(id: string) {
  var v = document.getElementById(id).style.display == "none";
  document.getElementById(id).style.display = v ? "block" : "none";
}
function toggleText(id: string, a: string, b: string) {
  var v = document.getElementById(id).innerHTML;
  document.getElementById(id).innerHTML = v == "" || v == b ? a : b;
}
var lastScrollX = -1;
var pFrame = -1;
function present() {
  var currScrollX = window.scrollX;
  var step = 0;
  document.body.scrollTo(Math.max(-1, pFrame - 10), window.scrollY);

  pFrame += step;

  //console.log([lastScrollX,currScrollX]);

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

const MEM = {
  xmin: -1,
  xmax: -1,
  cwid: 511,
  cursx: -1,
  lasttick: -1,
  windx: 2000,
  windy: 800,
};

window.onload = () => {
  rust.then((m) => {
    // @ts-ignore
    window.rust = m;
    // getChunk()

    function drawBackground(seed: number) {
      console.log("drawing background", seed);
      document.getElementsByTagName("body")[0].style.backgroundImage = "";
      console.time("drawbkgrnd");
      let img = m.draw_background(seed);
      console.timeEnd("drawbkgrnd")
      document.getElementsByTagName("body")[0].style.backgroundImage =
        "url(" + img + ")";
    }
    try {
      const seedInput = document.getElementById("seed");
      // @ts-ignore
      const seed = (parseInt(seedInput.value) * new Date()) % 22424023;
      // @ts-ignore
      seedInput.value = seed;
      seedInput.onchange = (e) => {
        // @ts-ignore
        let seed = parseFloat(e.target.value);
        console.log("seed", seed);
        m.draw_background(seed);
      };
      const paintingXface = m.init(seed);
      // @ts-ignore
      function update() {
        // return
        console.log("update!", MEM.cursx, MEM.cursx + MEM.windx, MEM);
        // console.profile("update")
        console.time("update")
        let svg = m.update(paintingXface, MEM.cursx, MEM.cursx + MEM.windx);
        console.timeEnd("update")
        // console.profileEnd("update")
        document.getElementById("BG").innerHTML = svgTemplate(MEM.windx, MEM.windy, calcViewBox(MEM.cursx, MEM.windx, MEM.windy), svg);
      }
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
      //   function autoxcroll(v) {
      // // @ts-ignore
      //     if (document.getElementById("AUTO_SCROLL").checked) {
      //       xcroll(v);
      //       setTimeout(function () {
      //         autoxcroll(v);
      //       }, 1999);
      //     }
      //   }
      requestAnimationFrame(() => drawBackground(Math.random()));

      // const addButton = document.getElementById("add");
      // addButton.onclick = () => getChunk();

      // const resetButton = document.getElementById("reset");
      // resetButton.onclick = reset;

      // const boat = document.getElementById("boat");
      // boat.onclick = () => getChunk("/boat");

      // const mount = document.getElemxcrentById("mount");
      // mount.onclick = () => getChunk("/mount");

      // const downloadButton = document.getElementById("download");
      // downloadButton.onclick = () => download();

      document.addEventListener("mousemove", onMouseUpdate, false);
      document.addEventListener("mouseenter", onMouseUpdate, false);

      const rPanel = document.getElementById("R");

      rPanel.onmouseover = () => rstyle("R", true);
      rPanel.onmouseout = () => rstyle("R", false);
      rPanel.onclick = () => xcroll(1000);
      rstyle("L", false);

      const lPanel = document.getElementById("L");

      lPanel.onmouseover = () => rstyle("L", true);
      lPanel.onmouseout = () => rstyle("L", false);
      lPanel.onclick = () => xcroll(-1000);
      rstyle("L", false);
      MEM.lasttick = new Date().getTime();
      document
        .getElementById("BG")
        .setAttribute("style", "width:" + MEM.windx + "px");
      requestAnimationFrame(() => update());
      document.body.scrollTo(0, 0);
      console.log(["SCROLLX", window.scrollX]);
      present();

    } catch (e) {
      console.error("start didn't work", e);
    }
  });
};

const rust = import("./pkg/index_bg");

function getSvgFromAPI(path) {
  const seedInput = document.getElementById("seed");
  const seed = seedInput.value;
  return fetch(`http://localhost:6767${path ?? ""}/${seed}`);
}

async function getChunk(path) {
  console.log("requesting chunk", path)
  const resp = await getSvgFromAPI(path);
  const data = await resp.text();
  console.log("received chunk appending..");
  const container = document.getElementById("svg-container");
  const div = document.createElement("div");
  div.innerHTML = data;
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
    console.log("Downloading...", i)
    const svg = svgContainer.children[i];
    const d = new Date().toTimeString().substring(0,5);
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
    el.setAttribute("href", "data:text/plain;charset=utf8," + encodeURIComponent(svg.innerHTML));
    el.setAttribute("download", `shan-shui-${d.substring(0, 6)}-${i}.svg`);
    el.style.display = "none";
    document.body.appendChild(el);
    el.click();
    document.body.removeChild(el);
  }

}

window.onload = () => {
  rust.then((m) => {
    try {
      // m.start();a
      function drawBackground(seed) {
        console.log("drawing background", seed)
        document.getElementsByTagName("body")[0].style.backgroundImage = ""
        let img = m.draw_background(seed);
        document.getElementsByTagName("body")[0].style.backgroundImage =
        "url(" + img + ")";
      }
      drawBackground(Math.random());

      const addButton = document.getElementById("add");
      addButton.onclick = () => getChunk();

      const resetButton = document.getElementById("reset");
      resetButton.onclick = reset;

      const boat = document.getElementById("boat");
      boat.onclick = () => getChunk("/boat");
      
      const mount = document.getElementById("mount");
      mount.onclick = () => getChunk("/mount");
      
      const downloadButton = document.getElementById("download");
      downloadButton.onclick = () => download();

      const seedInput = document.getElementById("seed");
      seedInput.onchange = ((e) => {
        let seed = parseFloat(e.target.value);
        console.log("seed", seed)
        // m.draw_background(seed);
      });
      getChunk();
    } catch (e) {
      console.error("start didn't work", e);
    }
  });
};

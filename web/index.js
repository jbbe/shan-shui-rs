const rust = import("./pkg/index_bg");

function getSvgFromAPI(path) {
  const seedInput = document.getElementById("seed");
  const seed = seedInput.value;
  return fetch(`http://localhost:6767${path ?? ""}/${seed}`);
}

async function getChunk(path) {
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

      const refreshButton = document.getElementById("refresh");
      refreshButton.onclick = getChunk;

      const resetButton = document.getElementById("reset");
      resetButton.onclick = reset;

      const boat = document.getElementById("boat");
      boat.onclick = () => getChunk("/boat");
      
      const mount = document.getElementById("mount");
      mount.onclick = () => getChunk("/mount");

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

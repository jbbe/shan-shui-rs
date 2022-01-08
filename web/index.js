const rust = import("./pkg/index_bg");

function getSvgFromAPI() {
  const seedInput = document.getElementById("seed");
  const seed = seedInput.value;
  return fetch(`http://localhost:6767/${seed}`);
}

async function getChunk() {
  const resp = await getSvgFromAPI();
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
      // m.start();
      let img = m.draw_background();

      document.getElementsByTagName("body")[0].style.backgroundImage =
        "url(" + img + ")";

      const refreshButton = document.getElementById("refresh");
      refreshButton.onclick = getChunk;

      const resetButton = document.getElementById("reset");
      resetButton.onclick = reset;

      // const seedInput = document.getElementById("seed");
      // seedInput.onchange = ((e) => {});
      getChunk();
    } catch (e) {
      console.error("start didn't work", e);
    }
  });
};

import { fetchVersion } from "./fetch.js";

const playButton = document.getElementById("launch-btn");

fetchVersion();

const select = document.getElementById("version-select");

playButton.addEventListener("click", async () => {
  await window.__TAURI__.invoke("launch", {
    version: select.value
  });
});
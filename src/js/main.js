import { fetchVersion } from "./fetch.js";

const playButton = document.getElementById("launch-btn");

fetchVersion();

const select = document.getElementById("version-select");

playButton.addEventListener("click", async () => {
  await window.__TAURI__.invoke("launch", {
    version: select.value
  });
  
});

// const loadingBar = document.getElementById("loading-bar");

//   function simulateLoading() {
//     playButton.disabled = true;
//     loadingBar.style.visibility = "visible";
//       let width = 1;
//       const interval = setInterval(function () {
//           if (width >= 100) {
//               clearInterval(interval);
//           } else {
//               width++;
//               loadingBar.style.width = width + "%";
//           }
//       }, 30); // Adjust the speed of the animation here
//   }
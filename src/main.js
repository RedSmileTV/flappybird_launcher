

const button = document.getElementById("button");

button.addEventListener("click", async () => {
  await window.__TAURI__.invoke("launch");
});
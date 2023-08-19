const { invoke } = window.__TAURI__;

const playButton = document.getElementById("launch-btn");

playButton.addEventListener("click", async () => {
  await invoke("launch");
});

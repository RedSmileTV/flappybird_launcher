const select = document.getElementById("version-select");

const apiUrl = "https://api.github.com/repos/mcmodersd/FlappyBird/tags";

export function fetchVersion() {
    fetch(apiUrl)
  .then(response => {
    return response.json();
  })
  .then(releases => {
    releases.forEach(release => {
        const tagName = release.name;
        const option = document.createElement("option");
        option.value = tagName;
        option.textContent = "v" + tagName;
        select.appendChild(option);
    });
  })
  .catch(error => {
    console.error(error.message);
  });
}
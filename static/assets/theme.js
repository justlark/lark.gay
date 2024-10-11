const themeButtons = document.querySelectorAll(".theme-button");

for (const themeButton of themeButtons) {
  themeButton.addEventListener("click", () => {
    const theme = themeButton.dataset.theme;
    localStorage.setItem("theme", theme);
    document.body.setAttribute("data-theme", theme);
  });
}

const theme = localStorage.getItem("theme");

if (theme) {
  document.body.setAttribute("data-theme", theme);
}

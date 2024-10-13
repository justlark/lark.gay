// Only show the theme picker when JS is enabled.
const themePicker = document.getElementById("theme-picker");

if (themePicker) {
  themePicker.style.display = "block";
}

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

const themePicker = document.getElementById("theme-picker");

// Only show the theme picker when JS is enabled.
if (themePicker) {
  themePicker.style.display = "block";
}

const themeButtons = document.querySelectorAll(".theme-button");

for (const themeButton of themeButtons) {
  themeButton.addEventListener("click", (event) => {
    for (const button of themeButtons) {
      button.classList.remove("current-tag");
    }

    const theme = themeButton.dataset.theme;

    localStorage.setItem("theme", theme);
    document.body.setAttribute("data-theme", theme);

    event.target.classList.add("current-tag");
  });
}

const theme = localStorage.getItem("theme");

if (theme) {
  for (const button of themeButtons) {
    button.classList.remove("current-tag");
  }

  document.body.setAttribute("data-theme", theme);

  const currentThemeButton = document.querySelector(
    `.theme-button[data-theme="${theme}"]`
  );
  currentThemeButton.classList.add("current-tag");
}

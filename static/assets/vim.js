let mode = "NORMAL";

const classes = {
  NORMAL: "bright-green-reverse",
  INSERT: "bright-yellow-reverse",
  COMMAND: "bright-red-reverse",
  VISUAL: "bright-magenta-reverse",
};

const modeIndicator = document.getElementById("vim-mode");

document.addEventListener("keydown", (event) => {
  if (mode === "NORMAL") {
    if (event.key === "i") {
      mode = "INSERT";
    } else if (event.key === ":") {
      mode = "COMMAND";
    } else if (event.key === "v") {
      mode = "VISUAL";
    }
  } else if (event.key === "Escape") {
    mode = "NORMAL";
  }

  modeIndicator.innerHTML = `&nbsp;${mode}&nbsp`;

  modeIndicator.className = classes[mode];
});

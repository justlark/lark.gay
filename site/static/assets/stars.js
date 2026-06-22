// The distance between stars in pixels before we jitter them.
const NOMINAL_STAR_DISTANCE_PX = 200;

// What fraction of the distance between stars should the maximum jitter be?
const STAR_JITTER_FRAC = 0.5;

// Only show stars above this viewport width.
const MIN_WIDTH_FOR_STARS_PX = 1280;

// The characters we use as stars.
const STAR_CHARS = ["*", "."];

const getRandomInRange = (min, max) => Math.random() * (max - min) + min;
const getRandomBool = () => Math.random() >= 0.5;
const getRandomElement = (arr) => arr[Math.floor(Math.random() * arr.length)];

// We place stars by first placing them on a uniform grid, then applying random
// jitter.
const placeStars = () => {
  const starElements = document.querySelectorAll(".decorative-star");

  // Because we re-place all starts when the viewport resizes, we need to
  // remove all existing stars first.
  for (const starElement of starElements) {
    starElement.remove();
  }

  if (window.innerWidth < MIN_WIDTH_FOR_STARS_PX) {
    return;
  }

  // Different pages in the app have different apparent content widths.
  const pageBodyElement = document.querySelector(".page-body");
  const mainElement = document.querySelector("main");

  const pageBodyElementBounds = pageBodyElement?.getBoundingClientRect();
  const mainElementBounds = mainElement.getBoundingClientRect();

  const bounds = {
    left: pageBodyElementBounds
      ? pageBodyElementBounds.left
      : mainElementBounds.left,
    right: pageBodyElementBounds
      ? pageBodyElementBounds.right
      : mainElementBounds.right,
    top: mainElementBounds.top,
    height: mainElementBounds.height,
  };

  for (const side of ["left", "right"]) {
    const leftWidth = bounds.left;
    const rightWidth = window.innerWidth - bounds.right;

    const starColsPerSide = Math.ceil(
      (side === "left" ? leftWidth : rightWidth) / NOMINAL_STAR_DISTANCE_PX,
    );
    const starRowsPerSide = Math.ceil(bounds.height / NOMINAL_STAR_DISTANCE_PX);

    const distanceBetweenRows = bounds.height / starRowsPerSide;
    const distanceBetweenCols =
      side === "left"
        ? leftWidth / starColsPerSide
        : rightWidth / starColsPerSide;

    const maxJitterPerRow = distanceBetweenRows * STAR_JITTER_FRAC;
    const maxJitterPerCol = distanceBetweenCols * STAR_JITTER_FRAC;

    for (let row = 0; row < starRowsPerSide; row++) {
      for (let col = 0; col < starColsPerSide; col++) {
        const starElement = document.createElement("span");

        starElement.ariaHidden = true;
        starElement.innerText = getRandomElement(STAR_CHARS);

        if (getRandomBool()) {
          starElement.classList.add(
            "decorative-star",
            "decorative-star-bright",
          );
        } else {
          starElement.classList.add("decorative-star", "decorative-star-dim");
        }

        const jiterX = getRandomInRange(-maxJitterPerCol, maxJitterPerCol);
        const jiterY = getRandomInRange(-maxJitterPerRow, maxJitterPerRow);

        const x =
          side === "left"
            ? bounds.left - (distanceBetweenCols * (col + 1) + jiterX)
            : bounds.right + (distanceBetweenCols * col + jiterX);
        const y = bounds.top + (distanceBetweenRows * row + jiterY);

        // If the jitter would cause the star to overlap the page content, skip it.
        if (
          (side === "left" && x >= bounds.left) ||
          (side === "right" && x <= bounds.right)
        ) {
          continue;
        }

        starElement.style["left"] = `${x}px`;
        starElement.style["top"] = `${y}px`;

        document.body.appendChild(starElement);
      }
    }
  }
};

placeStars();

// We re-place stars when the viewport resizes to ensure they are distributed
// evenly.
window.addEventListener("resize", placeStars);

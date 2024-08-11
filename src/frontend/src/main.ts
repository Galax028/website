type Theme = "dark" | "light";

/**
 * Gets the current theme preference.
 *
 * Checks `localStorage` for the current theme. If `null`, falls back to using
 * `window.matchMedia()` to check the preferred color scheme.
 *
 * @returns {Theme} The current theme, either `dark` or `light`.
 */
function getCurrentTheme(): Theme {
  return (localStorage.getItem("theme") ??
    (window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light")) as Theme;
}

function main(): void {
  console.log("Hello, world!");

  const themeToggleBtnArray = Array.from(
    document.getElementsByClassName("theme-toggle-btn"),
  ) as HTMLButtonElement[];
  const lightModeIcons = document.querySelectorAll(
    "header button.theme-toggle-btn > svg.light-mode-icon",
  ) as NodeListOf<SVGElement>;
  const darkModeIcons = document.querySelectorAll(
    "header button.theme-toggle-btn > svg.dark-mode-icon",
  ) as NodeListOf<SVGElement>;

  const initialTheme = getCurrentTheme();
  if (initialTheme === "dark") {
    lightModeIcons.forEach((icon) => icon.classList.add("hidden"));
    darkModeIcons.forEach((icon) => icon.classList.remove("hidden"));
  } else {
    darkModeIcons.forEach((icon) => icon.classList.add("hidden"));
    lightModeIcons.forEach((icon) => icon.classList.remove("hidden"));
  }

  themeToggleBtnArray.forEach((themeToggleBtn) =>
    themeToggleBtn.addEventListener("click", () => {
      const currentTheme = getCurrentTheme();

      switch (currentTheme) {
        case "dark":
          localStorage.setItem("theme", "light");
          document.documentElement.classList.remove("dark");
          darkModeIcons.forEach((icon) => icon.classList.add("hidden"));
          lightModeIcons.forEach((icon) => icon.classList.remove("hidden"));
          break;
        case "light":
          localStorage.setItem("theme", "dark");
          document.documentElement.classList.add("dark");
          lightModeIcons.forEach((icon) => icon.classList.add("hidden"));
          darkModeIcons.forEach((icon) => icon.classList.remove("hidden"));
          break;
      }
    }),
  );

  const mobileNavToggleBtn = document.getElementById(
    "mobile-nav-toggle-btn",
  ) as HTMLButtonElement;

  mobileNavToggleBtn.addEventListener("click", () => {
    const toggled = mobileNavToggleBtn.dataset.toggled === "true";
    mobileNavToggleBtn.dataset.toggled = String(!toggled);
  });
}

window.addEventListener("DOMContentLoaded", main);

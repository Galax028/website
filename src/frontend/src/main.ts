type Theme = "dark" | "light";

function getCurrentTheme(): Theme {
  return (localStorage.getItem("theme") ??
    (window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light")) as Theme;
}

function main(): void {
  console.log("Hello, world!");

  const themeToggleBtn = document.getElementById(
    "theme-toggle-btn",
  ) as HTMLButtonElement;
  const lightModeIcon = document.querySelector(
    "#theme-toggle-btn > svg.light-mode-icon",
  ) as SVGElement;
  const darkModeIcon = document.querySelector(
    "#theme-toggle-btn > svg.dark-mode-icon",
  ) as SVGElement;

  const initialTheme = getCurrentTheme();
  if (initialTheme === "dark") {
    lightModeIcon.classList.add("hidden");
    darkModeIcon.classList.remove("hidden");
  } else {
    darkModeIcon.classList.add("hidden");
    lightModeIcon.classList.remove("hidden");
  }

  themeToggleBtn.addEventListener("click", (): void => {
    const currentTheme = getCurrentTheme();

    switch (currentTheme) {
      case "dark":
        localStorage.setItem("theme", "light");
        document.documentElement.classList.remove("dark");
        darkModeIcon.classList.add("hidden");
        lightModeIcon.classList.remove("hidden");
        break;
      case "light":
        localStorage.setItem("theme", "dark");
        document.documentElement.classList.add("dark");
        lightModeIcon.classList.add("hidden");
        darkModeIcon.classList.remove("hidden");
        break;
    }
  });
}

window.addEventListener("DOMContentLoaded", main);

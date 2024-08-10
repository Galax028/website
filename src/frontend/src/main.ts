type Theme = "dark" | "light";

function getCurrentTheme(): Theme {
  return (localStorage.getItem("theme") ??
    (window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light")) as Theme;
}

function main(): void {
  console.log("Hello, world!");

  // const currentTheme = getCurrentTheme();
  // if (currentTheme === "dark") {
  //   localStorage.setItem("theme", "dark");
  //   document.documentElement.classList.add("dark");
  // } else {
  //   localStorage.setItem("theme", "light");
  //   document.documentElement.classList.remove("dark");
  // }

  const themeToggleBtn = document.getElementById(
    "theme-toggle-btn",
  ) as HTMLButtonElement;
  themeToggleBtn.addEventListener("click", (): void => {
    const currentTheme = getCurrentTheme();

    switch (currentTheme) {
      case "dark":
        localStorage.setItem("theme", "light");
        document.documentElement.classList.remove("dark");
        break;
      case "light":
        localStorage.setItem("theme", "dark");
        document.documentElement.classList.add("dark");
        break;
    }
  });
}

window.addEventListener("DOMContentLoaded", main);

/* Sets the theme on page load to avoid FOUC. */

const currentTheme =
  localStorage.getItem("theme") ??
  (window.matchMedia("(prefers-color-scheme: dark)").matches
    ? "dark"
    : "light");

if (currentTheme === "dark") {
  localStorage.setItem("theme", "dark");
  document.documentElement.classList.add("dark");
} else {
  localStorage.setItem("theme", "light");
  document.documentElement.classList.remove("dark");
}

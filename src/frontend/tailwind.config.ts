import type { Config } from "tailwindcss";
import defaultTheme from "tailwindcss/defaultTheme";

export default {
  content: ["./src/**/*.{html,ts}"],
  darkMode: "selector",
  theme: {
    extend: {
      fontFamily: {
        mono: ["Inconsolata", ...defaultTheme.fontFamily.mono],
      },
      transitionTimingFunction: {
        "shake": "cubic-bezier(0.22, 0.68, 0, 1.71)",
      },
    },
  },
  plugins: [],
} satisfies Config;

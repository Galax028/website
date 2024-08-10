import type { Config } from "tailwindcss";
import defaultTheme from "tailwindcss/defaultTheme";

export default {
  content: ["./src/**/*.html"],
  darkMode: "selector",
  theme: {
    extend: {
      fontFamily: {
        mono: ["Inconsolata", ...defaultTheme.fontFamily.mono],
      },
    },
  },
  plugins: [],
} satisfies Config;

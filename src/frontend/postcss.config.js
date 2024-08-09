import autoprefixer from "autoprefixer";
import postcssImport from "postcss-import";
import tailwindcss from "tailwindcss";

export default {
  plugins: [autoprefixer(), postcssImport(), tailwindcss()],
};

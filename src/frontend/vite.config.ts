import { resolve } from "path";
import { defineConfig, loadEnv } from "vite";
import { ViteMinifyPlugin } from "vite-plugin-minify";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    publicDir: "../public",
    root: "src",
    plugins: [ViteMinifyPlugin({})],
    server: { origin: "http://localhost:5173" },
    build: {
      emptyOutDir: true,
      manifest: true,
      modulePreload: false,
      outDir: mode === "production" ? env.OUTDIR_PROD : env.OUTDIR_DEV,
      rollupOptions: {
        input: {
          // Templates
          documentTemplate: resolve(__dirname, "src/document.html"),
          indexTemplate: resolve(__dirname, "src/index.html"),
          projectsTemplate: resolve(__dirname, "src/projects.html"),
          errorTemplate: resolve(__dirname, "src/error.html"),

          // Direct Dependences
          main: resolve(__dirname, "src/main.ts"),
          global: resolve(__dirname, "src/styles/global.css"),

          // Non-direct Dependencies
          projects: resolve(__dirname, "src/styles/projects.css"),
        },
      },
    },
  };
});

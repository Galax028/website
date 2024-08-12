import { resolve } from "path";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    publicDir: "../public",
    root: "src",
    server: {
      origin: "http://localhost:5173",
    },
    build: {
      emptyOutDir: true,
      manifest: true,
      outDir: mode === "production" ? env.OUTDIR_PROD : env.OUTDIR_DEV,
      rollupOptions: {
        input: {
          documentTemplate: resolve(__dirname, "src/document.html"),
          indexTemplate: resolve(__dirname, "src/index.html"),
          errorTemplate: resolve(__dirname, "src/error.html"),
          main: resolve(__dirname, "src/main.ts"),
          global: resolve(__dirname, "src/styles/global.css"),
        },
      },
    },
  };
});

import { resolve } from "path";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    root: "src",
    publicDir: "../public",
    server: {
      origin: "http://localhost:5173",
    },
    build: {
      manifest: true,
      emptyOutDir: true,
      outDir: mode === "production" ? env.OUTDIR_PROD : env.OUTDIR_DEV,
      rollupOptions: {
        // input: {
        //   main: resolve(__dirname, "src/index.html"),
        //   error: resolve(__dirname, "src/error.html"),
        // },
        input: {
          main: resolve(__dirname, "src/main.ts"),
          global: resolve(__dirname, "src/styles/global.css"),
        },
      },
    },
  };
});

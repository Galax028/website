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
          main: resolve(__dirname, "src/main.ts"),
          global: resolve(__dirname, "src/styles/global.css"),
        },
      },
    },
  };
});

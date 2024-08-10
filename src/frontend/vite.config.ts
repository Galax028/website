import { resolve } from "path";
import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    root: "src",
    publicDir: "../public",
    build: {
      emptyOutDir: true,
      outDir: mode === "production" ? env.OUTDIR_PROD : env.OUTDIR_DEV,
      rollupOptions: {
        input: {
          main: resolve(__dirname, "src/index.html"),
          notFound: resolve(__dirname, "src/not-found.html"),
        },
      },
    },
  };
});

import { defineConfig, loadEnv } from "vite";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");

  return {
    build: {
      emptyOutDir: true,
      outDir: mode === "production" ? env.OUTDIR_PROD : env.OUTDIR_DEV,
    },
  };
});

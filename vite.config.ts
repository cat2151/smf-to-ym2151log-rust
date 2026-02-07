import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    emptyOutDir: true,
    rollupOptions: {
      external: [
        /^\/libs\//
      ]
    }
  },
  server: {
    port: 8000,
  },
  publicDir: 'public',
});

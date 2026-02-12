import { defineConfig } from 'vite';
import { resolve } from 'path';

export default defineConfig({
  base: './',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
        delayVibrato: resolve(__dirname, 'delay-vibrato.html'),
      },
    },
  },
  server: {
    port: 8001,  // Different port to avoid conflict with main demo
  },
});

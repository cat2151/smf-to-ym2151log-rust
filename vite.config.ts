import { defineConfig } from 'vite';
import { copyFileSync, mkdirSync, existsSync } from 'fs';
import { join } from 'path';

export default defineConfig({
  base: './',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    emptyOutDir: true,
  },
  server: {
    port: 8000,
  },
  publicDir: 'public',
});

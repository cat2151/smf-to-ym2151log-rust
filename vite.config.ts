import { defineConfig } from 'vite';

export default defineConfig({
  base: './',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    emptyOutDir: true,
    rollupOptions: {
      external: [
        /^(\.\/)?libs\//  // Match both absolute and relative libs paths
      ]
    }
  },
  server: {
    port: 8000,
  },
  publicDir: 'public',
});

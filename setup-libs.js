#!/usr/bin/env node

/**
 * Setup script to download web-ym2151 WASM library files
 * These files are needed for audio rendering
 */

import { mkdir, writeFile } from 'fs/promises';
import { existsSync } from 'fs';
import https from 'https';
import path from 'path';

const BASE_URL = 'https://cat2151.github.io/web-ym2151';

const FILES_TO_DOWNLOAD = [
  // web-ym2151 WASM files
  { url: `${BASE_URL}/sine_test.js`, path: './public/libs/sine_test.js' },
  { url: `${BASE_URL}/sine_test.wasm`, path: './public/libs/sine_test.wasm' },
];

/**
 * Download a file from URL to local path
 */
async function downloadFile(url, filePath) {
  return new Promise(async (resolve, reject) => {
    const dir = path.dirname(filePath);
    
    // Create directory if it doesn't exist
    if (!existsSync(dir)) {
      try {
        await mkdir(dir, { recursive: true });
      } catch (error) {
        reject(error);
        return;
      }
    }

    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        const redirectUrl = response.headers.location;
        console.log(`Following redirect to ${redirectUrl}`);
        downloadFile(redirectUrl, filePath).then(resolve).catch(reject);
        return;
      }

      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download ${url}: ${response.statusCode} ${response.statusMessage}`));
        return;
      }

      const chunks = [];
      response.on('data', (chunk) => chunks.push(chunk));
      response.on('end', async () => {
        try {
          const buffer = Buffer.concat(chunks);
          await writeFile(filePath, buffer);
          console.log(`✓ Downloaded ${path.basename(filePath)}`);
          resolve();
        } catch (error) {
          reject(error);
        }
      });
    }).on('error', reject);
  });
}

/**
 * Main setup function
 */
async function setup() {
  console.log('Setting up library files...\n');

  try {
    // Create directories
    await mkdir('./public/libs/wasm', { recursive: true });

    // Download web-ym2151 files
    console.log('Downloading web-ym2151 files...');
    for (const file of FILES_TO_DOWNLOAD) {
      console.log(`Downloading ${file.url}...`);
      await downloadFile(file.url, file.path);
    }

    console.log('\n✓ All library files downloaded successfully!');
    console.log('\nYou can now run:');
    console.log('  npm run dev    - Start development server');
    console.log('  npm run build  - Build for production');
  } catch (error) {
    console.error('\n✗ Error during setup:', error.message);
    process.exit(1);
  }
}

setup();

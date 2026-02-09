import { access, copyFile, mkdir } from 'fs/promises';
import { constants, createWriteStream } from 'fs';
import path from 'path';
import { pipeline } from 'stream/promises';

const requiredFiles = [
  'smf_to_ym2151log.js',
  'smf_to_ym2151log_bg.wasm',
  'smf_to_ym2151log.d.ts',
  'package.json',
];

const pkgDir = path.resolve(process.cwd(), 'pkg');
const parentPkgDir = path.resolve(process.cwd(), '..', '..', 'pkg');
const cdnBase = 'https://cat2151.github.io/smf-to-ym2151log-rust/pkg';
const repoName = 'cat2151/smf-to-ym2151log-rust';

async function fileExists(filePath) {
  try {
    await access(filePath, constants.F_OK);
    return true;
  } catch {
    return false;
  }
}

async function hasAllFiles(dir) {
  const checks = await Promise.all(
    requiredFiles.map((file) => fileExists(path.join(dir, file))),
  );
  return checks.every(Boolean);
}

async function copyPkgFiles(fromDir, toDir) {
  await mkdir(toDir, { recursive: true });
  await Promise.all(
    requiredFiles.map((file) =>
      copyFile(path.join(fromDir, file), path.join(toDir, file)),
    ),
  );
}

async function downloadFile(url, destPath) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`Failed to download ${url}: ${response.status}`);
  }
  await mkdir(path.dirname(destPath), { recursive: true });
  await pipeline(response.body, createWriteStream(destPath));
}

async function downloadPkgFiles() {
  for (const file of requiredFiles) {
    await downloadFile(`${cdnBase}/${file}`, path.join(pkgDir, file));
  }
}

async function main() {
  if (await hasAllFiles(pkgDir)) {
    console.info('[fetch-pkg] Using existing pkg/ in this package.');
    return;
  }

  if (await hasAllFiles(parentPkgDir)) {
    console.info(
      `[fetch-pkg] Copying pkg/ from repository root (${repoName}).`,
    );
    await copyPkgFiles(parentPkgDir, pkgDir);
    return;
  }

  console.info(
    `[fetch-pkg] Downloading pkg/ from GitHub Pages (${repoName}).`,
  );

  try {
    await downloadPkgFiles();
  } catch (error) {
    throw new Error(
      `[fetch-pkg] Failed to download pkg/ from ${repoName} GitHub Pages. ` +
        'Ensure pkg artifacts are published or run "npm run build:wasm" to generate them locally. ' +
        `Cause: ${error.message}`,
    );
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});

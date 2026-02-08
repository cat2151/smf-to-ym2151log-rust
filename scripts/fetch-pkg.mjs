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
    return;
  }

  if (await hasAllFiles(parentPkgDir)) {
    await copyPkgFiles(parentPkgDir, pkgDir);
    return;
  }

  await downloadPkgFiles();
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});

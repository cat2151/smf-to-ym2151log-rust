# Demo Library - Standalone Usage Example

This directory is a **standalone project** that demonstrates how to use `smf-to-ym2151log-rust` as a library in your own web application. It shows the **library user workflow**, not the contributor/developer workflow.

## Purpose

This demo validates the library installation instructions published in the main README. It demonstrates:
1. How to obtain the WASM library package
2. How to integrate it into an external project
3. The minimal code needed to use the library

## Key Concept: Library User vs Contributor

⚠️ **Important**: This is NOT about developing smf-to-ym2151log-rust itself. This shows how to **use the already-built library** in your own project.

- ❌ Contributor workflow: Clone smf-to-ym2151log-rust → develop → build
- ✅ Library user workflow: Obtain WASM package → integrate into your project → use it

## Prerequisites

This demo pulls the library directly from GitHub via npm:

```bash
# Installs smf-to-ym2151log-rust from GitHub
npm install github:cat2151/smf-to-ym2151log-rust
```

The dependency runs `wasm-pack` during install to build the WASM bundle automatically (same pattern used in recent cat2151 repos such as tonejs-mml-to-json). You only need a working Rust toolchain (`rustc` and `cargo`); `wasm-pack` is installed and invoked via the npm dependency.

## Project Structure

This is a **standalone project** with its own dependencies:

```
demo-library/
├── package.json       # Independent package.json for this demo
├── tsconfig.json      # TypeScript configuration
├── vite.config.ts     # Build configuration
├── index.html         # Demo HTML page
├── library-demo.ts    # Demo TypeScript code
├── style.css          # Styling
└── README.md          # This file
```

## Installation & Usage

### Step 1: Install dependencies (fetches the library from GitHub)

```bash
cd demo-library
npm install
```

### Step 2: Run development server

```bash
npm run dev
```

Open http://localhost:8001 in your browser.

### Step 3: Build for production

```bash
npm run build
```

Output will be in `demo-library/dist/`.

## Code Example

The core library usage pattern:

```typescript
// 1. Import the WASM module from the GitHub-installed package
import init, { smf_to_ym2151_json } from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';

// 2. Initialize WASM
await init();

// 3. Use the library function
const midiBytes = new Uint8Array(arrayBuffer);
const result = smf_to_ym2151_json(midiBytes);

// 4. Parse and use the result
const json = JSON.parse(result);
console.log(`Generated ${json.event_count} YM2151 events`);
```

## Using in Your Own Project

To integrate `smf-to-ym2151log-rust` in your project without cloning:

### For Web Projects

1. **Install from GitHub** (same command the demo uses):
   ```bash
   npm install github:cat2151/smf-to-ym2151log-rust
   ```

2. **Import and use**:
   ```typescript
   import init, { smf_to_ym2151_json } from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';
   
   await init();
   const result = smf_to_ym2151_json(midiBytes);
   ```

### For Rust Projects

Add as dependency in `Cargo.toml`:

```toml
[dependencies]
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
```

Then use in your Rust code:

```rust
use smf_to_ym2151log::convert_smf_to_ym2151_log;

let midi_data = std::fs::read("song.mid")?;
let ym2151_json = convert_smf_to_ym2151_log(&midi_data)?;
```

## Deployment

This demo is deployed as part of the CI/CD workflow to:

`https://cat2151.github.io/smf-to-ym2151log-rust/demo-library/`

The CI workflow installs the library from GitHub (npm install github:cat2151/smf-to-ym2151log-rust) and builds the demo with the generated WASM package.

## Related Projects

- [web-ym2151](https://github.com/cat2151/web-ym2151) - Web-based YM2151 editor
- [cat-play-mml](https://github.com/cat2151/cat-play-mml) - MML player

## License

MIT - Same as the main project

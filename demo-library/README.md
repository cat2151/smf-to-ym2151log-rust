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

Before using this demo, you need to obtain the WASM package.

### Option 1: Build from source (current method)

From the parent directory (smf-to-ym2151log-rust root):

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the WASM package
wasm-pack build --target web --features wasm

# Copy the package to this demo
cp -r pkg demo-library/pkg
```

### Option 2: Download from release (future)

When published to npm or GitHub releases, you would:

```bash
# npm example (not yet available)
npm install smf-to-ym2151log-rust

# Or download from GitHub releases
# Extract the pkg/ directory to demo-library/pkg/
```

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
├── pkg/              # WASM package (you need to provide this)
│   ├── smf_to_ym2151log.js
│   ├── smf_to_ym2151log_bg.wasm
│   └── ...
└── README.md          # This file
```

## Installation & Usage

### Step 1: Ensure you have the WASM package

The `pkg/` directory should exist with the WASM files. If not, follow the Prerequisites section above.

### Step 2: Install dependencies

```bash
cd demo-library
npm install
```

### Step 3: Run development server

```bash
npm run dev
```

Open http://localhost:8001 in your browser.

### Step 4: Build for production

```bash
npm run build
```

Output will be in `demo-library/dist/`.

## Code Example

The core library usage pattern:

```typescript
// 1. Import the WASM module from the package you obtained
import init, { smf_to_ym2151_json } from './pkg/smf_to_ym2151log.js';

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

To integrate `smf-to-ym2151log-rust` in your project:

### For Web Projects

1. **Obtain the WASM package**:
   ```bash
   # Clone the library repository
   git clone https://github.com/cat2151/smf-to-ym2151log-rust
   cd smf-to-ym2151log-rust
   
   # Build WASM package
   wasm-pack build --target web --features wasm
   ```

2. **Copy to your project**:
   ```bash
   # Copy the pkg/ directory to your project
   cp -r pkg /path/to/your/project/
   ```

3. **Import and use**:
   ```typescript
   import init, { smf_to_ym2151_json } from './pkg/smf_to_ym2151log.js';
   
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

The CI workflow builds the WASM package and copies it to demo-library/pkg/ before building this demo.

## Related Projects

- [web-ym2151](https://github.com/cat2151/web-ym2151) - Web-based YM2151 editor
- [cat-play-mml](https://github.com/cat2151/cat-play-mml) - MML player

## License

MIT - Same as the main project

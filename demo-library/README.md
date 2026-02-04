# Demo Library

This directory contains a minimal demonstration of how to use `smf-to-ym2151log-rust` as a library in a web application.

## Purpose

This demo is designed to:
1. Show the minimal code required to integrate `smf-to-ym2151log-rust` as a library
2. Serve as a reference implementation for projects like `web-ym2151`
3. Demonstrate proper WASM initialization and usage patterns

## Key Differences from Main Demo

The main demo (`index.html` at root) is a full-featured application with:
- MML input support (optional)
- Tab-based UI
- Multiple file format support
- Extensive error handling

This library demo focuses on:
- **Minimal code**: Only essential library usage
- **Clear structure**: Easy to understand and copy
- **Single purpose**: MIDI to YM2151 conversion only

## File Structure

```
demo-library/
├── index.html         # Minimal HTML page
├── library-demo.ts    # TypeScript code showing library usage
├── style.css          # Simple styling
└── README.md          # This file
```

## Code Example

The core library usage pattern demonstrated here:

```typescript
// 1. Import the WASM module
import init, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';

// 2. Initialize WASM
await init();

// 3. Use the library function
const midiBytes = new Uint8Array(arrayBuffer);
const result = smf_to_ym2151_json(midiBytes);

// 4. Parse and use the result
const json = JSON.parse(result);
console.log(`Generated ${json.event_count} YM2151 events`);
```

## Building

This demo is built together with the main demo:

```bash
# Build WASM module first
wasm-pack build --target web --features wasm

# Install dependencies and build
npm install
npm run build
```

The build output will be in `dist/demo-library/`.

## Development

To develop and test locally:

```bash
# Build WASM
wasm-pack build --target web --features wasm

# Start development server
npm run dev
```

Then navigate to `http://localhost:8000/demo-library/`

## Deployment

This demo is automatically deployed to GitHub Pages as part of the main CI/CD workflow. It will be available at:

`https://cat2151.github.io/smf-to-ym2151log-rust/demo-library/`

## Using in Your Project

To integrate `smf-to-ym2151log-rust` in your own project:

1. **Add as dependency** (for Rust projects):
   ```toml
   [dependencies]
   smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
   ```

2. **Use WASM in web projects**:
   - Build the WASM package: `wasm-pack build --target web --features wasm`
   - Copy `pkg/` directory to your project
   - Follow the pattern shown in `library-demo.ts`

## Related Projects

- [web-ym2151](https://github.com/cat2151/web-ym2151) - Web-based YM2151 editor (intended to use this library)
- [cat-play-mml](https://github.com/cat2151/cat-play-mml) - MML player using this library

## License

MIT - Same as the main project

# MML Input Support via mmlabc-to-smf-rust Integration

This document explains how to use MML (Music Macro Language) input in the demo by integrating with the [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust) library.

## Architecture

The integration uses a two-WASM-module architecture:

```
MML Text Input
     ↓
JavaScript + web-tree-sitter
     ↓
Parse Tree JSON
     ↓
mmlabc-to-smf-wasm (External)
     ↓
SMF Binary
     ↓
smf-to-ym2151log WASM (This Project)
     ↓
YM2151 Register Log JSON
```

## Why This Approach?

The mmlabc-to-smf-rust library uses tree-sitter for MML parsing, which requires C compilation. However, it provides a WASM-compatible path using:
- **web-tree-sitter**: Browser-compatible parser (no C compilation needed)
- **mmlabc-to-smf-wasm**: Separate WASM crate that receives parse trees from web-tree-sitter

This allows full MML support in the browser without any C compilation requirements.

## Setup Instructions

### 1. Clone mmlabc-to-smf-rust Repository

```bash
git clone https://github.com/cat2151/mmlabc-to-smf-rust
cd mmlabc-to-smf-rust
```

### 2. Install Dependencies

```bash
cd demo
npm install  # Installs web-tree-sitter
```

### 3. Build WASM Modules

```bash
# Build mmlabc-to-smf WASM module
cd ../mmlabc-to-smf-wasm
wasm-pack build --target web

# Build tree-sitter grammar for WASM
cd ../tree-sitter-mml
npm install  # If tree-sitter-cli not already installed
npx tree-sitter build-wasm
```

### 4. Copy Required Files

Copy the following files to your smf-to-ym2151log-rust demo directory:

```bash
# From mmlabc-to-smf-rust repository root:
cd /path/to/mmlabc-to-smf-rust

# Copy mmlabc WASM module
cp -r mmlabc-to-smf-wasm/pkg /path/to/smf-to-ym2151log-rust/mmlabc-pkg

# Copy tree-sitter grammar WASM
cp tree-sitter-mml/tree-sitter-mml.wasm /path/to/smf-to-ym2151log-rust/

# Copy web-tree-sitter runtime
cp demo/node_modules/web-tree-sitter/web-tree-sitter.js /path/to/smf-to-ym2151log-rust/
cp demo/node_modules/web-tree-sitter/web-tree-sitter.wasm /path/to/smf-to-ym2151log-rust/
```

### 5. Build This Project's WASM Module

```bash
cd /path/to/smf-to-ym2151log-rust
wasm-pack build --target web --features wasm
```

### 6. Run the Demo

```bash
python3 -m http.server 8000
```

Then open http://localhost:8000/index-mml.html in your browser.

## Using the Demo

### MIDI File Tab
- Click "MIDI File" tab
- Select a .mid file
- View the YM2151 register log output

### MML Input Tab
- Click "MML Input" tab
- Enter MML code or click an example button
- Click "Convert to YM2151"
- View the YM2151 register log output

## MML Syntax Examples

- `cdefgab` - Simple melody
- `o5 l4 cdefgab` - With octave and length settings
- `c;e;g` - C major chord (multi-channel)
- `o4 c c g g a a g2 f f e e d d c2` - Twinkle Twinkle Little Star

For full MML syntax, see the [mmlabc-to-smf-rust grammar](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/tree-sitter-mml/grammar.js).

## Implementation Notes

### Why Not a Single Dependency?

The mmlabc-to-smf library is split into:
1. **Core library**: Converts tokens/AST/events to SMF (pure Rust, WASM-compatible)
2. **CLI with tree-sitter**: Native parser for command-line use
3. **WASM crate**: Browser interface using web-tree-sitter

We cannot directly depend on mmlabc-to-smf in our Cargo.toml for WASM because:
- The tree-sitter parsing happens in JavaScript (web-tree-sitter)
- The WASM modules communicate via JavaScript glue code
- Each WASM module is compiled separately and loaded independently

### File Structure

After setup, your directory should look like:

```
smf-to-ym2151log-rust/
├── index.html                          # Original MIDI-only demo
├── index-mml.html                      # MML-enabled demo
├── pkg/                                # This project's WASM (from wasm-pack)
│   ├── smf_to_ym2151log.js
│   └── smf_to_ym2151log_bg.wasm
├── mmlabc-pkg/                         # mmlabc WASM (copied from mmlabc-to-smf-rust)
│   ├── mmlabc_to_smf_wasm.js
│   └── mmlabc_to_smf_wasm_bg.wasm
├── tree-sitter-mml.wasm                # Tree-sitter grammar (copied)
├── web-tree-sitter.js                  # web-tree-sitter runtime (copied)
└── web-tree-sitter.wasm                # web-tree-sitter runtime (copied)
```

## Troubleshooting

### "MML WASM module not available"
- Ensure you've copied all required files from mmlabc-to-smf-rust
- Check browser console for detailed error messages
- Verify file paths in index-mml.html match your setup

### "Failed to initialize WASM"
- Ensure you've run `wasm-pack build --target web --features wasm`
- Check that pkg/ directory exists with the WASM files
- Verify you're serving files via HTTP server (not file://)

### CORS Errors
- Use a local HTTP server (python3 -m http.server)
- Don't open HTML files directly (file:// protocol)

## Reference

- [mmlabc-to-smf-rust Repository](https://github.com/cat2151/mmlabc-to-smf-rust)
- [mmlabc-to-smf-rust Demo README](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/demo/README.md)
- [web-tree-sitter Documentation](https://github.com/tree-sitter/tree-sitter/tree/master/lib/binding_web)

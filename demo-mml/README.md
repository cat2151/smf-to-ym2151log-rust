# MML Demo

This demo showcases MML (Music Macro Language) to YM2151 conversion.

## Setup Requirements

This demo requires the `mmlabc-to-smf-wasm` module from the [mmlabc-to-smf-rust](https://github.com/cat2151/mmlabc-to-smf-rust) repository.

### Setup Steps

1. Clone mmlabc-to-smf-rust repository
2. Build WASM module: `cd mmlabc-to-smf-wasm && wasm-pack build --target web`
3. Build tree-sitter grammar: `cd tree-sitter-mml && npx tree-sitter build-wasm`
4. Copy the following files to this demo directory:
   - `mmlabc-to-smf-wasm/pkg/` → `./mmlabc-pkg/`
   - `tree-sitter-mml/tree-sitter-mml.wasm` → `./`
   - `node_modules/web-tree-sitter/*.{js,wasm}` → `./`

For detailed instructions, see:
- [demo/README.md](https://github.com/cat2151/mmlabc-to-smf-rust/blob/main/demo/README.md) in mmlabc-to-smf-rust
- [MML_INTEGRATION.md](../MML_INTEGRATION.md) in this repository

## Development

```bash
# Install dependencies
npm install

# Run development server
npm run dev

# Build for production
npm run build
```

## Notes

- MML support is an **advanced feature** requiring external dependencies
- The main MIDI demo does not require these dependencies
- See the main demo at `../` for basic MIDI file conversion

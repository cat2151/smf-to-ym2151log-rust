# Demo Development Guide

This directory contains a web-based demo built with **Vite** and **TypeScript**.

## Quick Start

```bash
# Install dependencies
npm install

# Build WASM module
wasm-pack build --target web --features wasm

# Start development server
npm run dev
```

Open http://localhost:8000 in your browser.

## Project Structure

```
├── index.html              # Main HTML file (clean, no inline JS/CSS)
├── src/
│   ├── main.ts            # TypeScript entry point
│   └── style.css          # CSS styles
├── package.json           # Dependencies and scripts
├── tsconfig.json          # TypeScript configuration
├── vite.config.ts         # Vite build configuration
├── dist/                  # Production build output (generated)
└── pkg/                   # WASM build output (generated)
```

## Available Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Start development server with hot reload |
| `npm run build` | Build for production (outputs to `dist/`) |
| `npm run preview` | Preview production build locally |

## Development Workflow

1. **Edit Files**: Modify `src/main.ts` or `src/style.css`
2. **Auto-Reload**: Vite automatically reloads the page
3. **Type Checking**: TypeScript catches errors during development
4. **Build**: Run `npm run build` to create optimized production files

## Why This Architecture?

### Before (Monolithic)
- 401 lines of HTML with embedded CSS and JavaScript
- Hard to maintain and test
- No type safety
- Manual dependency management

### After (Vite + TypeScript)
- **Separation of Concerns**: HTML, CSS, TypeScript in separate files
- **Type Safety**: Catch errors at development time
- **Fast Development**: Instant hot module replacement
- **Modern Tooling**: ES modules, tree-shaking, minification
- **Maintainability**: Easier to modify, test, and debug

## Adding Features

### Adding New UI Elements

1. Edit `index.html` to add HTML structure
2. Edit `src/style.css` to add styles
3. Edit `src/main.ts` to add event handlers and logic

### Adding New Dependencies

```bash
npm install <package-name>
```

Then import in `src/main.ts`:

```typescript
import somePackage from 'some-package';
```

## Building for Production

```bash
# Build optimized files
npm run build

# Output is in dist/ directory
ls dist/

# Preview the build
npm run preview

# Or serve with any static file server
python3 -m http.server 8000 --directory dist
```

## Troubleshooting

### TypeScript Errors

Check `tsconfig.json` settings. The `@ts-expect-error` comment is used for WASM imports since type definitions aren't available.

### WASM Not Loading

1. Ensure `wasm-pack build --target web --features wasm` was run
2. Check that `pkg/` directory exists
3. Verify you're using a development server (not file://)

### Vite Build Issues

```bash
# Clear caches and reinstall
rm -rf node_modules dist
npm install
npm run build
```

## MML Support

To enable MML input (optional), see [MML_INTEGRATION.md](../MML_INTEGRATION.md) for setup instructions.

## Legacy Files

- `index-simple.html`: Original MIDI-only demo
- `index-mml-monolith.html`: Monolithic version with inline JS/CSS

These are kept for reference but not actively maintained.

# Waveform Rendering Implementation

This document describes the implementation of waveform rendering in the main demo using the web-ym2151 library and an in-app canvas renderer.

## Overview

The main demo now supports:
1. Converting MIDI files to YM2151 register logs (existing functionality)
2. Rendering the YM2151 audio in real-time using web-ym2151
3. Visualizing the waveform on a canvas with orange color on black background

## Implementation Details

### Library Dependencies

The implementation uses one external library:

1. **web-ym2151** (https://github.com/cat2151/web-ym2151)
   - Purpose: YM2151 FM synthesizer emulation
   - Generates audio from YM2151 register events
   - Provides WASM-based audio synthesis at 55930Hz sample rate

### Setup Process

Libraries are downloaded using `setup-libs.js`:

```bash
npm run setup-libs
```

This script:
- Downloads web-ym2151 WASM files from GitHub Pages
- Is cross-platform compatible (Windows/macOS/Linux)

### Integration Architecture

```
MIDI File
   ↓
smf-to-ym2151log WASM (Rust)
   ↓
YM2151 JSON Events
   ↓
web-ym2151 WASM (Emscripten)
   ↓
Audio Data (Float32Array)
Canvas Visualization (Orange on Black)
```

### Key Files

- `setup-libs.js` - Library download script
- `src/main.ts` - Main integration code
- `index.html` - Canvas element and play button
- `src/style.css` - Waveform section styling
- `.github/workflows/deploy-pages.yml` - Deployment configuration

### User Flow

1. User selects a MIDI file (.mid)
2. File is converted to YM2151 JSON (existing functionality)
3. Waveform section appears with a "Play Audio" button
4. User clicks "Play Audio"
5. Audio is generated from YM2151 events
6. Audio plays through Web Audio API
7. Waveform is displayed on canvas in real-time

### Visual Design

The waveform section features:
- Black background (#000000)
- Orange waveform (#ff8c00)
- Orange play button matching the waveform color
- Rounded corners and modern styling

### Technical Considerations

#### Memory Management
- WASM memory is properly allocated and freed for each audio generation
- Event data is written to WASM memory in C-compatible struct format
- Audio buffers are freed after extraction

#### Event Format
The implementation handles both formats:
- Hex strings: `{ "addr": "0x08", "data": "0x00" }`
- Numbers: `{ "addr": 8, "data": 0 }`

#### Canvas Setup
- Main canvas: 800x200px for waveform display
- Auto-scaling: Canvas width set to 100% for responsiveness

#### Performance
- Module initialization: ~20ms polling interval for fast detection
- Audio generation: On-demand when user clicks play
- Waveform rendering: Real-time using WebGL acceleration

## Build Process

### Development
```bash
npm install
npm run setup-libs
npm run dev
```

### Production
```bash
npm install
npm run setup-libs
wasm-pack build --target web --features wasm
npm run build
```

### Deployment
GitHub Actions workflow automatically:
1. Builds Rust WASM module
2. Downloads library files via setup-libs.js
3. Builds the main demo
4. Deploys to GitHub Pages

## Browser Compatibility

- Chrome/Edge: Full support
- Firefox: Full support
- Safari: Full support (requires Web Audio API)
- Mobile: Supported but may have audio playback restrictions

## Future Enhancements

Potential improvements:
- WAV export functionality
- Adjustable playback speed
- Multiple waveform color themes
- Zoom and pan controls
- Frequency spectrum display

## Version Pinning Policy

Following the cat2151 repository policy:
- **DO NOT pin versions** of cat2151 repositories
- Always use latest from default branch
- Daily bug fixes must be incorporated
- Setup script clones latest version automatically

## Troubleshooting

### Libraries not loading
- Run `npm run setup-libs` to download libraries
- Check that `public/libs/` directory exists
- Verify internet connection for downloading

### Audio not playing
- Check browser console for errors
- Verify Web Audio API is supported
- Check that WASM modules initialized successfully

### Waveform not displaying
- Ensure canvas element is present in DOM
- Check that oscilloscope initialized without errors
- Verify audio data is being generated

## Credits

- web-ym2151: YM2151 emulation and audio synthesis
- Both libraries by [@cat2151](https://github.com/cat2151)

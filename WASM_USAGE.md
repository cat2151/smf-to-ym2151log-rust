# WASM Usage Guide

This guide explains how to use smf-to-ym2151log-rust in a web browser via WebAssembly.

## Building the WASM Package

### Prerequisites

Install wasm-pack:

```bash
cargo install wasm-pack
```

### Build

Build the WASM package with the `wasm` feature:

```bash
wasm-pack build --target web --features wasm
```

This will create a `pkg` directory containing:
- `smf_to_ym2151log.js` - JavaScript wrapper
- `smf_to_ym2151log_bg.wasm` - WebAssembly binary
- `smf_to_ym2151log.d.ts` - TypeScript definitions
- `package.json` - Package metadata

## Usage in Browser

### Basic Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>SMF/MML to YM2151 Converter</title>
</head>
<body>
    <h2>MIDI File Input</h2>
    <input type="file" id="midi-file" accept=".mid,.midi">
    
    <h2>MML Input</h2>
    <textarea id="mml-input" placeholder="Enter MML code, e.g., cdefgab"></textarea>
    <button id="convert-mml">Convert MML</button>
    
    <h2>Output</h2>
    <pre id="output"></pre>

    <script type="module">
        import init, { smf_to_ym2151_json, mml_to_ym2151_json } from './pkg/smf_to_ym2151log.js';

        // Initialize WASM module
        await init();

        // Handle MIDI file selection
        document.getElementById('midi-file').addEventListener('change', async (e) => {
            const file = e.target.files[0];
            if (!file) return;

            // Read file as array buffer
            const buffer = await file.arrayBuffer();
            const bytes = new Uint8Array(buffer);

            // Convert to YM2151 JSON
            const result = smf_to_ym2151_json(bytes);
            
            // Display result
            document.getElementById('output').textContent = result;
        });

        // Handle MML conversion
        document.getElementById('convert-mml').addEventListener('click', () => {
            const mml = document.getElementById('mml-input').value;
            if (!mml) return;

            // Convert MML to YM2151 JSON
            const result = mml_to_ym2151_json(mml);
            
            // Display result
            document.getElementById('output').textContent = result;
        });
    </script>
</body>
</html>
```

### TypeScript Example

```typescript
import init, { smf_to_ym2151_json, mml_to_ym2151_json } from './pkg/smf_to_ym2151log';

async function convertMidiFile(file: File): Promise<string> {
    // Initialize WASM (only needed once)
    await init();

    // Read file as bytes
    const buffer = await file.arrayBuffer();
    const bytes = new Uint8Array(buffer);

    // Convert SMF to YM2151 JSON
    const jsonResult = smf_to_ym2151_json(bytes);
    
    return jsonResult;
}

async function convertMML(mml: string): Promise<string> {
    // Initialize WASM (only needed once)
    await init();

    // Convert MML to YM2151 JSON
    const jsonResult = mml_to_ym2151_json(mml);
    
    return jsonResult;
}

// Error handling for MIDI file
async function safeConvertMidi(file: File): Promise<object> {
    try {
        const jsonString = await convertMidiFile(file);
        const result = JSON.parse(jsonString);
        
        if (result.error) {
            console.error('Conversion error:', result.error);
            return { error: result.error };
        }
        
        return result;
    } catch (error) {
        console.error('Failed to convert:', error);
        return { error: error.message };
    }
}

// Error handling for MML
async function safeConvertMML(mml: string): Promise<object> {
    try {
        const jsonString = await convertMML(mml);
        const result = JSON.parse(jsonString);
        
        if (result.error) {
            console.error('Conversion error:', result.error);
            return { error: result.error };
        }
        
        return result;
    } catch (error) {
        console.error('Failed to convert:', error);
        return { error: error.message };
    }
}
```

## API Reference

### `smf_to_ym2151_json(smf_data: Uint8Array): string`

Converts Standard MIDI File binary data to YM2151 register log JSON.

**Parameters:**
- `smf_data`: Uint8Array - The binary content of a MIDI file

**Returns:**
- String - JSON string containing YM2151 register log

**Output Format (Success):**
```json
{
  "event_count": 100,
  "events": [
    {
      "time": 0.0,
      "addr": "0x01",
      "data": "0x00"
    },
    ...
  ]
}
```

**Output Format (Error):**
```json
{
  "error": "Error message describing what went wrong"
}
```

### `mml_to_ym2151_json(mml: string): string`

Converts MML (Music Macro Language) string to YM2151 register log JSON.

**Parameters:**
- `mml`: string - MML code (e.g., "cdefgab", "o5 l4 c;e;g")

**Returns:**
- String - JSON string containing YM2151 register log

**Supported MML Commands:**
- `c d e f g a b` - Notes (case-insensitive)
- `#` or `+` - Sharp (e.g., `c#`, `f+`)
- `-` - Flat (e.g., `b-`, `e-`)
- `o<n>` - Set octave (e.g., `o4`, `o5`)
- `>` - Increase octave by 1
- `<` - Decrease octave by 1
- `l<n>` - Set default note length (e.g., `l4` for quarter note, `l8` for eighth)
- `<note><n>` - Note-specific length (e.g., `c4`, `d8`, `e2`)
- `v<n>` - Set volume/velocity (0-15, where 15 is loudest)
- `r` - Rest
- `;` - Channel separator (for multi-channel/chords)

**MML Examples:**
```javascript
// Simple melody
mml_to_ym2151_json("cdefgab");

// With octave and length
mml_to_ym2151_json("o5 l4 cdefgab");

// C major chord (multi-channel)
mml_to_ym2151_json("c;e;g");

// Twinkle Twinkle Little Star
mml_to_ym2151_json("o4 c c g g a a g2 f f e e d d c2");

// With dynamics (volume changes)
mml_to_ym2151_json("o5 l8 v15 c d e f g v10 g g v8 a a v5 g");
```

**Output Format:** Same as `smf_to_ym2151_json` (success or error)

## Output Format

The YM2151 register log JSON has the following structure:

- `event_count`: Number of register write events
- `events`: Array of register write events
  - `time`: Time in seconds (f64)
  - `addr`: YM2151 register address (hex string, e.g., "0x08")
  - `data`: Data to write to register (hex string, e.g., "0x7F")

This format is compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc).

## Demo

A complete working demo with both MIDI file upload and MML textarea input is available in `index.html`.

### Online Demo

The demo is automatically deployed to GitHub Pages at:
https://cat2151.github.io/smf-to-ym2151log-rust/

**Features:**
- **MIDI File Tab**: Upload .mid files for conversion
- **MML Input Tab**: Enter MML code directly in the browser with clickable examples
- Supports both light and dark mode
- Real-time conversion in the browser (no server required)
- Example MML snippets included (simple melodies, chords, Twinkle Twinkle, etc.)

The GitHub Actions workflow (`.github/workflows/deploy-pages.yml`) automatically:
1. Builds the WASM package using wasm-pack
2. Deploys the demo page with the WASM module to GitHub Pages

This runs automatically on every push to the `main` branch.

### Local Demo

To run the demo locally:

1. Build the WASM package (see above)
2. Serve the directory with a local web server:
   ```bash
   # Using Python
   python3 -m http.server
   
   # Using Node.js
   npx http-server
   ```
3. Open http://localhost:8000/ in your browser
4. Try either:
   - **MIDI File tab**: Select a MIDI file to see the conversion result
   - **MML Input tab**: Enter MML code or click an example to convert

## Browser Compatibility

The WASM module requires:
- ES6 module support
- WebAssembly support
- File API support

All modern browsers (Chrome, Firefox, Safari, Edge) support these features.

## Performance

- Conversion is performed entirely in the browser
- No server round-trip required
- Performance is comparable to native Rust execution
- Large MIDI files (>1MB) may take a few seconds to process

## Troubleshooting

### "Failed to initialize WASM"

Make sure the WASM file is accessible and served with the correct MIME type:
- `application/wasm` for `.wasm` files
- Most static file servers handle this automatically

### "Failed to fetch .wasm file"

Ensure:
1. The `pkg` directory is in the correct location relative to your HTML file
2. You're serving the files from a web server (not opening HTML file directly)
3. CORS is configured correctly if loading from a different origin

### Import errors

Make sure you're using the correct import path:
```javascript
// Correct - relative to your HTML file
import init, { smf_to_ym2151_json } from './pkg/smf_to_ym2151log.js';
```

## Integration with Build Tools

### Vite

```javascript
// vite.config.js
export default {
  plugins: [
    // Add wasm plugin if needed
  ]
}
```

### Webpack

```javascript
// webpack.config.js
module.exports = {
  experiments: {
    asyncWebAssembly: true
  }
}
```

## License

This WASM interface is part of smf-to-ym2151log-rust and is licensed under the MIT License.

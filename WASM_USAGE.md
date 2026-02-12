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
    <title>SMF to YM2151 Converter</title>
</head>
<body>
    <input type="file" id="midi-file" accept=".mid,.midi">
    <pre id="output"></pre>

    <script type="module">
        import init, { smf_to_ym2151_json } from './pkg/smf_to_ym2151log.js';

        // Initialize WASM module
        await init();

        // Handle file selection
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
    </script>
</body>
</html>
```

### TypeScript Example

```typescript
import init, { smf_to_ym2151_json } from './pkg/smf_to_ym2151log';

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

// Error handling
async function safeConvert(file: File): Promise<object> {
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

### `smf_to_ym2151_json_with_attachment(smf_data: Uint8Array, attachment_json: Uint8Array): string`

Converts SMF data while honoring an optional attachment JSON payload (e.g., enabling delayed vibrato or overriding tones).

**Parameters:**
- `smf_data`: Uint8Array - The binary content of a MIDI file
- `attachment_json`: Uint8Array - Attachment JSON bytes. Pass an empty array when no attachment is provided.

**Example attachments:**

```json
{
  "DelayVibrato": true
}
```

```json
{
  "Tones": {
    "0": {
      "events": [
        { "time": 0, "addr": "0x20", "data": "0xC7" },
        { "time": 0, "addr": "0x60", "data": "0x10" }
      ]
    }
  }
}
```

## Output Format

The YM2151 register log JSON has the following structure:

- `event_count`: Number of register write events
- `events`: Array of register write events
  - `time`: Time in seconds (f64)
  - `addr`: YM2151 register address (hex string, e.g., "0x08")
  - `data`: Data to write to register (hex string, e.g., "0x7F")

This format is compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc).

## Demo

A complete working demo is available in `index.html`.

### Online Demo

The demo is automatically deployed to GitHub Pages at:
https://cat2151.github.io/smf-to-ym2151log-rust/

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
4. Select a MIDI file to see the conversion result

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

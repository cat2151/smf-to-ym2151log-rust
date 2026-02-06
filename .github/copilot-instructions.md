# GitHub Copilot Instructions for smf-to-ym2151log-rust

## Project Overview

This is a Rust implementation that converts Standard MIDI Files (SMF) to YM2151 FM synthesizer chip register write logs in JSON format. It's a Rust port of the Python version [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log).

**Main Purpose**: Convert MIDI files (.mid) → YM2151 register logs (JSON) compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc)

## Architecture

### 2-Pass Processing System

The project uses a two-pass architecture:

1. **Pass A (MIDI Parser)**: MIDI File → Intermediate Events JSON (for debugging)
   - Parses SMF Format 0 and Format 1
   - Outputs `<filename>_events.json` with normalized MIDI events
   
2. **Pass B (YM2151 Converter)**: Intermediate Events → YM2151 Register Log JSON (final output)
   - Converts MIDI events to YM2151 register writes
   - Outputs `<filename>_ym2151.json` compatible with ym2151-zig-cc

### Key Modules

- `src/midi/` - MIDI file parsing and event processing
  - `parser.rs` - SMF parsing logic
  - `events.rs` - MIDI event types
  - `utils.rs` - Utility functions (tempo conversion, note mapping)
  
- `src/ym2151/` - YM2151 conversion logic
  - `converter.rs` - Main conversion logic
  - `events.rs` - YM2151 event types
  - `init.rs` - YM2151 initialization sequences
  - `note_table.rs` - MIDI note to YM2151 KC/KF conversion

- `src/error.rs` - Error types using thiserror
- `src/lib.rs` - Library root
- `src/main.rs` - CLI entry point

## Build and Test

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>
```

**Test Structure**:
- Unit tests: Located in the same file as the code (`#[cfg(test)]` modules)
- Integration tests: `tests/integration_tests.rs`
- Test data: `tests/test_data/`

### Code Quality
```bash
# Format check (must pass before commit)
cargo fmt --check

# Lint check (must pass before commit)
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## Dependencies

### Production Dependencies
- `midly` (0.5) - SMF parsing library
- `serde` + `serde_json` - JSON serialization/deserialization
- `anyhow` - Error handling for application code
- `thiserror` - Error type definitions

### Development
- Standard Rust test framework (no additional test dependencies yet)

## Coding Conventions

### Language
- **Rust Edition 2021**
- Minimum Rust version: 1.70.0

### Style
- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` for linting (no warnings allowed in CI)
- Prefer explicit types over inference when it improves readability
- Use descriptive variable names
- **Comments**: English preferred for code comments and documentation to support international collaboration; Japanese is acceptable for domain-specific terms or in bilingual documentation files

### Error Handling
- Use `anyhow::Result` for application-level errors in binaries
- Use `thiserror` to define custom error types in libraries
- Propagate errors with `?` operator
- Avoid unwrap/expect in production code (ok in tests)

### Testing Guidelines
- Write unit tests for pure functions and algorithms
- Write integration tests for end-to-end workflows
- Use descriptive test names (e.g., `test_parse_simple_melody`)
- Test both success and error cases
- Keep test data files small and focused

### Documentation
- Document public APIs with doc comments (`///`)
- Include examples in doc comments where helpful
- Keep README.md and IMPLEMENTATION.md in sync with code changes

## JSON Output Formats

### Events JSON (_events.json)
Intermediate debug format:
```json
{
  "ticks_per_beat": 480,
  "tempo_bpm": 120.0,
  "events": [
    {
      "type": "note_on",
      "ticks": 0,
      "channel": 0,
      "note": 60,
      "velocity": 100
    }
  ]
}
```

### YM2151 Log JSON (_ym2151.json)
Final output format (must be compatible with ym2151-zig-cc):
```json
{
  "event_count": 50,
  "events": [
    {
      "time": 0,
      "addr": "0x08",
      "data": "0x00"
    }
  ]
}
```
- `time`: Sample time at 55930Hz sample rate
- `addr`: YM2151 register address (hex string)
- `data`: Data to write (hex string)

## Important References

- [YM2151 Datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf) - Official chip specification (Note: HTTP link, no HTTPS version available)
- [Python version](https://github.com/cat2151/smf-to-ym2151log) - Reference implementation
- [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc) - Output format specification

## Common Tasks

### Adding New MIDI Event Support
1. Add event type to `src/midi/events.rs`
2. Update parser in `src/midi/parser.rs`
3. Add conversion logic in `src/ym2151/converter.rs`
4. Add tests in `tests/integration_tests.rs`

### Modifying YM2151 Register Logic
1. Check YM2151 datasheet for register specifications
2. Update conversion logic in `src/ym2151/converter.rs`
3. If needed, update note table in `src/ym2151/note_table.rs`
4. Add tests to verify register values

### Adding CLI Options
1. Update `src/main.rs` argument parsing
2. Update README.md usage section
3. Add integration tests for new options

## CI/CD

The project uses GitHub Actions (`.github/workflows/ci.yml`):
- Runs on push and pull requests
- Executes: build, test, clippy, and fmt checks
- All checks must pass before merging

### Demo Deployment

The project deploys three demos to GitHub Pages via `.github/workflows/deploy-pages.yml`:
1. **Main MIDI Demo** (`/`) - Basic MIDI file to YM2151 conversion
2. **Library Demo** (`/demo-library/`) - Demonstrates library usage patterns
3. **MML Demo** (`/demo-mml/`) - Advanced MML to YM2151 conversion (requires external dependencies)

**CRITICAL: Demo Verification Requirements**

When making changes to demos or deployment:
1. **ALWAYS verify demos work after deployment** using a headless browser tool (e.g., Playwright, Puppeteer)
2. **Check for JavaScript errors** in browser console - errors are NOT optional or acceptable
3. **Test all demo functionality** - file uploads, conversions, UI interactions
4. **Verify external dependencies** - check if library versions or installation procedures have changed
5. **Validate deployment procedures** - ensure build steps, file copies, and deployment workflow are correct

**MML Demo Considerations**
- MML support requires external `mmlabc-to-smf-wasm` module
- MML errors are **critical issues** that must be addressed during development
- Agent is responsible for checking deployment procedures and library compatibility
- Agent must verify MML dependencies are correctly configured or appropriately documented

**Verification Script Example**
```javascript
// Example headless browser verification
const page = await browser.newPage();
await page.goto('https://cat2151.github.io/smf-to-ym2151log-rust/');
const errors = [];
page.on('console', msg => {
  if (msg.type() === 'error') errors.push(msg.text());
});
// Verify no errors after page load
if (errors.length > 0) throw new Error(`Demo has errors: ${errors}`);
```

## Project Status

This project is in active development. The implementation is functional but may not support all MIDI features yet. Check IMPLEMENTATION.md for detailed implementation progress and planned features.

# smf-to-ym2151log-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
  <a href="https://cat2151.github.io/smf-to-ym2151log-rust/"><img src="https://img.shields.io/badge/🚀-Live%20Demo-brightgreen.svg" alt="Live Demo"></a>
  <a href="https://cat2151.github.io/smf-to-ym2151log-rust/demo-library/"><img src="https://img.shields.io/badge/📦-Library%20Demo-blue.svg" alt="Library Demo"></a>
</p>

**Rust implementation for converting Standard MIDI Files (SMF) into YM2151 register write logs (JSON format)**

## WIP

Currently, it can only convert basic musical notes (do-re-mi) into a minimal JSON format.

More advanced features are planned for future implementations.

Frequent breaking changes are expected.

## Purpose

-   As a library, it is used by `cat-play-mml`. This is a Rust library crate for native applications (Usage Type 1).
-   As a library, it is planned for use by `web-ym2151`. This will be a WASM library for browsers (Usage Type 2).

### Current Limitations

#### Channel Allocation Strategy

The current implementation uses a **chord-count based static channel allocation strategy** and **drum channel prioritization**:

**1. Chord Count Analysis Phase**:
Before conversion, the MIDI file is analyzed to measure the maximum polyphony (number of simultaneous notes) for each MIDI channel by tracking overlapping note events.

**2. Static Allocation Based on Chord Count**:
YM2151 channels (0-7, 8 channels in total) are allocated based on the polyphony requirements of each MIDI channel.
- MIDI channels with higher polyphony acquire multiple YM2151 channels.
- Example: If MIDI ch0 requires 3 voices and MIDI ch1 requires 1 voice:
  - MIDI ch0 gets YM2151 ch0, ch1, ch2 (3 channels)
  - MIDI ch1 gets YM2151 ch3 (1 channel)
  - YM2151 ch4-ch7 remain available

**3. Drum Channel Prioritization Reordering**:
After the initial allocation, if MIDI channel 9 (General MIDI drum channel) is present, the allocation is reordered:
- MIDI channel 9 is prioritized to use YM2151 channel 0.
- Other channel allocations are swapped accordingly.
- **Reason**: Drums often have multiple note-on events on the same tick. YM2151 processes channels sequentially and requires a defined register access cycle. Assigning drums to channel 0 allows them to sound first, improving sound quality.

**Voice Management**:
- If a MIDI channel is assigned multiple YM2151 channels (polyphony > 1), notes are distributed in a round-robin fashion.
- Each note-on uses the next available voice within its allocation.
- Note-off events properly track which voice played which note.

**Limitations**:
- A total of 8 YM2151 channels are available.
- If the total polyphony of all MIDI channels exceeds 8, overflowed notes will use the last allocated channel.
- No dynamic voice stealing during playback (all allocations are static/pre-determined).

**Out of Scope**:
- Dynamic channel allocation during playback
- Voice stealing algorithms
- Real-time polyphony adjustment

These features are intentionally omitted to maintain simplicity and align with the project's goals.

## Overview

This is the Rust implementation of [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log).
It converts Standard MIDI Files (SMF) into YM2151 FM sound chip register write logs (JSON format).

## Features

-   **Two-Pass Processing Architecture**:
    -   **Pass A**: MIDI file → Intermediate event JSON (for debugging)
    -   **Pass B**: Intermediate events → YM2151 register log JSON (final output)
-   **Program Change Support**: Loads custom YM2151 tones from external JSON files (MIDI programs 0-127).
-   **WebAssembly (WASM) Compatibility**: Can run in web browsers via WASM (see [WASM_USAGE.md](WASM_USAGE.md)).
-   **Type Safety**: Robustness ensured by Rust's type system.
-   **High Performance**: Fast processing due to native compilation.
-   **Test-Driven Development**: Comprehensive unit and integration tests (73 tests).
-   **Standard Compliance**: Supports SMF Format 0 and Format 1.
-   **Library API**: Convenient API available for use in other Rust projects.

## Usage

### Installation

```bash
# Clone the repository
git clone https://github.com/cat2151/smf-to-ym2151log-rust.git
cd smf-to-ym2151log-rust

# Build and install
cargo install --path .
```

### Command-line Usage

```bash
# Convert a MIDI file
smf-to-ym2151log-rust song.mid

# Output files:
# - song_events.json  (Pass A: intermediate events for debugging)
# - song_ym2151.json  (Pass B: YM2151 register log)
```

### Using as a Library

It can be used as a library in other Rust projects:

```toml
# Cargo.toml
[dependencies]
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
```

For detailed API documentation: `cargo doc --open`

### WebAssembly (Browser) Usage

**Online Demo**: You can try it in your browser at https://cat2151.github.io/smf-to-ym2151log-rust/

**Library Demo**: A minimal library usage example can be found at https://cat2151.github.io/smf-to-ym2151log-rust/demo-library/

To build and use in a web browser:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the WASM package
wasm-pack build --target web --features wasm
```

Refer to [WASM_USAGE.md](WASM_USAGE.md) for detailed usage instructions and examples.

A full demo is available in `index.html`, and a minimal library usage demo is available in `demo-library/`.

### Output Example

```
smf-to-ym2151log-rust
Processing: song.mid

Pass A: Parsing MIDI file...
  ✓ Successfully parsed MIDI file
  - Ticks per beat: 480
  - Initial tempo: 120.00 BPM
  - Total events: 4

Saving intermediate events JSON...
  ✓ Saved: song_events.json

Pass B: Converting to YM2151 register log...
  ✓ Successfully converted to YM2151 log
  - Total YM2151 events: 42

Saving YM2151 log JSON...
  ✓ Saved: song_ym2151.json

=== CONVERSION COMPLETE ===
```

## Program Change Support

The converter supports tone switching via MIDI Program Change events (0-127). When a Program Change event is detected, the converter performs the following actions:

1.  **Searches for an external tone file** `tones/{program:03}.json` (e.g., `tones/042.json` for program 42).
2.  **Loads and applies the tone** if the file exists.
3.  **Uses the built-in default tone** if the file does not exist.

### Custom Tone Files

You can create custom YM2151 tones by placing JSON files in the `tones/` directory:

```bash
tones/
├── 000.json    # Program 0 (Acoustic Grand Piano)
├── 001.json    # Program 1 (Bright Acoustic Piano)
├── ...
└── 127.json    # Program 127 (Gunshot)
```

Each tone file defines YM2151 register writes to set FM synthesis parameters. Refer to [`tones/README.md`](tones/README.md) for detailed format documentation and examples.

### Usage Example

```bash
# 1. Create a custom tone for MIDI program 42
#    (e.g., a brass sound)
cat > tones/042.json << EOF
{
  "events": [
    { "time": 0.0, "addr": "0x20", "data": "0xC7" },
    { "time": 0.0, "addr": "0x38", "data": "0x00" },
    ...
  ]
}
EOF

# 2. Convert a MIDI file that uses program 42
smf-to-ym2151log-rust song.mid

# The converter will automatically use tones/042.json
# when program 42 is specified in a program change event.
```

## Development

### Prerequisites
- Rust 1.70.0 or later
- Cargo

### Build
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

# Run a specific test
cargo test midi_parser

# Test Coverage
cargo tarpaulin --out Html
```

### Code Quality
```bash
# Format check
cargo fmt --check

# Lint check
cargo clippy -- -D warnings

# Security check
cargo audit
```

## References

-   [Python implementation](https://github.com/cat2151/smf-to-ym2151log): The original Python implementation of this project.
-   [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): Reference for the output JSON format specification.
-   [YM2151 Data Sheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): Official specification document for the YM2151 chip.
# smf-to-ym2151log-rust

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

**Rust implementation to convert Standard MIDI Files (SMF) to YM2151 register write logs (JSON format)**

## WIP

Currently, it can only convert basic notes to a minimal JSON format.

More advanced implementations are planned for the future.

Frequent breaking changes are expected.

## Usage

It is used as a library by `cat-play-mml`.

### Current Constraints

#### Channel Assignment Strategy

The current implementation uses a **chord-count-based static channel assignment strategy** and **drum channel prioritization**:

**1. Chord Count Analysis Phase**:
Before conversion, the MIDI file is analyzed to measure the maximum chord count (polyphony) for each MIDI channel by tracking overlapping note events.

**2. Static Assignment Based on Chord Count**:
YM2151 channels (0-7, 8 channels total) are assigned based on the chord count requirements of each MIDI channel.
- MIDI channels with higher chord counts acquire multiple YM2151 channels.
- Example: If MIDI ch0 needs 3 notes and MIDI ch1 needs 1 note:
  - MIDI ch0 gets YM2151 ch0, ch1, ch2 (3 channels)
  - MIDI ch1 gets YM2151 ch3 (1 channel)
  - YM2151 ch4-ch7 remain available

**3. Drum Channel Prioritization Rearrangement**:
After initial assignment, if MIDI channel 9 (General MIDI drum channel) is present, the assignment is rearranged:
- MIDI channel 9 is prioritized to use YM2151 channel 0.
- Other channel assignments are swapped accordingly.
- **Reason**: Drums often have multiple note-on events on the same tick. YM2151 processes channels sequentially and requires a defined register access cycle, so assigning drums to channel 0 ensures they sound first, improving sound quality.

**Voice Management**:
- If a MIDI channel is assigned multiple YM2151 channels (chord count > 1), notes are distributed in a round-robin fashion.
- Each note-on uses the next available voice within its assignment.
- Note-off events properly track which voice played which note.

**Limitations**:
- A total of 8 YM2151 channels are available.
- If the total chord count of all MIDI channels exceeds 8, overflowed notes will use the last assigned channel.
- No dynamic voice stealing during playback (all assignments are static/pre-determined).

**Out of Scope**:
- Dynamic channel assignment during playback
- Voice stealing algorithms
- Real-time chord count adjustment

These features are intentionally omitted to maintain simplicity and align with the project's goals.

## Overview

This is a Rust implementation of [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log).
It converts Standard MIDI Files (SMF) into register write logs (JSON format) for the YM2151 FM sound chip.

## Features

- **Two-Pass Processing Architecture**:
  - **Pass A**: MIDI file → Intermediate Event JSON (for debugging)
  - **Pass B**: Intermediate Events → YM2151 Register Log JSON (final output)
- **Program Change Support**: Loads custom YM2151 tones from external JSON files (MIDI programs 0-127).
- **WebAssembly Support**: Can be executed in web browsers via WASM (see [WASM_USAGE.md](WASM_USAGE.md)).
- **Type Safety**: Robustness provided by Rust's type system.
- **High Performance**: Fast processing due to native compilation.
- **Test-Driven Development**: Comprehensive unit and integration tests (73 tests).
- **Standard Compliant**: Supports SMF Format 0 and Format 1.
- **Library API**: Provides a convenient API for use in other Rust projects.

## How to Use

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
# - song_events.json  (Pass A: Debugging intermediate events)
# - song_ym2151.json  (Pass B: YM2151 register log)
```

### Usage as a Library

You can use it as a library from other Rust projects:

```toml
# Cargo.toml
[dependencies]
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
```

Detailed API documentation: `cargo doc --open`

### WebAssembly (Browser) Usage

To build and use in a web browser:

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the WASM package
wasm-pack build --target web --features wasm
```

For detailed usage instructions and examples, refer to [WASM_USAGE.md](WASM_USAGE.md).

A complete demo is available in `demo.html`.

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

The converter supports tone switching via MIDI program change events (0-127). When a program change event is detected, the converter performs the following actions:

1. **Searches for an external tone file**: `tones/{program:03}.json` (e.g., `tones/042.json` for program 42).
2. **Loads and applies the tone** if the file exists.
3. **Uses the built-in default tone** if the file does not exist.

### Custom Tone Files

You can create custom YM2151 tones by placing JSON files in the `tones/` directory:

```bash
tones/
├── 000.json    # Program 0 (Acoustic Grand Piano)
├── 001.json    # Program 1 (Bright Acoustic Piano)
├── ...
└── 127.json    # Program 127 (Gunshot)
```

Each tone file defines YM2151 register writes to set FM synthesis parameters. For detailed format documentation and examples, refer to [`tones/README.md`](tones/README.md).

### Usage Example

```bash
# 1. Create a custom tone for MIDI Program 42
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

# 2. Convert a MIDI file that uses Program 42
smf-to-ym2151log-rust song.mid

# The converter will automatically use tones/042.json
# when Program 42 is specified in a program change.
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

### Test
```bash
# Run all tests
cargo test

# Run a specific test
cargo test midi_parser

# Test coverage
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

- [Python implementation](https://github.com/cat2151/smf-to-ym2151log): The original Python implementation of this project.
- [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): The source for the output JSON format specification.
- [YM2151 Datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): Official specification for the YM2151 chip.
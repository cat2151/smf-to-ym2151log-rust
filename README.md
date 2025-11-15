# smf-to-ym2151log-rust

**Rust implementation to convert Standard MIDI Files (SMF) to YM2151 register write logs (JSON format)**

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## WIP

Currently, it can only convert basic note events into a minimal JSON format.

More advanced features are planned for future implementations.

### Current Constraints

#### Channel Assignment Strategy

The current implementation uses a **static channel assignment strategy**. This strategy assigns MIDI channels to YM2151 channels using pre-analysis:

1.  **Pre-analysis Phase**: Before conversion, the SMF is analyzed to measure the maximum polyphony (number of simultaneous voices) for each MIDI channel.
2.  **Static Assignment**: YM2151 channels are assigned based on the polyphony requirements of each MIDI channel.
    -   Example: If MIDI ch0 requires 3 voices, MIDI ch1 requires 1 voice, and the remaining MIDI channels are unused:
        -   YM2151 ch0-ch2 correspond to MIDI ch0
        -   YM2151 ch3 corresponds to MIDI ch1
        -   YM2151 ch4-ch7 remain available

**Out of Scope**: Dynamic channel assignment (methods for changing MIDI-YM2151 channel assignments during playback) is not implemented. This decision aligns with the project's policy of prioritizing simplicity over complexity. Dynamic assignment would require complex voice stealing algorithms and state management, significantly increasing implementation complexity.

## Overview

This is a Rust implementation of [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log).
It converts Standard MIDI Files (SMF) into YM2151 FM sound chip register write logs (JSON format).

## Features

-   **Two-Pass Processing Architecture**:
    -   **Pass A**: MIDI File → Intermediate Event JSON (for debugging)
    -   **Pass B**: Intermediate Events → YM2151 Register Log JSON (final output)
-   **Program Change Support**: Loads custom YM2151 instrument definitions from external JSON files (MIDI Program 0-127)
-   **Type Safety**: Robustness provided by Rust's type system
-   **High Performance**: Fast processing due to native compilation
-   **Test-Driven Development**: Comprehensive unit and integration tests (73 tests)
-   **Compatibility**: JSON format compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc)
-   **Standard Support**: Supports SMF Format 0 and Format 1
-   **Library API**: Convenient API available for use in other Rust projects

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
# - song_events.json  (Pass A: Intermediate events for debugging)
# - song_ym2151.json  (Pass B: YM2151 register log)
```

### Use as a Library

It can be used as a library from other Rust projects:

```toml
# Cargo.toml
[dependencies]
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
```

Detailed API documentation: `cargo doc --open`

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

The converter supports instrument (patch) changes via MIDI Program Change events (0-127). When a Program Change event is detected, the converter performs the following actions:

1.  **Searches for an external instrument definition file**: `tones/{program:03}.json` (e.g., `tones/042.json` for Program 42)
2.  **Loads and applies the instrument definition** if the file exists
3.  **Uses a built-in default instrument** if the file does not exist

### Custom Instrument Files

You can create custom YM2151 instrument definitions by placing JSON files in the `tones/` directory:

```bash
tones/
├── 000.json    # Program 0 (Acoustic Grand Piano)
├── 001.json    # Program 1 (Bright Acoustic Piano)
├── ...
└── 127.json    # Program 127 (Gunshot)
```

Each instrument file defines YM2151 register writes to set FM synthesis parameters. For detailed format documentation and examples, refer to [`tones/README.md`](tones/README.md).

### Example Usage

```bash
# 1. Create a custom instrument definition for MIDI Program 42
#    (e.g., a brass sound)
cat > tones/042.json << EOF
{
  "events": [
    { "time": 0, "addr": "0x20", "data": "0xC7" },
    { "time": 0, "addr": "0x38", "data": "0x00" },
    ...
  ]
}
EOF

# 2. Convert a MIDI file that uses Program 42
smf-to-ym2151log-rust song.mid

# The converter will automatically use tones/042.json
# when Program 42 is specified in a program change event.
```

## Development

### Prerequisites

-   Rust 1.70.0 or later
-   Cargo

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

-   [Python implementation](https://github.com/cat2151/smf-to-ym2151log): The original Python implementation this project is based on
-   [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): Specifies the output JSON format
-   [YM2151 Datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): Official specification document for the YM2151 chip
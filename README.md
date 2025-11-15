# smf-to-ym2151log-rust

**Rust implementation to convert Standard MIDI Files (SMF) into YM2151 register write logs (JSON format)**

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## WIP

Currently, it can only convert basic notes (like "Do-Re-Mi") into a minimal JSON format.

More advanced implementations are planned for the future.

### Current Constraints

#### Channel Assignment Strategy

The current implementation uses a **static channel assignment strategy**. This strategy employs pre-analysis to assign MIDI channels to YM2151 channels:

1.  **Pre-analysis Phase**: Before conversion, the SMF is analyzed to measure the maximum polyphony (number of simultaneous voices) for each MIDI channel.
2.  **Static Assignment**: YM2151 channels are assigned based on each MIDI channel's polyphony requirements.
    *   Example: If MIDI ch0 requires 3 voices, MIDI ch1 requires 1 voice, and the remaining MIDI channels are unused:
        *   YM2151 ch0-ch2 correspond to MIDI ch0
        *   YM2151 ch3 corresponds to MIDI ch1
        *   YM2151 ch4-ch7 remain available

**Out of Scope**: Dynamic channel assignment (methods for changing MIDI-YM2151 channel assignments during playback) is not implemented. This decision aligns with the project's policy of prioritizing simplicity over complexity. Dynamic assignment would require complex voice stealing algorithms and state management, significantly increasing implementation complexity.

## Overview

This is a Rust implementation of [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log).
It converts Standard MIDI Files (SMF) into register write logs (JSON format) for the YM2151 FM sound chip.

## Features

-   **2-Pass Processing Architecture**:
    -   **Pass A**: MIDI file → Intermediate event JSON (for debugging)
    -   **Pass B**: Intermediate events → YM2151 register log JSON (final output)
-   **Type Safety**: Robustness through Rust's type system
-   **High Performance**: Fast processing due to native compilation
-   **Test-Driven Development**: Comprehensive unit and integration tests (51 tests)
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

### Command Line Usage

```bash
# Convert a MIDI file
smf-to-ym2151log-rust song.mid

# Output files:
# - song_events.json  (Pass A: Intermediate events for debugging)
# - song_ym2151.json  (Pass B: YM2151 register log)
```

### Use as a Library

Can be used as a library from other Rust projects:

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

## Development

### Prerequisites
-   Rust 1.70.0 or higher
-   Cargo

### Build
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Tests
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

-   [Python Implementation](https://github.com/cat2151/smf-to-ym2151log): The original Python implementation this project is based on
-   [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): Specifies the output JSON format
-   [YM2151 Datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): Official specification for the YM2151 chip
# smf-to-ym2151log-rust

**Rust implementation to convert Standard MIDI Files (SMF) to YM2151 register write logs (JSON format)**

<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
</p>

## Overview

[smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log) のRust版実装です。
Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換します。

This is a Rust implementation of smf-to-ym2151log that converts Standard MIDI Files (SMF) to YM2151 FM synthesizer chip register write logs in JSON format.

## Project Status

**✅ Implementation Complete!**

すべての実装フェーズが完了しました。詳細な実装計画については [IMPLEMENTATION.md](IMPLEMENTATION.md) をご覧ください。

All implementation phases are complete. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for detailed implementation plan.

### Completed Phases
- ✅ Phase 1: Project Foundation Establishment
- ✅ Phase 2: MIDI Parsing Implementation
- ✅ Phase 3: MIDI to YM2151 Conversion Utilities
- ✅ Phase 4: YM2151 Conversion Implementation
- ✅ Phase 5: Main Program Integration
- ✅ Phase 6: Documentation and Polishing

### Quality Metrics
- ✅ All tests passed (51 tests passing: 28 unit + 15 integration + 8 doc tests)
- ✅ Code Coverage: Good
- ✅ `cargo fmt`: Passed
- ✅ `cargo clippy`: No warnings
- ✅ `cargo audit`: No vulnerabilities

## Features

- **Two-pass processing architecture**:
  - **Pass A**: MIDI file → Intermediate Events JSON (for debugging)
  - **Pass B**: Intermediate Events → YM2151 Register Log JSON (final output)
- **Type Safety**: Robustness through Rust's type system
- **High Performance**: Fast processing via native compilation
- **Test-Driven Development**: Comprehensive unit and integration tests (51 tests)
- **Compatibility**: JSON format compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc)
- **Standard Support**: Supports SMF Format 0 and Format 1
- **Library API**: Convenient API usable from other Rust projects

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

### Library Usage

Can be used as a library from other Rust projects:

```toml
# Cargo.toml
[dependencies]
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
```

```rust
// Convert SMF to YM2151 log in memory
use smf_to_ym2151log::convert_smf_to_ym2151_log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get SMF byte data (e.g., from another library)
    let smf_bytes = std::fs::read("song.mid")?;
    
    // Convert to YM2151 JSON log in a single call
    let ym2151_json = convert_smf_to_ym2151_log(&smf_bytes)?;
    
    println!("{}", ym2151_json);
    Ok(())
}
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

# Run specific tests
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

## Project Structure

Planned project structure:

```
smf-to-ym2151log-rust/
├── Cargo.toml           # Project configuration
├── README.md            # This file
├── IMPLEMENTATION.md    # Implementation plan
├── LICENSE              # License
├── src/
│   ├── main.rs         # Main entry point
│   ├── lib.rs          # Library root
│   ├── error.rs        # Error type definitions
│   ├── midi/           # MIDI processing module
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── events.rs
│   │   └── utils.rs
│   └── ym2151/         # YM2151 processing module
│       ├── mod.rs
│       ├── converter.rs
│       ├── events.rs
│       ├── init.rs
│       └── note_table.rs
└── tests/
    ├── integration_tests.rs
    └── test_data/
        └── test.mid
```

## References

- [Python implementation](https://github.com/cat2151/smf-to-ym2151log): The Python implementation that this project is based on
- [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): Origin of the output JSON format specification
- [YM2151 Datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): Official specification document for the YM2151 chip

## License

Please refer to the [LICENSE](LICENSE) file.

## Contributing

Issues and pull requests are welcome. It is recommended to review the implementation plan before starting work.

## Author

cat2151
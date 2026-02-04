# MML Input Support Analysis for smf-to-ym2151log-rust

## Issue #55: Investigation of mmlabc-to-smf-rust Library Integration

### Executive Summary

**Conclusion**: The `mmlabc-to-smf-rust` library **cannot be used directly** in WASM environments due to its dependency on tree-sitter, which requires native C compilation and Node.js tooling.

---

## Why mmlabc-to-smf-rust Cannot Be Used in WASM

### 1. **Tree-sitter Dependency Requires C Compilation**

The library uses tree-sitter for MML parsing (Pass 1), which has the following blockers:

**From Cargo.toml:**
```toml
[features]
cli = ["tree-sitter", "clap", "cc"]

[build-dependencies]
cc = { version = "1.0", optional = true }

[dependencies]
tree-sitter = { version = "0.22", optional = true }
```

**From build.rs:**
- Requires `npx tree-sitter generate` command (Node.js toolchain)
- Compiles C code using `cc::Build`
- Generates `parser.c` from `grammar.js`

**WASM Incompatibilities:**
- `wasm32-unknown-unknown` target cannot compile C code via `cc` crate
- No Node.js/npx available in WASM runtime
- Tree-sitter's C parser cannot be compiled to WebAssembly through standard Rust toolchain

### 2. **Feature Flag Architecture Issue**

**Current structure:**
```rust
#[cfg(feature = "cli")]
pub mod pass1_parser;  // ← Depends on tree-sitter

pub mod pass2_ast;      // ✓ Pure Rust
pub mod pass3_events;   // ✓ Pure Rust  
pub mod pass4_midi;     // ✓ Pure Rust
```

Pass 1 (parser) is gated behind the `cli` feature which includes tree-sitter. Without Pass 1, the library provides no way to convert MML strings to tokens.

**The API gap:**
- Pass 2-4 are pure Rust and WASM-compatible
- Pass 1 (MML string → tokens) is tree-sitter-dependent
- No alternative tokenization API exists

---

## How to Make mmlabc-to-smf-rust WASM-Compatible

### Option A: Add Alternative Parser (Recommended)

**Modify mmlabc-to-smf-rust to add a WASM-compatible parser:**

#### 1. Create Alternative Tokenizer Module

Add `src/pass1_simple_parser.rs`:
```rust
//! Simple MML tokenizer without tree-sitter dependency
//! WASM-compatible, pure Rust implementation

pub fn parse_mml_to_tokens(mml: &str) -> Result<Vec<Token>> {
    // Hand-written parser logic
    // Supports basic MML subset
}
```

#### 2. Expose Public API for WASM Use

Modify `src/lib.rs`:
```rust
pub mod pass2_ast;
pub mod pass3_events;
pub mod pass4_midi;

#[cfg(feature = "cli")]
pub mod pass1_parser;  // Tree-sitter based

#[cfg(not(feature = "cli"))]
pub mod pass1_simple_parser;  // Pure Rust alternative

pub fn mml_to_smf_bytes(mml: &str) -> Result<Vec<u8>> {
    #[cfg(feature = "cli")]
    let tokens = pass1_parser::parse(mml)?;
    
    #[cfg(not(feature = "cli"))]
    let tokens = pass1_simple_parser::parse_mml_to_tokens(mml)?;
    
    let ast = pass2_ast::tokens_to_ast(&tokens);
    let events = pass3_events::ast_to_events(&ast);
    pass4_midi::events_to_midi(&events)
}
```

#### 3. Update Cargo.toml

```toml
[features]
default = []
cli = ["tree-sitter", "clap", "cc"]
wasm = ["wasm-bindgen"]

[dependencies]
wasm-bindgen = { version = "0.2", optional = true }
```

#### 4. Add WASM Binding

Add `src/wasm.rs`:
```rust
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn mml_to_smf(mml: &str) -> Vec<u8> {
    crate::mml_to_smf_bytes(mml).unwrap_or_default()
}
```

**Benefits:**
- Maintains full tree-sitter functionality for CLI
- Provides WASM-compatible path for browser use
- Reuses existing Pass 2-4 (70% of codebase)
- Clean separation of concerns

**Trade-offs:**
- Simple parser won't support all MML features
- Need to maintain two parser implementations
- Documentation must clarify feature differences

---

### Option B: Extract Core as Separate Crate

**Create a new crate structure:**

```
mmlabc-to-smf/
├── mmlabc-core/          # WASM-compatible core
│   ├── pass2_ast.rs      # Move from main crate
│   ├── pass3_events.rs   # Move from main crate
│   └── pass4_midi.rs     # Move from main crate
│
└── mmlabc-to-smf/        # CLI with tree-sitter
    ├── pass1_parser.rs   # Tree-sitter based
    └── cli.rs
```

**Benefits:**
- Clear separation between WASM-compatible core and CLI
- No feature flag complexity
- Easy to use from other projects

**Trade-offs:**
- Requires repository restructuring
- Need to maintain two crates
- API versioning complexity

---

### Option C: Use Tree-sitter WASM Build

**Experimental approach using tree-sitter's WASM support:**

Tree-sitter does support WASM, but:
- Requires pre-building parser to WASM
- Complex build pipeline
- Not well-integrated with wasm-pack
- Still needs separate build step

**Not recommended** due to complexity and maintenance burden.

---

## Recommended Implementation Path

### Step 1: Modify mmlabc-to-smf-rust (Option A)

1. Add `src/pass1_simple_parser.rs` with basic MML support
2. Add public `mml_to_smf_bytes()` API
3. Add WASM feature and bindings
4. Update Cargo.toml with proper features

### Step 2: Use from smf-to-ym2151log-rust

```toml
[dependencies]
mmlabc-to-smf = { git = "https://github.com/cat2151/mmlabc-to-smf-rust", default-features = false, features = ["wasm"] }
```

```rust
// src/wasm.rs
use mmlabc_to_smf::mml_to_smf;

#[wasm_bindgen]
pub fn mml_to_ym2151_json(mml: &str) -> String {
    let smf_bytes = mml_to_smf(mml);
    smf_to_ym2151_json(&smf_bytes)
}
```

### Step 3: Update Demo

- Add MML textarea to index.html
- Call `mml_to_ym2151_json()` from JavaScript
- Display results

---

## Technical Details: Pass 2-4 Analysis

**These modules are already WASM-compatible:**

### Pass 2: tokens_to_ast()
- Input: `Vec<Token>`
- Output: `Ast` (Abstract Syntax Tree)
- Pure Rust, no external dependencies
- ✅ WASM-ready

### Pass 3: ast_to_events()
- Input: `Ast`
- Output: `Vec<MidiEvent>`
- Pure Rust, no external dependencies
- ✅ WASM-ready

### Pass 4: events_to_midi()
- Input: `Vec<MidiEvent>`
- Output: `Vec<u8>` (SMF bytes)
- Uses `midly` crate (pure Rust, WASM-compatible)
- ✅ WASM-ready

**The only blocker is Pass 1 (tokenization).**

---

## Effort Estimation

### Option A (Recommended):
- **Simple parser implementation**: 4-6 hours
  - Basic note support: 2 hours
  - Octave, length, volume: 2 hours
  - Multi-channel: 1 hour
  - Testing: 1 hour
- **WASM integration**: 1-2 hours
- **Total**: 5-8 hours

### Option B:
- **Crate restructuring**: 3-4 hours
- **WASM bindings**: 1-2 hours
- **Testing**: 2 hours
- **Total**: 6-8 hours

---

## Conclusion

The `mmlabc-to-smf-rust` library is **designed for library use** but currently **requires modifications** to support WASM environments. The library's Pass 2-4 are already WASM-compatible; only Pass 1 (tree-sitter parser) blocks WASM usage.

**Recommended action**: Implement Option A by adding a simple pure-Rust parser alternative to mmlabc-to-smf-rust, allowing it to be used from WASM while maintaining full tree-sitter functionality for CLI users.

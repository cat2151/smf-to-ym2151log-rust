# smf-to-ym2151log-rust

**Standard MIDI Files (SMF) をYM2151レジスタ書き込みログ（JSON形式）に変換するRust実装**

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## WIP

まだドレミを最低限のJSONに変換できる程度です

今後より高度な実装をしていく予定です

## 概要

[smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log) のRust版実装です。
Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換します。

## 特徴

- **2パス処理アーキテクチャ**:
  - **パスA**: MIDIファイル → 中間イベントJSON（デバッグ用）
  - **パスB**: 中間イベント → YM2151レジスタログJSON（最終出力）
- **型安全性**: Rustの型システムによる堅牢性
- **高パフォーマンス**: ネイティブコンパイルによる高速処理
- **テスト駆動開発**: 包括的なユニットテストと統合テスト (51 tests)
- **互換性**: [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc) と互換性のあるJSON形式
- **標準対応**: SMF Format 0 および Format 1 をサポート
- **ライブラリAPI**: 他のRustプロジェクトから利用可能な便利なAPI

## 使い方

### インストール

```bash
# リポジトリをクローン
git clone https://github.com/cat2151/smf-to-ym2151log-rust.git
cd smf-to-ym2151log-rust

# ビルドとインストール
cargo install --path .
```

### コマンドライン使用

```bash
# MIDIファイルの変換
smf-to-ym2151log-rust song.mid

# 出力ファイル:
# - song_events.json  (パスA: デバッグ用中間イベント)
# - song_ym2151.json  (パスB: YM2151レジスタログ)
```

### ライブラリとして使用

他のRustプロジェクトからライブラリとして利用できます：

```toml
# Cargo.toml
[dependencies]
smf-to-ym2151log = { git = "https://github.com/cat2151/smf-to-ym2151log-rust" }
```

詳細なAPIドキュメント: `cargo doc --open`

### 出力例

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

## 開発

### 前提条件
- Rust 1.70.0 以上
- Cargo

### ビルド (Build)
```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト
```bash
# すべてのテストを実行
cargo test

# 特定のテストを実行
cargo test midi_parser

# テストカバレッジ
cargo tarpaulin --out Html
```

### コード品質 (Code Quality)
```bash
# フォーマットチェック
cargo fmt --check

# lintチェック
cargo clippy -- -D warnings

# セキュリティチェック
cargo audit
```

## 参照

- [Python版実装](https://github.com/cat2151/smf-to-ym2151log): このプロジェクトの元になったPython実装
- [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): 出力JSON形式の仕様元
- [YM2151 データシート](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): YM2151チップの公式仕様書

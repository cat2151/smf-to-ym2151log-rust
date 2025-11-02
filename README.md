# smf-to-ym2151log-rust

**Standard MIDI Files (SMF) をYM2151レジスタ書き込みログ（JSON形式）に変換するRust実装**

<p align="left">
  <a href="README.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
</p>

## 概要 (Overview)

[smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log) のRust版実装です。
Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換します。

This is a Rust implementation of smf-to-ym2151log that converts Standard MIDI Files (SMF) to YM2151 FM synthesizer chip register write logs in JSON format.

## プロジェクトステータス (Project Status)

**✅ 実装完了 - Implementation Complete!**

すべての実装フェーズが完了しました。詳細な実装計画については [IMPLEMENTATION.md](IMPLEMENTATION.md) をご覧ください。

All implementation phases are complete. See [IMPLEMENTATION.md](IMPLEMENTATION.md) for detailed implementation plan.

### 完了したフェーズ (Completed Phases)
- ✅ フェーズ1: プロジェクト基盤構築
- ✅ フェーズ2: MIDI解析実装
- ✅ フェーズ3: MIDI→YM2151変換ユーティリティ
- ✅ フェーズ4: YM2151変換実装
- ✅ フェーズ5: メインプログラム統合
- ✅ フェーズ6: ドキュメントと仕上げ

### 品質指標 (Quality Metrics)
- ✅ すべてのテストが合格 (48 tests passing: 28 unit + 13 integration + 7 doc tests)
- ✅ コードカバレッジ: 良好
- ✅ `cargo fmt`: 合格
- ✅ `cargo clippy`: 警告なし
- ✅ `cargo audit`: 脆弱性なし

## 特徴 (Features)

- **2パス処理アーキテクチャ**:
  - **パスA**: MIDIファイル → 中間イベントJSON（デバッグ用）
  - **パスB**: 中間イベント → YM2151レジスタログJSON（最終出力）
- **型安全性**: Rustの型システムによる堅牢性
- **高パフォーマンス**: ネイティブコンパイルによる高速処理
- **テスト駆動開発**: 包括的なユニットテストと統合テスト (48 tests)
- **互換性**: [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc) と互換性のあるJSON形式
- **標準対応**: SMF Format 0 および Format 1 をサポート

## 使い方 (Usage)

### インストール (Installation)

```bash
# リポジトリをクローン
git clone https://github.com/cat2151/smf-to-ym2151log-rust.git
cd smf-to-ym2151log-rust

# ビルドとインストール
cargo install --path .
```

### 基本的な使い方 (Basic Usage)

```bash
# MIDIファイルの変換
smf-to-ym2151log-rust song.mid

# 出力ファイル:
# - song_events.json  (パスA: デバッグ用中間イベント)
# - song_ym2151.json  (パスB: YM2151レジスタログ)
```

### 出力例 (Output Example)

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

## 開発 (Development)

### 前提条件 (Prerequisites)
- Rust 1.70.0 以上
- Cargo

### ビルド (Build)
```bash
# デバッグビルド
cargo build

# リリースビルド
cargo build --release
```

### テスト (Test)
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

# リントチェック
cargo clippy -- -D warnings

# セキュリティチェック
cargo audit
```

## プロジェクト構造 (Project Structure)

予定されているプロジェクト構造：

```
smf-to-ym2151log-rust/
├── Cargo.toml           # プロジェクト設定
├── README.md            # このファイル
├── IMPLEMENTATION.md    # 実装計画書
├── LICENSE              # ライセンス
├── src/
│   ├── main.rs         # メインエントリーポイント
│   ├── lib.rs          # ライブラリルート
│   ├── error.rs        # エラー型定義
│   ├── midi/           # MIDI処理モジュール
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   ├── events.rs
│   │   └── utils.rs
│   └── ym2151/         # YM2151処理モジュール
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

## 参照 (References)

- [Python版実装](https://github.com/cat2151/smf-to-ym2151log): このプロジェクトの元になったPython実装
- [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc): 出力JSON形式の仕様元
- [YM2151 データシート](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf): YM2151チップの公式仕様書

## ライセンス (License)

[LICENSE](LICENSE) ファイルを参照してください。

## 貢献 (Contributing)

イシューやプルリクエストを歓迎します。実装計画書を確認してから作業を開始することをお勧めします。

## 作者 (Author)

cat2151

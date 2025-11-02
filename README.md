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

現在、実装計画フェーズです。詳細な実装計画書を作成しました。

**📋 実装計画書**: [IMPLEMENTATION.md](IMPLEMENTATION.md)

### 計画書の内容
- ✅ 入出力仕様の詳細
- ✅ アーキテクチャ設計（2パス処理）
- ✅ 利用ライブラリの選定と理由
- ✅ テスト方針（ユニットテスト、統合テスト）
- ✅ 6段階の段階的実装計画
- ✅ Python版との比較分析

## 特徴 (Features)

計画されている主な機能：

- **2パス処理アーキテクチャ**:
  - **パスA**: MIDIファイル → 中間イベントJSON（デバッグ用）
  - **パスB**: 中間イベント → YM2151レジスタログJSON（最終出力）
- **型安全性**: Rustの型システムによる堅牢性
- **高パフォーマンス**: ネイティブコンパイルによる高速処理
- **テスト駆動開発**: 包括的なユニットテストと統合テスト
- **互換性**: [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc) と互換性のあるJSON形式

## 予定されている使い方 (Planned Usage)

```bash
# インストール
cargo install --path .

# MIDIファイルの変換
smf-to-ym2151log-rust song.mid

# 出力ファイル:
# - song_events.json  (パスA: デバッグ用中間イベント)
# - song_ym2151.json  (パスB: YM2151レジスタログ)
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

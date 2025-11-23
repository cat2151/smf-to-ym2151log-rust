Last updated: 2025-11-24


# プロジェクト概要生成プロンプト（来訪者向け）

## 生成するもの：
- projectを3行で要約する
- プロジェクトで使用されている技術スタックをカテゴリ別に整理して説明する
- プロジェクト全体のファイル階層ツリー（ディレクトリ構造を図解）
- プロジェクト全体のファイルそれぞれの説明
- プロジェクト全体の関数それぞれの説明
- プロジェクト全体の関数の呼び出し階層ツリー

## 生成しないもの：
- Issues情報（開発者向け情報のため）
- 次の一手候補（開発者向け情報のため）
- ハルシネーションしそうなもの（例、存在しない機能や計画を勝手に妄想する等）

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Project Overview

## プロジェクト概要
[以下の形式で3行でプロジェクトを要約]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 技術スタック
[使用している技術をカテゴリ別に整理して説明]
- フロントエンド: [フロントエンド技術とその説明]
- 音楽・オーディオ: [音楽・オーディオ関連技術とその説明]
- 開発ツール: [開発支援ツールとその説明]
- テスト: [テスト関連技術とその説明]
- ビルドツール: [ビルド・パース関連技術とその説明]
- 言語機能: [言語仕様・機能とその説明]
- 自動化・CI/CD: [自動化・継続的統合関連技術とその説明]
- 開発標準: [コード品質・統一ルール関連技術とその説明]

## ファイル階層ツリー
```
[プロジェクトのディレクトリ構造をツリー形式で表現]
```

## ファイル詳細説明
[各ファイルの役割と機能を詳細に説明]

## 関数詳細説明
[各関数の役割、引数、戻り値、機能を詳細に説明]

## 関数呼び出し階層ツリー
```
[関数間の呼び出し関係をツリー形式で表現]
```
```


以下のプロジェクト情報を参考にして要約を生成してください：

## プロジェクト情報
名前: 
説明: # smf-to-ym2151log-rust

**Standard MIDI Files (SMF) をYM2151レジスタ書き込みログ（JSON形式）に変換するRust実装**

<p align="left">
  <a href="README.ja.md"><img src="https://img.shields.io/badge/🇯🇵-Japanese-red.svg" alt="Japanese"></a>
  <a href="README.md"><img src="https://img.shields.io/badge/🇺🇸-English-blue.svg" alt="English"></a>
</p>

## WIP

まだドレミを最低限のJSONに変換できる程度です

今後より高度な実装をしていく予定です

### 現在の制約

#### チャンネル割り当て戦略

現在の実装では、**和音数ベースの静的チャンネル割り当て戦略**と**ドラムチャンネル優先**を使用しています：

**1. 和音数分析フェーズ**: 
変換前に、MIDIファイルを分析し、重複するノートイベントを追跡することで各MIDIチャンネルの最大和音数（同時発音数）を測定します。

**2. 和音数に基づく静的割り当て**:
YM2151チャンネル（0-7、合計8チャンネル）を各MIDIチャンネルの和音数要求に基づいて割り当てます。
- 和音数が多いMIDIチャンネルは複数のYM2151チャンネルを取得
- 例: MIDI ch0が3和音、MIDI ch1が1和音を必要とする場合：
  - MIDI ch0はYM2151 ch0、ch1、ch2を取得（3チャンネル）
  - MIDI ch1はYM2151 ch3を取得（1チャンネル）
  - YM2151 ch4-ch7は利用可能なまま

**3. ドラムチャンネル優先の並び替え**:
初期割り当ての後、MIDIチャンネル9（General MIDIドラムチャンネル）が存在する場合、割り当てが並び替えられます：
- MIDIチャンネル9がYM2151チャンネル0を使用するよう優先される
- 他のチャンネル割り当ては適宜入れ替えられる
- **理由**: ドラムは同一tick上で複数のnote onが発生することが多い。YM2151はチャンネルを順次処理し、規定のレジスタアクセスサイクルが必要なため、ドラムをチャンネル0に割り当てることで最初に発音され、音質が向上する。

**ボイス管理**:
- MIDIチャンネルに複数のYM2151チャンネルが割り当てられている場合（和音数 > 1）、ノートはラウンドロビン方式で分配される
- 各note-onは割り当て内の次の利用可能なボイスを使用
- note-offイベントは、どのボイスがどのノートを演奏したかを適切に追跡

**制限事項**:
- 利用可能なYM2151チャンネルは合計8つ
- 全MIDIチャンネルの合計和音数が8を超える場合、オーバーフローしたノートは最後に割り当てられたチャンネルを使用
- 再生中の動的ボイススティーリングなし（すべての割り当ては静的/事前決定）

**スコープ外**: 
- 再生中の動的チャンネル割り当て
- ボイススティーリングアルゴリズム
- リアルタイム和音数調整

これらの機能は、シンプルさを保つため、プロジェクトの目標に沿って意図的に省略されています。


## 概要

[smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log) のRust版実装です。
Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換します。

## 特徴

- **2パス処理アーキテクチャ**:
  - **パスA**: MIDIファイル → 中間イベントJSON（デバッグ用）
  - **パスB**: 中間イベント → YM2151レジスタログJSON（最終出力）
- **プログラムチェンジ対応**: 外部JSONファイルからカスタムYM2151音色を読み込み (MIDIプログラム0-127)
- **型安全性**: Rustの型システムによる堅牢性
- **高パフォーマンス**: ネイティブコンパイルによる高速処理
- **テスト駆動開発**: 包括的なユニットテストと統合テスト (73 tests)
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

## プログラムチェンジ対応

コンバータはMIDIプログラムチェンジイベント（0-127）による音色切り替えに対応しています。プログラムチェンジイベントが検出されると、コンバータは以下の動作をします：

1. **外部音色ファイルを検索** `tones/{program:03}.json` （例：プログラム42の場合は `tones/042.json`）
2. **音色をロードして適用** ファイルが存在する場合
3. **内蔵デフォルト音色を使用** ファイルが存在しない場合

### カスタム音色ファイル

`tones/` ディレクトリにJSONファイルを配置することで、カスタムYM2151音色を作成できます：

```bash
tones/
├── 000.json    # プログラム0 (アコースティックグランドピアノ)
├── 001.json    # プログラム1 (ブライトアコースティックピアノ)
├── ...
└── 127.json    # プログラム127 (ガンショット)
```

各音色ファイルは、FM合成パラメータを設定するためのYM2151レジスタ書き込みを定義します。詳細なフォーマットドキュメントと例については、[`tones/README.md`](tones/README.md) を参照してください。

### 使用例

```bash
# 1. MIDIプログラム42用のカスタム音色を作成
#    （例：ブラス音）
cat > tones/042.json << EOF
{
  "events": [
    { "time": 0.0, "addr": "0x20", "data": "0xC7" },
    { "time": 0.0, "addr": "0x38", "data": "0x00" },
    ...
  ]
}
EOF

# 2. プログラム42を使用するMIDIファイルを変換
smf-to-ym2151log-rust song.mid

# コンバータはプログラムチェンジでプログラム42が
# 指定されると自動的に tones/042.json を使用します
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


依存関係:
{}

## ファイル階層ツリー
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📖 IMPLEMENTATION.md
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📁 generated-docs/
📁 issue-notes/
  📖 21.md
  📖 22.md
  📖 23.md
  📖 25.md
  📖 28.md
  📖 30.md
  📖 32.md
  📖 33.md
📁 src/
  📄 error.rs
  📄 lib.rs
  📄 main.rs
  📁 midi/
    📄 events.rs
    📄 mod.rs
    📄 parser.rs
    📄 utils.rs
  📁 ym2151/
    📄 converter.rs
    📄 events.rs
    📄 init.rs
    📄 mod.rs
    📄 note_table.rs
    📄 tone.rs
📁 tests/
  📄 create_test_midi.py
  📄 integration_tests.rs
  📁 test_data/
    📄 multi_channel.mid
    📄 multi_track.mid
    📄 program_change.mid
    📄 simple_melody.mid
    📄 tempo_change.mid
📁 tones/
  📊 000.json
  📖 README.md

## ファイル詳細分析


## 関数呼び出し階層
関数呼び出し階層を分析できませんでした

## プロジェクト構造（ファイル一覧）
IMPLEMENTATION.md
README.ja.md
README.md
issue-notes/21.md
issue-notes/22.md
issue-notes/23.md
issue-notes/25.md
issue-notes/28.md
issue-notes/30.md
issue-notes/32.md
issue-notes/33.md
tones/000.json
tones/README.md

上記の情報を基に、プロンプトで指定された形式でプロジェクト概要を生成してください。
特に以下の点を重視してください：
- 技術スタックは各カテゴリごとに整理して説明
- ファイル階層ツリーは提供された構造をそのまま使用
- ファイルの説明は各ファイルの実際の内容と機能に基づく
- 関数の説明は実際に検出された関数の役割に基づく
- 関数呼び出し階層は実際の呼び出し関係に基づく


---
Generated at: 2025-11-24 07:06:52 JST

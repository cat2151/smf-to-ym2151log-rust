Last updated: 2025-11-24

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRustツールです。
- 2パス処理アーキテクチャを採用し、MIDIファイルから中間イベントを生成し、それを最終的なYM2151ログに変換します。
- プログラムチェンジイベントに対応し、外部JSONファイルからカスタムYM2151音色をロードして適用できます。

## 技術スタック
- フロントエンド: 該当なし (コマンドラインインターフェースとして動作)
- 音楽・オーディオ: Standard MIDI Files (SMF) 形式の解析、YM2151 FM音源チップのレジスタ制御ロジック
- 開発ツール:
    - Git / GitHub: ソースコード管理、バージョン管理
    - Cargo: Rustのビルドシステム、パッケージマネージャー、依存関係管理
    - `cargo install`: アプリケーションのインストール
    - `cargo doc`: APIドキュメント生成
- テスト:
    - `cargo test`: Rust標準のテストフレームワークを使用したユニットテストおよび統合テスト (73 tests)
    - `cargo tarpaulin`: テストカバレッジ測定とHTMLレポート生成
- ビルドツール: Cargo (Rustプロジェクトのコンパイル、リンク、パッケージング)
- 言語機能:
    - Rust: 高パフォーマンス、メモリ安全性、厳格な型システム
    - JSON: 中間イベント、YM2151レジスタログ、カスタム音色ファイルのデータ交換フォーマット
- 自動化・CI/CD:
    - `cargo audit`: プロジェクトの依存関係に対する既知の脆弱性チェック
- 開発標準:
    - `cargo fmt`: Rustコードの自動フォーマット、コードスタイル統一
    - `cargo clippy`: Rustコードの静的解析（Linter）、潜在的なバグや非効率なコードの検出

## ファイル階層ツリー
```
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
```

## ファイル詳細説明
- `Cargo.toml`: Rustプロジェクトのメタデータ（プロジェクト名、バージョンなど）と依存関係を定義する設定ファイル。
- `src/main.rs`: コマンドラインアプリケーションのエントリポイント。CLI引数の解析、MIDIファイルの読み込み、変換処理の呼び出し、結果のファイル保存を行います。
- `src/lib.rs`: プロジェクトのコアロジックをライブラリとして提供するためのエントリポイント。他のRustプロジェクトからこの変換機能を利用する際に使用されます。
- `src/error.rs`: プロジェクト全体で使用されるカスタムエラータイプを定義し、エラーハンドリングを一元化します。
- `src/midi/events.rs`: MIDIファイルからパースされた各種MIDIイベント（Note On/Off, Tempo Change, Program Changeなど）のデータ構造を定義します。
- `src/midi/mod.rs`: `src/midi/` ディレクトリ内のモジュール（`events.rs`, `parser.rs`, `utils.rs`）を公開し、MIDI関連機能のルートモジュールとして機能します。
- `src/midi/parser.rs`: Standard MIDI Files (SMF) を読み込み、その内容を内部表現（中間イベント）にパースするロジックを実装しています。
- `src/midi/utils.rs`: MIDI関連の補助的な機能やユーティリティ関数を提供します。
- `src/ym2151/converter.rs`: MIDIイベントのストリームをYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換する主要なロジックを含みます。チャンネル割り当て戦略もここで処理されます。
- `src/ym2151/events.rs`: YM2151レジスタ書き込みログの各イベント（アドレス、データ、タイムスタンプ）のデータ構造を定義します。
- `src/ym2151/init.rs`: YM2151チップの初期化に関するレジスタ設定や補助的なロジックを扱います。
- `src/ym2151/mod.rs`: `src/ym2151/` ディレクトリ内のモジュールを公開し、YM2151関連機能のルートモジュールとして機能します。
- `src/ym2151/note_table.rs`: MIDIノート番号とYM2151の周波数設定値（OPMパラメータ）のマッピングを定義するテーブルを含みます。
- `src/ym2151/tone.rs`: 外部JSONファイルからYM2151音色設定を読み込み、それをYM2151レジスタ書き込みイベントに変換・適用するロジックを実装しています。
- `tones/`: カスタムYM2151音色定義を格納するディレクトリ。
- `tones/000.json`: MIDIプログラム0番に対応するデフォルトのYM2151音色定義です。
- `tones/README.md`: `tones` ディレクトリ内のJSONファイルのフォーマットや使用方法に関する説明を提供します。
- `tests/integration_tests.rs`: プロジェクト全体の機能を検証するための統合テストコードです。
- `tests/create_test_midi.py`: 変換処理のテストに使用するMIDIファイルをプログラム的に生成するためのPythonスクリプトです。
- `tests/test_data/`: テストに使用されるサンプルMIDIファイル群です。
- `README.ja.md` / `README.md`: プロジェクトの概要、機能、使い方、開発方法などを記述したドキュメント（日本語版と英語版）。
- `LICENSE`: プロジェクトのライセンス情報。
- `IMPLEMENTATION.md`: プロジェクトの実装に関する詳細な技術ドキュメント（開発者向け）。
- `generated-docs/`: `cargo doc` コマンドで生成されるAPIドキュメントの出力先。

## 関数詳細説明
- `main()`: アプリケーションのエントリポイント。コマンドライン引数を解析し、MIDIファイルパスを取得して変換処理のオーケストレーションを行います。成功または失敗をユーザーに通知します。
- `midi::parser::parse_midi_file(file_path: &Path)`:
    - 役割: 指定されたファイルパスのStandard MIDI File (SMF) をパースし、内部的なMIDIイベントのリストを生成します。
    - 引数: `file_path` - パース対象のMIDIファイルへのパス。
    - 戻り値: パースされたMIDIイベントのリスト、またはエラー。
    - 機能: MIDIファイルのヘッダー、トラックチャンクを解析し、Tempo Change, Note On/Off, Program Changeなどのイベントをタイムスタンプ付きの構造体に変換します。
- `ym2151::converter::convert_to_ym2151_log(midi_events: Vec<MidiEvent>, config: Config)`:
    - 役割: MIDIイベントのリストをYM2151レジスタ書き込みログのリストに変換します。
    - 引数: `midi_events` - `parse_midi_file` から得られたMIDIイベントのリスト。`config` - 変換設定（例：デフォルト音色パス）。
    - 戻り値: YM2151レジスタ書き込みイベントのリスト、またはエラー。
    - 機能: チャンネル割り当て戦略に基づきYM2151チャンネルをMIDIチャンネルに割り当て、MIDIイベント（Note On/Off, Program Changeなど）をYM2151レジスタ書き込み命令にマッピングします。
- `ym2151::converter::assign_channels(midi_events: &[MidiEvent])`:
    - 役割: MIDIファイル内の各MIDIチャンネルの和音数を分析し、YM2151の8つのチャンネルを最適な方法で割り当てます。
    - 引数: `midi_events` - 分析対象のMIDIイベントのリスト。
    - 戻り値: MIDIチャンネルからYM2151チャンネルへの割り当てマップ。
    - 機能: 和音数ベースの静的割り当てとドラムチャンネル優先の並び替えロジックを実装し、YM2151チャンネルの効率的な利用を目指します。
- `ym2151::tone::load_tone(program_number: u8)`:
    - 役割: 指定されたプログラム番号に対応するカスタムYM2151音色定義JSONファイルを `tones/` ディレクトリからロードします。
    - 引数: `program_number` - ロードする音色のMIDIプログラム番号 (0-127)。
    - 戻り値: ロードされた音色データ、またはエラー（ファイルが見つからない場合など）。
    - 機能: ファイルパスを構築し、JSONファイルを読み込んでパースし、YM2151レジスタ設定として扱える内部形式に変換します。
- `ym2151::tone::apply_tone(tone_data: &ToneData, channel_index: u8)`:
    - 役割: ロードされたYM2151音色データを、特定のYM2151チャンネルに適用するためのレジスタ書き込みイベントとして生成します。
    - 引数: `tone_data` - ロードされた音色データ。`channel_index` - 適用対象のYM2151チャンネル番号。
    - 戻り値: 音色適用に必要なYM2151レジスタ書き込みイベントのリスト。
    - 機能: 音色データに含まれるレジスタ設定を、指定されたYM2151チャンネルのアドレス空間に合わせてオフセットし、イベントとして出力します。

## 関数呼び出し階層ツリー
```
main()
└── midi::parser::parse_midi_file()
└── ym2151::converter::convert_to_ym2151_log()
    ├── ym2151::converter::assign_channels()
    └── (MIDIイベントの反復処理)
        ├── ym2151::tone::load_tone()  (プログラムチェンジイベント時)
        └── ym2151::tone::apply_tone() (プログラムチェンジイベント時)

---
Generated at: 2025-11-24 07:07:11 JST

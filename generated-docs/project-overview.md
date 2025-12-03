Last updated: 2025-12-04

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust実装です。
- 2パス処理アーキテクチャを採用し、プログラムチェンジによる音色切り替えと型安全性・高パフォーマンスを実現しています。
- CLIツールとしてだけでなく、他のRustプロジェクトへのライブラリ提供も視野に入れた設計となっています。

## 技術スタック
- フロントエンド: コマンドラインインターフェース（CLI）として動作するため、特別なフロントエンド技術は使用していません。標準入出力およびファイルシステムを介してユーザーと対話します。
- 音楽・オーディオ: Standard MIDI Files (SMF) の解析と、ヤマハYM2151 FM音源チップのレジスタ操作ロジックが主要な技術要素です。
- 開発ツール:
    - `cargo`: Rustのプロジェクト管理ツール（ビルド、テスト、依存関係管理）
    - `git`: バージョン管理システム
    - `cargo tarpaulin`: Rustプロジェクトのテストカバレッジを測定
- テスト: Rustの標準テストフレームワークを使用し、`cargo test` コマンドでユニットテストおよび統合テストを実行します。`cargo tarpaulin` でテストカバレッジを測定可能です。
- ビルドツール: Rustの標準ビルドツールである `cargo` を使用し、ソースコードのコンパイルと実行可能ファイルの生成を行います。
- 言語機能: Rust言語（バージョン1.70.0以上）で実装されており、その型システムによる堅牢性とネイティブコンパイルによる高パフォーマンスが特徴です。
- 自動化・CI/CD: 明示的なCI/CDツールは指定されていませんが、`cargo` コマンドを用いたビルド、テスト、品質チェックのフローは、CI/CDパイプラインに容易に統合可能です。
- 開発標準:
    - `cargo fmt`: コードフォーマットの自動適用とチェックによるコードスタイルの統一
    - `cargo clippy`: Rustコードのlintチェックによる潜在的なバグや非効率なコードの検出

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📄 _config.yml
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 21.md
  📖 22.md
  📖 23.md
  📖 25.md
  📖 28.md
  📖 30.md
  📖 32.md
  📖 33.md
  📖 34.md
  📖 36.md
  📖 38.md
  📖 39.md
  📖 41.md
  📖 43.md
  📖 45.md
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
    📄 channel_allocation.rs
    📄 converter.rs
    📄 converter_tests.rs
    📄 event_processor.rs
    📄 events.rs
    📄 init.rs
    📄 mod.rs
    📄 note_table.rs
    📄 tempo_map.rs
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
-   **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定します。
-   **`Cargo.lock`**: プロジェクトの依存関係の正確なバージョンを記録し、ビルドの再現性を保証します。
-   **`Cargo.toml`**: Rustプロジェクトの設定ファイルで、プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   **`LICENSE`**: プロジェクトのライセンス情報が含まれています。
-   **`README.ja.md` / `README.md`**: プロジェクトの概要、使い方、開発方法などを日本語と英語で説明するドキュメントです。
-   **`_config.yml`**: GitHub Pagesなどのサイト設定ファイルです。
-   **`generated-docs/`**: `cargo doc` コマンドで生成されるAPIドキュメントが出力されるディレクトリです。
-   **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルです。
-   **`issue-notes/`**: 開発中の特定の問題や検討事項に関するメモがMarkdown形式で格納されています。
-   **`src/error.rs`**: プロジェクト全体で使用されるカスタムエラー型とエラーハンドリングロジックを定義します。
-   **`src/lib.rs`**: プロジェクトのライブラリクレートのエントリポイントです。他のRustプロジェクトから利用される公開APIが含まれます。
-   **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリポイントです。コマンドライン引数を解析し、変換処理の全体をオーケストレーションします。
-   **`src/midi/events.rs`**: MIDIイベントのデータ構造（ノートオン、ノートオフ、テンポチェンジなど）を定義します。
-   **`src/midi/mod.rs`**: `midi` モジュールのエントリポイントで、関連するサブモジュールを公開します。
-   **`src/midi/parser.rs`**: Standard MIDI Files (SMF) を解析し、内部的な中間イベント表現に変換するロジックが含まれています（パスA）。
-   **`src/midi/utils.rs`**: MIDI関連のヘルパー関数やユーティリティロジックを提供します。
-   **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのチャンネルに割り当てるためのロジックを実装します。和音数分析やドラムチャンネル優先の戦略が含まれます。
-   **`src/ym2151/converter.rs`**: 中間MIDIイベントをYM2151レジスタ書き込みログ（JSON形式）に変換する中核ロジックを実装します（パスB）。
-   **`src/ym2151/converter_tests.rs`**: `converter.rs` モジュールに特化した単体テストコードです。
-   **`src/ym2151/event_processor.rs`**: MIDIイベントをYM2151の具体的なレジスタ操作に変換する詳細なロジックを処理します。
-   **`src/ym2151/events.rs`**: YM2151レジスタ書き込みイベントのデータ構造を定義します。
-   **`src/ym2151/init.rs`**: YM2151チップの初期化レジスタ設定に関するロジックを扱います。
-   **`src/ym2151/mod.rs`**: `ym2151` モジュールのエントリポイントで、関連するサブモジュールを公開します。
-   **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151の周波数設定値のマッピングを管理します。
-   **`src/ym2151/tempo_map.rs`**: MIDIのテンポチェンジイベントを処理し、時間軸のマッピングを提供します。
-   **`src/ym2151/tone.rs`**: YM2151の音色（プログラムチェンジ）データを管理し、外部JSONファイルからのロード機能を提供します。
-   **`tests/create_test_midi.py`**: 統合テストで使用するテスト用MIDIファイルを生成するためのPythonスクリプトです。
-   **`tests/integration_tests.rs`**: プロジェクト全体の機能を検証する統合テストコードです。実際のMIDIファイルをインプットとして変換処理全体をテストします。
-   **`tests/test_data/`**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
-   **`tones/`**: プログラムチェンジイベントに対応するカスタムYM2151音色定義（JSONファイル）が格納されるディレクトリです。
-   **`tones/000.json`**: プログラムチェンジ0番（アコースティックグランドピアノ）のデフォルト音色定義です。
-   **`tones/README.md`**: `tones` ディレクトリの目的とカスタム音色ファイルのフォーマットに関する説明ドキュメントです。

## 関数詳細説明
このプロジェクトは多くの小さな関数で構成されていますが、来訪者向けに主要な処理ブロックを抽象化した関数の役割を説明します。

-   **`parse_midi_file(path: &Path) -> Result<ParsedMidi, Error>`**:
    -   役割: 指定されたパスのStandard MIDI Files (SMF) を読み込み、内部的な中間イベント表現 `ParsedMidi` に解析します。
    -   引数: `path` (MIDIファイルへのパス)
    -   戻り値: 成功時には解析されたMIDIデータ、失敗時にはエラーを返します。
    -   機能: MIDIヘッダ、トラック、イベントデータをバイナリから抽出し、構造化されたデータに変換します（パスAの主要機能）。

-   **`convert_midi_to_ym2151_log(parsed_midi: ParsedMidi, output_path: &Path) -> Result<ConversionStats, Error>`**:
    -   役割: 解析済みの中間MIDIイベントをYM2151 FM音源チップのレジスタ書き込みログのJSON形式に変換し、指定されたパスに保存します。
    -   引数: `parsed_midi` (解析済みのMIDIデータ), `output_path` (出力JSONファイルへのパス)
    -   戻り値: 成功時には変換統計情報、失敗時にはエラーを返します。
    -   機能: チャンネル割り当て、テンポマップの適用、ノートイベントのYM2151レジスタ値への変換、プログラムチェンジによる音色適用などを統合して実行します（パスBの主要機能）。

-   **`allocate_ym2151_channels(midi_events: &[MidiEvent]) -> ChannelAllocationMap`**:
    -   役割: MIDIファイル内のノートイベントを分析し、各MIDIチャンネルに最適なYM2151チャンネルを割り当てます。
    -   引数: `midi_events` (MIDIイベントのリスト)
    -   戻り値: 各MIDIチャンネルに対応するYM2151チャンネルの割り当てマップを返します。
    -   機能: 和音数を計測し、和音数ベースの静的割り当てとドラムチャンネル（MIDIチャンネル9）優先の並び替えロジックを適用します。

-   **`load_program_tone(program_number: u8) -> ToneData`**:
    -   役割: 指定されたプログラム番号（0-127）に対応するYM2151音色データをロードします。
    -   引数: `program_number` (MIDIプログラムチェンジ番号)
    -   戻り値: ロードされた音色データ（`tones/` ディレクトリのJSONファイル、または内蔵デフォルト）を返します。
    -   機能: 外部のカスタム音色JSONファイルを検索・読み込み、ファイルが存在しない場合はデフォルト音色を使用します。

-   **`process_midi_events_for_ym2151(events: &[MidiEvent], channel_map: ChannelAllocationMap, tempo_map: TempoMap) -> Vec<YM2151Event>`**:
    -   役割: MIDIイベントストリームをYM2151レジスタ書き込みイベントのストリームに変換します。
    -   引数: `events` (中間MIDIイベント), `channel_map` (チャンネル割り当て), `tempo_map` (テンポ情報)
    -   戻り値: YM2151レジスタ書き込みイベントのリストを返します。
    -   機能: MIDIノートオン/オフ、プログラムチェンジ、テンポチェンジなどのイベントをYM2151の具体的なレジスタ操作にマッピングします。

-   **`main()` (src/main.rs)**:
    -   役割: プログラムのコマンドラインエントリポイントです。
    -   引数: なし（コマンドライン引数はRustの標準ライブラリで処理されます）。
    -   戻り値: なし（通常、プロセス終了コードを返します）。
    -   機能: コマンドライン引数を解析し、`parse_midi_file` と `convert_midi_to_ym2151_log` を呼び出して全体の変換プロセスを制御します。

-   **`convert(midi_data: &[u8], config: ConvertConfig) -> Result<Vec<u8>, Error>` (src/lib.rs)**:
    -   役割: ライブラリとして、メモリ上のMIDIデータを直接YM2151ログデータに変換します。
    -   引数: `midi_data` (MIDIファイルの生バイトデータ), `config` (変換設定)
    -   戻り値: 成功時にはYM2151ログのJSONバイトデータ、失敗時にはエラーを返します。
    -   機能: `main` 関数と同様の変換ロジックを、CLIを介さずにプログラム的に利用するためのAPIとして提供します。

## 関数呼び出し階層ツリー
```
プロジェクトのソースコードからの自動分析により、詳細な関数呼び出し階層ツリーを生成することはできませんでした。

しかし、プロジェクトの主要な処理フローは以下のようになっています：

1.  **エントリポイント**:
    *   コマンドラインからの実行: `main()` 関数 (`src/main.rs`)
    *   ライブラリとしての利用: `smf_to_ym2151log::convert()` 公開API (`src/lib.rs`)

2.  **MIDIファイル解析 (パスA)**:
    *   エントリポイントから、`midi::parser::parse_midi_file()` が呼び出され、MIDIファイルが解析されます。
    *   `midi::parser` 内部では、MIDIヘッダやトラックデータ、個々のイベント（`midi::events` で定義）が読み込まれ、中間形式のデータ構造が生成されます。

3.  **YM2151ログ変換 (パスB)**:
    *   解析された中間MIDIデータは、`ym2151::converter::convert_midi_to_ym2151_log()` または `ym2151::converter::Converter` のメソッドに渡されます。
    *   この変換プロセス中に、以下のモジュールや関数が相互に作用します。
        *   `ym2151::channel_allocation::allocate_ym2151_channels()`: 各MIDIチャンネルにYM2151チャンネルを割り当てます。
        *   `ym2151::tempo_map`: テンポイベントを処理し、タイムスタンプを管理します。
        *   `ym2151::tone::load_program_tone()`: MIDIプログラムチェンジに応じてYM2151音色データをロードします。
        *   `ym2151::event_processor`: 個々のMIDIイベントをYM2151レジスタ書き込みイベント（`ym2151::events` で定義）に変換する詳細なロジックを処理します。
        *   `ym2151::note_table`: MIDIノートとYM2151の周波数設定値間のマッピングを提供します。

4.  **出力**:
    *   最終的に生成されたYM2151レジスタ書き込みイベントのリストは、JSON形式でファイルに書き出されます。

---
Generated at: 2025-12-04 07:08:37 JST

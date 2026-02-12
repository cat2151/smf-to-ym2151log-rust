Last updated: 2026-02-13

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールです。
- ネイティブアプリケーションのコマンドラインツールとして、またWebブラウザ向けWebAssembly (WASM) ライブラリとして利用可能です。
- MIDIイベント解析、YM2151チャンネル割り当て、カスタム音色適用などを行い、高精度なYM2151再現を目指します。

## 技術スタック
- フロントエンド:
    - **TypeScript**: Webデモ (`demo-library/`) のロジック記述に用いられ、型安全な開発をサポートします。
    - **HTML**: Webデモ (`demo-library/`) のユーザーインターフェース構造を定義します。
    - **Vite**: Webデモの高速な開発サーバーとビルドプロセスを提供します。
    - **WebAssembly (WASM)**: Rustで書かれた変換ロジックをWebブラウザで実行可能にする技術です。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: 入力として使用される標準的なMIDIファイル形式（Format 0およびFormat 1をサポート）。
    - **YM2151 FM音源チップ**: 出力されるレジスタ書き込みログの対象となる、ヤマハ製のFM音源チップ。
    - **MIDIイベント処理**: ノートオン/オフ、プログラムチェンジ、テンポ変更など、MIDIファイル内の各種イベントを解析・処理します。
- 開発ツール:
    - **Rust Cargo**: Rustプロジェクトのビルド、テスト、依存関係管理を行う公式ツールです。
    - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、JavaScriptから利用可能なパッケージを生成します。
    - **Git**: ソースコードのバージョン管理システムです。
    - **npm/yarn**: JavaScript/TypeScriptプロジェクトのパッケージ管理ツール（`package.json`および`package-lock.json`で使用）。
- テスト:
    - **Rust Cargo test**: ユニットテストおよび統合テストの実行フレームワーク。
    - **Tarpaulin**: Rustコードのテストカバレッジを測定し、HTMLレポートなどを生成します。
- ビルドツール:
    - **Rust Cargo build**: Rustコードをネイティブ実行ファイルまたはライブラリにコンパイルします。
    - **wasm-pack build**: RustコードをWebAssemblyモジュールにビルドします。
    - **Vite**: Webデモのバンドルと最適化を行います。
- 言語機能:
    - **Rust**: 高い安全性（メモリ安全性、型安全性）とパフォーマンスを両立するプログラミング言語。
- 自動化・CI/CD:
    - (特筆すべきCI/CDパイプラインは明示されていませんが、以下のツールはCI/CDで一般的に利用されます。)
- 開発標準:
    - **Rust Cargo fmt**: コードの自動フォーマットを行い、コードスタイルを統一します。
    - **Rust Cargo clippy**: 静的解析ツールとして、一般的なコーディングミスや改善点を指摘します。
    - **Rust Cargo audit**: プロジェクトの依存関係に存在する既知のセキュリティ脆弱性をチェックします。

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📄 LICENSE
📖 README.ja.md
📖 README.md
📖 WASM_USAGE.md
📄 _config.yml
📁 demo-library/
  📄 .gitignore
  📘 delay-vibrato-demo.ts
  🌐 delay-vibrato.html
  📘 globals.d.ts
  🌐 index.html
  📘 library-demo.ts
  📊 package-lock.json
  📊 package.json
  🎨 style.css
  📘 tone-json-demo.ts
  🌐 tone-json.html
  📊 tsconfig.json
  📘 vite.config.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
📊 package-lock.json
📊 package.json
📁 src/
  📄 error.rs
  📄 lib.rs
  📄 main.rs
  📁 midi/
    📄 events.rs
    📄 mod.rs
    📄 parser.rs
    📄 utils.rs
  📄 wasm.rs
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
- **`.gitignore`**: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の具体的なバージョンを記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのビルド設定、依存関係、メタデータなどを定義するマニフェストファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報を記述しています。
- **`README.ja.md`**: プロジェクトの日本語での概要、利用方法、開発情報などを記述したドキュメントです。
- **`README.md`**: プロジェクトの英語での概要、利用方法、開発情報などを記述したドキュメントです。
- **`WASM_USAGE.md`**: WebAssembly (WASM) としてこのライブラリを利用する方法について詳細に説明したドキュメントです。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルです。
- **`demo-library/`**: WebAssembly版ライブラリのデモアプリケーションが含まれるディレクトリです。
    - **`demo-library/delay-vibrato-demo.ts`**: ディレイビブラート機能に特化したWebデモのTypeScriptロジックファイルです。
    - **`demo-library/delay-vibrato.html`**: ディレイビブラートWebデモのHTML構造を定義するファイルです。
    - **`demo-library/globals.d.ts`**: Webデモで使用されるグローバル変数の型定義ファイルです。
    - **`demo-library/index.html`**: メインのWebデモページのHTML構造を定義するファイルです。
    - **`demo-library/library-demo.ts`**: WebAssemblyライブラリの基本的な使用方法を示すWebデモのTypeScriptロジックファイルです。
    - **`demo-library/package-lock.json`**: `demo-library`内のJavaScript/TypeScriptプロジェクトの依存関係の正確なバージョンを記録します。
    - **`demo-library/package.json`**: `demo-library`内のJavaScript/TypeScriptプロジェクトのメタデータと依存関係を定義します。
    - **`demo-library/style.css`**: `demo-library`内のWebデモの外観を定義するスタイルシートファイルです。
    - **`demo-library/tone-json-demo.ts`**: カスタム音色JSONの読み込みと適用に特化したWebデモのTypeScriptロジックファイルです。
    - **`demo-library/tone-json.html`**: カスタム音色JSON WebデモのHTML構造を定義するファイルです。
    - **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    - **`demo-library/vite.config.ts`**: Vite開発サーバーとビルドツールの設定ファイルです。
- **`generated-docs/`**: `cargo doc`などによって生成されるAPIドキュメントが格納されるディレクトリです。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルです。
- **`package-lock.json`**: ルートディレクトリのJavaScript/TypeScript依存関係の正確なバージョンを記録します。
- **`package.json`**: ルートディレクトリのJavaScript/TypeScriptプロジェクトのメタデータと依存関係を定義します（主に`demo-library`との連携のため）。
- **`src/`**: プロジェクトの主要なRustソースコードが格納されるディレクトリです。
    - **`src/error.rs`**: プロジェクト全体で利用されるカスタムエラー型を定義し、エラーハンドリングを統一します。
    - **`src/lib.rs`**: このプロジェクトがライブラリとして提供する機能のエントリポイントです。公開APIを定義します。
    - **`src/main.rs`**: コマンドラインツールとして実行される際のエントリポイントです。MIDIファイルの読み込みと変換処理を調整します。
    - **`src/midi/`**: Standard MIDI File (SMF) の解析に関連するモジュールです。
        - **`src/midi/events.rs`**: MIDIイベントのデータ構造（ノートオン/オフ、プログラムチェンジなど）を定義します。
        - **`src/midi/mod.rs`**: `midi`モジュールのルートファイルで、サブモジュールを公開します。
        - **`src/midi/parser.rs`**: MIDIファイルのバイトデータを解析し、`midi::events`で定義された構造体に変換するロジックを実装します。
        - **`src/midi/utils.rs`**: MIDIデータ処理に関連する補助的なユーティリティ関数を提供します。
    - **`src/wasm.rs`**: WebAssembly (WASM) へのバインディングを提供し、JavaScript/TypeScriptからRustのコアロジックを呼び出せるようにします。
    - **`src/ym2151/`**: YM2151レジスタログへの変換に関連するモジュールです。
        - **`src/ym2151/channel_allocation.rs`**: YM2151の8チャンネルに、入力MIDIチャンネルをどのように割り当てるかを決定するロジック（和音数ベース、ドラム優先など）を実装します。
        - **`src/ym2151/converter.rs`**: 解析されたMIDIイベントを、YM2151レジスタ書き込みログの形式に変換する主要なロジックを実装します。
        - **`src/ym2151/converter_tests.rs`**: `ym2151/converter`モジュールの機能テストコードが含まれています。
        - **`src/ym2151/event_processor.rs`**: 個々のMIDIイベントをYM2151レジスタ操作に変換する詳細な処理ロジックを管理します。
        - **`src/ym2151/events.rs`**: YM2151レジスタ書き込みログのデータ構造を定義します。
        - **`src/ym2151/init.rs`**: YM2151チップの初期化に関連するレジスタ設定を定義します。
        - **`src/ym2151/mod.rs`**: `ym2151`モジュールのルートファイルで、サブモジュールを公開します。
        - **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151の周波数設定値（キーコード、FB、KON/KOFなど）のマッピング情報を提供します。
        - **`src/ym2151/tempo_map.rs`**: MIDIファイル内のテンポ変更を追跡し、正確なYM2151イベントタイミングを計算するためのロジックを管理します。
        - **`src/ym2151/tone.rs`**: YM2151の音色データ（FMオペレータ設定など）を管理し、カスタム音色ファイルの読み込みを処理します。
- **`tests/`**: 統合テストおよびテスト関連の補助ファイルが格納されるディレクトリです。
    - **`tests/create_test_midi.py`**: テスト目的でシンプルなMIDIファイルを生成するためのPythonスクリプトです。
    - **`tests/integration_tests.rs`**: プロジェクト全体の機能を検証する統合テストコードが含まれています。
    - **`tests/test_data/`**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
- **`tones/`**: MIDIプログラムチェンジイベントに対応するカスタムYM2151音色（JSON形式）が格納されるディレクトリです。
    - **`tones/000.json`**: プログラムチェンジ0番に対応するカスタムYM2151音色定義ファイルです。
    - **`tones/README.md`**: `tones`ディレクトリ内のカスタム音色JSONファイルのフォーマットと使用方法について説明します。

## 関数詳細説明
- **`setStatus(message: string)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: デモアプリケーションのステータスメッセージをUIに表示します。
    - 引数: `message` (string) - 表示するステータスメッセージ。
    - 戻り値: なし。
- **`setEventCountDisplay(count: number)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: 変換されたイベント数を表示するUI要素を更新します。
    - 引数: `count` (number) - 表示するイベント数。
    - 戻り値: なし。
- **`updateOutput(output: string)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: 変換結果のJSON文字列をデモUIに表示します。
    - 引数: `output` (string) - 表示するJSON文字列。
    - 戻り値: なし。
- **`updatePlayButtonState(isPlaying: boolean)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: オーディオ再生ボタンの状態（再生中か否か）を更新します。
    - 引数: `isPlaying` (boolean) - 再生中であれば`true`、そうでなければ`false`。
    - 戻り値: なし。
- **`initializeWasm()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: WebAssemblyモジュールを非同期で初期化し、利用可能な状態にします。
    - 引数: なし。
    - 戻り値: `Promise<void>`。
- **`readAttachmentBytes(file: File)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts, demo-library/library-demo.ts):
    - 役割: 指定されたファイルからバイトデータを非同期で読み込みます。
    - 引数: `file` (File) - 読み込むファイルオブジェクト。
    - 戻り値: `Promise<Uint8Array>` (読み込まれたバイトデータ)。
- **`runConversion(midiBytes: Uint8Array, tonesJson: Uint8Array | null)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: MIDIバイトデータとオプションのカスタム音色JSONデータを受け取り、WebAssemblyを通じて変換処理を実行します。
    - 引数: `midiBytes` (Uint8Array) - MIDIファイルの生バイトデータ。`tonesJson` (Uint8Array | null) - カスタム音色JSONファイルの生バイトデータ（存在しない場合は`null`）。
    - 戻り値: `Promise<{ events: string; ym2151: string; } | undefined>` (中間イベントJSONとYM2151ログJSON、または`undefined`)。
- **`ensureWebYm2151()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: WebAssemblyモジュールがロードされていることを確認し、未ロードであればロードします。
    - 引数: なし。
    - 戻り値: なし。
- **`handlePlay(ym2151Json: string)`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: 提供されたYM2151ログJSONデータを使用してオーディオ再生を開始します。
    - 引数: `ym2151Json` (string) - YM2151レジスタログのJSON文字列。
    - 戻り値: なし。
- **`setupAttachmentEditor()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: カスタム音色ファイルなどのアタッチメントを扱うためのUI要素を設定します。
    - 引数: なし。
    - 戻り値: なし。
- **`setupMidiInput()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: MIDIファイルをアップロードするためのUI要素を設定し、ファイル選択時のイベント処理を定義します。
    - 引数: なし。
    - 戻り値: なし。
- **`bootstrapWebYm()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: WebAssemblyデモアプリケーションの初期起動処理とイベントハンドラの設定を行います。
    - 引数: なし。
    - 戻り値: `Promise<void>`。
- **`main()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: デモスクリプトの主要なエントリポイントとして、初期設定とデモロジックの開始を調整します。
    - 引数: なし。
    - 戻り値: なし。
- **`cleanup()`** (demo-library/delay-vibrato-demo.ts, demo-library/tone-json-demo.ts):
    - 役割: デモの終了時または状態変更時に必要なクリーンアップ処理（例: オーディオキャッシュのクリア）を実行します。
    - 引数: なし。
    - 戻り値: なし。
- **`initWasm()`** (demo-library/library-demo.ts):
    - 役割: WebAssemblyモジュールを初期化します。
    - 引数: なし。
    - 戻り値: `Promise<void>`。
- **`displayResult(eventsJson: string, ym2151Json: string)`** (demo-library/library-demo.ts):
    - 役割: 中間イベントJSONとYM2151ログJSONの変換結果をUIに表示します。
    - 引数: `eventsJson` (string) - 中間イベントのJSON文字列。`ym2151Json` (string) - YM2151ログのJSON文字列。
    - 戻り値: なし。
- **`showError(message: string)`** (demo-library/library-demo.ts):
    - 役割: エラーメッセージをUIに表示します。
    - 引数: `message` (string) - 表示するエラーメッセージ。
    - 戻り値: なし。
- **`setupFileInput()`** (demo-library/library-demo.ts):
    - 役割: MIDIファイル入力要素を設定し、ファイルが選択された際の処理を定義します。
    - 引数: なし。
    - 戻り値: なし。
- **`playAudioWithOverlay(data: string)`** (demo-library/globals.d.ts):
    - 役割: 提供されたYM2151ログデータに基づいてオーディオを再生し、再生中にUIオーバーレイを表示します。
    - 引数: `data` (string) - 再生するYM2151ログJSON文字列。
    - 戻り値: `Promise<void>`。
- **`clearAudioCache()`** (demo-library/globals.d.ts):
    - 役割: 再生システムのオーディオキャッシュをクリアします。
    - 引数: なし。
    - 戻り値: なし。
- **`smf_to_ym2151_log_main()`** (src/main.rs):
    - 役割: コマンドラインツールとして実行される際のエントリポイントで、MIDIファイルの読み込み、変換、結果の出力といった一連の処理を統括します。
    - 引数: なし (コマンドライン引数から設定を読み取ります)。
    - 戻り値: `Result<(), Box<dyn Error>>` (処理が成功した場合は`Ok(())`、エラーが発生した場合は`Err`を返します)。
- **`convert_smf_to_ym2151_log(midi_data: &[u8], config: &Config)`** (src/lib.rs):
    - 役割: ライブラリの主要な公開関数であり、SMFバイトデータと設定情報を受け取り、YM2151レジスタログを生成します。
    - 引数: `midi_data` (&[u8]) - Standard MIDI Fileのバイトデータ。`config` (&Config) - 変換処理のための設定オプション。
    - 戻り値: `Result<(String, String), Error>` (中間イベントJSONとYM2151ログJSONのタプル、またはエラーを返します)。
- **`parse_midi(data: &[u8])`** (src/midi/parser.rs):
    - 役割: MIDIファイルのバイトデータを解析し、内部で使用されるMIDIイベントのリストに変換します。
    - 引数: `data` (&[u8]) - MIDIファイルの生バイトデータ。
    - 戻り値: `Result<Vec<MidiEvent>, MidiParseError>` (解析されたMIDIイベントのベクタ、または解析エラーを返します)。
- **`convert_events_to_ym2151_log(events: &[MidiEvent], config: &Config)`** (src/ym2151/converter.rs):
    - 役割: 解析されたMIDIイベントのリストをYM2151のレジスタ書き込みログに変換します。このプロセスには、チャンネル割り当て、テンポ処理、音色適用などが含まれます。
    - 引数: `events` (&[MidiEvent]) - 解析済みのMIDIイベントのリスト。`config` (&Config) - 変換設定。
    - 戻り値: `Result<String, Ym2151ConvertError>` (YM2151レジスタログのJSON文字列、または変換エラーを返します)。
- **`alloc_channels(track_data: &TrackData)`** (src/ym2151/channel_allocation.rs):
    - 役割: MIDIトラックデータに基づいて、YM2151の限られたチャンネル（8つ）にMIDIチャンネルを静的に割り当てる戦略（同時発音数とドラム優先）を実行します。
    - 引数: `track_data` (&TrackData) - 各MIDIチャンネルの和音数などの情報を含むデータ。
    - 戻り値: `ChannelAllocation` (割り当てられたチャンネル情報)。
- **`process_midi_event(event: &MidiEvent, context: &mut EventProcessorContext)`** (src/ym2151/event_processor.rs):
    - 役割: 個々のMIDIイベント（ノートオン/オフ、プログラムチェンジなど）を処理し、YM2151レジスタへの書き込みイベントに変換します。
    - 引数: `event` (&MidiEvent) - 処理するMIDIイベント。`context` (&mut EventProcessorContext) - イベント処理の現在の状態。
    - 戻り値: なし (コンテキスト内部でYM2151イベントを生成します)。
- **`load_tone(program_number: u8, config: &Config)`** (src/ym2151/tone.rs):
    - 役割: 指定されたMIDIプログラム番号（0-127）に対応するYM2151音色データをロードします。外部JSONファイル（`tones/`ディレクトリ内）または内蔵のデフォルト音色を利用します。
    - 引数: `program_number` (u8) - MIDIプログラム番号。`config` (&Config) - 設定情報。
    - 戻り値: `Result<Tone, ToneLoadError>` (ロードされた音色データ、または音色ロードエラーを返します)。

## 関数呼び出し階層ツリー
```
- if (demo-library/delay-vibrato-demo.ts)
  - setStatus (demo-library/delay-vibrato-demo.ts)
    - setEventCountDisplay ()
      - updateOutput ()
      - updatePlayButtonState ()
      - initializeWasm ()
      - readAttachmentBytes ()
      - runConversion ()
      - ensureWebYm2151 ()
      - handlePlay ()
      - setupAttachmentEditor ()
      - setupMidiInput ()
      - bootstrapWebYm ()
      - main ()
      - cleanup ()
      - catch ()
      - addEventListener ()
      - playAudioWithOverlay ()
      - clearAudioCache ()
- initWasm (demo-library/library-demo.ts)
  - displayResult ()
    - showError ()
    - setupFileInput ()

---
Generated at: 2026-02-13 07:12:52 JST

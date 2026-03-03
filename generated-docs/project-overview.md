Last updated: 2026-03-04

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製ツールおよびライブラリです。
- ネイティブアプリケーションとWebAssembly (WASM) を介したブラウザ環境の両方で利用可能で、高パフォーマンスと型安全性を特徴とします。
- 和音数ベースの静的チャンネル割り当てやカスタム音色対応、2パス処理アーキテクチャによりSMFの高度な変換を実現します。

## 技術スタック
- フロントエンド: 
    - **TypeScript**: WebAssemblyデモアプリケーションのロジック記述に使用される型付けされたJavaScriptのスーパーセットです。
    - **HTML**: WebAssemblyデモのユーザーインターフェース構造を定義します。
    - **CSS**: WebAssemblyデモのスタイリングを担当します。
- 音楽・オーディオ: 
    - **Standard MIDI Files (SMF)**: プロジェクトの入力となる、音楽情報をデジタル形式で格納する標準ファイル形式です。
    - **YM2151 FM音源チップ**: プロジェクトの変換ターゲットとなる、レトロなFM音源チップです。
- 開発ツール: 
    - **Rust**: プロジェクトの主要なプログラミング言語であり、その強力な型システムとパフォーマンスを活用しています。
    - **Cargo**: Rustのビルドシステムおよびパッケージマネージャーです。
    - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、Webブラウザで利用可能なパッケージを生成するためのツールです。
    - **Vite**: WebAssemblyデモの開発サーバーおよびビルドツールとして使用されます。
    - **Biome**: デモプロジェクトのコードフォーマッターおよびリンターとして利用されます。
- テスト: 
    - **cargo test**: Rustプロジェクトのユニットテストおよび統合テストを実行するための標準ツールです。
    - **cargo tarpaulin**: Rustコードのテストカバレッジを測定し、HTMLレポートなどを生成します。
- ビルドツール: 
    - **Cargo**: Rustプロジェクトのビルド、依存関係管理を行います。
    - **wasm-pack**: WebAssemblyターゲット向けにビルドします。
    - **Vite**: フロントエンドのデモアプリケーションをビルドします。
- 言語機能: 
    - **Rustの型システムと所有権**: コンパイル時の安全性とメモリ安全性を保証し、高パフォーマンスなコードを実現します。
    - **TypeScriptの型付け**: デモアプリケーションのコード品質と保守性を向上させます。
- 自動化・CI/CD: 
    - **cargo fmt**: Rustコードのフォーマットを自動的にチェックし、統一されたコードスタイルを強制します。
    - **cargo clippy**: Rustコードの一般的な間違いや改善点を指摘するリンターです。
    - **cargo audit**: プロジェクトの依存関係に既知のセキュリティ脆弱性がないかチェックします。
- 開発標準: 
    - **Rust Standard Library**: Rustの標準ライブラリを利用し、堅牢なアプリケーションを構築します。
    - **コードフォーマット (cargo fmt, biome)**: プロジェクト全体のコードスタイルの一貫性を保ちます。

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
  📊 biome.json
  📘 delay-vibrato-demo.ts
  🌐 delay-vibrato.html
  📘 globals.d.ts
  🌐 index.html
  📘 library-demo.ts
  📘 log-visualizer.ts
  📘 mml-support.ts
  📊 package-lock.json
  📊 package.json
  📘 pop-noise-demo.ts
  🌐 pop-noise.html
  📘 portamento-soft-lfo-demo.ts
  🌐 portamento-soft-lfo.html
  📘 shared-demo.ts
  🎨 style.css
  📘 tone-json-attachment.ts
  📘 tone-json-demo.ts
  📘 tone-json-mml.ts
  🌐 tone-json.html
  📊 tsconfig.json
  📘 vite.config.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 105.md
  📖 111.md
  📖 112.md
  📖 115.md
  📖 122.md
  📖 123.md
  📖 125.md
  📖 126.md
  📖 128.md
  📖 131.md
  📖 133.md
  📖 22.md
  📖 33.md
  📖 45.md
  📖 47.md
  📖 66-resolution.md
  📖 70.md
  📖 83.md
  📖 90.md
  📖 91.md
  📖 93.md
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
    📁 converter/
      📄 pitch_effects.rs
      📄 register_effects.rs
      📄 waveform.rs
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
- **README.ja.md / README.md**: プロジェクトの概要、使い方、開発方法などを記述した日本語版および英語版のドキュメントです。
- **WASM_USAGE.md**: WebAssembly (WASM) としてこのライブラリを使用する方法について詳細に説明したドキュメントです。
- **Cargo.toml**: Rustプロジェクトの設定ファイルで、依存関係、メタデータ、ビルド設定などが定義されています。
- **Cargo.lock**: `Cargo.toml`に基づいて解決された、全ての依存関係の正確なバージョンを記録します。
- **src/main.rs**: プロジェクトのコマンドラインインターフェース (CLI) エントリポイントです。MIDIファイルの変換処理をコマンドラインから実行します。
- **src/lib.rs**: プロジェクトのRustライブラリクレートのエントリポイントです。他のRustプロジェクトやWebAssemblyから利用されるAPIを定義します。
- **src/error.rs**: プロジェクト全体で利用されるカスタムエラー型を定義し、エラーハンドリングを統一します。
- **src/midi/events.rs**: MIDIイベントの構造とタイプを定義します。
- **src/midi/mod.rs**: `src/midi`モジュールのルートファイルで、サブモジュールをエクスポートします。
- **src/midi/parser.rs**: Standard MIDI Files (SMF) をパースし、内部的なMIDIイベント構造に変換するロジックを含みます。
- **src/midi/utils.rs**: MIDIイベントの処理やSMFに関するユーティリティ関数を提供します。
- **src/wasm.rs**: Rustの機能をWebAssemblyとして公開するための関数と構造体を定義し、JavaScriptからの呼び出しを可能にします。
- **src/ym2151/channel_allocation.rs**: MIDIチャンネルをYM2151の限られた8チャンネルに割り当てるための複雑なロジックを実装しています。和音数とドラムチャンネルの優先順位に基づいて割り当てを行います。
- **src/ym2151/converter.rs**: MIDIイベントをYM2151レジスタ書き込みログに変換する主要なロジックを含みます。2パス処理の中核をなします。
- **src/ym2151/converter_tests.rs**: `ym2151`モジュールの変換ロジックに対するユニットテストを含みます。
- **src/ym2151/event_processor.rs**: MIDIイベントをYM2151のイベントシーケンスに変換する具体的な処理ロジックを定義します。
- **src/ym2151/events.rs**: YM2151レジスタ書き込みログのイベント構造を定義します。
- **src/ym2151/init.rs**: YM2151チップの初期状態やデフォルト設定に関するデータを含みます。
- **src/ym2151/mod.rs**: `src/ym2151`モジュールのルートファイルで、サブモジュールをエクスポートします。
- **src/ym2151/note_table.rs**: MIDIノート番号とYM2151の周波数設定値（FN/BLOCK）のマッピングテーブルを提供します。
- **src/ym2151/tempo_map.rs**: MIDIファイルのテンポ変更イベントを管理し、正確な時間計算を可能にするためのテンポマップロジックを提供します。
- **src/ym2151/tone.rs**: YM2151の音色データ構造とその管理ロジックを定義します。プログラムチェンジイベントに基づいてカスタム音色をロードする機能を含みます。
- **src/ym2151/converter/pitch_effects.rs**: ピッチベンドやビブラートなどのピッチ関連エフェクトをYM2151レジスタ操作に変換するロジックを含みます。
- **src/ym2151/converter/register_effects.rs**: YM2151の各種レジスタ（ボリューム、ADSRなど）に対するエフェクトや設定変更を処理するロジックを含みます。
- **src/ym2151/converter/waveform.rs**: YM2151の波形設定に関するロジックを提供します。
- **tests/integration_tests.rs**: プロジェクト全体の主要な機能が正しく連携して動作するかを確認するための統合テストを含みます。
- **tests/create_test_midi.py**: 統合テストで使用するMIDIファイルを自動生成するためのPythonスクリプトです。
- **tests/test_data/**: 統合テストで使用されるサンプルMIDIファイルを格納します。
- **tones/**: MIDIプログラムチェンジに対応するカスタムYM2151音色（JSON形式）を格納するディレクトリです。
- **demo-library/**: WebAssembly版ライブラリの動作をブラウザで確認するためのデモアプリケーション関連のファイル群です。
    - **demo-library/*.ts**: デモアプリケーションのTypeScriptソースファイルで、WASMの呼び出し、UIの操作、ログの視覚化などを担当します。
    - **demo-library/*.html**: デモアプリケーションの各ページのHTML構造を定義します。
    - **demo-library/log-visualizer.ts**: YM2151のレジスタログを視覚的に表示するためのロジックを提供します。
    - **demo-library/mml-support.ts**: MML（Music Macro Language）形式の入力をSMFに変換する（関連プロジェクトの機能を利用する）ためのサポートロジックを含みます。
    - **demo-library/shared-demo.ts**: 各デモ間で共通して利用される機能やヘルパー関数を提供します。
    - **demo-library/style.css**: デモアプリケーションの共通スタイリングを定義します。
    - **demo-library/tone-json-attachment.ts**: 添付されたカスタム音色JSONファイルを処理するロジックを含みます。
    - **demo-library/vite.config.ts**: Viteビルドツールの設定ファイルです。

## 関数詳細説明
このセクションでは、プロジェクトの主要な関数について、その役割、引数、戻り値、機能について説明します。

**Rust側の主要な関数（`src/`以下）**:

-   **`main` (src/main.rs)**
    -   **役割**: コマンドラインアプリケーションのエントリポイント。
    -   **機能**: CLI引数を解析し、指定されたMIDIファイルをYM2151ログに変換します。中間イベントと最終YM2151ログをJSONファイルとして出力します。
-   **`convert_smf_to_ym2151_log` (src/lib.rs or src/ym2151/converter.rs)**
    -   **役割**: Standard MIDI File (SMF) をYM2151レジスタ書き込みログに変換する主要な関数。
    -   **引数**: MIDIファイルデータ、オプションのカスタム音色データなど。
    -   **戻り値**: YM2151レジスタ書き込みイベントのリスト（JSON形式または内部データ構造）。
    -   **機能**: MIDIファイルをパースし、YM2151のチャンネル割り当て戦略を適用し、各種MIDIイベント（ノートオン/オフ、プログラムチェンジ、テンポ変更など）をYM2151のレジスタ操作に変換します。
-   **`parse_midi_file` (src/midi/parser.rs)**
    -   **役割**: MIDIファイルデータを解析し、内部的なMIDIイベントのリストを生成する。
    -   **引数**: 生のMIDIファイルバイトデータ。
    -   **戻り値**: パースされたMIDIイベントのリストとメタデータ。
    -   **機能**: SMFフォーマット0または1を読み込み、トラック、テンポ、ノートなどのMIDIイベントを抽出します。
-   **`allocate_channels` (src/ym2151/channel_allocation.rs)**
    -   **役割**: MIDIチャンネルの要求に基づいてYM2151の8つのチャンネルを割り当てる。
    -   **引数**: 各MIDIチャンネルの最大和音数情報。
    -   **戻り値**: 各MIDIチャンネルに割り当てられたYM2151チャンネルのマップ。
    -   **機能**: 和音数ベースの静的割り当て戦略と、ドラムチャンネル優先のルールに従ってYM2151チャンネルを割り振ります。
-   **`process_midi_events` (src/ym2151/event_processor.rs)**
    -   **役割**: パースされたMIDIイベントをYM2151固有のイベントに変換し、テンポや音色変更を適用する。
    -   **引数**: MIDIイベントのリスト、チャンネル割り当てマップ、テンポマップ、音色データ。
    -   **戻り値**: 変換されたYM2151イベントのシーケンス。
    -   **機能**: MIDIイベントのタイムベースをYM2151のタイムベースに変換し、プログラムチェンジによる音色切り替え、ピッチベンドなどのエフェクトを処理します。
-   **`get_tone_data` (src/ym2151/tone.rs)**
    -   **役割**: 指定されたプログラム番号に対応するYM2151音色データを取得する。
    -   **引数**: MIDIプログラム番号。
    -   **戻り値**: YM2151音色定義データ。
    -   **機能**: `tones/`ディレクトリ内のJSONファイルからカスタム音色をロードするか、デフォルトの音色データを提供します。
-   **`wasm_entry_point` (src/wasm.rs)** (例、公開される関数名)
    -   **役割**: WebAssemblyモジュールとして公開される主要な関数。
    -   **引数**: MIDIファイルのバイト配列、カスタム音色JSON文字列など。
    -   **戻り値**: YM2151レジスタログのJSON文字列。
    -   **機能**: Webブラウザから呼び出され、MIDIデータの変換処理を実行し、結果をJavaScriptに返します。

**TypeScript側の主要な関数（`demo-library/`以下）**:

-   **`initializeWasm` / `initWasm` (demo-library/*.ts, shared-demo.ts)**
    -   **役割**: WebAssemblyモジュールを初期化し、RustでコンパイルされたライブラリをJavaScript環境で利用可能にする。
    -   **機能**: `smf_to_ym2151log_rust/pkg`からWASMモジュールをインポート・ロードします。
-   **`runConversion` (demo-library/*.ts)**
    -   **役割**: ユーザー入力（MIDIファイル、カスタム音色）を受け取り、WASMを通じて変換処理を実行する。
    -   **機能**: ファイルリーダーでMIDIファイルを読み込み、必要に応じて添付の音色データをパースし、WASMの変換関数を呼び出します。結果をUIに表示します。
-   **`handlePlay` (demo-library/*.ts)**
    -   **役割**: 変換されたYM2151ログをWebブラウザで再生する。
    -   **機能**: `web-ym2151`ライブラリ（外部の音源再生ライブラリ）を利用して、生成されたYM2151ログデータを再生します。
-   **`createLogVisualizer` / `renderFromJson` (demo-library/log-visualizer.ts)**
    -   **役割**: YM2151のレジスタ書き込みログを視覚的に表示する。
    -   **機能**: JSON形式のログデータをパースし、イベントのタイムラインやチャンネルごとの活動をグラフィカルに表現します。
-   **`setupMmlInput` / `setupMidiInput` / `setupAttachmentEditor` (demo-library/*.ts)**
    -   **役割**: デモページのUI要素（テキストエリア、ファイル入力、ボタンなど）を初期化し、イベントリスナーを設定する。
    -   **機能**: ユーザーがMIDIファイルやMML、カスタム音色JSONを入力できるよう、各種フォーム要素を準備します。
-   **`updateOutputWithState` / `displayResult` (demo-library/*.ts)**
    -   **役割**: 変換結果やデモの状態をWebページの出力エリアに表示する。
    -   **機能**: 変換の進行状況、エラーメッセージ、最終的なYM2151ログなどをHTML要素に反映します。
-   **`readAttachmentBytes` / `parseAttachmentField` (demo-library/*.ts)**
    -   **役割**: カスタム音色などの添付ファイルデータを読み込み、処理する。
    -   **機能**: `FileReader`を使用してファイル入力からバイトデータを取得したり、テキストエリアからJSON文字列をパースしたりします。
-   **`treeToJson` (demo-library/mml-support.ts, demo-library/tone-json-mml.ts)**
    -   **役割**: MMLパーサーが生成した構文木をJSON形式に変換する。
    -   **機能**: 構文木を再帰的に走査し、SMF変換に適したJSON構造を構築します。

## 関数呼び出し階層ツリー
```
- main (src/main.rs)
  - convert_smf_to_ym2151_log (src/lib.rs or src/ym2151/converter.rs)
    - parse_midi_file (src/midi/parser.rs)
    - allocate_channels (src/ym2151/channel_allocation.rs)
    - process_midi_events (src/ym2151/event_processor.rs)
      - get_tone_data (src/ym2151/tone.rs)
      - apply_pitch_effects (src/ym2151/converter/pitch_effects.rs)
      - apply_register_effects (src/ym2151/converter/register_effects.rs)

- wasm_entry_point (src/wasm.rs) (WebAssembly経由の主要な呼び出し)
  - convert_smf_to_ym2151_log (src/lib.rs or src/ym2151/converter.rs)

- initializeWasm (demo-library/*.ts, shared-demo.ts)
  - (wasm-packによって生成されたJSモジュールをロード)

- runConversion (demo-library/*.ts)
  - readAttachmentBytes (demo-library/delay-vibrato-demo.tsなど)
  - wasm_entry_point (via WASM binding)
  - updateOutputWithState (demo-library/*.ts)
  - updatePlayButtonState (demo-library/*.ts)

- handlePlay (demo-library/*.ts)
  - playAudioWithOverlay (globals.d.ts - 外部ライブラリ機能)
  - clearWebYmAudioCache (demo-library/shared-demo.ts)
    - cleanup (demo-library/shared-demo.ts)
  - createLogVisualizer (demo-library/log-visualizer.ts)
    - renderFromJson (demo-library/log-visualizer.ts)
      - parseHexByte (demo-library/log-visualizer.ts)
      - detectChannel (demo-library/log-visualizer.ts)
      - normalizeEvents (demo-library/log-visualizer.ts)
      - laneColor (demo-library/log-visualizer.ts)
      - createLane (demo-library/log-visualizer.ts)
      - computeTrackWidth (demo-library/log-visualizer.ts)
      - ensureGlobalLane (demo-library/log-visualizer.ts)
      - renderEmpty (demo-library/log-visualizer.ts)

- setupMmlInput (demo-library/*.ts)
  - setupMmlToSmf (demo-library/mml-support.ts)
    - ensureMmlRuntime (demo-library/mml-support.ts)
      - getMmlParser (demo-library/tone-json-mml.ts)
      - getParseTreeJsonToSmf (demo-library/tone-json-mml.ts)
    - treeToJson (demo-library/mml-support.ts)

- convertMmlToSmf (demo-library/tone-json-demo.ts)
  - treeToJson (demo-library/tone-json-mml.ts)

- buildEventsFromCompact (demo-library/tone-json-attachment.ts)
  - normalizeAttachmentText (demo-library/tone-json-attachment.ts)

- Various UI setup functions (e.g., setupAttachmentEditor, setupMidiInput, setupPlayButton)
  - setStatus (demo-library/shared-demo.ts)
  - setEventCountDisplay (demo-library/shared-demo.ts)
  - ensureWasmInitialized (demo-library/shared-demo.ts)
  - ensureWebYm2151 (demo-library/shared-demo.ts)
  - updateOutput (demo-library/shared-demo.ts)
  - parseAttachmentField (demo-library/shared-demo.ts)

---
Generated at: 2026-03-04 07:11:31 JST

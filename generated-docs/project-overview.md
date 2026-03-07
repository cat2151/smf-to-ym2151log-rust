Last updated: 2026-03-08

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をヤマハYM2151 FM音源チップ向けのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- ネイティブアプリケーション向けRustライブラリとして、`cat-play-mml` などのプロジェクトで利用されています。
- WebAssembly (WASM) を通じてブラウザ上でも動作し、オンラインデモや`web-ym2151`などのWebアプリケーションで利用可能です。

## 技術スタック
- フロントエンド: **TypeScript**, **HTML**, **CSS**, **Vite** (モダンなフロントエンドビルドツール), **WebAssembly (WASM)** (RustコードをWebブラウザで実行可能にするバイナリ形式)
- 音楽・オーディオ: **Standard MIDI Files (SMF)** (音楽データの標準フォーマット), **YM2151 FM音源** (ヤマハのFM音源チップ), **MML** (MMLabc-to-smf-rustとの連携によりMMLのサポートを示唆)
- 開発ツール: **Rust Cargo** (Rustのパッケージマネージャ・ビルドシステム), **wasm-pack** (RustをWASMにビルドするツール), **Git** (バージョン管理システム), **Python** (テストデータ生成用), **npm/Yarn** (JavaScript/TypeScriptプロジェクトのパッケージ管理)
- テスト: **Rust標準テストフレームワーク** (`cargo test`によるユニットテスト・統合テスト), **Tarpaulin** (Rustコードのテストカバレッジツール)
- ビルドツール: **Rust Cargo**, **wasm-pack**, **Vite** (フロントエンドアプリケーションのビルドツール)
- 言語機能: **Rust** (主要な開発言語), **TypeScript** (Webデモのスクリプト言語), **JavaScript** (WASMバインディングとWebデモの実行環境), **JSON** (中間イベント、YM2151ログ、カスタム音色定義)
- 自動化・CI/CD: (直接的なCI/CDツールは記述なし。開発標準に記載の品質ツールがCI/CDで利用されることがあります。)
- 開発標準: **Rustfmt** (Rustコードフォーマッター), **Clippy** (RustコードのLinter/静的解析ツール), **Biome** (TypeScript/JavaScriptのリンター・フォーマッター), **テスト駆動開発 (TDD)** (開発プロセスにおいてテストを重視する手法)

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
  📖 123.md
  📖 126.md
  📖 145.md
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
    📄 utils_tests.rs
  📄 wasm.rs
  📁 ym2151/
    📄 channel_allocation.rs
    📁 converter/
      📄 pitch_effects.rs
      📄 register_effects.rs
      📄 waveform.rs
    📄 converter.rs
    📁 converter_tests/
      📄 basic.rs
      📄 channels.rs
      📄 effects.rs
      📄 programs.rs
    📄 converter_tests.rs
    📄 event_processor.rs
    📄 event_processor_tests.rs
    📄 events.rs
    📄 init.rs
    📄 mod.rs
    📄 note_table.rs
    📄 tempo_map.rs
    📄 tone.rs
📁 tests/
  📄 create_test_midi.py
  📄 integration_conversion.rs
  📄 integration_midi.rs
  📄 integration_multichannel.rs
  📄 integration_program_change.rs
  📄 integration_wasm.rs
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
-   `.gitignore`: Gitによるバージョン管理の対象外とするファイルやディレクトリを指定します。
-   `Cargo.lock`: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
-   `Cargo.toml`: Rustプロジェクトの設定ファイル。プロジェクト名、バージョン、依存クレート、機能などを定義します。
-   `LICENSE`: 本プロジェクトのライセンス情報（著作権や利用条件）を記載しています。
-   `README.ja.md`: プロジェクトの概要、使い方、機能などを日本語で説明する主要なドキュメントです。
-   `README.md`: プロジェクトの概要、使い方、機能などを英語で説明する主要なドキュメントです。
-   `WASM_USAGE.md`: WebAssembly (WASM) バージョンのライブラリをWebブラウザやJavaScript環境で利用する方法に関する詳細なドキュメントです。
-   `_config.yml`: GitHub Pagesなどの静的サイトジェネレーターの設定ファイルです。
-   `demo-library/`: WebAssembly版ライブラリの動作をブラウザで確認するためのデモアプリケーション関連ファイル群を格納しています。
    -   `demo-library/.gitignore`: デモライブラリ開発時にGit管理から除外するファイルを指定します。
    -   `demo-library/biome.json`: デモライブラリのTypeScript/JavaScriptコードに対するBiomeリンターおよびフォーマッターの設定ファイルです。
    -   `demo-library/delay-vibrato-demo.ts`: ディレイビブラート機能のデモを実装するTypeScriptコードです。
    -   `demo-library/delay-vibrato.html`: ディレイビブラートデモのHTMLページです。
    -   `demo-library/globals.d.ts`: デモライブラリ全体で利用されるグローバルな型定義を宣言します。
    -   `demo-library/index.html`: WebAssembly版ライブラリの主要なオンラインデモページです。
    -   `demo-library/library-demo.ts`: Webブラウザでライブラリの基本的な使い方を示すTypeScriptコードです。
    -   `demo-library/log-visualizer.ts`: YM2151レジスタ書き込みログを視覚的に表示するためのTypeScriptコードです。
    -   `demo-library/mml-support.ts`: MML (Music Macro Language) からStandard MIDI Fileへの変換をサポートするデモ用のTypeScriptコードです。
    -   `demo-library/package-lock.json`: `demo-library`のNode.jsパッケージの依存関係とそのバージョンをロックするファイルです。
    -   `demo-library/package.json`: `demo-library`のNode.jsプロジェクトのメタデータと依存関係を定義します。
    -   `demo-library/pop-noise-demo.ts`: ポップノイズ（音源チップのノイズ）の挙動に関するデモを実装するTypeScriptコードです。
    -   `demo-library/pop-noise.html`: ポップノイズデモのHTMLページです。
    -   `demo-library/portamento-soft-lfo-demo.ts`: ポルタメントやソフトLFO（低周波発振器）機能のデモを実装するTypeScriptコードです。
    -   `demo-library/portamento-soft-lfo.html`: ポルタメント・ソフトLFOデモのHTMLページです。
    -   `demo-library/shared-demo.ts`: 複数のデモで共通して利用される機能やユーティリティロジックをまとめたTypeScriptコードです。
    -   `demo-library/style.css`: デモページのレイアウトやデザインを定義するスタイルシートです。
    -   `demo-library/tone-json-attachment.ts`: カスタムYM2151音色をJSONファイルとして添付する機能のデモ用TypeScriptコードです。
    -   `demo-library/tone-json-demo.ts`: JSON音色アタッチメント機能のデモを実装するTypeScriptコードです。
    -   `demo-library/tone-json-mml.ts`: JSON音色とMMLの連携をサポートするデモ用のTypeScriptコードです。
    -   `demo-library/tone-json.html`: JSON音色アタッチメントデモのHTMLページです。
    -   `demo-library/tsconfig.json`: TypeScriptコンパイラの設定ファイルです。
    -   `demo-library/vite.config.ts`: Viteビルドツールの設定ファイルです。
-   `generated-docs/`: Rustのドキュメント生成ツール (`cargo doc`) によって生成されたAPIドキュメントの出力先ディレクトリです。
-   `googled947dc864c270e07.html`: Googleサイト認証のために使用されるHTMLファイルです。
-   `issue-notes/`: 開発過程で発生したIssueに関する詳細なメモや解決策が格納されています。
    -   `issue-notes/*.md`: 各Issueに関連する詳細なMarkdown形式のメモファイルです。
-   `package-lock.json`: プロジェクトルートのNode.jsパッケージの依存関係とそのバージョンをロックするファイルです。
-   `package.json`: プロジェクトルートのNode.jsプロジェクトのメタデータと依存関係を定義します。
-   `src/`: Rustのソースコードのルートディレクトリです。
    -   `src/error.rs`: アプリケーション固有のエラー型とエラー処理ロジックを定義します。
    -   `src/lib.rs`: 本プロジェクトのRustライブラリとしての主要なエントリーポイントで、外部に公開するAPIを定義します。
    -   `src/main.rs`: コマンドラインツールとして本プログラムを実行する際のメインエントリーポイントです。
    -   `src/midi/`: Standard MIDI File (SMF) のパースとイベント処理に関するモジュールです。
        -   `src/midi/events.rs`: MIDIイベントのデータ構造（ノートオン、プログラムチェンジなど）を定義します。
        -   `src/midi/mod.rs`: `midi`モジュール内のサブモジュールを公開し、モジュール全体を構成します。
        -   `src/midi/parser.rs`: SMFを読み込み、MIDIイベントのシーケンスに変換するパースロジックを実装します。
        -   `src/midi/utils.rs`: MIDIデータ処理に関連する共通のユーティリティ関数を提供します。
        -   `src/midi/utils_tests.rs`: `src/midi/utils.rs`で定義された関数の単体テストを格納します。
    -   `src/wasm.rs`: WebAssembly (WASM) バインディングのためのラッパー関数と、JavaScriptとのインターフェースを定義します。
    -   `src/ym2151/`: YM2151 FM音源チップのレジスタログへの変換ロジックを格納するモジュールです。
        -   `src/ym2151/channel_allocation.rs`: MIDIチャンネルをYM2151の限られた8チャンネルに効率的に割り当てるためのロジックを実装します。
        -   `src/ym2151/converter/`: MIDIイベントからYM2151レジスタログへの変換における特定のエフェクト処理に関するサブモジュールです。
            -   `src/ym2151/converter/pitch_effects.rs`: ピッチベンド、ビブラート、ポルタメントなどのピッチ関連エフェクトの処理を実装します。
            -   `src/ym2151/converter/register_effects.rs`: YM2151レジスタに対する様々なエフェクト（エンベロープ設定など）の処理を実装します。
            -   `src/ym2151/converter/waveform.rs`: YM2151の波形設定に関する処理を実装します。
        -   `src/ym2151/converter.rs`: MIDIイベントストリームをYM2151レジスタ書き込みログに変換する主要なロジックを実装します。
        -   `src/ym2151/converter_tests/`: `ym2151`変換処理の特定の側面をテストするための統合テスト群です。
            -   `src/ym2151/converter_tests/basic.rs`: 基本的なMIDIイベントからYM2151ログへの変換テスト。
            -   `src/ym2151/converter_tests/channels.rs`: チャンネル割り当てロジックのテスト。
            -   `src/ym2151/converter_tests/effects.rs`: ピッチエフェクトやレジスタエフェクトのテスト。
            -   `src/ym2151/converter_tests/programs.rs`: プログラムチェンジイベントによる音色切り替えのテスト。
        -   `src/ym2151/converter_tests.rs`: `ym2151`変換テストモジュールのトップレベルファイル。
        -   `src/ym2151/event_processor.rs`: MIDIイベントをYM2151に適した中間イベント形式に変換する処理を担います。
        -   `src/ym2151/event_processor_tests.rs`: `event_processor.rs`の単体テストを格納します。
        -   `src/ym2151/events.rs`: YM2151レジスタ書き込みログイベントや関連するデータ構造を定義します。
        -   `src/ym2151/init.rs`: YM2151チップの初期化シーケンスやデフォルト設定に関するロジックを実装します。
        -   `src/ym2151/mod.rs`: `ym2151`モジュール内のサブモジュールを公開し、モジュール全体を構成します。
        -   `src/ym2151/note_table.rs`: MIDIノート番号とYM2151の周波数パラメータ（F-Number/BLOCK）のマッピングテーブルなどを定義します。
        -   `src/ym2151/tempo_map.rs`: テンポチェンジイベントを処理し、正確な時間計算を行うためのテンポマップを管理します。
        -   `src/ym2151/tone.rs`: YM2151の音色（プログラム）を管理し、カスタム音色JSONの読み込みロジックを実装します。
-   `tests/`: プロジェクト全体の統合テストコードを格納するディレクトリです。
    -   `tests/create_test_midi.py`: 統合テストで使用する様々なMIDIファイルをプログラムで生成するためのPythonスクリプトです。
    -   `tests/integration_conversion.rs`: MIDIからYM2151ログへの変換プロセス全体の統合テストです。
    -   `tests/integration_midi.rs`: MIDIパース機能に関する統合テストです。
    -   `tests/integration_multichannel.rs`: 複数MIDIチャンネルの処理とYM2151チャンネル割り当てに関する統合テストです。
    -   `tests/integration_program_change.rs`: プログラムチェンジイベントとカスタム音色読み込みに関する統合テストです。
    -   `tests/integration_wasm.rs`: WebAssembly (WASM) バインディングが正しく機能するかどうかの統合テストです。
    -   `tests/test_data/`: 統合テストで使用されるサンプルMIDIファイルを格納するディレクトリです。
        -   `tests/test_data/*.mid`: 様々なシナリオをテストするためのMIDIファイルです。
-   `tones/`: MIDIプログラムチェンジに対応するカスタムYM2151音色定義をJSON形式で格納するディレクトリです。
    -   `tones/000.json`: MIDIプログラム0番（アコースティックグランドピアノ）に対応するYM2151音色定義のJSONファイルです。
    -   `tones/README.md`: `tones`ディレクトリの利用方法やJSONフォーマットに関する説明ドキュメントです。

## 関数詳細説明
-   `nextRequestId()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: UI操作や非同期処理の管理のために、一意なリクエストIDを生成します。
    -   引数: なし。
    -   戻り値: `number` (一意な数値ID)。
-   `isLatestRequest(requestId: number)` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: 指定されたリクエストIDが現在処理中の最新のリクエストであるかを確認します。これにより、古い処理結果が誤ってUIに反映されるのを防ぎます。
    -   引数: `requestId` (number): 確認するリクエストのID。
    -   戻り値: `boolean`。
-   `updateOutputWithState(state: object)` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: アプリケーションの内部状態オブジェクトに基づいて、デモの出力エリア（ログ表示など）を更新します。
    -   引数: `state` (object): 更新に利用する状態オブジェクト。
    -   戻り値: なし。
-   `updatePlayButtonState(isPlaying: boolean)` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: オーディオの再生状態（再生中か停止中か）に応じて、再生/停止ボタンの見た目やテキストを更新します。
    -   引数: `isPlaying` (boolean): 現在オーディオが再生中であるかを示す真偽値。
    -   戻り値: なし。
-   `initializeWasm()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: WebAssemblyモジュールを非同期でロードし、Rustで実装されたコア機能をブラウザのJavaScript環境から利用できるように初期化します。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `readAttachmentBytes(elementId: string)` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: 指定されたDOM要素（例: ファイル入力フィールドやテキストエリア）から、カスタム音色JSONなどの添付ファイルの内容をバイトデータとして読み込みます。
    -   引数: `elementId` (string): 添付ファイルの内容を読み込むDOM要素のID。
    -   戻り値: `Promise<Uint8Array | null>` (読み込んだバイトデータ、またはエラーの場合は`null`)。
-   `runConversion(midiBytes: Uint8Array, attachmentBytes: Uint8Array | null)` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: 入力されたMIDIファイルのバイトデータと、オプションの添付ファイル（カスタム音色JSONなど）のバイトデータを使用して、YM2151レジスタ書き込みログへの変換処理を実行します。
    -   引数: `midiBytes` (Uint8Array): 変換対象のStandard MIDI Fileのバイトデータ。 `attachmentBytes` (Uint8Array | null): 変換時に適用するカスタム設定（例: JSON形式の音色データ）のバイトデータ。
    -   戻り値: 変換結果を含む `Promise<object>`。
-   `handlePlay(ym2151Log: object)` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: 変換によって生成されたYM2151レジスタ書き込みログデータを受け取り、それをブラウザのWebAudio APIを利用してFM音源としてエミュレートし、オーディオ再生を開始します。
    -   引数: `ym2151Log` (object): 再生するYM2151レジスタ書き込みログデータ。
    -   戻り値: なし。
-   `setupAttachmentEditor()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: カスタム音色JSONなどを入力・編集するためのUI要素（テキストエリアなど）を初期化し、イベントハンドラを設定します。
    -   引数: なし。
    -   戻り値: なし。
-   `setupMmlInput()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: MML (Music Macro Language) コードを入力するためのUI要素（テキストエリアなど）を初期化し、関連するイベントハンドラを設定します。
    -   引数: なし。
    -   戻り値: なし。
-   `setupMidiInput()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: MIDIファイルをアップロードするためのUI要素（ファイル選択ボタンなど）を初期化し、ファイル選択イベントを処理するハンドラを設定します。
    -   引数: なし。
    -   戻り値: なし。
-   `bootstrapWebYm()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: WebYm2151オーディオ再生システムの初期化と起動を行います。WebAudio APIのコンテキスト作成や、YM2151エミュレータの準備を含みます。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `main()` (`demo-library/delay-vibrato-demo.ts` 他):
    -   役割: 各デモアプリケーションの主要なエントリーポイント。ページのDOMがロードされた後に実行され、UIの初期設定、イベントリスナーの登録、初期データのロードなどを行います。
    -   引数: なし。
    -   戻り値: なし。
-   `playAudioWithOverlay(ym2151Log: object, context: AudioContext)` (`demo-library/globals.d.ts`):
    -   役割: YM2151ログを再生し、同時にUI上にオーバーレイ（視覚化など）を表示する機能です。
    -   引数: `ym2151Log` (object): YM2151レジスタログデータ。 `context` (AudioContext): Web Audio APIのオーディオコンテキスト。
    -   戻り値: なし。
-   `clearAudioCache()` (`demo-library/globals.d.ts`):
    -   役割: デモで使用されるオーディオ関連のキャッシュデータをクリアします。
    -   引数: なし。
    -   戻り値: なし。
-   `initWasm()` (`demo-library/library-demo.ts`):
    -   役割: WebAssemblyモジュールを初期化し、ライブラリデモで使用可能にするためのラッパー関数です。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `displayResult(result: object)` (`demo-library/library-demo.ts`):
    -   役割: 変換処理やその他の操作の結果をデモのUI上に表示します。
    -   引数: `result` (object): 表示する結果データ。
    -   戻り値: なし。
-   `showError(error: Error)` (`demo-library/library-demo.ts`):
    -   役割: 発生したエラーメッセージをデモのUI上に表示し、ユーザーにエラーを通知します。
    -   引数: `error` (Error): 表示するエラーオブジェクト。
    -   戻り値: なし。
-   `setupFileInput()` (`demo-library/library-demo.ts`):
    -   役割: ファイル入力UI要素を設定し、ファイルが選択された際のイベント処理（ファイルの読み込みなど）を行います。
    -   引数: なし。
    -   戻り値: なし。
-   `parseHexByte(hexString: string)` (`demo-library/log-visualizer.ts`):
    -   役割: 16進数形式の文字列（例: "C7"）をパースし、対応するバイト値の数値に変換します。
    -   引数: `hexString` (string): パースする16進数文字列。
    -   戻り値: `number` (変換されたバイト値)。
-   `detectChannel(event: object)` (`demo-library/log-visualizer.ts`):
    -   役割: YM2151レジスタログイベントオブジェクトから、そのイベントがどのYM2151チャンネルに属するかを検出します。
    -   引数: `event` (object): YM2151ログイベントオブジェクト。
    -   戻り値: `number` (チャンネル番号)。
-   `buildNoteSegments(events: Array<object>)` (`demo-library/log-visualizer.ts`):
    -   役割: YM2151ログイベントの配列を分析し、各ノートの開始・終了を示すセグメント（区間）の情報を構築します。視覚化に利用されます。
    -   引数: `events` (Array<object>): YM2151ログイベントの配列。
    -   戻り値: ノートセグメントの配列。
-   `computeKcRange(noteSegments: Array<object>)` (`demo-library/log-visualizer.ts`):
    -   役割: ノートセグメントの配列から、ノートのKey Code (KC) の最小値と最大値を計算します。これも視覚化のスケール調整に必要です。
    -   引数: `noteSegments` (Array<object>): ノートセグメントの配列。
    -   戻り値: `object` (KCの最小値と最大値を含むオブジェクト)。
-   `noteYPosition(kc: number, kcMin: number, kcMax: number, height: number)` (`demo-library/log-visualizer.ts`):
    -   役割: 与えられたKey Code (KC) に基づいて、YM2151ログ視覚化におけるノートの縦方向の描画位置（Y座標）を計算します。
    -   引数: `kc` (number): 描画対象のキーコード。 `kcMin` (number): 検出された最小キーコード。 `kcMax` (number): 検出された最大キーコード。 `height` (number): 描画エリアの高さ。
    -   戻り値: `number` (計算されたY座標)。
-   `normalizeEvents(events: Array<object>)` (`demo-library/log-visualizer.ts`):
    -   役割: YM2151ログイベントを視覚化に適した一貫性のある形式に正規化します。
    -   引数: `events` (Array<object>): YM2151ログイベントの配列。
    -   戻り値: 正規化されたイベントの配列。
-   `laneColor(channel: number)` (`demo-library/log-visualizer.ts`):
    -   役割: 指定されたYM2151チャンネル番号に対応する色（CSSカラー文字列）を返します。チャンネルごとに色分けして視覚的に区別するためです。
    -   引数: `channel` (number): YM2151チャンネル番号。
    -   戻り値: `string` (CSSカラー文字列)。
-   `createLane(container: HTMLElement, channel: number, width: number, height: number, color: string)` (`demo-library/log-visualizer.ts`):
    -   役割: YM2151ログの視覚化のために、特定のチャンネルに対応するグラフィックレーン（通常はSVG要素）を作成し、指定されたコンテナに追加します。
    -   引数: `container` (HTMLElement): レーンを追加するDOMコンテナ要素。 `channel` (number): チャンネル番号。 `width` (number): レーンの幅。 `height` (number): レーンの高さ。 `color` (string): レーンの色。
    -   戻り値: `SVGGElement` (作成されたSVGグループ要素)。
-   `computeTrackWidth(events: Array<object>)` (`demo-library/log-visualizer.ts`):
    -   役割: YM2151ログイベントのタイムスタンプを分析し、ログ全体の時間範囲に基づいて視覚化トラックの適切な幅を計算します。
    -   引数: `events` (Array<object>): YM2151ログイベントの配列。
    -   戻り値: `number` (計算されたトラックの幅)。
-   `createLogVisualizer(containerId: string)` (`demo-library/log-visualizer.ts`):
    -   役割: 指定されたDOM要素IDのコンテナ内に、YM2151レジスタログを視覚化するためのツールインスタンスを生成・初期化します。
    -   引数: `containerId` (string): 視覚化ツールを配置するHTML要素のID。
    -   戻り値: `object` (視覚化ツールインスタンス、レンダリングメソッドなどを含む)。
-   `renderEmpty(message: string)` (`demo-library/log-visualizer.ts`):
    -   役割: ログ視覚化ツールに、データがない場合やエラー発生時に表示する空の状態メッセージをレンダリングします。
    -   引数: `message` (string): 表示するメッセージテキスト。
    -   戻り値: なし。
-   `renderFromJson(jsonLog: object)` (`demo-library/log-visualizer.ts`):
    -   役割: JSON形式のYM2151レジスタログデータを受け取り、それを解析して視覚化ツール上にグラフィックとしてレンダリングします。
    -   引数: `jsonLog` (object): JSON形式のYM2151レジスタログオブジェクト。
    -   戻り値: なし。
-   `ensureGlobalLane(svg: SVGSVGElement, width: number, height: number)` (`demo-library/log-visualizer.ts`):
    -   役割: YM2151ログ視覚化の背景や共通要素を描画するためのグローバルなSVGレーン（トラック）が存在することを確認し、必要に応じて作成します。
    -   引数: `svg` (SVGSVGElement): 親となるSVG要素。 `width` (number): レーンの幅。 `height` (number): レーンの高さ。
    -   戻り値: なし。
-   `setupMmlToSmf(mmlInputId: string, smfOutputId: string)` (`demo-library/mml-support.ts`):
    -   役割: MML入力エリアとSMF出力エリアの間の連携を設定し、MMLコードからSMFへの変換をトリガーするイベントハンドラを登録します。
    -   引数: `mmlInputId` (string): MML入力テキストエリアのDOM要素ID。 `smfOutputId` (string): SMF出力エリアのDOM要素ID。
    -   戻り値: なし。
-   `setupPlayButton()` (`demo-library/pop-noise-demo.ts`):
    -   役割: デモの再生ボタンのUIを設定し、クリックイベントなどのイベントリスナーを登録します。
    -   引数: なし。
    -   戻り値: なし。
-   `bootstrap()` (`demo-library/pop-noise-demo.ts`):
    -   役割: ポップノイズデモアプリケーション全体の初期化と起動を行うメイン関数です。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `ensureWasmInitialized()` (`demo-library/shared-demo.ts`):
    -   役割: WebAssemblyモジュールが既に初期化されていることを確認します。未初期化の場合は初期化処理を呼び出します。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `setStatus(message: string)` (`demo-library/shared-demo.ts`):
    -   役割: デモのUI上に現在のステータスメッセージを表示し、ユーザーに処理の進行状況や結果を伝えます。
    -   引数: `message` (string): 表示するステータスメッセージ。
    -   戻り値: なし。
-   `setEventCountDisplay(count: number)` (`demo-library/shared-demo.ts`):
    -   役割: 処理されたイベントの総数（MIDIイベント数など）をUI上に表示します。
    -   引数: `count` (number): 表示するイベントの数。
    -   戻り値: なし。
-   `ensureWebYm2151()` (`demo-library/shared-demo.ts`):
    -   役割: WebYm2151オーディオ再生ライブラリがWebブラウザ環境にロードされ、利用可能であることを確認します。
    -   引数: なし。
    -   戻り値: `Promise<void>`。
-   `clearWebYmAudioCache()` (`demo-library/shared-demo.ts`):
    -   役割: WebYm2151オーディオ再生ライブラリが内部的に持つキャッシュをクリアします。
    -   引数: なし。
    -   戻り値: なし。
-   `updateOutput(midiEventsJson: string, ym2151LogJson: string, outputElementId: string)` (`demo-library/shared-demo.ts`):
    -   役割: 中間MIDIイベントのJSONと最終的なYM2151ログのJSONを、指定されたUI要素に整形して表示します。
    -   引数: `midiEventsJson` (string): 中間MIDIイベントのJSON文字列。 `ym2151LogJson` (string): YM2151レジスタログのJSON文字列。 `outputElementId` (string): 出力先のDOM要素のID。
    -   戻り値: なし。
-   `parseAttachmentField(attachmentEditorId: string)` (`demo-library/shared-demo.ts`):
    -   役割: 添付ファイルエディタ（テキストエリアなど）からテキスト内容をパースし、JSON形式などの構造化データとして処理します。
    -   引数: `attachmentEditorId` (string): 添付ファイルエディタのDOM要素ID。
    -   戻り値: `string | null` (パースされたテキスト内容、またはエラーの場合は`null`)。
-   `cleanup()` (`demo-library/shared-demo.ts`):
    -   役割: デモアプリケーションが終了またはリセットされる際に、リソースの解放や状態のリセットを行うクリーンアップ処理です。
    -   引数: なし。
    -   戻り値: なし。
-   `buildEventsFromCompact(compactToneJson: object)` (`demo-library/tone-json-attachment.ts`):
    -   役割: 簡潔なフォーマットで記述されたYM2151音色JSONオブジェクトを受け取り、詳細なYM2151イベントの配列に展開して構築します。
    -   引数: `compactToneJson` (object): 簡潔な音色定義JSONオブジェクト。
    -   戻り値: YM2151イベントの配列。
-   `serializeWithStatus(rawText: string, statusElementId: string)` (`demo-library/tone-json-attachment.ts`):
    -   役割: 生のテキストデータをシリアライズし、その処理のステータスを指定されたUI要素に表示します。
    -   引数: `rawText` (string): シリアライズ対象のテキストデータ。 `statusElementId` (string): ステータスを表示するDOM要素のID。
    -   戻り値: `object | null` (シリアライズされたオブジェクト、またはエラーの場合は`null`)。
-   `normalizeAttachmentText(text: string)` (`demo-library/tone-json-attachment.ts`):
    -   役割: 添付されるテキストデータ（例: JSON文字列）を、コメント除去や空白文字の調整などによって正規化します。
    -   引数: `text` (string): 正規化するテキスト。
    -   戻り値: `string` (正規化されたテキスト)。
-   `convertMmlToSmf()` (`demo-library/tone-json-demo.ts`):
    -   役割: MML (Music Macro Language) コードをStandard MIDI File (SMF) 形式に変換する処理をトリガーします。
    -   引数: なし。
    -   戻り値: `Promise<Uint8Array | null>` (SMFのバイトデータ、またはエラーの場合は`null`)。
-   `getMmlParser()` (`demo-library/tone-json-mml.ts`):
    -   役割: MMLコードを解析するためのパーサーインスタンスを取得します。
    -   引数: なし。
    -   戻り値: MMLパーサーオブジェクト。
-   `getParseTreeJsonToSmf(parseTreeJson: object)` (`demo-library/tone-json-mml.ts`):
    -   役割: MMLパーサーによって生成されたパースツリーのJSON表現を受け取り、それをStandard MIDI File (SMF) のバイトデータに変換します。
    -   引数: `parseTreeJson` (object): MMLのパースツリーを示すJSONオブジェクト。
    -   戻り値: `Uint8Array` (SMFバイトデータ)。
-   `treeToJson(tree: object)` (`demo-library/tone-json-mml.ts`):
    -   役割: 構文木（パースツリー）オブジェクトを標準的なJSON形式に変換します。
    -   引数: `tree` (object): 変換する構文木オブジェクト。
    -   戻り値: `object` (JSON形式のオブジェクト)。
-   `ensureMmlRuntime()` (`demo-library/tone-json-mml.ts`):
    -   役割: MMLのパーサーや変換ロジックが実行されるためのランタイム環境が、Webブラウザにロードされて準備ができていることを確認します。
    -   引数: なし。
    -   戻り値: `Promise<void>`。

## 関数呼び出し階層ツリー
```
- nextRequestId (demo-library/delay-vibrato-demo.ts)
  - isLatestRequest ()
    - updateOutputWithState ()
    - updatePlayButtonState ()
    - initializeWasm ()
    - readAttachmentBytes ()
    - runConversion ()
    - handlePlay ()
    - setupAttachmentEditor ()
    - setupMmlInput ()
    - setupMidiInput ()
    - bootstrapWebYm ()
    - main ()
    - playAudioWithOverlay ()
    - createLogVisualizer ()
    - renderFromJson ()
    - setupMmlToSmf ()
    - ensureWasmInitialized ()
    - setStatus ()
    - setEventCountDisplay ()
    - ensureWebYm2151 ()
    - updateOutput ()
    - parseAttachmentField ()
    - setupPlayButton ()
    - bootstrap ()
- initWasm (demo-library/library-demo.ts)
  - displayResult ()
    - showError ()
    - setupFileInput ()
- parseHexByte (demo-library/log-visualizer.ts)
  - detectChannel ()
    - buildNoteSegments ()
    - computeKcRange ()
    - noteYPosition ()
    - normalizeEvents ()
    - laneColor ()
    - createLane ()
    - computeTrackWidth ()
    - renderEmpty ()
    - ensureGlobalLane ()
- getMmlParser ()
  - getParseTreeJsonToSmf ()
    - treeToJson ()
    - ensureMmlRuntime ()
- clearAudioCache ()
- clearWebYmAudioCache ()
  - cleanup ()
- buildEventsFromCompact (demo-library/tone-json-attachment.ts)
  - serializeWithStatus ()
    - normalizeAttachmentText ()
- convertMmlToSmf ()

---
Generated at: 2026-03-08 07:09:03 JST

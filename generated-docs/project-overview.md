Last updated: 2026-04-20

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製アプリケーションおよびライブラリです。
- WebAssembly (WASM) に対応し、ブラウザ環境での利用も可能で、`cat-play-mml`や`web-ym2151`などの関連プロジェクトで活用されています。
- 2パス処理アーキテクチャ、外部JSONファイルからのカスタムYM2151音色読み込み、和音数に基づく静的なチャンネル割り当て戦略が特徴です。

## 技術スタック
- フロントエンド:
    - **HTML/CSS**: デモUIの構造とスタイリングに使用されています。
    - **TypeScript**: デモアプリケーションのロジック開発に使用されており、WASMモジュールとの連携を強化しています。
    - **JavaScript**: WebAssemblyモジュールのロードと実行、およびブラウザDOM操作に使用されます。
    - **Vite**: デモライブラリのビルドツールとして使用され、高速な開発サーバーとバンドルを提供します。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: 入力フォーマットとして、標準MIDIファイルを解析します。
    - **YM2151 FM音源チップ**: 出力されるレジスタ書き込みログの対象となるFM音源チップです。
    - **JSON**: 中間イベントと最終的なYM2151レジスタログ、およびカスタム音色の定義に使用されます。
- 開発ツール:
    - **Rust**: プロジェクトのコアロジックを実装している主要なプログラミング言語です。
    - **Cargo**: Rustの公式ビルドシステムおよびパッケージマネージャーで、依存関係の管理、ビルド、テスト、ドキュメント生成を行います。
    - **git**: ソースコードのバージョン管理システムです。
    - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、Webブラウザで利用可能なパッケージを生成するツールです。
    - **npm / yarn**: JavaScriptエコシステムのパッケージマネージャーで、デモライブラリの依存関係管理に使用されます (package.jsonから推測)。
    - **biome**: TypeScript/JavaScriptのコードフォーマッターおよびリンターです。
- テスト:
    - **`cargo test`**: Rustコードのユニットテストと統合テストを実行します。
    - **`cargo tarpaulin`**: Rustプロジェクトのテストカバレッジを測定し、レポートを生成します。
- ビルドツール:
    - **Cargo**: Rustプロジェクトのビルドを行います。
    - **wasm-pack**: RustクレートをWebAssemblyにビルドし、Web環境向けに最適化されたパッケージを生成します。
    - **Vite**: デモライブラリのフロントエンドビルドを処理します。
- 言語機能:
    - **Rust**: 型安全性、メモリ安全性、高パフォーマンスを保証する強力な言語機能を提供します。
    - **WebAssembly (WASM)**: ブラウザ上でネイティブに近い速度でRustコードを実行可能にするバイナリ形式です。
- 自動化・CI/CD:
    - **`cargo fmt --check`**: Rustコードのフォーマットがスタイルガイドに準拠しているかをチェックします。
    - **`cargo clippy`**: Rustコードのリンターで、一般的なエラー、アンチパターン、パフォーマンスの問題を検出します。
    - **`cargo audit`**: Rustプロジェクトの依存関係に既知のセキュリティ脆弱性がないかをチェックします。
- 開発標準:
    - **Rustfmt**: Rustコードの自動フォーマッターで、コードの一貫性を保ちます。
    - **Clippy**: Rustコードの品質を向上させるためのリンターです。
    - **Biome**: デモライブラリのTypeScriptコードのフォーマットとリンティングを管理します。

## ファイル階層ツリー
```
📄 .gitattributes
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
  📘 log-visualizer-lfo.ts
  📘 log-visualizer-note-segments.ts
  📘 log-visualizer-pitch-canvas.ts
  📘 log-visualizer.ts
  📘 mml-support.ts
  📊 package-lock.json
  📊 package.json
  📘 pop-noise-demo.ts
  📘 pop-noise-detector.ts
  🌐 pop-noise.html
  📘 portamento-soft-lfo-demo.ts
  🌐 portamento-soft-lfo.html
  📘 random-tone.ts
  📘 shared-demo.ts
  🎨 style.css
  📘 tone-interpolation-demo.ts
  🌐 tone-interpolation.html
  📘 tone-json-attachment.ts
  📘 tone-json-demo.ts
  📘 tone-json-mml.ts
  🌐 tone-json.html
  📊 tsconfig.json
  📘 vite.config.ts
  📘 wav-exporter.ts
  📘 waveform-canvas.ts
  📘 waveform-viewer.ts
  📘 ym2151-utils.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 105.md
  📖 111.md
  📖 112.md
  📖 115.md
  📖 123.md
  📖 157.md
  📖 166.md
  📖 177.md
  📖 178.md
  📖 180.md
  📖 181.md
  📖 183.md
  📖 184.md
  📖 185.md
  📖 186.md
  📖 187.md
  📖 188.md
  📖 189.md
  📖 198.md
  📖 200.md
  📖 201.md
  📖 211.md
  📖 22.md
  📖 238.md
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
  📄 api.rs
  📄 error.rs
  📄 lib.rs
  📄 main.rs
  📁 midi/
    📄 events.rs
    📄 mod.rs
    📄 parser.rs
    📄 utils.rs
    📄 utils_tests.rs
  📁 options/
    📄 attachments.rs
    📄 effects.rs
    📄 mod.rs
    📄 tests.rs
  📄 wasm.rs
  📁 ym2151/
    📄 channel_allocation.rs
    📁 converter/
      📄 event_accumulator.rs
      📄 pitch_effects.rs
      📁 register_effects/
        📄 common.rs
        📄 mod.rs
        📄 pop_noise.rs
        📄 register_lfo.rs
        📄 state_cache.rs
        📄 tone_interpolation.rs
      📄 register_fields.rs
      📄 waveform.rs
    📄 converter.rs
    📁 converter_tests/
      📄 attachments.rs
      📁 attachments_change_to_next_tone/
        📄 guards.rs
        📄 interpolation.rs
        📄 keep_fields.rs
        📄 mod.rs
      📄 attachments_program_effects.rs
      📄 basic.rs
      📄 channels.rs
      📄 drums.rs
      📄 effects.rs
      📄 lfo.rs
      📄 portamento.rs
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
  📄 integration_public_api.rs
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

*   **`.gitattributes`**: Gitがファイルを扱う際の属性（例: 行末コード）を定義します。
*   **`.gitignore`**: Gitがバージョン管理から除外するファイルやディレクトリを指定します。
*   **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
*   **`Cargo.toml`**: Rustプロジェクトの設定ファイル。プロジェクト名、バージョン、依存関係、機能フラグなどを定義します。
*   **`LICENSE`**: プロジェクトのライセンス情報（著作権、利用条件）を記載しています。
*   **`README.ja.md` / `README.md`**: プロジェクトの概要、使い方、目的などを日本語と英語で説明するメインドキュメントです。
*   **`WASM_USAGE.md`**: WebAssembly (WASM) を利用したデモの具体的な使用方法やビルド手順を説明します。
*   **`_config.yml`**: GitHub PagesのJekyll設定ファイルである可能性が高いです。
*   **`demo-library/`**: WebAssembly版ライブラリの利用方法を示すブラウザ向けデモアプリケーションのソースコードを格納します。
    *   **`demo-library/biome.json`**: Biomeによるコードフォーマットおよびリンティングの設定ファイルです。
    *   **`demo-library/delay-vibrato-demo.ts`**: ディレイビブラート機能のデモを実装するTypeScriptファイルです。WASMによる変換、UI操作、オーディオ再生を含みます。
    *   **`demo-library/delay-vibrato.html`**: ディレイビブラートデモのウェブページ構造を定義するHTMLファイルです。
    *   **`demo-library/globals.d.ts`**: グローバルスコープで利用される型定義ファイルです。
    *   **`demo-library/index.html`**: メインのデモランディングページです。
    *   **`demo-library/library-demo.ts`**: ライブラリの基本的な使用方法を示すデモのTypeScriptファイルです。
    *   **`demo-library/log-visualizer-lfo.ts`**: YM2151ログのLFO（低周波発振器）イベントを視覚化するコンポーネントです。
    *   **`demo-library/log-visualizer-note-segments.ts`**: ノートイベントを視覚化するためのセグメントを構築するロジックを含みます。
    *   **`demo-library/log-visualizer-pitch-canvas.ts`**: ピッチ情報をキャンバスに描画する機能を提供します。
    *   **`demo-library/log-visualizer.ts`**: YM2151レジスタログを視覚化するための主要なコンポーネントです。
    *   **`demo-library/mml-support.ts`**: MML (Music Macro Language) からSMFへの変換をサポートする機能を提供します。
    *   **`demo-library/package-lock.json`**: `package.json`で定義された依存関係の正確なバージョンとツリー構造を記録します。
    *   **`demo-library/package.json`**: デモライブラリのメタデータとJavaScript/TypeScriptの依存関係を定義します。
    *   **`demo-library/pop-noise-demo.ts`**: ポップノイズ検出機能のデモを実装するTypeScriptファイルです。
    *   **`demo-library/pop-noise-detector.ts`**: YM2151ログ内のポップノイズを検出するロジックを含みます。
    *   **`demo-library/pop-noise.html`**: ポップノイズデモのウェブページ構造を定義するHTMLファイルです。
    *   **`demo-library/portamento-soft-lfo-demo.ts`**: ポルタメントやソフトLFOのデモを実装するTypeScriptファイルです。
    *   **`demo-library/portamento-soft-lfo.html`**: ポルタメント・ソフトLFOデモのウェブページ構造を定義するHTMLファイルです。
    *   **`demo-library/random-tone.ts`**: ランダムなYM2151音色を生成するためのユーティリティ関数を提供します。
    *   **`demo-library/shared-demo.ts`**: 複数のデモ間で共有される共通のロジック（WASM初期化、ステータス表示など）を定義します。
    *   **`demo-library/style.css`**: デモUIのスタイルを定義するCSSファイルです。
    *   **`demo-library/tone-interpolation-demo.ts`**: 音色補間機能のデモを実装するTypeScriptファイルです。
    *   **`demo-library/tone-interpolation.html`**: 音色補間デモのウェブページ構造を定義するHTMLファイルです。
    *   **`demo-library/tone-json-attachment.ts`**: 音色JSONデータの添付とシリアライズに関するユーティリティを提供します。
    *   **`demo-library/tone-json-demo.ts`**: 音色JSONの読み込みと適用に関するデモを実装するTypeScriptファイルです。
    *   **`demo-library/tone-json-mml.ts`**: MMLから音色JSONを生成する機能や、それをSMFに変換するサポートを提供します。
    *   **`demo-library/tone-json.html`**: 音色JSONデモのウェブページ構造を定義するHTMLファイルです。
    *   **`demo-library/vite.config.ts`**: Viteビルドツールの設定ファイルです。
    *   **`demo-library/wav-exporter.ts`**: 生成されたオーディオデータをWAV形式でエクスポートする機能を提供します。
    *   **`demo-library/waveform-canvas.ts`**: 音源波形を描画するためのキャンバス操作ロジックを含みます。
    *   **`demo-library/waveform-viewer.ts`**: 音源波形を視覚化し、ズームや再生位置調整などの機能を提供するコンポーネントです。
    *   **`demo-library/ym2151-utils.ts`**: YM2151関連のユーティリティ関数（例: 16進数パース）を含みます。
*   **`generated-docs/`**: `cargo doc`などによって生成されたドキュメントが格納されるディレクトリです。
*   **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルである可能性が高いです。
*   **`issue-notes/`**: 開発中の課題や検討事項に関するメモ（開発者向け情報のため、詳細は省略）。
*   **`src/`**: Rustソースコードのルートディレクトリです。
    *   **`src/api.rs`**: ライブラリとして外部に公開されるAPIの定義を含みます。
    *   **`src/error.rs`**: プロジェクト内で使用されるカスタムエラー型を定義します。
    *   **`src/lib.rs`**: Rustライブラリクレートのエントリーポイント。主要な機能やモジュールを公開します。WASMバインディングもここから公開されることがあります。
    *   **`src/main.rs`**: コマンドラインアプリケーションのエントリーポイントです。
    *   **`src/midi/`**: MIDIファイル解析に関連するモジュールです。
        *   **`src/midi/events.rs`**: MIDIイベントのデータ構造を定義します。
        *   **`src/midi/parser.rs`**: Standard MIDI Files (SMF) を解析し、内部の中間イベント形式に変換するロジックを含みます。
        *   **`src/midi/utils.rs`**: MIDIデータ処理に関するユーティリティ関数を提供します。
    *   **`src/options/`**: 変換オプションや設定に関連するモジュールです。
        *   **`src/options/attachments.rs`**: カスタム音色などの添付ファイルを処理するロジックを定義します。
        *   **`src/options/effects.rs`**: YM2151ログに適用されるエフェクト（効果）に関する設定を定義します。
    *   **`src/wasm.rs`**: WebAssembly (WASM) バインディングを定義し、Rustの機能をJavaScriptから呼び出せるようにします。
    *   **`src/ym2151/`**: YM2151レジスタログ変換に関連するモジュールです。
        *   **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのチャンネルに割り当てるための戦略（和音数ベース、ドラム優先など）を実装します。
        *   **`src/ym2151/converter.rs`**: MIDI中間イベントをYM2151レジスタ書き込みログに変換する主要なロジックを実装します。
        *   **`src/ym2151/converter/event_accumulator.rs`**: イベントを一時的に蓄積し、変換処理に渡す役割を担います。
        *   **`src/ym2151/converter/pitch_effects.rs`**: ピッチベンドやビブラートなど、ピッチに関連するYM2151エフェクトを処理します。
        *   **`src/ym2151/converter/register_effects/`**: 特定のレジスタ操作による特殊効果を処理するモジュール群です。
            *   **`src/ym2151/converter/register_effects/pop_noise.rs`**: YM2151のポッノイズ低減に関するレジスタ操作を処理します。
            *   **`src/ym2151/converter/register_effects/register_lfo.rs`**: レジスタによるLFO（低周波発振器）効果を処理します。
            *   **`src/ym2151/converter/register_effects/tone_interpolation.rs`**: 音色補間に関するレジスタ操作を処理します。
        *   **`src/ym2151/converter/register_fields.rs`**: YM2151レジスタの各フィールドに関する定義を含みます。
        *   **`src/ym2151/event_processor.rs`**: YM2151の個々のイベントを処理するロジックを含みます。
        *   **`src/ym2151/events.rs`**: YM2151イベントのデータ構造を定義します。
        *   **`src/ym2151/init.rs`**: YM2151チップの初期化に関するレジスタ設定を定義します。
        *   **`src/ym2151/note_table.rs`**: ノート番号とYM2151の周波数設定値のマッピングを管理します。
        *   **`src/ym2151/tempo_map.rs`**: MIDIのテンポ情報からYM2151のタイムラインを構築します。
        *   **`src/ym2151/tone.rs`**: YM2151音色のデータ構造と管理ロジックを定義します。
*   **`tests/`**: 統合テストコードを格納するディレクトリです。
    *   **`tests/create_test_midi.py`**: テスト用のMIDIファイルを生成するPythonスクリプトです。
    *   **`tests/integration_*.rs`**: 変換、MIDI解析、マルチチャンネル、プログラムチェンジ、公開API、WASM機能など、各側面に関する統合テストです。
    *   **`tests/test_data/`**: 統合テストで使用されるサンプルMIDIファイルを格納します。
*   **`tones/`**: カスタムYM2151音色定義をJSON形式で格納するディレクトリです。
    *   **`tones/000.json`**: プログラム0番（アコースティックグランドピアノ）のデフォルト音色定義です。
    *   **`tones/README.md`**: カスタム音色JSONファイルのフォーマットと使用方法について説明します。

## 関数詳細説明

*   **`computeHash(data: string)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 与えられた文字列からハッシュ値を計算します。
    *   引数: `data` (文字列) - ハッシュ化する入力データ。
    *   戻り値: (文字列) - 計算されたハッシュ値。
    *   機能: リクエストの一意性識別やキャッシュのキー生成などに利用される可能性があります。
*   **`nextRequestId()`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 次のリクエストIDを生成し、返します。
    *   引数: なし。
    *   戻り値: (数値) - 新しい一意のリクエストID。
    *   機能: 非同期処理において、最新のリクエストを追跡するために使用されます。
*   **`isLatestRequest(requestId: number)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 指定されたリクエストIDが現在処理されている最新のリクエストであるかを確認します。
    *   引数: `requestId` (数値) - チェックするリクエストID。
    *   戻り値: (真偽値) - 最新であれば`true`、そうでなければ`false`。
    *   機能: 古い非同期処理の結果が新しい結果を上書きするのを防ぐために使用されます。
*   **`updateOutputWithState(state: any)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: アプリケーションのUI出力エリアを、提供された状態オブジェクトに基づいて更新します。
    *   引数: `state` (任意の型) - 更新するUIの状態情報を含むオブジェクト。
    *   戻り値: なし。
    *   機能: 変換結果やデバッグ情報、エラーメッセージなどをユーザーインターフェースに表示します。
*   **`updatePlayButtonState(isPlaying: boolean)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: オーディオ再生ボタンの状態（再生中か停止中か）を更新します。
    *   引数: `isPlaying` (真偽値) - 現在オーディオが再生中であれば`true`、そうでなければ`false`。
    *   戻り値: なし。
    *   機能: 再生ボタンのテキストや有効/無効状態を切り替えることで、ユーザーに現在の再生状態をフィードバックします。
*   **`initializeWasm()`** (demo-library/delay-vibrato-demo.ts):
    *   役割: WebAssembly (WASM) モジュールを初期化し、Rustでコンパイルされたコードをブラウザ環境で利用可能にします。
    *   引数: なし。
    *   戻り値: (Promise<void>) - 初期化が完了すると解決するPromise。
    *   機能: `smf-to-ym2151log-rust`のコアロジックをブラウザで実行するための準備を行います。
*   **`readAttachmentBytes(inputElementId: string)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 指定されたIDを持つファイル入力要素から、添付ファイルのバイトデータを非同期で読み込みます。
    *   引数: `inputElementId` (文字列) - ファイル入力要素のDOM ID。
    *   戻り値: (Promise<Uint8Array | null>) - ファイルのバイトデータ、またはファイルが選択されていない場合は`null`。
    *   機能: カスタムYM2151音色ファイルなどの外部ファイルを読み込むために使用されます。
*   **`runConversion(midiBytes: Uint8Array, attachments: any[], options: any)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: MIDIバイトデータとオプションの添付データ、設定を受け取り、Rust/WASMモジュールを使用してYM2151ログへの変換を実行します。
    *   引数:
        *   `midiBytes` (Uint8Array) - 変換するMIDIファイルのバイトデータ。
        *   `attachments` (任意の型配列) - カスタム音色などの添付データ。
        *   `options` (任意の型) - 変換設定を含むオブジェクト。
    *   戻り値: (Promise<any>) - 変換結果（YM2151ログなど）を含むPromise。
    *   機能: プロジェクトの主要な機能であるMIDIからYM2151ログへの変換をトリガーします。
*   **`handlePlay(ym2151Log: any)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 生成されたYM2151ログを基に、オーディオ再生を開始します。
    *   引数: `ym2151Log` (任意の型) - 再生するYM2151レジスタ書き込みログデータ。
    *   戻り値: (Promise<void>) - 再生処理が完了すると解決するPromise。
    *   機能: 変換結果を実際に音として聴くためのオーディオ再生ロジックを実行します。
*   **`setupAttachmentEditor(editorId: string)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 指定されたIDの添付ファイルエディタを初期設定します。
    *   引数: `editorId` (文字列) - 添付ファイルエディタ要素のDOM ID。
    *   戻り値: なし。
    *   機能: ユーザーがカスタム音色などの添付データを編集・入力するためのUIを設定します。
*   **`setupMmlInput(inputId: string, outputId: string)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: MML (Music Macro Language) 入力フィールドを設定し、MMLからSMFへの変換ロジックをバインドします。
    *   引数:
        *   `inputId` (文字列) - MML入力フィールドのDOM ID。
        *   `outputId` (文字列) - SMF出力表示エリアのDOM ID。
    *   戻り値: なし。
    *   機能: MML入力に対応するデモで、ユーザーがMMLコードを入力すると自動的にSMFに変換されるようにします。
*   **`setupMidiInput(inputId: string)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: MIDIファイル入力フィールドを設定します。
    *   引数: `inputId` (文字列) - MIDIファイル入力要素のDOM ID。
    *   戻り値: なし。
    *   機能: ユーザーがMIDIファイルをアップロードできるようにするためのUI要素を設定します。
*   **`bootstrapWebYm()`** (demo-library/delay-vibrato-demo.ts):
    *   役割: WebYM2151プレイヤーを初期化します。
    *   引数: なし。
    *   戻り値: (Promise<void>) - 初期化が完了すると解決するPromise。
    *   機能: ブラウザ上でYM2151のサウンドをエミュレートするためのプレイヤーを準備します。
*   **`applyRandomToneToAttachment()`** (demo-library/delay-vibrato-demo.ts):
    *   役割: 添付ファイルエディタにランダムなYM2151音色を生成して適用します。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: ユーザーが手動で音色を作成する手間を省き、ランダムな音色で変換・再生を試すことができます。
*   **`setupRandomToneButton(buttonId: string)`** (demo-library/delay-vibrato-demo.ts):
    *   役割: ランダム音色生成ボタンを設定します。
    *   引数: `buttonId` (文字列) - ランダム音色ボタンのDOM ID。
    *   戻り値: なし。
    *   機能: ボタンクリック時に`applyRandomToneToAttachment`を呼び出すイベントリスナーを登録します。
*   **`main()`** (demo-library/delay-vibrato-demo.ts):
    *   役割: デモアプリケーションの主要なエントリーポイントです。ページのロード時に実行される初期化処理やイベントリスナーの設定を行います。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: デモ全体の起動と初期設定を統括します。
*   **`initWasm(options: any)`** (demo-library/library-demo.ts):
    *   役割: WebAssemblyモジュールを初期化し、必要に応じて設定を適用します。
    *   引数: `options` (任意の型) - WASMモジュールの初期化オプション。
    *   戻り値: (Promise<void>) - 初期化が完了すると解決するPromise。
    *   機能: デモ内でWASM機能を使用するための共通の初期化処理です。
*   **`displayResult(midiEvents: any, ym2151Log: any)`** (demo-library/library-demo.ts):
    *   役割: MIDIイベントとYM2151ログの変換結果をUIに表示します。
    *   引数:
        *   `midiEvents` (任意の型) - 解析されたMIDIイベントデータ。
        *   `ym2151Log` (任意の型) - 変換されたYM2151ログデータ。
    *   戻り値: なし。
    *   機能: 変換の成功をユーザーに示し、結果データを閲覧できるようにします。
*   **`showError(error: any)`** (demo-library/library-demo.ts):
    *   役割: エラーメッセージをユーザーインターフェースに表示します。
    *   引数: `error` (任意の型) - 表示するエラー情報。
    *   戻り値: なし。
    *   機能: 変換や再生中に発生した問題をユーザーに通知します。
*   **`setupFileInput(fileInputId: string, outputDivId: string)`** (demo-library/library-demo.ts):
    *   役割: ファイル入力要素を設定し、ファイルが選択された際の処理を定義します。
    *   引数:
        *   `fileInputId` (文字列) - ファイル入力要素のDOM ID。
        *   `outputDivId` (文字列) - 出力結果を表示する`div`要素のDOM ID。
    *   戻り値: なし。
    *   機能: ユーザーがMIDIファイルを選択し、その内容を読み込んで変換処理をトリガーします。
*   **`resolveRegisterForChannel(ch: number, regMap: Map<number, number>)`** (demo-library/log-visualizer-lfo.ts):
    *   役割: 指定されたチャンネルのレジスタ値を解決します。
    *   引数:
        *   `ch` (数値) - チャンネル番号。
        *   `regMap` (Map<number, number>) - レジスタ番号と値のマッピング。
    *   戻り値: (数値) - 解決されたレジスタ値。
    *   機能: LFO視覚化のために、特定のチャンネルのLFOレジスタ値を特定します。
*   **`collectLfoEvents(log: any)`** (demo-library/log-visualizer-lfo.ts):
    *   役割: YM2151ログからLFO（低周波発振器）に関するイベントを収集します。
    *   引数: `log` (任意の型) - YM2151レジスタ書き込みログ。
    *   戻り値: (配列) - 収集されたLFOイベントの配列。
    *   機能: ログ全体からLFO関連のレジスタ変更を抽出し、LFOの動作を分析できるようにします。
*   **`renderLfoLane(ctx: CanvasRenderingContext2D, events: any[], timeToX: (time: number) => number)`** (demo-library/log-visualizer-lfo.ts):
    *   役割: LFOイベントをキャンバス上の「レーン」として描画します。
    *   引数:
        *   `ctx` (CanvasRenderingContext2D) - 描画コンテキスト。
        *   `events` (任意の型配列) - 描画するLFOイベントの配列。
        *   `timeToX` ((time: number) => number) - タイムスタンプをキャンバスのX座標に変換する関数。
    *   戻り値: なし。
    *   機能: LFOの変化を時間軸に沿ってグラフィカルに表示し、視覚的に理解しやすくします。
*   **`buildNoteSegments(log: any)`** (demo-library/log-visualizer-note-segments.ts):
    *   役割: YM2151ログからノートイベントに基づいた視覚化用のセグメントを構築します。
    *   引数: `log` (任意の型) - YM2151レジスタ書き込みログ。
    *   戻り値: (配列) - ノートセグメントの配列。
    *   機能: タイムライン上でノートの開始と終了を表現するためのデータ構造を生成します。
*   **`notePitch(noteNum: number)`** (demo-library/log-visualizer-note-segments.ts):
    *   役割: MIDIノート番号から対応するピッチ（周波数）を計算します。
    *   引数: `noteNum` (数値) - MIDIノート番号。
    *   戻り値: (数値) - 計算されたピッチ。
    *   機能: 視覚化のためにノートの高さを決定するために使用されます。
*   **`computePitchRange(segments: any[])`** (demo-library/log-visualizer-note-segments.ts):
    *   役割: ノートセグメントの配列から、最小ピッチと最大ピッチを計算し、全体的なピッチ範囲を決定します。
    *   引数: `segments` (任意の型配列) - ノートセグメントの配列。
    *   戻り値: ({ min: number, max: number }) - 最小ピッチと最大ピッチを含むオブジェクト。
    *   機能: ピッチキャンバスの描画範囲を動的に調整するために使用されます。
*   **`noteYPosition(pitch: number, minPitch: number, maxPitch: number, canvasHeight: number)`** (demo-library/log-visualizer-note-segments.ts):
    *   役割: 与えられたピッチを、指定されたピッチ範囲とキャンバスの高さに基づいてY座標にマッピングします。
    *   引数:
        *   `pitch` (数値) - マッピングするピッチ。
        *   `minPitch` (数値) - ピッチ範囲の最小値。
        *   `maxPitch` (数値) - ピッチ範囲の最大値。
        *   `canvasHeight` (数値) - キャンバスの高さ（ピクセル）。
    *   戻り値: (数値) - キャンバス上のY座標。
    *   機能: ピッチキャンバス上でノートを正確な垂直位置に描画するために使用されます。
*   **`renderPitchCanvas(canvas: HTMLCanvasElement, log: any, minPitch: number, maxPitch: number)`** (demo-library/log-visualizer-pitch-canvas.ts):
    *   役割: YM2151ログのピッチ情報を指定されたキャンバスに描画します。
    *   引数:
        *   `canvas` (HTMLCanvasElement) - 描画対象のHTMLキャンバス要素。
        *   `log` (任意の型) - YM2151レジスタ書き込みログ。
        *   `minPitch` (数値) - 描画範囲の最小ピッチ。
        *   `maxPitch` (数値) - 描画範囲の最大ピッチ。
    *   戻り値: なし。
    *   機能: 各ノートのピッチ変化を時間軸に沿ってグラフィカルに表示し、メロディラインなどを視覚的に確認できるようにします。
*   **`detectChannel(log: any)`** (demo-library/log-visualizer.ts):
    *   役割: YM2151ログからアクティブなチャンネルを検出します。
    *   引数: `log` (任意の型) - YM2151レジスタ書き込みログ。
    *   戻り値: (配列) - アクティブなチャンネル番号の配列。
    *   機能: ログに実際にイベントがあるチャンネルのみを抽出し、視覚化の対象とします。
*   **`normalizeEvents(log: any)`** (demo-library/log-visualizer.ts):
    *   役割: YM2151ログ内のイベントデータを正規化し、視覚化に適した形式に変換します。
    *   引数: `log` (任意の型) - YM2151レジスタ書き込みログ。
    *   戻り値: (オブジェクト) - 正規化されたイベントデータ。
    *   機能: イベントのタイムスタンプや値のフォーマットを統一し、描画処理を容易にします。
*   **`laneColor(channel: number)`** (demo-library/log-visualizer.ts):
    *   役割: 指定されたチャンネル番号に基づいて、対応する色コードを返します。
    *   引数: `channel` (数値) - チャンネル番号。
    *   戻り値: (文字列) - CSSの色コード。
    *   機能: 複数のチャンネルを視覚化する際に、各チャンネルを区別しやすくするために色分けします。
*   **`createLane(container: HTMLElement)`** (demo-library/log-visualizer.ts):
    *   役割: ログ視覚化のために、新しい「レーン」（チャンネルごとの表示エリア）のDOM要素を作成し、コンテナに追加します。
    *   引数: `container` (HTMLElement) - レーンを追加する親DOM要素。
    *   戻り値: (HTMLElement) - 作成されたレーン要素。
    *   機能: 各YM2151チャンネルのイベントを個別の視覚化エリアに表示するための構造を生成します。
*   **`computeTrackWidth(container: HTMLElement)`** (demo-library/log-visualizer.ts):
    *   役割: 視覚化トラックの推奨される幅を計算します。
    *   引数: `container` (HTMLElement) - トラックを含むコンテナ要素。
    *   戻り値: (数値) - 計算されたトラックの幅。
    *   機能: 視覚化エリアのレイアウトを調整するために使用されます。
*   **`formatInactiveChannels(channels: number[])`** (demo-library/log-visualizer.ts):
    *   役割: 非アクティブなYM2151チャンネルのリストを整形して表示可能な文字列にします。
    *   引数: `channels` (数値配列) - 非アクティブなチャンネル番号の配列。
    *   戻り値: (文字列) - フォーマットされた文字列。
    *   機能: どのチャンネルが利用されていないかをユーザーに分かりやすく伝えます。
*   **`createLogVisualizer(container: HTMLElement)`** (demo-library/log-visualizer.ts):
    *   役割: YM2151ログを視覚化するための主要なUIコンポーネントを生成し、指定されたコンテナにアタッチします。
    *   引数: `container` (HTMLElement) - ビジュアライザーを追加する親DOM要素。
    *   戻り値: (オブジェクト) - 視覚化ツールのインターフェース（`renderFromJson`などのメソッドを含む）。
    *   機能: ログ視覚化ツールのインスタンスを作成し、その後の描画処理を管理します。
*   **`renderEmpty()`** (demo-library/log-visualizer.ts):
    *   役割: ログがない状態の空の視覚化表示をレンダリングします。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: 初期状態やエラー時など、表示するログがない場合に空のプレースホルダーを表示します。
*   **`renderFromJson(logJson: any)`** (demo-library/log-visualizer.ts):
    *   役割: YM2151ログのJSONデータを受け取り、それを解析して視覚化コンポーネントに描画します。
    *   引数: `logJson` (任意の型) - YM2151レジスタ書き込みログのJSONデータ。
    *   戻り値: なし。
    *   機能: 変換されたYM2151ログを実際にグラフィカルな形でユーザーに提示します。
*   **`ensureGlobalLane(laneId: string)`** (demo-library/log-visualizer.ts):
    *   役割: 指定されたIDのグローバルレーンが存在することを確認し、なければ作成します。
    *   引数: `laneId` (文字列) - グローバルレーンのDOM ID。
    *   戻り値: (HTMLElement) - グローバルレーン要素。
    *   機能: チャンネルに依存しない共通の視覚化要素（例: テンポトラック）を管理します。
*   **`setLfoRegisters(registers: any)`** (demo-library/log-visualizer.ts):
    *   役割: LFOレジスタの値を設定し、視覚化に反映させます。
    *   引数: `registers` (任意の型) - 設定するLFOレジスタ値を含むオブジェクト。
    *   戻り値: なし。
    *   機能: LFOの動作を動的に調整したり、デモで特定のLFO設定を適用したりするために使用されます。
*   **`setupMmlToSmf(mmlInputId: string, smfOutputId: string)`** (demo-library/mml-support.ts):
    *   役割: MML入力とSMF出力のUI要素を設定し、MMLからSMFへの変換機能をバインドします。
    *   引数:
        *   `mmlInputId` (文字列) - MML入力フィールドのDOM ID。
        *   `smfOutputId` (文字列) - SMF出力表示エリアのDOM ID。
    *   戻り値: なし。
    *   機能: ユーザーがMMLコードを入力すると、バックエンドのMMLパーサーを通じてSMFデータに変換されるようにします。
*   **`detectPopNoise(ym2151Log: any)`** (demo-library/pop-noise-detector.ts):
    *   役割: YM2151レジスタ書き込みログを分析し、潜在的なポップノイズの発生箇所を検出します。
    *   引数: `ym2151Log` (任意の型) - YM2151レジスタ書き込みログ。
    *   戻り値: (配列) - 検出されたポップノイズイベントの配列。
    *   機能: YM2151音源の特性上発生しやすいノイズを特定し、その対策のデモなどに利用されます。
*   **`getToneEditorGenerator(attachmentText: string)`** (demo-library/random-tone.ts):
    *   役割: 添付された音色JSONテキストに基づいて、音色エディタのジェネレーターを取得します。
    *   引数: `attachmentText` (文字列) - 添付された音色JSONテキスト。
    *   戻り値: (関数) - 音色設定を生成する関数。
    *   機能: 音色エディタが既存の音色設定を基に新しいランダムな音色を生成する際に使用されます。
*   **`generateRandomToneRegisters(existing?: any)`** (demo-library/random-tone.ts):
    *   役割: ランダムなYM2151音色レジスタ値を生成します。既存の音色データがあればそれを参考にします。
    *   引数: `existing` (任意の型, オプション) - 既存の音色レジスタデータ。
    *   戻り値: (オブジェクト) - 生成されたランダムな音色レジスタ値。
    *   機能: デモで素早く異なる音色を試すためのランダム生成機能を提供します。
*   **`generateRandomInterpolationPairRegisters(existing?: any)`** (demo-library/random-tone.ts):
    *   役割: 音色補間用のランダムなレジスタペアを生成します。既存のデータがあればそれを参考にします。
    *   引数: `existing` (任意の型, オプション) - 既存の補間ペアレジスタデータ。
    *   戻り値: (オブジェクト) - 生成されたランダムな補間ペアレジスタ値。
    *   機能: 音色補間デモで、ランダムな補間設定を素早く試すために使用されます。
*   **`parseAttachmentEntries(attachment: any)`** (demo-library/random-tone.ts):
    *   役割: 添付データから音色エントリーを解析します。
    *   引数: `attachment` (任意の型) - 解析する添付データ。
    *   戻り値: (Map<string, any>) - 解析されたエントリーのマッピング。
    *   機能: 音色設定が記述された添付ファイルを構造化されたデータとして読み込みます。
*   **`validateRandomToneAttachment(text: string)`** (demo-library/random-tone.ts):
    *   役割: ランダムに生成された音色添付テキストのJSON構造が正しいか検証します。
    *   引数: `text` (文字列) - 検証する添付JSONテキスト。
    *   戻り値: (真偽値) - 有効であれば`true`、無効であれば`false`。
    *   機能: 不正なJSON形式が原因でエラーが発生するのを防ぎます。
*   **`upsertEntryRegisters(entries: Map<string, any>, entryKey: string, registers: any)`** (demo-library/random-tone.ts):
    *   役割: 指定されたエントリーキーとレジスタ値を使って、音色エントリーを更新または挿入します。
    *   引数:
        *   `entries` (Map<string, any>) - 更新するエントリーのマッピング。
        *   `entryKey` (文字列) - 更新または挿入するエントリーのキー。
        *   `registers` (任意の型) - 設定するレジスタ値。
    *   戻り値: (Map<string, any>) - 更新されたエントリーのマッピング。
    *   機能: 音色データの部分的な更新や追加を行います。
*   **`upsertAttachmentRegisters(attachments: Map<number, any>, program: number, registers: any)`** (demo-library/random-tone.ts):
    *   役割: プログラム番号とレジスタ値を使って、添付音色設定を更新または挿入します。
    *   引数:
        *   `attachments` (Map<number, any>) - 更新する添付音色のマッピング。
        *   `program` (数値) - 更新または挿入するプログラム番号。
        *   `registers` (任意の型) - 設定するレジスタ値。
    *   戻り値: (Map<number, any>) - 更新された添付音色のマッピング。
    *   機能: プログラムチェンジに対応するカスタム音色データを管理・更新します。
*   **`upsertInterpolationAttachmentRegisters(attachments: Map<number, any>, program: number, pairRegisters: any)`** (demo-library/random-tone.ts):
    *   役割: プログラム番号とレジスタペア値を使って、音色補間用の添付設定を更新または挿入します。
    *   引数:
        *   `attachments` (Map<number, any>) - 更新する添付音色のマッピング。
        *   `program` (数値) - 更新または挿入するプログラム番号。
        *   `pairRegisters` (任意の型) - 設定するレジスタペア値。
    *   戻り値: (Map<number, any>) - 更新された添付音色のマッピング。
    *   機能: 音色補間デモで、プログラムチェンジに対応する補間設定を管理・更新します。
*   **`buildRandomInterpolationAttachment()`** (demo-library/random-tone.ts):
    *   役割: ランダムな音色補間添付ファイルを構築します。
    *   引数: なし。
    *   戻り値: (文字列) - JSON形式のランダムな音色補間添付データ。
    *   機能: 音色補間デモで、ランダムな設定をすぐに試せるように添付データ全体を生成します。
*   **`ensureWasmInitialized()`** (demo-library/shared-demo.ts):
    *   役割: WebAssemblyモジュールが確実に初期化されていることを確認します。初期化されていない場合は初期化を試みます。
    *   引数: なし。
    *   戻り値: (Promise<void>) - WASMが初期化されると解決するPromise。
    *   機能: 複数のデモでWASM機能を利用する前に共通して呼び出され、確実に利用可能な状態にします。
*   **`setStatus(message: string)`** (demo-library/shared-demo.ts):
    *   役割: UIのステータス表示領域にメッセージを設定します。
    *   引数: `message` (文字列) - 表示するステータスメッセージ。
    *   戻り値: なし。
    *   機能: ユーザーに現在の処理状況や結果をリアルタイムで通知します。
*   **`setEventCountDisplay(count: number)`** (demo-library/shared-demo.ts):
    *   役割: UIのイベント数表示領域に数値を設定します。
    *   引数: `count` (数値) - 表示するイベント数。
    *   戻り値: なし。
    *   機能: 変換されたYM2151イベントの総数などをユーザーに表示します。
*   **`ensureWebYm2151()`** (demo-library/shared-demo.ts):
    *   役割: WebYM2151プレイヤーが利用可能であることを確認し、必要に応じて初期化します。
    *   引数: なし。
    *   戻り値: (Promise<any>) - WebYM2151プレイヤーのインスタンスを解決するPromise。
    *   機能: オーディオ再生機能を利用するデモで共通して呼び出されます。
*   **`clearWebYmAudioCache()`** (demo-library/shared-demo.ts):
    *   役割: WebYM2151プレイヤーのオーディオキャッシュをクリアします。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: 新しい変換結果を再生する前に、古いオーディオデータをクリアしてメモリを解放します。
*   **`updateOutput(content: string, append: boolean = false)`** (demo-library/shared-demo.ts):
    *   役割: 指定されたコンテンツでUIの出力領域を更新します。必要に応じて既存の内容に追加できます。
    *   引数:
        *   `content` (文字列) - 出力するコンテンツ。
        *   `append` (真偽値, オプション) - `true`の場合、コンテンツを既存の内容に追加します。デフォルトは`false`（上書き）。
    *   戻り値: なし。
    *   機能: 変換ログ、デバッグ情報、エラーメッセージなどをUIに出力します。
*   **`parseAttachmentField(text: string)`** (demo-library/shared-demo.ts):
    *   役割: 添付フィールドのテキスト内容を解析し、構造化されたデータに変換します。
    *   引数: `text` (文字列) - 解析する添付フィールドのテキスト。
    *   戻り値: (任意の型) - 解析された添付データ。
    *   機能: ユーザーが入力したJSON形式の添付データを処理可能なオブジェクトに変換します。
*   **`cleanup()`** (demo-library/shared-demo.ts):
    *   役割: デモアプリケーションのクリーンアップ処理を実行します。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: デモのリソース解放や状態リセットなどを行います。
*   **`mod(n: number, m: number)`** (demo-library/shared-demo.ts):
    *   役割: 剰余演算（`n % m`）を実行し、結果が常に正の値になるようにします。
    *   引数:
        *   `n` (数値) - 被除数。
        *   `m` (数値) - 除数。
    *   戻り値: (数値) - 計算された正の剰余。
    *   機能: ループ処理やインデックス計算などで、結果が負になるのを防ぎます。
*   **`buildEventsFromCompact(text: string)`** (demo-library/tone-json-attachment.ts):
    *   役割: コンパクトな形式のJSONテキストから、YM2151イベントの配列を構築します。
    *   引数: `text` (文字列) - コンパクトなJSON形式の入力テキスト。
    *   戻り値: (オブジェクト) - 構築されたイベントとステータスを含むオブジェクト。
    *   機能: 簡潔に記述された音色設定を、内部で使用される詳細なイベント形式に展開します。
*   **`serializeWithStatus(data: any)`** (demo-library/tone-json-attachment.ts):
    *   役割: データオブジェクトをJSON文字列にシリアライズし、成功/失敗のステータス情報を含めます。
    *   引数: `data` (任意の型) - シリアライズするデータ。
    *   戻り値: (オブジェクト) - シリアライズされた文字列とステータスを含むオブジェクト。
    *   機能: デモUIでJSONデータを表示する際に、パースエラーの有無などをユーザーに通知します。
*   **`normalizeAttachmentText(text: string)`** (demo-library/tone-json-attachment.ts):
    *   役割: 添付ファイルとして入力されたJSONテキストを正規化し、コメントなどを除去してクリーンなJSON形式にします。
    *   引数: `text` (文字列) - 正規化する入力テキスト。
    *   戻り値: (文字列) - 正規化されたJSONテキスト。
    *   機能: ユーザーが入力したJSONの書式揺れやコメントを吸収し、パースを容易にします。
*   **`convertMmlToSmf(mml: string)`** (demo-library/tone-json-demo.ts):
    *   役割: MML (Music Macro Language) コードをStandard MIDI File (SMF) 形式に変換します。
    *   引数: `mml` (文字列) - 変換するMMLコード。
    *   戻り値: (Promise<Uint8Array>) - 変換されたSMFのバイトデータを解決するPromise。
    *   機能: MML入力デモで、MMLコードをSMF形式の入力データに変換するために使用されます。
*   **`getMmlParser()`** (demo-library/tone-json-mml.ts):
    *   役割: MMLパーサーのインスタンスを取得します。
    *   引数: なし。
    *   戻り値: (任意の型) - MMLパーサーのオブジェクト。
    *   機能: MMLからSMFへの変換に使用されるパーサーモジュールを初期化または取得します。
*   **`getParseTreeJsonToSmf()`** (demo-library/tone-json-mml.ts):
    *   役割: パースツリーのJSONからSMFに変換するユーティリティを取得します。
    *   引数: なし。
    *   戻り値: (任意の型) - 変換ユーティリティのオブジェクト。
    *   機能: MMLパーサーが生成した中間形式から最終的なSMFバイトデータを生成します。
*   **`treeToJson(tree: any)`** (demo-library/tone-json-mml.ts):
    *   役割: パースツリーのデータ構造をJSON形式に変換します。
    *   引数: `tree` (任意の型) - パースツリーオブジェクト。
    *   戻り値: (文字列) - JSON形式の文字列。
    *   機能: デバッグ目的や、中間形式の可視化のためにパースツリーをJSON化します。
*   **`ensureMmlRuntime()`** (demo-library/tone-json-mml.ts):
    *   役割: MMLランタイム環境が確実に初期化されていることを確認します。
    *   引数: なし。
    *   戻り値: (Promise<void>) - ランタイムが初期化されると解決するPromise。
    *   機能: MMLパーサーを使用する前に、必要な依存関係や環境設定が完了していることを保証します。
*   **`encodeWav(samples: Float32Array, sampleRate: number)`** (demo-library/wav-exporter.ts):
    *   役割: 生のオーディオサンプルデータ（Float32Array）をWAVファイル形式にエンコードします。
    *   引数:
        *   `samples` (Float32Array) - オーディオサンプルデータ。
        *   `sampleRate` (数値) - サンプリングレート (Hz)。
    *   戻り値: (Uint8Array) - WAVファイル形式にエンコードされたバイトデータ。
    *   機能: 生成されたYM2151オーディオを標準的なWAVファイルとして保存できるようにします。
*   **`writeAscii(view: DataView, offset: number, s: string)`** (demo-library/wav-exporter.ts):
    *   役割: 指定されたDataViewのオフセット位置に、ASCII文字列を書き込みます。
    *   引数:
        *   `view` (DataView) - 書き込み対象のDataViewオブジェクト。
        *   `offset` (数値) - 書き込みを開始するオフセット。
        *   `s` (文字列) - 書き込むASCII文字列。
    *   戻り値: なし。
    *   機能: WAVヘッダーの一部など、固定のASCII文字列をバイナリデータに書き込むために使用されます。
*   **`downloadWav(buffer: Uint8Array, filename: string)`** (demo-library/wav-exporter.ts):
    *   役割: WAV形式のバイトデータをユーザーのブラウザにダウンロードさせます。
    *   引数:
        *   `buffer` (Uint8Array) - ダウンロードするWAVファイルのバイトデータ。
        *   `filename` (文字列) - ダウンロード時のファイル名。
    *   戻り値: なし。
    *   機能: 生成されたWAVファイルをユーザーのローカルマシンに保存するための機能を提供します。
*   **`drawEmpty(canvas: HTMLCanvasElement)`** (demo-library/waveform-canvas.ts):
    *   役割: 指定されたキャンバスをクリアし、空の状態（背景のみ）を描画します。
    *   引数: `canvas` (HTMLCanvasElement) - 描画対象のキャンバス要素。
    *   戻り値: なし。
    *   機能: 波形ビューアの初期化やリセット時にキャンバスをクリアするために使用されます。
*   **`drawWaveform(canvas: HTMLCanvasElement, waveform: Float32Array, color: string)`** (demo-library/waveform-canvas.ts):
    *   役割: 指定されたキャンバスにオーディオ波形を描画します。
    *   引数:
        *   `canvas` (HTMLCanvasElement) - 描画対象のキャンバス要素。
        *   `waveform` (Float32Array) - 描画する波形データ。
        *   `color` (文字列) - 波形の描画色。
    *   戻り値: なし。
    *   機能: 生成されたYM2151オーディオの波形を視覚的に表示します。
*   **`extractNoteBoundaries(ym2151Log: any)`** (demo-library/waveform-viewer.ts):
    *   役割: YM2151ログからノートイベントの開始と終了のタイミングを抽出し、境界データを生成します。
    *   引数: `ym2151Log` (任意の型) - YM2151レジスタ書き込みログ。
    *   戻り値: (配列) - ノートの境界オブジェクトの配列。
    *   機能: 波形ビューアでノートオン/オフのタイミングを視覚的に強調するために使用されます。
*   **`normalizeAmplitude(data: Float32Array)`** (demo-library/waveform-viewer.ts):
    *   役割: オーディオデータ（Float32Array）の振幅を正規化し、最大振幅を特定の範囲（例: -1.0から1.0）にスケーリングします。
    *   引数: `data` (Float32Array) - 正規化するオーディオデータ。
    *   戻り値: (Float32Array) - 正規化されたオーディオデータ。
    *   機能: 波形表示の際に、音量の大小に関わらず視認しやすいように表示範囲を調整します。
*   **`createWaveformViewer(container: HTMLElement)`** (demo-library/waveform-viewer.ts):
    *   役割: オーディオ波形を視覚的に表示するためのビューアコンポーネントを生成し、指定されたコンテナにアタッチします。
    *   引数: `container` (HTMLElement) - 波形ビューアを追加する親DOM要素。
    *   戻り値: (オブジェクト) - 波形ビューアのインターフェース（`renderFromJson`, `setZoom`などのメソッドを含む）。
    *   機能: YM2151ログから生成されたオーディオ波形をインタラクティブに閲覧するためのUIを提供します。
*   **`getWindowDurS(timeRange: { start: number, end: number })`** (demo-library/waveform-viewer.ts):
    *   役割: 指定された時間範囲（秒単位）からウィンドウの継続時間（秒）を計算します。
    *   引数: `timeRange` (オブジェクト) - 開始時刻と終了時刻を含むオブジェクト。
    *   戻り値: (数値) - ウィンドウの継続時間（秒）。
    *   機能: 波形ビューアの表示範囲を計算するために使用されます。
*   **`clampViewStart(viewStart: number, waveformDurS: number, windowDurS: number)`** (demo-library/waveform-viewer.ts):
    *   役割: 波形ビューアの開始時刻を、全体の波形長とウィンドウサイズに基づいて適切な範囲にクランプ（制限）します。
    *   引数:
        *   `viewStart` (数値) - 提案されるビューの開始時刻（秒）。
        *   `waveformDurS` (数値) - 波形全体の継続時間（秒）。
        *   `windowDurS` (数値) - 表示ウィンドウの継続時間（秒）。
    *   戻り値: (数値) - クランプされたビューの開始時刻（秒）。
    *   機能: ユーザーが波形をスクロールする際に、表示範囲が波形の外に出ないようにします。
*   **`updatePositionLabel(current: number, total: number)`** (demo-library/waveform-viewer.ts):
    *   役割: 現在の再生位置と全体の継続時間を示すラベルをUIで更新します。
    *   引数:
        *   `current` (数値) - 現在の時刻（秒）。
        *   `total` (数値) - 全体の継続時間（秒）。
    *   戻り値: なし。
    *   機能: ユーザーに再生位置をリアルタイムでフィードバックします。
*   **`render()`** (demo-library/waveform-viewer.ts):
    *   役割: 波形ビューアの表示内容を再レンダリングします。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: ズームやスクロール、データ変更などの後にビューアの表示を更新します。
*   **`updateBoundariesAndRender()`** (demo-library/waveform-viewer.ts):
    *   役割: 波形の境界情報を更新し、その後にビューアを再レンダリングします。
    *   引数: なし。
    *   戻り値: なし。
    *   機能: ノートの境界線などが変更された際に表示を更新します。
*   **`synthesizeAndRender()`** (demo-library/waveform-viewer.ts):
    *   役割: YM2151ログからオーディオ波形を合成し、ビューアにレンダリングします。
    *   引数: なし。
    *   戻り値: (Promise<void>) - 合成とレンダリングが完了すると解決するPromise。
    *   機能: ログデータから実際の音の波形データを生成し、視覚的に表示します。
*   **`setZoom(zoomLevel: number)`** (demo-library/waveform-viewer.ts):
    *   役割: 波形ビューアのズームレベルを設定します。
    *   引数: `zoomLevel` (数値) - 新しいズームレベル。
    *   戻り値: なし。
    *   機能: ユーザーが波形を拡大・縮小できるようにします。
*   **`endDrag(e: MouseEvent)`** (demo-library/waveform-viewer.ts):
    *   役割: マウスのドラッグ操作が終了した際の処理を実行します。
    *   引数: `e` (MouseEvent) - マウスイベントオブジェクト。
    *   戻り値: なし。
    *   機能: 波形をドラッグして移動させる機能の終了を処理します。
*   **`parseHexByte(hexString: string)`** (demo-library/ym2151-utils.ts):
    *   役割: 16進数表記の文字列を解析し、対応するバイト（数値）を返します。
    *   引数: `hexString` (文字列) - 解析する16進数文字列。
    *   戻り値: (数値 | undefined) - 解析されたバイト値、または不正な場合は`undefined`。
    *   機能: YM2151レジスタ値が16進数で与えられた際に、それを数値に変換するために使用されます。

## 関数呼び出し階層ツリー
```
- if (demo-library/delay-vibrato-demo.ts)
  - computeHash ()
    - nextRequestId ()
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
      - applyRandomToneToAttachment ()
      - setupRandomToneButton ()
      - main ()
      - catch ()
      - playAudioWithOverlay ()
      - createLogVisualizer ()
      - renderFromJson ()
      - setupMmlToSmf ()
      - generateRandomToneRegisters ()
      - upsertAttachmentRegisters ()
      - ensureWasmInitialized ()
      - setStatus ()
      - setEventCountDisplay ()
      - ensureWebYm2151 ()
      - updateOutput ()
      - normalizeAttachmentText ()
      - updateRegisterReflectionStatus ()
      - countRegisterNormalizationTargets ()
      - setupPlayButton ()
      - setupWavExportButton ()
      - bootstrap ()
      - validateRandomToneAttachment ()
      - createWaveformViewer ()
      - exportWav ()
      - setLfoRegisters ()
      - extractLfoRegistersFromAttachment ()
      - syncLfoRegisters ()
      - generateRandomInterpolationPairRegisters ()
      - upsertInterpolationAttachmentRegisters ()
      - buildRandomInterpolationAttachment ()
  - initWasm ()
    - displayResult ()
      - showError ()
      - setupFileInput ()
      - clear ()
  - resolveRegisterForChannel ()
    - collectLfoEvents ()
      - renderLfoLane ()
      - createLane ()
      - parseHexByte ()
  - buildNoteSegments ()
    - notePitch ()
      - computePitchRange ()
      - noteYPosition ()
  - renderPitchCanvas ()
  - detectChannel ()
    - normalizeEvents ()
      - laneColor ()
      - computeTrackWidth ()
      - formatInactiveChannels ()
      - renderEmpty ()
      - ensureGlobalLane ()
  - getMmlParser ()
    - getParseTreeJsonToSmf ()
      - treeToJson ()
      - ensureMmlRuntime ()
  - detectPopNoise ()
  - getToneEditorGenerator ()
    - parseAttachmentEntries ()
      - upsertEntryRegisters ()
  - clearAudioCache ()
    - generateAudioFromJson ()
  - clearWebYmAudioCache ()
    - parseAttachmentField ()
      - cleanup ()
  - buildEventsFromCompact ()
    - serializeWithStatus ()
  - convertMmlToSmf ()
  - drawEmpty ()
    - drawWaveform ()
  - downloadWav ()
    - encodeWav ()
      - writeAscii ()
  - extractNoteBoundaries ()
    - normalizeAmplitude ()
      - getWindowDurS ()
      - clampViewStart ()
      - updatePositionLabel ()
      - render ()
      - updateBoundariesAndRender ()
      - synthesizeAndRender ()
      - setZoom ()
- for (demo-library/log-visualizer-lfo.ts)
- while (demo-library/log-visualizer-pitch-canvas.ts)
- mod (demo-library/shared-demo.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-04-20 07:15:20 JST

Last updated: 2026-03-21

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をヤマハYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製ツールです。
- ネイティブアプリケーション向けライブラリおよびWebAssemblyとしてWebブラウザでも利用可能なクロスプラットフォーム対応が特徴です。
- 2パス処理アーキテクチャ、プログラムチェンジ対応、型安全性、高パフォーマンスにより、堅牢な変換機能を提供します。

## 技術スタック
- フロントエンド:
  - **JavaScript/TypeScript**: デモライブラリの大部分で利用され、ブラウザ上での変換・再生デモを構築します。
  - **HTML/CSS**: デモページの構造とスタイリングに使用されています。
  - **Vite**: TypeScriptベースのデモアプリケーションを開発・ビルドするための高速な開発サーバー兼バンドラーです。
  - **WebAssembly (WASM)**: Rustで書かれたコア変換ロジックをウェブブラウザで実行可能にするために利用されています。
- 音楽・オーディオ:
  - **Standard MIDI Files (SMF)**: プロジェクトの入力フォーマットとして使用される標準的なMIDIファイル形式です。
  - **YM2151 FM音源チップ**: 変換後のレジスタログの出力ターゲットとなるFM音源チップです。
  - **JSON**: 中間イベントおよび最終的なYM2151レジスタ書き込みログの出力フォーマットとして使用されます。
- 開発ツール:
  - **Rust**: プロジェクトの主要なプログラミング言語です。型安全性とパフォーマンスに優れています。
  - **Cargo**: Rustの標準的なビルドシステムおよびパッケージマネージャーです。
  - **wasm-pack**: RustプロジェクトをWebAssemblyにコンパイルし、JavaScriptから利用可能なパッケージを生成するためのツールです。
  - **Git**: バージョン管理システムとして利用されています。
- テスト:
  - **`cargo test`**: Rustの組み込みテストフレームワークで、ユニットテストおよび統合テスト（73のテストが記述されています）に活用されています。
  - **`cargo tarpaulin`**: テストカバレッジを測定し、レポートを生成するために使用されます。
- ビルドツール:
  - **Cargo**: Rustのビルドプロセス全般を管理します。
  - **Vite**: `demo-library`フォルダ内のTypeScriptベースのデモアプリケーションのビルドと開発サーバーを提供します。
- 言語機能:
  - **Rust 1.70.0 以上**: プロジェクトが依存するRustのバージョンであり、モダンな言語機能やパフォーマンス改善を利用しています。
- 自動化・CI/CD:
  - **`cargo install`**: プロジェクトをローカル環境にビルドし、インストールするためのコマンドです。
- 開発標準:
  - **`cargo fmt`**: Rustコードのフォーマットを自動的に適用・チェックし、コードの一貫性を保ちます。
  - **`cargo clippy`**: Rustコードのリンティングを行い、一般的なエラーや非効率なコードを検出します。
  - **`cargo audit`**: 依存関係のセキュリティ脆弱性をチェックし、安全な開発を支援します。
  - **`biome.json`**: `demo-library`フォルダに存在し、TypeScript/JavaScriptコードのフォーマット、リンティング、その他の開発標準を定義するために使用されています。

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
  📖 224.md
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
      📄 event_accumulator.rs
      📄 pitch_effects.rs
      📄 register_effects.rs
      📄 register_fields.rs
      📄 waveform.rs
    📄 converter.rs
    📁 converter_tests/
      📄 attachments.rs
      📄 attachments_change_to_next_tone.rs
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
-   **`.gitignore`**: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
-   **`Cargo.lock`**: Cargoが依存関係を解決した結果を記録し、ビルドの再現性を保証します。
-   **`Cargo.toml`**: Rustプロジェクトの設定ファイル。プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   **`LICENSE`**: プロジェクトのライセンス情報が記述されています。
-   **`README.ja.md`**: プロジェクトの日本語による概要、機能、利用方法などを説明する主要なドキュメントです。
-   **`README.md`**: プロジェクトの英語による概要、機能、利用方法などを説明する主要なドキュメントです。
-   **`WASM_USAGE.md`**: WebAssembly (WASM) としてこのライブラリをブラウザ環境で使用する方法について詳細に説明するドキュメントです。
-   **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレータで使用される設定ファイルで、デモページの公開設定に関連します。
-   **`demo-library/`**: Webブラウザ向けのデモアプリケーションやライブラリの利用例を格納するディレクトリです。
    -   **`demo-library/.gitignore`**: `demo-library`ディレクトリ内のGit管理から除外するファイルを指定します。
    -   **`demo-library/biome.json`**: TypeScript/JavaScriptコードのフォーマットやリンティングルールを定義する設定ファイルです。
    -   **`demo-library/delay-vibrato-demo.ts`**: 遅延ビブラート効果のデモロジックを実装するTypeScriptファイルです。WASMモジュールを利用してMIDIをYM2151ログに変換し、再生します。
    -   **`demo-library/delay-vibrato.html`**: 遅延ビブラートデモのウェブページ構造を定義するHTMLファイルです。
    -   **`demo-library/globals.d.ts`**: グローバルスコープで定義される型宣言ファイルで、オーディオ再生機能などの型情報を提供します。
    -   **`demo-library/index.html`**: デモライブラリのメインエントリーポイントとなるウェブページです。
    -   **`demo-library/library-demo.ts`**: ライブラリとしての基本的な利用方法を示すデモロジックを実装するTypeScriptファイルです。
    -   **`demo-library/log-visualizer-lfo.ts`**: YM2151ログ内のLFO（低周波発振器）イベントを視覚化するためのロジックを含むTypeScriptファイルです。
    -   **`demo-library/log-visualizer-note-segments.ts`**: YM2151ログ内のノートイベントをセグメント化し、ピッチや音長を視覚化するためのロジックを含むTypeScriptファイルです。
    -   **`demo-library/log-visualizer-pitch-canvas.ts`**: ピッチ情報の視覚化を担当するCanvas描画ロジックを含むTypeScriptファイルです。
    -   **`demo-library/log-visualizer.ts`**: YM2151レジスタ書き込みログ全体を視覚化するための主要なロジックを含むTypeScriptファイルです。各チャンネルのイベントをレーン形式で表示します。
    -   **`demo-library/mml-support.ts`**: MML (Music Macro Language) からSMFへの変換をサポートするためのTypeScriptファイルです。
    -   **`demo-library/package-lock.json`**: `demo-library`内のnpm依存関係の正確なバージョンを記録します。
    -   **`demo-library/package.json`**: `demo-library`のnpmパッケージ設定ファイルで、依存関係やスクリプトなどを定義します。
    -   **`demo-library/pop-noise-demo.ts`**: ポップノイズ検出機能のデモロジックを実装するTypeScriptファイルです。ランダムな音色適用やWAVエクスポート機能を含みます。
    -   **`demo-library/pop-noise-detector.ts`**: YM2151ログからポップノイズの発生を検出するためのロジックを含むTypeScriptファイルです。
    -   **`demo-library/pop-noise.html`**: ポップノイズ検出デモのウェブページ構造を定義するHTMLファイルです。
    -   **`demo-library/portamento-soft-lfo-demo.ts`**: ポルタメントとソフトLFO効果のデモロジックを実装するTypeScriptファイルです。
    -   **`demo-library/portamento-soft-lfo.html`**: ポルタメントとソフトLFOデモのウェブページ構造を定義するHTMLファイルです。
    -   **`demo-library/shared-demo.ts`**: 複数のデモで共有されるユーティリティ関数や初期化ロジックを含むTypeScriptファイルです。WASMモジュールの初期化やステータス表示などを担当します。
    -   **`demo-library/style.css`**: デモページのスタイルを定義するCSSファイルです。
    -   **`demo-library/tone-interpolation-demo.ts`**: 音色補間機能のデモロジックを実装するTypeScriptファイルです。
    -   **`demo-library/tone-interpolation.html`**: 音色補間デモのウェブページ構造を定義するHTMLファイルです。
    -   **`demo-library/tone-json-attachment.ts`**: JSON形式の音色データ（アタッチメント）の構築とシリアライズを扱うTypeScriptファイルです。
    -   **`demo-library/tone-json-demo.ts`**: JSON音色データを利用するデモロジックを実装するTypeScriptファイルです。
    -   **`demo-library/tone-json-mml.ts`**: JSON音色とMMLの連携をサポートするTypeScriptファイルです。
    -   **`demo-library/tone-json.html`**: JSON音色デモのウェブページ構造を定義するHTMLファイルです。
    -   **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    -   **`demo-library/vite.config.ts`**: Viteビルドツールの設定ファイルです。
    -   **`demo-library/wav-exporter.ts`**: YM2151ログからWAVファイルをエクスポートするためのロジックを含むTypeScriptファイルです。
    -   **`demo-library/waveform-canvas.ts`**: YM2151波形をCanvasに描画するためのロジックを含むTypeScriptファイルです。
    -   **`demo-library/waveform-viewer.ts`**: 波形表示ビューアの主要なロジックを含むTypeScriptファイルで、ズームやドラッグ、WAVエクスポートなどの機能を提供します。
    -   **`demo-library/ym2151-utils.ts`**: YM2151関連のユーティリティ関数（例: 16進数パース）を含むTypeScriptファイルです。
-   **`generated-docs/`**: `cargo doc`コマンドによって生成されるAPIドキュメントが格納されます。
-   **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルです。
-   **`issue-notes/`**: プロジェクト開発中の課題や検討事項に関するメモが格納されています。
-   **`package-lock.json`**: ルートディレクトリのnpm依存関係の正確なバージョンを記録します。
-   **`package.json`**: ルートディレクトリのnpmパッケージ設定ファイルで、依存関係、スクリプトなどを定義します。
-   **`src/`**: Rustソースコードの主要なディレクトリです。
    -   **`src/error.rs`**: カスタムエラー型とエラーハンドリングロジックを定義します。
    -   **`src/lib.rs`**: ライブラリクレートの主要なエントリーポイントです。パブリックAPIを定義し、他のモジュールをエクスポートします。
    -   **`src/main.rs`**: コマンドラインアプリケーションのエントリーポイントです。MIDIファイルの読み込み、変換、JSON出力などのCLIロジックを実装します。
    -   **`src/midi/`**: MIDIファイルのパースとイベント処理に関連するモジュールです。
        -   **`src/midi/events.rs`**: MIDIイベント構造体を定義します。
        -   **`src/midi/mod.rs`**: `midi`モジュールのルートファイルで、サブモジュールをエクスポートします。
        -   **`src/midi/parser.rs`**: Standard MIDI Files (SMF) をパースし、内部イベント表現に変換するロジックを実装します。
        -   **`src/midi/utils.rs`**: MIDI関連のユーティリティ関数を提供します。
        -   **`src/midi/utils_tests.rs`**: `src/midi/utils.rs`の単体テストが含まれます。
    -   **`src/wasm.rs`**: WebAssembly (WASM) バインディングと、ブラウザからRustロジックを呼び出すためのインターフェースを定義します。
    -   **`src/ym2151/`**: YM2151レジスタログ変換に関連するモジュールです。
        -   **`src/ym2151/channel_allocation.rs`**: YM2151チャンネルの割り当て戦略（和音数ベース、ドラム優先など）を実装します。
        -   **`src/ym2151/converter/`**: YM2151レジスタログ変換の具体的なロジックを含むサブモジュールです。
            -   **`src/ym2151/converter/event_accumulator.rs`**: 変換中にイベントを蓄積し、YM2151レジスタ書き込みを生成するロジックです。
            -   **`src/ym2151/converter/pitch_effects.rs`**: ピッチベンドやポルタメントなどのピッチ関連効果をYM2151レジスタに変換するロジックです。
            -   **`src/ym2151/converter/register_effects.rs`**: YM2151レジスタに対する様々な効果（LFOなど）を処理するロジックです。
            -   **`src/ym2151/converter/register_fields.rs`**: YM2151レジスタの個々のフィールド（例：OPMレジスタのアタックレートやディケイレート）を扱う構造体やユーティリティを定義します。
            -   **`src/ym2151/converter/waveform.rs`**: YM2151の波形設定に関連するロジックを定義します。
        -   **`src/ym2151/converter.rs`**: YM2151レジスタログ変換のメインロジックを定義します。2パス処理のパスBを担当します。
        -   **`src/ym2151/converter_tests/`**: YM2151コンバータのテストケース群です。
            -   **`src/ym2151/converter_tests/attachments.rs`**: 音色アタッチメントに関連するテストです。
            -   **`src/ym2151/converter_tests/attachments_change_to_next_tone.rs`**: 音色アタッチメントが次の音色に切り替わる動作のテストです。
            -   **`src/ym2151/converter_tests/attachments_program_effects.rs`**: プログラムチェンジによる音色アタッチメント効果のテストです。
            -   **`src/ym2151/converter_tests/basic.rs`**: 基本的なYM2151変換のテストです。
            -   **`src/ym2151/converter_tests/channels.rs`**: YM2151チャンネル割り当てに関するテストです。
            -   **`src/ym2151/converter_tests/drums.rs`**: ドラムチャンネルの変換に関するテストです。
            -   **`src/ym2151/converter_tests/effects.rs`**: さまざまな音響効果の変換に関するテストです。
            -   **`src/ym2151/converter_tests/lfo.rs`**: LFO（低周波発振器）変換に関するテストです。
            -   **`src/ym2151/converter_tests/portamento.rs`**: ポルタメント効果の変換に関するテストです。
            -   **`src/ym2151/converter_tests/programs.rs`**: プログラムチェンジと音色の変換に関するテストです。
        -   **`src/ym2151/converter_tests.rs`**: `src/ym2151/converter.rs`の統合テストやその他のテストが含まれます。
        -   **`src/ym2151/event_processor.rs`**: 中間イベントを処理し、YM2151レジスタイベントに変換する役割を持つロジックです。
        -   **`src/ym2151/event_processor_tests.rs`**: `src/ym2151/event_processor.rs`の単体テストが含まれます。
        -   **`src/ym2151/events.rs`**: YM2151関連のイベント構造体（例：レジスタ書き込みイベント）を定義します。
        -   **`src/ym2151/init.rs`**: YM2151チップの初期化状態やリセットロジックを定義します。
        -   **`src/ym2151/mod.rs`**: `ym2151`モジュールのルートファイルで、サブモジュールをエクスポートします。
        -   **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151の周波数設定（FN/BLF）とのマッピングを扱うテーブルやロジックです。
        -   **`src/ym2151/tempo_map.rs`**: MIDIテンポイベントから時間のマッピングを構築するロジックです。
        -   **`src/ym2151/tone.rs`**: YM2151の音色（プログラム）データ構造と、外部JSONファイルからのロードロジックを定義します。
-   **`tests/`**: 統合テストファイルが格納されるディレクトリです。
    -   **`tests/create_test_midi.py`**: テスト用のMIDIファイルを生成するためのPythonスクリプトです。
    -   **`tests/integration_conversion.rs`**: 全体的な変換プロセスをテストする統合テストファイルです。
    -   **`tests/integration_midi.rs`**: MIDIパース機能に特化した統合テストファイルです。
    -   **`tests/integration_multichannel.rs`**: マルチチャンネルMIDIファイルの変換に関する統合テストファイルです。
    -   **`tests/integration_program_change.rs`**: プログラムチェンジ機能の変換に関する統合テストファイルです。
    -   **`tests/integration_wasm.rs`**: WebAssembly (WASM) ビルドと実行に関する統合テストファイルです。
    -   **`tests/test_data/`**: 統合テストで使用されるサンプルMIDIファイルが格納されるディレクトリです。
        -   **`tests/test_data/multi_channel.mid`**: 複数のMIDIチャンネルを含むテスト用MIDIファイルです。
        -   **`tests/test_data/multi_track.mid`**: 複数のトラックを含むテスト用MIDIファイルです。
        -   **`tests/test_data/program_change.mid`**: プログラムチェンジイベントを含むテスト用MIDIファイルです。
        -   **`tests/test_data/simple_melody.mid`**: シンプルなメロディを含むテスト用MIDIファイルです。
        -   **`tests/test_data/tempo_change.mid`**: テンポチェンジイベントを含むテスト用MIDIファイルです。
-   **`tones/`**: カスタムYM2151音色（プログラムチェンジ用）のJSONファイルが格納されるディレクトリです。
    -   **`tones/000.json`**: プログラム0番用のデフォルトYM2151音色定義JSONファイルです。
    -   **`tones/README.md`**: カスタム音色JSONファイルのフォーマットと使用方法を説明するドキュメントです。

## 関数詳細説明
-   **`computeHash` (demo-library/delay-vibrato-demo.ts)**: リクエストのハッシュ値を計算します。これにより、一意な識別子を生成し、デモの状態管理に利用されます。
-   **`nextRequestId` (demo-library/delay-vibrato-demo.ts)**: 次のリクエストIDを生成します。非同期処理の追跡や、古い処理結果が誤って表示されるのを防ぐために利用されます。
-   **`isLatestRequest` (demo-library/delay-vibrato-demo.ts)**: 現在のリクエストが最新のものであるかをチェックします。これにより、処理結果の表示の整合性を保ちます。
-   **`updateOutputWithState` (demo-library/delay-vibrato-demo.ts)**: デモの現在の状態に基づいて、出力表示を更新します。変換結果や再生状態などをUIに反映させます。
-   **`updatePlayButtonState` (demo-library/delay-vibrato-demo.ts)**: 再生ボタンの有効/無効状態を更新します。オーディオが生成されているか、再生中かによって制御されます。
-   **`initializeWasm` (demo-library/delay-vibrato-demo.ts)**: WebAssemblyモジュールを初期化します。Rustで書かれた変換ロジックをブラウザで利用可能にします。
-   **`readAttachmentBytes` (demo-library/delay-vibrato-demo.ts)**: 添付ファイル（音色JSONなど）のバイトデータを非同期で読み込みます。
-   **`runConversion` (demo-library/delay-vibrato-demo.ts)**: MIDIファイルをYM2151ログに変換する処理を実行します。初期化されたWASMモジュールを利用してRustのコアロジックを呼び出します。
-   **`handlePlay` (demo-library/delay-vibrato-demo.ts)**: 再生ボタンがクリックされたときのイベントを処理します。変換されたYM2151ログに基づいて音声を生成し再生します。
-   **`setupAttachmentEditor` (demo-library/delay-vibrato-demo.ts)**: 音色アタッチメント編集用のUIコンポーネントをセットアップします。
-   **`setupMmlInput` (demo-library/delay-vibrato-demo.ts)**: MML入力フィールドと関連機能をセットアップします。MMLからSMFへの変換サポートを含みます。
-   **`setupMidiInput` (demo-library/delay-vibrato-demo.ts)**: MIDIファイル入力フィールドと関連機能をセットアップします。ユーザーがMIDIファイルをアップロードできるようにします。
-   **`bootstrapWebYm` (demo-library/delay-vibrato-demo.ts)**: WebYM2151オーディオエンジンを初期化し、デモページに統合します。これにより、ブラウザでのYM2151サウンド生成が可能になります。
-   **`main` (demo-library/delay-vibrato-demo.ts)**: デモアプリケーションの主要なエントリーポイント関数。各種初期化処理とイベントハンドラの設定を行います。
-   **`if` (demo-library/delay-vibrato-demo.ts)**: 条件分岐のためのキーワードです。指定された条件が真である場合に、続くコードブロックを実行します。
-   **`catch` (demo-library/delay-vibrato-demo.ts)**: 例外処理のためのキーワードです。`try`ブロック内で発生したエラーを捕捉し、指定されたエラーハンドリングロジックを実行します。
-   **`initWasm` (demo-library/library-demo.ts)**: WebAssemblyモジュールを初期化します。`delay-vibrato-demo.ts`の同名関数と類似の役割を果たしますが、こちらはシンプルなデモ用に特化しています。
-   **`displayResult` (demo-library/library-demo.ts)**: 変換結果をウェブページ上に表示します。YM2151ログや視覚化結果をユーザーに提示します。
-   **`showError` (demo-library/library-demo.ts)**: エラーメッセージをウェブページに表示します。ユーザーに問題が発生したことを伝えます。
-   **`setupFileInput` (demo-library/library-demo.ts)**: ファイル入力要素をセットアップし、MIDIファイルが選択されたときの処理を定義します。
-   **`resolveRegisterForChannel` (demo-library/log-visualizer-lfo.ts)**: 指定されたチャンネルのレジスタ情報を解決します。LFOイベントの視覚化に使用されます。
-   **`collectLfoEvents` (demo-library/log-visualizer-lfo.ts)**: YM2151ログからLFO関連イベントを収集します。LFOの動きをグラフィカルに表示するためのデータ準備です。
-   **`renderLfoLane` (demo-library/log-visualizer-lfo.ts)**: LFOイベントの視覚化レーンを描画します。LFOの変化を時間軸で表示します。
-   **`for` (demo-library/log-visualizer-lfo.ts)**: ループ処理のためのキーワードです。コレクションの各要素に対して、または特定の回数だけコードブロックを反復実行します。
-   **`buildNoteSegments` (demo-library/log-visualizer-note-segments.ts)**: YM2151ログからノートセグメント（音符の開始、終了、ピッチなど）を構築します。音符の視覚化の基礎データとなります。
-   **`notePitch` (demo-library/log-visualizer-note-segments.ts)**: ノートのピッチを計算します。視覚化での垂直方向の位置を決定するために使用されます。
-   **`computePitchRange` (demo-library/log-visualizer-note-segments.ts)**: 表示するピッチ範囲を計算します。これにより、Canvasの描画範囲が適切に設定されます。
-   **`noteYPosition` (demo-library/log-visualizer-note-segments.ts)**: ノートのY軸位置を計算します。ピッチに基づいたノートの垂直配置を行います。
-   **`renderPitchCanvas` (demo-library/log-visualizer-pitch-canvas.ts)**: ピッチ視覚化用のCanvasに描画を行います。音符の高さの変化をグラフィカルに表現します。
-   **`while` (demo-library/log-visualizer-pitch-canvas.ts)**: 条件が真である間、コードブロックを繰り返し実行するループ制御キーワードです。特定の条件が満たされなくなるまで処理を続けます。
-   **`detectChannel` (demo-library/log-visualizer.ts)**: イベントから関連するチャンネルを検出します。複数のチャンネルにまたがるイベントを適切に分類します。
-   **`normalizeEvents` (demo-library/log-visualizer.ts)**: YM2151イベントを視覚化に適した形式に正規化します。異なるイベントタイプを統一的に扱えるようにします。
-   **`laneColor` (demo-library/log-visualizer.ts)**: 視覚化レーンの色を決定します。チャンネルごとに異なる色を割り当てることで、視覚的な区別を容易にします。
-   **`createLane` (demo-library/log-visualizer.ts)**: 視覚化レーンのDOM要素を作成します。各チャンネルやグローバルイベントの表示領域を構成します。
-   **`computeTrackWidth` (demo-library/log-visualizer.ts)**: 視覚化トラックの幅を計算します。表示領域のレイアウトを調整します。
-   **`formatInactiveChannels` (demo-library/log-visualizer.ts)**: 非アクティブなチャンネルの表示をフォーマットします。使用されていないチャンネルを分かりやすく表示します。
-   **`createLogVisualizer` (demo-library/log-visualizer.ts)**: YM2151ログ視覚化コンポーネントのインスタンスを作成します。ログ表示の中心的なオブジェクトです。
-   **`renderEmpty` (demo-library/log-visualizer.ts)**: 空の視覚化状態を描画します。データがない場合の初期表示やリセット時に使用されます。
-   **`renderFromJson` (demo-library/log-visualizer.ts)**: JSON形式のYM2151ログから視覚化を描画します。変換結果のJSONを解析し、グラフィカルに表示します。
-   **`ensureGlobalLane` (demo-library/log-visualizer.ts)**: グローバルなイベント（全チャンネルに影響する）を表示するレーンを確保します。テンポ変更などのイベントを表示します。
-   **`setLfoRegisters` (demo-library/log-visualizer.ts)**: LFOレジスタの値を設定します。LFOの動きをプログラム的に制御するために使用されます。
-   **`setupMmlToSmf` (demo-library/mml-support.ts)**: MMLをSMFに変換する機能のセットアップを行います。MML入力がSMF変換パイプラインに接続されるようにします。
-   **`updateRegisterReflectionStatus` (demo-library/pop-noise-demo.ts)**: レジスタ反映ステータスを更新します。ポップノイズ検出デモのUI要素です。
-   **`countRegisterNormalizationTargets` (demo-library/pop-noise-demo.ts)**: レジスタ正規化の対象となる数をカウントします。デモの内部ロジックで使用されます。
-   **`setupPlayButton` (demo-library/pop-noise-demo.ts)**: 再生ボタンをセットアップします。ポップノイズデモで音声再生を制御します。
-   **`setupWavExportButton` (demo-library/pop-noise-demo.ts)**: WAVエクスポートボタンをセットアップします。生成されたYM2151ログからWAVファイルを保存する機能を提供します。
-   **`getToneEditorGenerator` (demo-library/pop-noise-demo.ts)**: 音色エディタのジェネレータ関数を取得します。音色の動的な調整に使用されます。
-   **`applyRandomToneToAttachment` (demo-library/pop-noise-demo.ts)**: アタッチメントにランダムな音色を適用します。ポップノイズのテストやデモで使用されます。
-   **`setupRandomToneButton` (demo-library/pop-noise-demo.ts)**: ランダム音色ボタンをセットアップします。ワンクリックでランダムな音色を生成する機能を提供します。
-   **`bootstrap` (demo-library/pop-noise-demo.ts)**: デモアプリケーション全体のブートストラップ処理を行います。初期化、イベントリスナーの設定などを行います。
-   **`detectPopNoise` (demo-library/pop-noise-detector.ts)**: YM2151ログ内のポップノイズ発生を検出します。特定のレジスタ変化パターンを分析します。
-   **`extractLfoRegistersFromAttachment` (demo-library/portamento-soft-lfo-demo.ts)**: 添付された音色データからLFOレジスタ情報を抽出します。音色データに含まれるLFO設定を利用します。
-   **`syncLfoRegisters` (demo-library/portamento-soft-lfo-demo.ts)**: LFOレジスタの値を同期します。UIと内部状態間でLFO設定を一致させます。
-   **`ensureWasmInitialized` (demo-library/shared-demo.ts)**: WebAssemblyモジュールが初期化されていることを確認します。未初期化の場合、初期化処理をトリガーします。
-   **`setStatus` (demo-library/shared-demo.ts)**: ユーザーインターフェース上のステータス表示を更新します。処理状況やエラーメッセージなどを表示します。
-   **`setEventCountDisplay` (demo-library/shared-demo.ts)**: イベントカウント表示を更新します。MIDIイベント数やYM2151イベント数を表示します。
-   **`ensureWebYm2151` (demo-library/shared-demo.ts)**: WebYM2151オーディオエンジンが準備されていることを確認します。必要に応じて初期化します。
-   **`clearWebYmAudioCache` (demo-library/shared-demo.ts)**: WebYM2151のオーディオキャッシュをクリアします。再生成時に古い音源が混ざらないようにします。
-   **`updateOutput` (demo-library/shared-demo.ts)**: 出力エリアのコンテンツを更新する汎用関数です。テキストや視覚化結果などを表示します。
-   **`parseAttachmentField` (demo-library/shared-demo.ts)**: 添付フィールドからデータをパースします。カスタム音色データなどの入力を処理します。
-   **`cleanup` (demo-library/shared-demo.ts)**: リソースをクリーンアップします。オーディオコンテキストの停止やメモリ解放などを行います。
-   **`mod` (demo-library/shared-demo.ts)**: モジュロ演算を行います。剰余を計算するために使用されます。
-   **`buildRandomAttachment` (demo-library/tone-interpolation-demo.ts)**: ランダムな音色アタッチメントを生成します。音色補間デモで動的な音色変化を生成するために使用されます。
-   **`buildEventsFromCompact` (demo-library/tone-json-attachment.ts)**: コンパクトなJSON形式からYM2151イベントを構築します。音色JSONデータを内部形式に変換します。
-   **`serializeWithStatus` (demo-library/tone-json-attachment.ts)**: ステータス情報付きでデータをシリアライズします。変換過程や結果をJSONとして出力する際に利用されます。
-   **`normalizeAttachmentText` (demo-library/tone-json-attachment.ts)**: 添付テキストを正規化します。入力された音色JSONテキストを整形します。
-   **`convertMmlToSmf` (demo-library/tone-json-demo.ts)**: MMLをSMFに変換します。MML入力デモの主要な変換ステップです。
-   **`getMmlParser` (demo-library/tone-json-mml.ts)**: MMLパーサーを取得します。MML文字列を解析するためのツールを提供します。
-   **`getParseTreeJsonToSmf` (demo-library/tone-json-mml.ts)**: パースツリーJSONからSMFへの変換関数を取得します。MML解析結果をSMFに変換するロジックをラップします。
-   **`treeToJson` (demo-library/tone-json-mml.ts)**: ツリー構造をJSON形式に変換します。MMLパーサーの出力ツリーを可視化可能なJSONに変換します。
-   **`ensureMmlRuntime` (demo-library/tone-json-mml.ts)**: MMLランタイムが準備されていることを確認します。MML関連機能を使用する前に必要な環境を整えます。
-   **`encodeWav` (demo-library/wav-exporter.ts)**: 生データをWAV形式にエンコードします。YM2151のオーディオ出力を標準的なWAVファイルとして保存可能にします。
-   **`writeAscii` (demo-library/wav-exporter.ts)**: アスキー文字列をバイト配列として書き込みます。WAVヘッダーの構築などに使用されます。
-   **`downloadWav` (demo-library/wav-exporter.ts)**: 生成されたWAVデータをダウンロードさせます。ブラウザのダウンロード機能を利用してファイル保存を促します。
-   **`drawEmpty` (demo-library/waveform-canvas.ts)**: 波形Canvasに空の状態を描画します。波形がない場合の初期表示やクリア時に使用されます。
-   **`drawWaveform` (demo-library/waveform-canvas.ts)**: YM2151波形をCanvasに描画します。オーディオデータの視覚化を行います。
-   **`parseHexByte` (demo-library/ym2151-utils.ts)**: 16進数文字列をバイト値にパースします。YM2151レジスタ値などの16進数データを数値に変換します。
-   **`extractNoteBoundaries` (demo-library/waveform-viewer.ts)**: YM2151ログからノートの境界（開始/終了時間）を抽出します。波形ビューアの表示範囲決定に利用されます。
-   **`normalizeAmplitude` (demo-library/waveform-viewer.ts)**: 波形データの振幅を正規化します。表示が適切なスケールになるように調整します。
-   **`createWaveformViewer` (demo-library/waveform-viewer.ts)**: 波形ビューアコンポーネントのインスタンスを作成します。波形表示機能の中心的なオブジェクトです。
-   **`getWindowDurS` (demo-library/waveform-viewer.ts)**: 表示ウィンドウの持続時間（秒）を取得します。ズームレベルに基づいて表示される時間長を計算します。
-   **`clampViewStart` (demo-library/waveform-viewer.ts)**: ビューの開始位置を有効な範囲にクランプ（制限）します。表示範囲がデータの外に出ないようにします。
-   **`updatePositionLabel` (demo-library/waveform-viewer.ts)**: 現在の表示位置を示すラベルを更新します。ユーザーに再生/表示位置をフィードバックします。
-   **`render` (demo-library/waveform-viewer.ts)**: 波形ビューアを再描画します。ズームやスクロールなどの操作後に表示を更新します。
-   **`updateBoundariesAndRender` (demo-library/waveform-viewer.ts)**: 表示境界を更新し、波形を再描画します。データ全体の範囲に基づいて表示を調整します。
-   **`synthesizeAndRender` (demo-library/waveform-viewer.ts)**: 波形を合成し、描画します。YM2151ログから波形データを生成し、Canvasに表示します。
-   **`setZoom` (demo-library/waveform-viewer.ts)**: 波形表示のズームレベルを設定します。ユーザーが波形を拡大・縮小できるようにします。
-   **`endDrag` (demo-library/waveform-viewer.ts)**: ドラッグ操作が終了した際の処理を行います。スクロール位置の確定などを行います。
-   **`clear` (demo-library/waveform-viewer.ts)**: 波形ビューアの表示をクリアします。新しいデータがロードされる前などに使用されます。
-   **`exportWav` (demo-library/waveform-viewer.ts)**: 表示中の波形をWAVファイルとしてエクスポートします。ユーザーが波形を保存できるようにします。
-   **`playAudioWithOverlay` (demo-library/globals.d.ts)**: オーディオ再生とオーバーレイ表示を行います。音声再生中に視覚的なフィードバックを提供します。
-   **`clearAudioCache` (demo-library/globals.d.ts)**: オーディオキャッシュをクリアします。不要なオーディオデータをメモリから解放します。
-   **`generateAudioFromJson` (demo-library/globals.d.ts)**: JSONデータからオーディオを生成します。YM2151ログJSONを元に音源データを生成します。

## 関数呼び出し階層ツリー
```
- if (demo-library/delay-vibrato-demo.ts)
  - computeHash (demo-library/delay-vibrato-demo.ts)
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
      - main ()
      - catch ()
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
      - updateRegisterReflectionStatus ()
      - countRegisterNormalizationTargets ()
      - setupPlayButton ()
      - setupWavExportButton ()
      - getToneEditorGenerator ()
      - applyRandomToneToAttachment ()
      - setupRandomToneButton ()
      - bootstrap ()
      - normalizeAttachmentText ()
      - createWaveformViewer ()
      - exportWav ()
      - setLfoRegisters (demo-library/log-visualizer.ts)
      - extractLfoRegistersFromAttachment ()
      - syncLfoRegisters ()
      - buildRandomAttachment ()
  - initWasm (demo-library/library-demo.ts)
    - displayResult ()
      - showError ()
      - setupFileInput ()
      - clear ()
  - resolveRegisterForChannel (demo-library/log-visualizer-lfo.ts)
    - collectLfoEvents ()
      - renderLfoLane ()
      - createLane ()
      - parseHexByte ()
  - buildNoteSegments (demo-library/log-visualizer-note-segments.ts)
    - notePitch ()
      - computePitchRange ()
      - noteYPosition ()
  - renderPitchCanvas (demo-library/log-visualizer-pitch-canvas.ts)
  - detectChannel (demo-library/log-visualizer.ts)
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
  - detectPopNoise (demo-library/pop-noise-detector.ts)
  - clearAudioCache ()
    - generateAudioFromJson ()
  - clearWebYmAudioCache ()
    - cleanup ()
  - buildEventsFromCompact (demo-library/tone-json-attachment.ts)
    - serializeWithStatus ()
  - convertMmlToSmf ()
  - drawEmpty (demo-library/waveform-canvas.ts)
    - drawWaveform ()
  - downloadWav ()
    - encodeWav (demo-library/wav-exporter.ts)
      - writeAscii ()
  - extractNoteBoundaries (demo-library/waveform-viewer.ts)
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
Generated at: 2026-03-21 07:11:44 JST

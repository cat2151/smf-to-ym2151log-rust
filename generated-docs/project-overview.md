Last updated: 2026-04-03

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップ向けのレジスタ書き込みログ（JSON形式）へ変換するツールです。
- Rust言語で開発されており、ネイティブアプリケーションおよびWebAssemblyによるブラウザ環境で利用可能です。
- 和音数ベースのチャンネル割り当てやプログラムチェンジによる音色変更に対応し、高精度な音源再現を目指します。

## 技術スタック
- フロントエンド: 
  - **TypeScript**: JavaScriptの型安全な上位セット。デモサイトのインタラクティブな機能開発に利用されます。
  - **HTML/CSS**: デモサイトの構造とスタイリングを定義します。
  - **Vite**: 高速な開発サーバーとバンドラとして、デモサイトのビルドおよび開発環境を提供します。
  - **WebAssembly (WASM)**: Rustで書かれた変換ロジックをウェブブラウザ上で実行可能にする技術です。
- 音楽・オーディオ: 
  - **Standard MIDI Files (SMF)**: プロジェクトの入力形式であり、MIDIデータ記述の標準フォーマットです。
  - **YM2151 FM音源**: 出力されるレジスタ書き込みログのターゲットとなる、歴史的なFM音源チップです。
  - **JSON**: 中間イベント、最終的なYM2151レジスタログ、およびカスタム音色定義のファイルフォーマットとして利用されます。
  - **WAV**: デモサイトにおいて、変換されたYM2151ログから音声をWAV形式でエクスポートする機能に利用されます。
- 開発ツール: 
  - **Rust**: 高性能で安全なシステムプログラミング言語であり、本プロジェクトの主要な実装言語です。
  - **Cargo**: Rustの公式なビルドシステムおよびパッケージマネージャーで、依存関係の管理、ビルド、テストなどを担当します。
  - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、JavaScriptから利用できるパッケージを生成するためのツールです。
  - **Git**: ソースコードのバージョン管理システムとして利用されています。
- テスト: 
  - **Cargo test**: Rustプロジェクトに組み込まれたテストフレームワークを利用し、ユニットテストと統合テストを実行します。
  - **Cargo tarpaulin**: テストカバレッジを測定し、テスト網羅率をレポートします。
- ビルドツール: 
  - **Cargo**: Rustプロジェクトのビルドプロセスを管理します。
  - **wasm-pack**: WebAssembly向けのビルドを特化して行います。
  - **Vite**: デモサイトのTypeScript/HTML/CSSアセットをバンドルおよび最適化します。
- 言語機能: 
  - **Rust**: 型安全性、メモリ安全性、高パフォーマンスを保証する現代的なプログラミングパラダイムを提供します。
- 自動化・CI/CD: 
  - プロジェクトのREADMEには具体的なCI/CDパイプラインの記述はありませんが、`cargo`コマンド群（`build`, `test`, `fmt`, `clippy`, `audit`）は開発プロセスにおける自動化と品質保証の基盤となっています。
- 開発標準: 
  - **Cargo fmt**: Rustコードのフォーマットを自動的に適用し、コードスタイルの一貫性を保ちます。
  - **Cargo clippy**: Rustコードの一般的な間違いや非効率なコードパターンを検出するLintツールです。
  - **Cargo audit**: プロジェクトの依存関係に既知のセキュリティ脆弱性がないかをチェックします。
  - **Biome**: `demo-library`において、JavaScript/TypeScriptコードのフォーマットおよびリンティングを管理します。

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
- **`.gitignore`**: Gitのバージョン管理から除外するファイルやディレクトリを指定します。
- **`Cargo.lock`**: `Cargo.toml`で指定された依存関係の正確なバージョンを記録し、ビルドの再現性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定などを定義します。
- **`LICENSE`**: プロジェクトの利用条件を定めるライセンス情報が記述されています。
- **`README.ja.md` / `README.md`**: プロジェクトの概要、目的、機能、使い方などを日本語および英語で説明するメインドキュメントです。
- **`WASM_USAGE.md`**: WebAssembly (WASM) を使用してブラウザ環境で本ライブラリを利用する方法に関する詳細なガイドを提供します。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルです。
- **`googled947dc864c270e07.html`**: Google Search Consoleなどのサービスでサイト所有権を認証するために使用されるファイルです。
- **`demo-library/` ディレクトリ**: 本プロジェクトのWebAssembly版デモンストレーションサイトに関連するファイル群です。
    - **`demo-library/.gitignore`**: デモライブラリ固有のGit追跡除外設定です。
    - **`demo-library/biome.json`**: デモライブラリのTypeScript/JavaScriptコードに対するフォーマッターおよびリンターの設定ファイルです。
    - **`demo-library/delay-vibrato-demo.ts`**: ディレイビブラート機能のデモンストレーションを行うTypeScriptコードです。
    - **`demo-library/delay-vibrato.html`**: ディレイビブラートデモのウェブページ構造を定義するHTMLファイルです。
    - **`demo-library/globals.d.ts`**: デモ内で使用されるグローバルな型定義を宣言するTypeScriptの宣言ファイルです。
    - **`demo-library/index.html`**: デモサイトのメインエントリポイントとなるHTMLファイルです。
    - **`demo-library/library-demo.ts`**: ライブラリの基本的な利用方法を示すデモンストレーションコードです。
    - **`demo-library/log-visualizer-lfo.ts`**: YM2151のLFO (低周波発振器) イベントを視覚化するためのロジックを含みます。
    - **`demo-library/log-visualizer-note-segments.ts`**: ノートイベントの時間とピッチをセグメントとして表現し、視覚化するためのロジックです。
    - **`demo-library/log-visualizer-pitch-canvas.ts`**: 音源のピッチ変化をキャンバスに描画する処理を担います。
    - **`demo-library/log-visualizer.ts`**: YM2151レジスタ書き込みログ全体の視覚化を担当するメインのロジックです。
    - **`demo-library/mml-support.ts`**: MML (Music Macro Language) からSMFへの変換をサポートする機能を提供します。
    - **`demo-library/package-lock.json`**: Node.jsプロジェクトの依存関係のツリー構造とバージョンを固定します。
    - **`demo-library/package.json`**: デモライブラリのメタデータとJavaScript/TypeScriptの依存関係を定義します。
    - **`demo-library/pop-noise-demo.ts`**: ポップノイズ検出と対策に関するデモンストレーションコードです。
    - **`demo-library/pop-noise-detector.ts`**: YM2151ログからポップノイズの原因となる可能性のあるイベントを検出するロジックです。
    - **`demo-library/pop-noise.html`**: ポップノイズデモのウェブページ構造を定義するHTMLファイルです。
    - **`demo-library/portamento-soft-lfo-demo.ts`**: ポルタメントやソフトLFOの効果をデモンストレーションするコードです。
    - **`demo-library/portamento-soft-lfo.html`**: ポルタメント・ソフトLFOデモのウェブページ構造を定義するHTMLファイルです。
    - **`demo-library/random-tone.ts`**: ランダムなYM2151音色設定を生成するためのヘルパー関数群です。
    - **`demo-library/shared-demo.ts`**: 複数のデモ間で共通して使用されるユーティリティ関数や初期化ロジックを集約しています。
    - **`demo-library/style.css`**: デモサイト全体のスタイルを定義するCSSファイルです。
    - **`demo-library/tone-interpolation-demo.ts`**: 音色補間機能のデモンストレーションを行うTypeScriptコードです。
    - **`demo-library/tone-interpolation.html`**: 音色補間デモのウェブページ構造を定義するHTMLファイルです。
    - **`demo-library/tone-json-attachment.ts`**: カスタム音色JSONファイルを添付データとして扱い、変換するロジックです。
    - **`demo-library/tone-json-demo.ts`**: カスタム音色JSONの利用方法をデモンストレーションするコードです。
    - **`demo-library/tone-json-mml.ts`**: MMLから音色JSONを生成する処理に関連するロジックです。
    - **`demo-library/tone-json.html`**: 音色JSONデモのウェブページ構造を定義するHTMLファイルです。
    - **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定を定義します。
    - **`demo-library/vite.config.ts`**: Viteビルドツールによるデモサイトのビルド設定を定義します。
    - **`demo-library/wav-exporter.ts`**: 変換されたYM2151ログからWAV形式のオーディオファイルを生成・エクスポートする機能を提供します。
    - **`demo-library/waveform-canvas.ts`**: 音源の波形データを描画するためのキャンバス操作ロジックです。
    - **`demo-library/waveform-viewer.ts`**: 波形キャンバスを操作し、ズームや再生位置調整などの機能を提供するビューアです。
    - **`demo-library/ym2151-utils.ts`**: YM2151関連のデータ（例：16進数バイト）をパースするユーティリティ関数です。
- **`generated-docs/`**: `cargo doc`コマンドによって生成されるAPIドキュメントが格納されるディレクトリです。
- **`issue-notes/`**: 開発中の課題や検討事項を記録したMarkdownファイル群です。
- **`package-lock.json` / `package.json`**: （ルートディレクトリ直下のこれらのファイルは、デモライブラリ以外のプロジェクト全体のNode.js依存関係管理またはGitHub Pagesなどの設定に関連する可能性があります。）
- **`src/` ディレクトリ**: Rust言語で書かれたプロジェクトの主要なソースコードが格納されています。
    - **`src/api.rs`**: 外部クレートやWebAssemblyから利用される公開APIを定義します。
    - **`src/error.rs`**: プロジェクト全体で使用されるカスタムエラータイプを定義し、一貫したエラーハンドリングを提供します。
    - **`src/lib.rs`**: Rustライブラリクレートのエントリポイントで、他のモジュールの宣言と公開を行います。
    - **`src/main.rs`**: コマンドラインアプリケーションのエントリポイントで、引数解析や変換処理の実行フローを定義します。
    - **`src/midi/`**: MIDIファイルパーシング関連のモジュールです。
        - **`src/midi/events.rs`**: Standard MIDI Fileのイベント構造体を定義します。
        - **`src/midi/mod.rs`**: `midi`モジュールのルートファイルです。
        - **`src/midi/parser.rs`**: SMFを解析し、中間イベントに変換する主要なロジックを実装します。
        - **`src/midi/utils.rs`**: MIDIデータの処理に役立つユーティリティ関数を提供します。
        - **`src/midi/utils_tests.rs`**: `midi/utils.rs`で定義された関数の単体テストです。
    - **`src/options/`**: 変換処理に影響を与えるオプション設定を扱うモジュールです。
        - **`src/options/attachments.rs`**: プログラムチェンジなどでロードされるカスタム音色データ（アタッチメント）の処理ロジックを定義します。
        - **`src/options/effects.rs`**: MIDIイベントからYM2151レジスタへの変換中に適用される各種エフェクト（例: ビブラート、ポルタメント）の設定と処理ロジックです。
        - **`src/options/mod.rs`**: `options`モジュールのルートファイルです。
        - **`src/options/tests.rs`**: `options`モジュールの単体テストです。
    - **`src/wasm.rs`**: WebAssembly (WASM) バインディングを定義し、JavaScriptからRust関数を呼び出せるようにするインターフェースを提供します。
    - **`src/ym2151/`**: YM2151レジスタ書き込みログへの変換ロジックを格納する主要モジュールです。
        - **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルの同時発音数に基づいて、YM2151の限られた8チャンネルを効率的に割り当てる戦略を実装します。
        - **`src/ym2151/converter/`**: YM2151レジスタログ変換の中心的なロジックを担います。
            - **`src/ym2151/converter/event_accumulator.rs`**: MIDIイベントを時系列で収集し、YM2151イベントに変換するための状態を管理します。
            - **`src/ym2151/converter/pitch_effects.rs`**: ピッチベンドやビブラートなど、YM2151のピッチレジスタに影響を与えるエフェクトの処理を実装します。
            - **`src/ym2151/converter/register_effects/`**: 個々のYM2151レジスタ書き込みを生成する詳細なエフェクト処理を担います。
                - **`src/ym2151/converter/register_effects/common.rs`**: 共通のレジスタエフェクト処理を定義します。
                - **`src/ym2151/converter/register_effects/mod.rs`**: `register_effects`モジュールのルートファイルです。
                - **`src/ym2151/converter/register_effects/pop_noise.rs`**: ポップノイズ低減のためのレジスタ操作に関するロジックを実装します。
                - **`src/ym2151/converter/register_effects/register_lfo.rs`**: YM2151のLFOレジスタ（低周波発振器）に関する処理を実装します。
                - **`src/ym2151/converter/register_effects/state_cache.rs`**: YM2151のレジスタ状態をキャッシュし、不要なレジスタ書き込みを最適化します。
                - **`src/ym2151/converter/register_effects/tone_interpolation.rs`**: 音色間のスムーズな変化を実現するための補間処理を実装します。
            - **`src/ym2151/converter/register_fields.rs`**: YM2151レジスタの個々のフィールド（例: アタックレート、ディケイレートなど）の処理ロジックを定義します。
            - **`src/ym2151/converter/waveform.rs`**: YM2151の波形レジスタに関する処理を実装します。
        - **`src/ym2151/converter.rs`**: YM2151レジスタログへの主要な変換ロジックを提供します。
        - **`src/ym2151/converter_tests/`**: `converter`モジュールおよびそのサブモジュールの詳細な単体テスト群です。
        - **`src/ym2151/converter_tests.rs`**: `converter`モジュールの主要なテスト定義ファイルです。
        - **`src/ym2151/event_processor.rs`**: YM2151イベントを処理し、最終的なレジスタ書き込みログを生成する役割を担います。
        - **`src/ym2151/event_processor_tests.rs`**: `event_processor.rs`の単体テストです。
        - **`src/ym2151/events.rs`**: YM2151に特化したイベントのデータ構造を定義します。
        - **`src/ym2151/init.rs`**: YM2151チップの初期化レジスタ設定に関するロジックを定義します。
        - **`src/ym2151/mod.rs`**: `ym2151`モジュールのルートファイルです。
        - **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151のキーオン/オフに必要な周波数情報などのマッピングを管理します。
        - **`src/ym2151/tempo_map.rs`**: MIDIファイルからテンポ情報を抽出し、YM2151のタイムラインにマッピングするロジックです。
        - **`src/ym2151/tone.rs`**: YM2151音色のデータ構造とその処理を定義します。
- **`tests/` ディレクトリ**: プロジェクト全体の統合テストが格納されています。
    - **`tests/create_test_midi.py`**: 統合テストで使用されるMIDIファイルを生成するためのPythonスクリプトです。
    - **`tests/integration_conversion.rs`**: 変換プロセス全体の統合テストです。
    - **`tests/integration_midi.rs`**: MIDIファイルのパースに関する統合テストです。
    - **`tests/integration_multichannel.rs`**: マルチチャンネルMIDIファイルの変換に関する統合テストです。
    - **`tests/integration_program_change.rs`**: プログラムチェンジ機能の統合テストです。
    - **`tests/integration_public_api.rs`**: 公開APIの動作に関する統合テストです。
    - **`tests/integration_wasm.rs`**: WebAssemblyバイナリの機能に関する統合テストです。
    - **`tests/test_data/`**: 統合テストで入力として使用される各種MIDIファイルが格納されています。
- **`tones/` ディレクトリ**: プログラムチェンジイベントによってロードされるカスタムYM2151音色を定義するJSONファイルが格納されています。
    - **`tones/000.json`**: プログラム番号000に対応するYM2151のデフォルト音色設定（例: グランドピアノ）です。
    - **`tones/README.md`**: このディレクトリに格納されるJSONファイルのフォーマットと使用方法について説明します。

## 関数詳細説明
- **`computeHash (demo-library/delay-vibrato-demo.ts)`**: 入力コンテンツ（文字列やオブジェクト）のハッシュ値を計算します。主にキャッシュ管理やリクエストの重複判定に利用されます。
- **`nextRequestId (demo-library/delay-vibrato-demo.ts)`**: 新しいユニークなリクエストIDを生成し、非同期処理の追跡に利用されます。
- **`isLatestRequest (demo-library/delay-vibrato-demo.ts)`**: 指定されたリクエストIDが現在アクティブな最新のリクエストであるかを確認します。
- **`updateOutputWithState (demo-library/delay-vibrato-demo.ts)`**: 変換結果やデモの状態に基づいて、ウェブページの出力表示を更新します。
- **`updatePlayButtonState (demo-library/delay-vibrato-demo.ts)`**: 再生ボタンの状態（有効/無効）を更新します。
- **`initializeWasm (demo-library/delay-vibrato-demo.ts)`**: WebAssemblyモジュールを初期化し、Rustコードをブラウザ環境で利用可能にします。
- **`readAttachmentBytes (demo-library/delay-vibrato-demo.ts)`**: ファイル入力などから添付データ（カスタム音色JSONなど）のバイトデータを読み取ります。
- **`runConversion (demo-library/delay-vibrato-demo.ts)`**: 入力MIDIデータとオプションを用いて、YM2151レジスタログへの変換処理を実行します。
- **`handlePlay (demo-library/delay-vibrato-demo.ts)`**: 生成されたYM2151ログを基に音源を再生するイベントを処理します。
- **`setupAttachmentEditor (demo-library/delay-vibrato-demo.ts)`**: ウェブページの添付データ（音色JSON）編集UIを初期化します。
- **`setupMmlInput (demo-library/delay-vibrato-demo.ts)`**: MML入力フィールドと関連するイベントリスナーをセットアップします。
- **`setupMidiInput (demo-library/delay-vibrato-demo.ts)`**: MIDIファイル入力フィールドと関連するイベントリスナーをセットアップします。
- **`bootstrapWebYm (demo-library/delay-vibrato-demo.ts)`**: `web-ym2151`ライブラリを初期化し、オーディオ再生環境を準備します。
- **`applyRandomToneToAttachment (demo-library/delay-vibrato-demo.ts)`**: 添付データ（音色JSON）にランダムな音色設定を適用します。
- **`setupRandomToneButton (demo-library/delay-vibrato-demo.ts)`**: ランダム音色生成ボタンのイベントリスナーをセットアップします。
- **`main (demo-library/delay-vibrato-demo.ts)`**: ディレイビブラートデモの主要なエントリポイントです。
- **`if (demo-library/delay-vibrato-demo.ts)`**: 条件分岐ロジックの一部です。
- **`catch (demo-library/delay-vibrato-demo.ts)`**: エラーハンドリングロジックの一部です。
- **`playAudioWithOverlay (demo-library/globals.d.ts)`**: 音声を再生し、必要に応じてオーバーレイ表示を行うグローバル関数です。
- **`clearAudioCache (demo-library/globals.d.ts)`**: オーディオキャッシュをクリアし、リソースを解放します。
- **`generateAudioFromJson (demo-library/globals.d.ts)`**: YM2151ログJSONデータからオーディオを生成します。
- **`initWasm (demo-library/library-demo.ts)`**: WebAssemblyモジュールの初期化を行います。
- **`displayResult (demo-library/library-demo.ts)`**: 変換結果をウェブページに表示します。
- **`showError (demo-library/library-demo.ts)`**: エラーメッセージをウェブページに表示します。
- **`setupFileInput (demo-library/library-demo.ts)`**: ファイル入力要素をセットアップします。
- **`resolveRegisterForChannel (demo-library/log-visualizer-lfo.ts)`**: 特定のチャンネルと時刻におけるYM2151レジスタ値を解決します。
- **`collectLfoEvents (demo-library/log-visualizer-lfo.ts)`**: YM2151ログからLFO関連のイベントを収集します。
- **`renderLfoLane (demo-library/log-visualizer-lfo.ts)`**: LFOイベントを視覚化するためのレーンを描画します。
- **`buildNoteSegments (demo-library/log-visualizer-note-segments.ts)`**: ノートイベントを視覚化するためのセグメントデータを構築します。
- **`notePitch (demo-library/log-visualizer-note-segments.ts)`**: ノートのピッチ情報を計算します。
- **`computePitchRange (demo-library/log-visualizer-note-segments.ts)`**: 表示するピッチの範囲を計算します。
- **`noteYPosition (demo-library/log-visualizer-note-segments.ts)`**: ノートが描画されるY座標を計算します。
- **`renderPitchCanvas (demo-library/log-visualizer-pitch-canvas.ts)`**: ピッチ変化の視覚化をキャンバスに描画します。
- **`detectChannel (demo-library/log-visualizer.ts)`**: YM2151ログからチャンネル情報を検出し、整理します。
- **`normalizeEvents (demo-library/log-visualizer.ts)`**: YM2151イベントデータを視覚化に適した形式に正規化します。
- **`laneColor (demo-library/log-visualizer.ts)`**: 視覚化レーンの色を決定します。
- **`createLane (demo-library/log-visualizer.ts)`**: 視覚化用の新しいレーン要素を作成します。
- **`computeTrackWidth (demo-library/log-visualizer.ts)`**: 視覚化トラックの幅を計算します。
- **`formatInactiveChannels (demo-library/log-visualizer.ts)`**: 非アクティブなチャンネルの情報をフォーマットします。
- **`createLogVisualizer (demo-library/log-visualizer.ts)`**: ログ視覚化コンポーネントを初期化し、生成します。
- **`renderEmpty (demo-library/log-visualizer.ts)`**: 空のログ視覚化ビューを描画します。
- **`renderFromJson (demo-library/log-visualizer.ts)`**: YM2151ログJSONデータから視覚化を描画します。
- **`ensureGlobalLane (demo-library/log-visualizer.ts)`**: グローバルな視覚化レーンが存在することを保証します。
- **`setLfoRegisters (demo-library/log-visualizer.ts)`**: LFO関連のレジスタ値を設定します。
- **`setupMmlToSmf (demo-library/mml-support.ts)`**: MMLからSMFへの変換機能をセットアップします。
- **`updateRegisterReflectionStatus (demo-library/pop-noise-demo.ts)`**: レジスタ反射の状態表示を更新します。
- **`countRegisterNormalizationTargets (demo-library/pop-noise-demo.ts)`**: レジスタ正規化のターゲット数をカウントします。
- **`setupPlayButton (demo-library/pop-noise-demo.ts)`**: 再生ボタンのイベントリスナーをセットアップします。
- **`setupWavExportButton (demo-library/pop-noise-demo.ts)`**: WAVエクスポートボタンのイベントリスナーをセットアップします。
- **`bootstrap (demo-library/pop-noise-demo.ts)`**: ポップノイズデモの初期化処理を実行します。
- **`detectPopNoise (demo-library/pop-noise-detector.ts)`**: YM2151ログ内のイベントからポップノイズの可能性を検出します。
- **`extractLfoRegistersFromAttachment (demo-library/portamento-soft-lfo-demo.ts)`**: 添付データからLFOレジスタ設定を抽出します。
- **`syncLfoRegisters (demo-library/portamento-soft-lfo-demo.ts)`**: LFOレジスタの状態を同期します。
- **`getToneEditorGenerator (demo-library/random-tone.ts)`**: 音色エディタ用のランダム音色ジェネレーターを取得します。
- **`generateRandomToneRegisters (demo-library/random-tone.ts)`**: ランダムなYM2151音色レジスタ設定を生成します。
- **`generateRandomInterpolationPairRegisters (demo-library/random-tone.ts)`**: ランダムな音色補間ペアレジスタ設定を生成します。
- **`parseAttachmentEntries (demo-library/random-tone.ts)`**: 添付データのエントリを解析します。
- **`validateRandomToneAttachment (demo-library/random-tone.ts)`**: ランダム音色添付データの妥当性を検証します。
- **`upsertEntryRegisters (demo-library/random-tone.ts)`**: エントリレジスタを更新または挿入します。
- **`upsertAttachmentRegisters (demo-library/random-tone.ts)`**: 添付データ内のレジスタを更新または挿入します。
- **`upsertInterpolationAttachmentRegisters (demo-library/random-tone.ts)`**: 補間添付データ内のレジスタを更新または挿入します。
- **`buildRandomInterpolationAttachment (demo-library/random-tone.ts)`**: ランダムな音色補間添付データを構築します。
- **`ensureWasmInitialized (demo-library/shared-demo.ts)`**: WebAssemblyが初期化されていることを確認します。
- **`setStatus (demo-library/shared-demo.ts)`**: デモのステータスメッセージを設定します。
- **`setEventCountDisplay (demo-library/shared-demo.ts)`**: イベント数の表示を更新します。
- **`ensureWebYm2151 (demo-library/shared-demo.ts)`**: `web-ym2151`インスタンスが利用可能であることを確認します。
- **`clearWebYmAudioCache (demo-library/shared-demo.ts)`**: `web-ym2151`のオーディオキャッシュをクリアします。
- **`updateOutput (demo-library/shared-demo.ts)`**: デモの出力エリアを更新します。
- **`parseAttachmentField (demo-library/shared-demo.ts)`**: 添付入力フィールドの値を解析します。
- **`cleanup (demo-library/shared-demo.ts)`**: デモ終了時のクリーンアップ処理を行います。
- **`mod (demo-library/shared-demo.ts)`**: 剰余演算を実行します（おそらくユーティリティ関数）。
- **`buildEventsFromCompact (demo-library/tone-json-attachment.ts)`**: コンパクトな音色JSONデータからYM2151イベントを構築します。
- **`serializeWithStatus (demo-library/tone-json-attachment.ts)`**: データとステータス情報を含めてシリアライズします。
- **`normalizeAttachmentText (demo-library/tone-json-attachment.ts)`**: 添付テキストデータを正規化します。
- **`convertMmlToSmf (demo-library/tone-json-demo.ts)`**: MMLをSMF形式に変換します。
- **`getMmlParser (demo-library/tone-json-mml.ts)`**: MMLパーサーインスタンスを取得します。
- **`getParseTreeJsonToSmf (demo-library/tone-json-mml.ts)`**: パースツリーからSMF JSONへの変換機能を取得します。
- **`treeToJson (demo-library/tone-json-mml.ts)`**: 構文木をJSON形式に変換します。
- **`ensureMmlRuntime (demo-library/tone-json-mml.ts)`**: MMLランタイムが利用可能であることを確認します。
- **`encodeWav (demo-library/wav-exporter.ts)`**: 音声データをWAV形式にエンコードします。
- **`writeAscii (demo-library/wav-exporter.ts)`**: WAVヘッダーにASCII文字列を書き込みます。
- **`downloadWav (demo-library/wav-exporter.ts)`**: 生成されたWAVファイルをダウンロードします。
- **`drawEmpty (demo-library/waveform-canvas.ts)`**: 空の波形キャンバスを描画します。
- **`drawWaveform (demo-library/waveform-canvas.ts)`**: 音源の波形データをキャンバスに描画します。
- **`parseHexByte (demo-library/ym2151-utils.ts)`**: 16進数文字列をバイト値にパースします。
- **`extractNoteBoundaries (demo-library/waveform-viewer.ts)`**: 音源ログからノートの開始と終了時刻を抽出します。
- **`normalizeAmplitude (demo-library/waveform-viewer.ts)`**: 波形の振幅を正規化し、表示に適したスケールにします。
- **`createWaveformViewer (demo-library/waveform-viewer.ts)`**: 波形ビューアコンポーネントを初期化し、生成します。
- **`getWindowDurS (demo-library/waveform-viewer.ts)`**: 現在のビューポートの持続時間（秒）を取得します。
- **`clampViewStart (demo-library/waveform-viewer.ts)`**: ビューポートの開始位置を表示可能な範囲に制限します。
- **`updatePositionLabel (demo-library/waveform-viewer.ts)`**: 波形ビューアの現在位置を示すラベルを更新します。
- **`render (demo-library/waveform-viewer.ts)`**: 波形ビューアの内容を再描画します。
- **`updateBoundariesAndRender (demo-library/waveform-viewer.ts)`**: 波形の境界を更新し、ビューアを再描画します。
- **`synthesizeAndRender (demo-library/waveform-viewer.ts)`**: 音声を合成し、波形として描画します。
- **`setZoom (demo-library/waveform-viewer.ts)`**: 波形ビューアのズームレベルを設定します。
- **`endDrag (demo-library/waveform-viewer.ts)`**: ドラッグ操作が終了した際の処理を実行します。
- **`clear (demo-library/waveform-viewer.ts)`**: 波形ビューアの表示内容をクリアします。
- **`exportWav (demo-library/waveform-viewer.ts)`**: 波形ビューアの現在の表示内容をWAVファイルとしてエクスポートします。

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
      - setLfoRegisters (demo-library/log-visualizer.ts)
      - extractLfoRegistersFromAttachment ()
      - syncLfoRegisters ()
      - generateRandomInterpolationPairRegisters ()
      - upsertInterpolationAttachmentRegisters ()
      - buildRandomInterpolationAttachment ()
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
  - getToneEditorGenerator (demo-library/random-tone.ts)
    - parseAttachmentEntries ()
      - upsertEntryRegisters ()
  - clearAudioCache ()
    - generateAudioFromJson ()
  - clearWebYmAudioCache ()
    - parseAttachmentField ()
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
Generated at: 2026-04-03 07:15:59 JST

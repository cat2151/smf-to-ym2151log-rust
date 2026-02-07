Last updated: 2026-02-08

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRustベースのツールです。
- 2パス処理アーキテクチャ、プログラムチェンジ対応、型安全性、高パフォーマンスを特徴とします。
- コマンドラインツールとして、またWebAssembly (WASM) を介してWebブラウザ向けライブラリとしても利用可能です。

## 技術スタック
- フロントエンド:
    - **TypeScript**: Webアプリケーションのロジック開発に使用されるJavaScriptのスーパーセットです。
    - **Vite**: 高速な開発サーバーとモダンなビルドツールチェーンを提供し、Webプロジェクトの開発を効率化します。
    - **HTML/CSS**: Webデモアプリケーションの構造とスタイリングを定義するために使用されます。
    - **web-tree-sitter**: MML (Music Macro Language) の解析など、ツリー構造を持つテキストを扱うためのWebAssemblyライブラリです。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: プロジェクトの入力形式となる、デジタル楽器間の通信を定義する標準フォーマットです。
    - **YM2151 FM音源チップ**: 変換後のターゲットとなる、特定のFM音源チップのレジスタ書き込みログを生成します。
    - **JSON形式**: YM2151レジスタ書き込みログの中間および最終出力フォーマットとして使用されます。
    - **Web Audio API**: ブラウザ上でオーディオを生成、処理、分析するためのWeb標準APIであり、Webデモでの音源再生に利用されます。
- 開発ツール:
    - **Rust**: 高性能で安全なシステムプログラミング言語であり、プロジェクトの主要な実装言語です。
    - **Cargo**: Rustの公式なビルドシステムとパッケージマネージャーで、依存関係の管理とプロジェクトのビルドを効率化します。
    - **wasm-pack**: Rustで書かれたコードをWebAssemblyにコンパイルし、JavaScriptから利用可能なパッケージを生成するためのツールです。
- テスト:
    - **`cargo test`**: Rust言語に組み込まれた単体テストおよび結合テストフレームワークです。
    - **`cargo tarpaulin`**: Rustプロジェクトのコードカバレッジを測定し、レポートを生成するためのツールです。
- ビルドツール:
    - **Rust Compiler (rustc)**: Rustソースコードをネイティブ実行可能ファイルまたはWebAssemblyにコンパイルします。
    - **Vite**: フロントエンド資産（HTML, CSS, TypeScriptなど）のビルドとバンドルに使用されます。
- 言語機能:
    - **Rustの型システム**: コンパイル時に厳格な型チェックを行うことで、プログラムの堅牢性と安全性を保証します。
- 自動化・CI/CD:
    - **自動化されたテスト**: `cargo test`による広範なテストが、コード変更が既存の機能に影響を与えないことを保証します。
- 開発標準:
    - **`cargo fmt`**: Rustコードのフォーマットを自動的に統一し、コードの一貫性を保ちます。
    - **`cargo clippy`**: Rustコードの一般的な間違いや非慣用的な表現を検出する静的解析リンターです。
    - **`cargo audit`**: プロジェクトの依存関係における既知のセキュリティ脆弱性をチェックします。

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📖 DEMO_README.md
📖 DEMO_SEPARATION.md
📄 LICENSE
📖 MML_INTEGRATION.md
📖 README.ja.md
📖 README.md
📖 WASM_USAGE.md
📖 WAVEFORM_RENDERING.md
📄 _config.yml
📁 demo-library/
  📄 .gitignore
  📖 README.md
  🌐 index.html
  📘 library-demo.ts
  📊 package-lock.json
  📊 package.json
  🎨 style.css
  📊 tsconfig.json
  📘 vite.config.ts
📁 demo-mml/
  📄 .gitignore
  📖 README.md
  🌐 index.html
  📘 mml-demo.ts
  📊 package-lock.json
  📊 package.json
  🎨 style.css
  📊 tsconfig.json
  📘 vite.config.ts
🌐 googled947dc864c270e07.html
🌐 index.html
📊 package-lock.json
📊 package.json
📜 setup-libs.js
📁 src/
  📄 error.rs
  📄 lib.rs
  📄 main.rs
  📘 main.ts
  📁 midi/
    📄 events.rs
    📄 mod.rs
    📄 parser.rs
    📄 utils.rs
  🎨 style.css
  📘 ui-utils.ts
  📘 vite-env.d.ts
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
  📘 ym2151-audio-utils.ts
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
📊 tsconfig.json
📜 verify-demos.js
📘 vite.config.ts
```

## ファイル詳細説明
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定します。
- **`Cargo.lock`**: プロジェクトの依存関係の正確なバージョンを記録し、ビルドの一貫性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（名前、バージョン、依存関係など）とビルド設定を定義します。
- **`DEMO_README.md`**: デモに関する一般的な情報を提供するMarkdown形式のドキュメントです。
- **`DEMO_SEPARATION.md`**: デモの実装に関する設計上の分離について説明するドキュメントです。
- **`LICENSE`**: プロジェクトの利用条件を定めるライセンス情報です。
- **`MML_INTEGRATION.md`**: MML（Music Macro Language）との統合について説明するドキュメントです。
- **`README.ja.md`**: プロジェクトの概要と使い方を日本語で説明するメインのドキュメントです。
- **`README.md`**: プロジェクトの概要と使い方を英語で説明するメインのドキュメントです。
- **`WASM_USAGE.md`**: WebAssembly (WASM) としてこのライブラリを使用する方法を説明するドキュメントです。
- **`WAVEFORM_RENDERING.md`**: 波形レンダリングに関する情報やドキュメントです。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルです。
- **`demo-library/`**: WebAssemblyライブラリの使用例を示すデモアプリケーションのディレクトリです。
    - **`demo-library/README.md`**: ライブラリデモの個別の説明ドキュメントです。
    - **`demo-library/index.html`**: ライブラリデモのウェブページのエントリポイントです。
    - **`demo-library/library-demo.ts`**: WebAssemblyライブラリの呼び出しとデモのUIロジックを実装したTypeScriptファイルです。
    - **`demo-library/package-lock.json`**: Node.jsプロジェクトの依存関係の正確なバージョンを記録します。
    - **`demo-library/package.json`**: Node.jsプロジェクトのメタデータと依存関係を定義します。
    - **`demo-library/style.css`**: ライブラリデモのウェブページの外観を定義するCSSファイルです。
    - **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    - **`demo-library/vite.config.ts`**: Viteビルドツールによるデモのビルド設定ファイルです。
- **`demo-mml/`**: MML入力からのYM2151ログ変換デモアプリケーションのディレクトリです。
    - **`demo-mml/README.md`**: MMLデモの個別の説明ドキュメントです。
    - **`demo-mml/index.html`**: MMLデモのウェブページのエントリポイントです。
    - **`demo-mml/mml-demo.ts`**: MMLの解析、YM2151ログへの変換、オーディオ再生、波形表示などの複雑なUIロジックを実装したTypeScriptファイルです。
    - **`demo-mml/package-lock.json`**: Node.jsプロジェクトの依存関係の正確なバージョンを記録します。
    - **`demo-mml/package.json`**: Node.jsプロジェクトのメタデータと依存関係を定義します。
    - **`demo-mml/style.css`**: MMLデモのウェブページの外観を定義するCSSファイルです。
    - **`demo-mml/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    - **`demo-mml/vite.config.ts`**: ViteビルドツールによるMMLデモのビルド設定ファイルです。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルです。
- **`index.html`**: プロジェクトのトップページ、またはメインデモのエントリポイントとなるウェブページです。
- **`package-lock.json`**: Node.jsプロジェクト全体の依存関係の正確なバージョンを記録します。
- **`package.json`**: Node.jsプロジェクト全体のメタデータと依存関係を定義します。
- **`setup-libs.js`**: 外部ライブラリのダウンロードやセットアップを行うJavaScriptスクリプトです。
- **`src/`**: 主要なRustとTypeScriptのソースコードが格納されているディレクトリです。
    - **`src/error.rs`**: プロジェクト全体で使用されるカスタムエラータイプを定義します。
    - **`src/lib.rs`**: Rustライブラリの主要なエントリポイントであり、クレート全体の公開APIを提供します。
    - **`src/main.rs`**: コマンドラインアプリケーションのエントリポイントで、MIDIファイル変換のメインロジックを実行します。
    - **`src/main.ts`**: Webデモの主要なUIロジックとWebAssembly連携を担うTypeScriptファイルです。
    - **`src/midi/`**: MIDIファイルの解析と処理に関連するモジュールが含まれています。
        - **`src/midi/events.rs`**: MIDIイベントの内部表現と構造を定義します。
        - **`src/midi/mod.rs`**: `midi`モジュールのルートファイルで、サブモジュールを公開します。
        - **`src/midi/parser.rs`**: Standard MIDI Files (SMF) をパースし、内部の中間イベント表現に変換するロジックを実装します。
        - **`src/midi/utils.rs`**: MIDIデータ処理に関するユーティリティ関数を提供します。
    - **`src/style.css`**: メインのWebデモの外観を定義するCSSファイルです。
    - **`src/ui-utils.ts`**: ユーザーインターフェース操作に関する共通のユーティリティ関数を定義します。
    - **`src/vite-env.d.ts`**: Vite環境の型定義ファイルです。
    - **`src/wasm.rs`**: Rustで実装された機能をWebAssembly経由でJavaScriptに公開するためのブリッジコードを含みます。
    - **`src/ym2151/`**: YM2151 FM音源のレジスタログ変換に関連するモジュールが含まれています。
        - **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのチャンネルに割り当てる戦略を実装します。
        - **`src/ym2151/converter.rs`**: MIDI中間イベントをYM2151レジスタ書き込みログに変換する主要なロジックを実装します。
        - **`src/ym2151/converter_tests.rs`**: YM2151コンバータのテストコードです。
        - **`src/ym2151/event_processor.rs`**: YM2151イベントのタイムライン処理と状態管理ロジックです。
        - **`src/ym2151/events.rs`**: YM2151レジスタイベントの内部表現と構造を定義します。
        - **`src/ym2151/init.rs`**: YM2151チップの初期化レジスタデータやその他の初期設定を定義します。
        - **`src/ym2151/mod.rs`**: `ym2151`モジュールのルートファイルで、サブモジュールを公開します。
        - **`src/ym2151/note_table.rs`**: MIDIノート番号からYM2151のFMパラメータ（周波数、オクターブなど）へのマッピングデータを含みます。
        - **`src/ym2151/tempo_map.rs`**: MIDIファイル内のテンポ変更イベントを管理し、正確なタイミング計算を可能にします。
        - **`src/ym2151/tone.rs`**: YM2151のカスタム音色データ（プログラムチェンジ用）の読み込みと適用を管理します。
    - **`src/ym2151-audio-utils.ts`**: YM2151レジスタログJSONデータからWeb Audio APIを使用して音声を生成するユーティリティ関数を提供します。
- **`tests/`**: 結合テストおよびテストデータが格納されているディレクトリです。
    - **`tests/create_test_midi.py`**: テスト用のStandard MIDI Filesを生成するためのPythonスクリプトです。
    - **`tests/integration_tests.rs`**: プロジェクト全体の主要な機能が正しく連携して動作するかを確認する結合テストコードです。
    - **`tests/test_data/`**: 結合テストで使用されるサンプルMIDIファイル群です。
- **`tones/`**: MIDIプログラムチェンジに対応するカスタムYM2151音色定義JSONファイルが格納されているディレクトリです。
    - **`tones/000.json`**: プログラム番号0のアコースティックグランドピアノのYM2151音色定義です。他の番号のJSONも同様に音色を定義します。
    - **`tones/README.md`**: カスタム音色ファイルの作成方法とフォーマットについて説明するドキュメントです。
- **`tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
- **`verify-demos.js`**: デモページが正しく機能するかを検証するためのJavaScriptスクリプトです。
- **`vite.config.ts`**: Viteビルドツールのプロジェクト全体のビルド設定ファイルです。

## 関数詳細説明
- **`initWasm` (demo-library/library-demo.ts, src/main.ts)**: WebAssemblyモジュールを初期化し、Rustで書かれた変換機能をブラウザのJavaScript環境で利用可能にします。引数なし、戻り値はPromise<void>またはWebAssemblyモジュールインスタンス。
- **`displayResult` (demo-library/library-demo.ts, src/main.ts)**: MIDIファイルの変換結果（例えば、中間イベントJSONやYM2151レジスタログJSON）をウェブページ上に表示します。引数は変換結果データ、戻り値なし。
- **`showError` (demo-library/library-demo.ts, demo-mml/mml-demo.ts, src/main.ts)**: 発生したエラーメッセージをウェブページ上の指定された領域に表示します。引数はエラーメッセージ文字列、戻り値なし。
- **`setupFileInput` (demo-library/library-demo.ts, src/main.ts)**: ファイル入力要素にイベントリスナーを設定し、ユーザーがMIDIファイルをアップロードした際の処理を開始します。引数なし、戻り値なし。
- **`updatePlayButtonState` (demo-mml/mml-demo.ts, src/main.ts)**: オーディオ再生ボタンの状態（有効/無効、表示テキストなど）を、現在の再生状態に応じて更新します。引数は再生可能かどうかの真偽値、戻り値なし。
- **`showPlayOverlay` (demo-mml/mml-demo.ts, src/main.ts)**: オーディオ再生時のオーバーレイ（例えば、ロード中表示）を表示します。引数なし、戻り値なし。
- **`hidePlayOverlay` (demo-mml/mml-demo.ts, src/main.ts)**: オーディオ再生時のオーバーレイを非表示にします。引数なし、戻り値なし。
- **`stopPlayback` (demo-mml/mml-demo.ts, src/main.ts)**: 現在再生中のオーディオを停止し、関連するオーディオコンテキストをリセットします。引数なし、戻り値なし。
- **`resetAudioState` (demo-mml/mml-demo.ts, src/main.ts)**: オーディオ再生に関する内部状態（バッファ、コンテキストなど）を初期状態にリセットします。引数なし、戻り値なし。
- **`prepareAudioBuffer` (demo-mml/mml-demo.ts, src/main.ts)**: YM2151ログから生成されたオーディオデータをWeb Audio APIのオーディオバッファとして準備します。引数はオーディオデータ、戻り値はAudioBufferのPromise。
- **`startPlayback` (demo-mml/mml-demo.ts, src/main.ts)**: 準備されたオーディオバッファを使用して、実際のオーディオ再生を開始します。引数はAudioBuffer、戻り値なし。
- **`playAudioAndVisualize` (demo-mml/mml-demo.ts, src/main.ts)**: YM2151ログから音声を生成し、再生するとともに、その波形を視覚的に表示します。引数はYM2151ログデータ、戻り値なし。
- **`hideWaveformSection` (demo-mml/mml-demo.ts)**: 波形表示領域を非表示にします。引数なし、戻り値なし。
- **`showWaveformSection` (demo-mml/mml-demo.ts)**: 波形表示領域を表示します。引数なし、戻り値なし。
- **`initWebYm2151` (demo-mml/mml-demo.ts, src/main.ts)**: WebYm2151というオーディオライブラリ（またはWebAssemblyモジュール）を初期化します。引数なし、戻り値はPromise<void>。
- **`initAll` (demo-mml/mml-demo.ts)**: MMLデモアプリケーションの全ての初期化処理（UIセットアップ、WebAssemblyロードなど）を実行します。引数なし、戻り値なし。
- **`treeToJSON` (demo-mml/mml-demo.ts)**: （おそらくMMLの構文解析結果である）ツリー構造をJSON形式に変換します。引数はツリーデータ、戻り値はJSON文字列。
- **`convertMML` (demo-mml/mml-demo.ts)**: MML文字列をYM2151レジスタログ形式に変換します。引数はMML文字列、戻り値はYM2151ログデータ。
- **`loadMMLExample` (demo-mml/mml-demo.ts)**: サンプルのMMLコードをロードし、デモインターフェースに表示します。引数なし、戻り値なし。
- **`setupPlayButton` (demo-mml/mml-demo.ts, src/main.ts)**: 再生ボタンに関連するイベントリスナーを設定します。引数なし、戻り値なし。
- **`setupPlayOverlay` (demo-mml/mml-demo.ts, src/main.ts)**: 再生中のオーバーレイ要素の表示/非表示を制御するロジックをセットアップします。引数なし、戻り値なし。
- **`setupEventListeners` (demo-mml/mml-demo.ts, src/main.ts)**: アプリケーション全体の主要なイベントリスナー（ファイル入力、ボタンクリックなど）を設定します。引数なし、戻り値なし。
- **`appendError` (src/main.ts)**: エラーメッセージを累積して表示する領域に追加します。引数はエラーメッセージ文字列、戻り値なし。
- **`setRenderingOverlay` (src/ui-utils.ts)**: レンダリング処理中のインジケータ（オーバーレイ）の表示状態を制御します。引数は表示するかどうかの真偽値、戻り値なし。
- **`loadWebYm2151Script` (src/ym2151-audio-utils.ts)**: WebYm2151関連のJavaScriptスクリプトを動的にロードします。引数なし、戻り値はPromise<void>。
- **`parseEventField` (src/ym2151-audio-utils.ts)**: YM2151 JSONログ内のイベントフィールドをパースし、処理に適した形式に変換します。引数はイベントフィールドの文字列、戻り値はパースされたデータ。
- **`generateAudioFromYm2151Json` (src/ym2151-audio-utils.ts)**: YM2151レジスタログのJSONデータを受け取り、Web Audio APIを使用してオーディオデータを生成します。引数はYM2151ログJSON、戻り値は生成されたオーディオデータのPromise。
- **`renderWaveform` (src/ym2151-audio-utils.ts)**: 生成されたオーディオデータから波形を抽出し、Canvasなどの要素に描画して視覚化します。引数はオーディオデータとCanvas要素、戻り値なし。
- **`downloadFile` (setup-libs.js)**: 指定されたURLからファイルをダウンロードします。引数はURLと保存パス、戻り値はPromise<void>。
- **`setup` (setup-libs.js)**: 必要な外部ライブラリのセットアップ処理を実行します。引数なし、戻り値はPromise<void>。
- **`verifyPage` (verify-demos.js)**: デモページの特定の要素が存在するか、または期待通りに動作するかを検証します。引数はURL、戻り値はPromise<boolean>。
- **`main` (verify-demos.js)**: デモ検証スクリプトの主要な実行ロジックです。複数のデモページの検証を調整します。引数なし、戻り値はPromise<void>。
- **`if`, `for`, `catch`**: これらは特定の機能を実行する関数ではなく、プログラムの制御フローを管理するための言語構造です。`if`は条件分岐、`for`は繰り返し処理、`catch`は例外処理に使用されます。

## 関数呼び出し階層ツリー
```
- if (demo-library/library-demo.ts)
  - initWasm (demo-library/library-demo.ts)
    - displayResult ()
      - showError ()
      - setupFileInput ()
      - updatePlayButtonState (demo-mml/mml-demo.ts)
      - showPlayOverlay ()
      - hidePlayOverlay ()
      - stopPlayback ()
      - resetAudioState ()
      - prepareAudioBuffer ()
      - startPlayback ()
      - playAudioAndVisualize ()
      - initWebYm2151 ()
      - setupPlayButton ()
      - setupPlayOverlay ()
      - setupEventListeners ()
      - appendError ()
      - setRenderingOverlay ()
      - loadWebYm2151Script ()
      - generateAudioFromYm2151Json ()
      - renderWaveform ()
  - hideWaveformSection ()
    - showWaveformSection ()
      - initAll ()
      - treeToJSON ()
      - convertMML ()
      - loadMMLExample ()
  - catch (demo-library/library-demo.ts)
    - downloadFile (setup-libs.js)
      - setup ()
    - parseEventField ()
    - verifyPage (verify-demos.js)
      - main ()
- for (demo-mml/mml-demo.ts)

---
Generated at: 2026-02-08 07:09:33 JST

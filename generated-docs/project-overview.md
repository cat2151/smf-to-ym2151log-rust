Last updated: 2026-03-07

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のライブラリおよびCLIツールです。
- ネイティブアプリケーションとWebAssembly (WASM) を通じてWebブラウザの両方で利用可能で、高いパフォーマンスと型安全性を兼ね備えています。
- 和音数に基づく動的なYM2151チャンネル割り当て戦略、プログラムチェンジによるカスタム音色対応など、高度なMIDIイベント処理を提供します。

## 技術スタック
- フロントエンド: HTML (デモページの構造), CSS (デモページのスタイル), JavaScript/TypeScript (デモページのロジック, WASM連携), Vite (デモページの高速ビルドツール), WebAssembly (Rustコードをブラウザで実行可能にする技術)
- 音楽・オーディオ: Standard MIDI Files (SMF) (入力フォーマット), YM2151 (FM音源チップの出力ターゲット), JSON (中間イベント、YM2151レジスタログ、カスタム音色定義)
- 開発ツール: Rust (主要開発言語), Cargo (Rustのビルドシステムとパッケージマネージャ), git (バージョン管理), wasm-pack (WASMパッケージ生成), cargo doc (APIドキュメント生成), biome (TypeScript/JavaScriptのコード品質ツール), npm/package.json (フロントエンド依存管理)
- テスト: `cargo test` (Rust組み込みテストフレームワーク), `cargo tarpaulin` (テストカバレッジレポート生成)
- ビルドツール: Cargo (Rustプロジェクトビルド), wasm-pack (WASMビルド), Vite (デモ用フロントエンドビルド)
- 言語機能: Rustの強力な型システム (堅牢なコードの実現)
- 自動化・CI/CD: (特になし。開発手順として記載されているインストール・ビルド・テストコマンドが主要なフロー。)
- 開発標準: `cargo fmt` (コードフォーマット), `cargo clippy` (コードリンター), `cargo audit` (セキュリティ脆弱性監査)

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
-   **`.gitignore`**: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
-   **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
-   **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   **`LICENSE`**: プロジェクトのライセンス情報が記述されています。
-   **`README.ja.md`**, **`README.md`**: プロジェクトの概要、目的、使い方、開発情報などを日本語と英語で説明するドキュメントです。
-   **`WASM_USAGE.md`**: WebAssembly (WASM) 環境でこのライブラリを使用する方法について詳細に説明します。
-   **`_config.yml`**: GitHub PagesのJekyll設定ファイルで、デモサイトの生成に使用されます。
-   **`demo-library/`**: WebAssembly版のデモサイトやライブラリの利用例を含むディレクトリです。
    -   **`biome.json`**: TypeScript/JavaScriptのコードフォーマットやリンティングに関する設定を定義します。
    -   **`delay-vibrato-demo.ts`**, **`delay-vibrato.html`**: YM2151のディレイビブラート機能の動作を示すデモページとそのロジックを記述したスクリプトです。
    -   **`globals.d.ts`**: デモページ全体で利用されるグローバルな型定義を提供します。
    -   **`index.html`**: デモサイトのメインのエントリポイントとなるHTMLファイルです。
    -   **`library-demo.ts`**: このプロジェクトがライブラリとして他のRustプロジェクトやWebAssemblyプロジェクトでどのように利用できるかを示すデモスクリプトです。
    -   **`log-visualizer.ts`**: 変換されたYM2151レジスタ書き込みログを視覚的に表示するためのJavaScript/TypeScriptコードです。
    -   **`mml-support.ts`**: Music Macro Language (MML) からStandard MIDI File (SMF) への変換をサポートするスクリプトです。
    -   **`package-lock.json`**, **`package.json`**: Node.jsパッケージマネージャ（npm）の依存関係管理ファイルです。
    -   **`pop-noise-demo.ts`**, **`pop-noise.html`**: ポップノイズ対策機能の動作を示すデモページとそのロジックを記述したスクリプトです。
    -   **`portamento-soft-lfo-demo.ts`**, **`portamento-soft-lfo.html`**: YM2151のポルタメントやソフトLFO機能の動作を示すデモページとそのロジックを記述したスクリプトです。
    -   **`shared-demo.ts`**: 各デモページで共通して利用されるユーティリティ関数やロジックがまとめられています。
    -   **`style.css`**: デモページのスタイルを定義するCSSファイルです。
    -   **`tone-json-attachment.ts`**: 外部から読み込むカスタムYM2151音色JSONファイルを処理するためのスクリプトです。
    -   **`tone-json-demo.ts`**, **`tone-json-mml.ts`**, **`tone-json.html`**: プログラムチェンジによるカスタム音色JSONの適用をデモンストレーションするページとその関連スクリプトです。
    -   **`tsconfig.json`**: TypeScriptコンパイラの設定を定義します。
    -   **`vite.config.ts`**: Vite (フロントエンドビルドツール) の設定ファイルです。
-   **`generated-docs/`**: `cargo doc`コマンドなどで生成されるAPIドキュメントが格納される可能性があるディレクトリです。
-   **`googled947dc864c270e07.html`**: Google Search Consoleなどのサイト認証に使用されるHTMLファイルです。
-   **`issue-notes/`**: 開発中に検討されたIssueや設計に関するメモが格納されています。
-   **`src/`**: プロジェクトの主要なRustソースコードが格納されているディレクトリです。
    -   **`error.rs`**: プロジェクト全体で利用されるカスタムエラー型を定義します。
    -   **`lib.rs`**: プロジェクトのライブラリクレートのエントリポイントで、主要な変換APIが定義されています。
    -   **`main.rs`**: コマンドラインインターフェース（CLIツール）のエントリポイントです。
    -   **`midi/`**: Standard MIDI Files (SMF) の解析に関連するモジュール群です。
        -   **`events.rs`**: MIDIイベントのデータ構造（例：Note On, Note Off, Tempo Change）を定義します。
        -   **`mod.rs`**: `midi`モジュールのルートファイルです。
        -   **`parser.rs`**: SMFフォーマット0および1を解析し、内部のMIDIイベント表現に変換するロジックを実装します。
        -   **`utils.rs`**: MIDI関連のヘルパー関数やユーティリティを提供します。
        -   **`utils_tests.rs`**: `utils.rs`で定義された関数の単体テストが含まれています。
    -   **`wasm.rs`**: WebAssembly (WASM) へのバインディングを提供するためのロジックを記述します。
    -   **`ym2151/`**: YM2151レジスタ書き込みログへの変換に関連するモジュール群です。
        -   **`channel_allocation.rs`**: MIDIチャンネルをYM2151の利用可能なチャンネルに割り当てる戦略（和音数ベース、ドラム優先など）を実装します。
        -   **`converter/`**: YM2151レジスタログ変換の詳細なロジックを含むサブモジュールです。
            -   **`pitch_effects.rs`**: ピッチベンドやビブラートなどのピッチ関連のエフェクトを処理します。
            -   **`register_effects.rs`**: YM2151レジスタへのエフェクト適用ロジックを定義します。
            -   **`waveform.rs`**: YM2151の波形設定に関連する処理を扱います。
        -   **`converter.rs`**: MIDIイベントの中間表現をYM2151レジスタ書き込みログに変換する主要なロジックを実装します。
        -   **`converter_tests/`**: `converter.rs`のテストコード群です。
            -   **`basic.rs`**: 基本的な変換シナリオのテスト。
            -   **`channels.rs`**: チャンネル割り当てロジックのテスト。
            -   **`effects.rs`**: ピッチベンドなどのエフェクト処理のテスト。
            -   **`programs.rs`**: プログラムチェンジによる音色切り替えのテスト。
        -   **`converter_tests.rs`**: `ym2151::converter`モジュールの主要な統合テストを含みます。
        -   **`event_processor.rs`**: MIDIイベントをYM2151が解釈できるイベント形式に変換する処理を行います。
        -   **`event_processor_tests.rs`**: `event_processor.rs`の単体テストです。
        -   **`events.rs`**: YM2151レジスタ書き込みイベントのデータ構造を定義します。
        -   **`init.rs`**: YM2151チップの初期化レジスタ設定を扱います。
        -   **`mod.rs`**: `ym2151`モジュールのルートファイルです。
        -   **`note_table.rs`**: MIDIノート番号とYM2151の周波数データ（FT/FB）間のマッピングを管理します。
        -   **`tempo_map.rs`**: MIDIファイルのテンポ情報に基づき、イベントのタイムスタンプを管理・変換します。
        -   **`tone.rs`**: YM2151の音色（パッチ）定義とその管理ロジックを扱います。
-   **`tests/`**: プロジェクト全体の統合テストを含むディレクトリです。
    -   **`create_test_midi.py`**: テスト用のMIDIファイルをプログラムで生成するためのPythonスクリプトです。
    -   **`integration_conversion.rs`**: MIDIからYM2151ログへのエンドツーエンド変換プロセスの統合テストです。
    -   **`integration_midi.rs`**: MIDIファイルのパーシングに関する統合テストです。
    -   **`integration_multichannel.rs`**: マルチチャンネルMIDI入力の処理に関する統合テストです。
    -   **`integration_program_change.rs`**: プログラムチェンジ機能の統合テストです。
    -   **`integration_wasm.rs`**: WebAssembly (WASM) バインディングの機能に関する統合テストです。
    -   **`test_data/`**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
-   **`tones/`**: カスタムYM2151音色をJSON形式で定義し、プログラムチェンジによってロードするためのディレクトリです。
    -   **`000.json`**: プログラム番号000番（アコースティックグランドピアノなど）のYM2151音色定義JSONの例です。
    **`README.md`**: `tones`ディレクトリ内のJSONファイルフォーマットに関する詳細な説明を提供します。

## 関数詳細説明
-   **`nextRequestId`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: デモアプリケーション内で非同期リクエストを一意に識別するためのIDを生成します。
    -   機能: 連続する整数IDを提供し、最新のリクエストのみを処理するためのメカニズムをサポートします。
-   **`isLatestRequest`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: 特定のリクエストIDが現在処理すべき最新のリクエストであるかを確認します。
    -   機能: ユーザー操作が高速に行われた際に、古い処理結果が最新の表示を上書きするのを防ぎます。
-   **`updateOutputWithState`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: 現在のアプリケーションの状態に基づいて出力エリアを更新します。
    -   機能: 変換結果やエラーメッセージなどをユーザーインターフェースに表示します。
-   **`updatePlayButtonState`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: 再生ボタンの有効/無効状態を更新します。
    -   機能: 変換処理中やエラー発生時など、状況に応じてユーザーが再生ボタンを操作できるかを制御します。
-   **`initializeWasm`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: WebAssemblyモジュールを初期化します。
    -   機能: RustでコンパイルされたWASMコードをブラウザ環境でロードし、JavaScriptから利用できるように準備します。
-   **`readAttachmentBytes`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: ユーザーがアップロードしたファイル（アタッチメント）の内容をバイト配列として読み込みます。
    -   機能: カスタム音色JSONなどの外部ファイルを処理するためにファイルデータを取得します。
-   **`runConversion`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: MIDIファイルをYM2151レジスタログに変換する主要な処理を実行します。
    -   機能: ユーザーが入力したMIDIデータやカスタム音色情報に基づいて変換処理を呼び出し、結果を返します。
-   **`handlePlay`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: 変換されたYM2151ログを再生するイベントを処理します。
    -   機能: YM2151レジスタログをブラウザ上でエミュレートして再生を開始します。
-   **`setupAttachmentEditor`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: カスタム音色などのアタッチメントを編集・入力するUI要素を設定します。
    -   機能: エディタの初期化やイベントリスナーの登録を行い、ユーザーが音色データを編集できるようにします。
-   **`setupMmlInput`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: MML (Music Macro Language) 入力フィールドをセットアップします。
    -   機能: ユーザーがMMLを入力し、それをMIDIに変換するためのUI要素とイベント処理を準備します。
-   **`setupMidiInput`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: MIDIファイル入力フィールドをセットアップします。
    -   機能: ユーザーがMIDIファイルをアップロードし、そのファイルを変換処理に渡すためのUI要素とイベント処理を準備します。
-   **`bootstrapWebYm`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: WebAudioを使用したYM2151音源エミュレータを初期化します。
    -   機能: YM2151ログをブラウザで再生するために必要なWebAudioコンテキストや関連コンポーネントをセットアップします。
-   **`main`** (demo-library/delay-vibrato-demo.ts 他):
    -   役割: 各デモページの主要なエントリポイントです。
    -   機能: ページのロード時にデモの初期設定、UI要素のセットアップ、イベントリスナーの登録などを行います。
-   **`playAudioWithOverlay`** (demo-library/globals.d.ts):
    -   役割: オーバーレイ表示を伴ってオーディオを再生します。
    -   機能: デモサイトでYM2151ログを再生する際に、再生状況を示す視覚的なフィードバックを提供します。
-   **`clearAudioCache`** (demo-library/globals.d.ts):
    -   役割: オーディオキャッシュをクリアします。
    -   機能: 再生済みのオーディオデータをメモリから解放し、リソースを最適化します。
-   **`initWasm`** (demo-library/library-demo.ts):
    -   役割: WebAssemblyモジュールを初期化します。
    -   機能: JavaScriptからRustの機能を呼び出せるようにするためのWASMバインディングをロードします。
-   **`displayResult`** (demo-library/library-demo.ts):
    -   役割: 変換結果をユーザーインターフェースに表示します。
    -   機能: 成功メッセージや変換されたJSONログなどを適切にフォーマットして出力エリアに表示します。
-   **`showError`** (demo-library/library-demo.ts):
    -   役割: エラーメッセージをユーザーに表示します。
    -   機能: 変換失敗時や入力エラー時などに、問題を説明するメッセージを出力エリアに表示します。
-   **`setupFileInput`** (demo-library/library-demo.ts):
    -   役割: ファイル入力要素をセットアップします。
    -   機能: ユーザーがMIDIファイルを選択した際のイベントハンドラを登録し、ファイルの読み込みを準備します。
-   **`parseHexByte`** (demo-library/log-visualizer.ts):
    -   役割: 16進数文字列をバイト値に解析します。
    -   機能: YM2151ログ内のレジスタアドレスやデータ値を数値として扱えるように変換します。
-   **`detectChannel`** (demo-library/log-visualizer.ts):
    -   役割: YM2151レジスタアドレスから関連するチャンネルを検出します。
    -   機能: ログイベントを視覚化する際に、どのYM2151チャンネルに属するイベントかを特定します。
-   **`normalizeEvents`** (demo-library/log-visualizer.ts):
    -   役割: YM2151ログイベントを視覚化しやすい形式に正規化します。
    -   機能: イベントのタイムスタンプやデータを処理し、視覚化ツールが期待する形式に調整します。
-   **`laneColor`** (demo-library/log-visualizer.ts):
    -   役割: 視覚化レーンに適用する色を決定します。
    -   機能: YM2151チャンネルごとに異なる色を割り当て、視覚的な区別を容易にします。
-   **`createLane`** (demo-library/log-visualizer.ts):
    -   役割: YM2151ログ視覚化のための個別のレーン（トラック）を作成します。
    -   機能: 各YM2151チャンネルに対応する表示領域をDOMに生成します。
-   **`computeTrackWidth`** (demo-library/log-visualizer.ts):
    -   役割: 視覚化トラックの幅を計算します。
    -   機能: イベントの総時間や密度に基づいて、トラックの表示サイズを動的に調整します。
-   **`createLogVisualizer`** (demo-library/log-visualizer.ts):
    -   役割: YM2151ログを視覚化するコンポーネントを生成します。
    -   機能: 変換されたレジスタログを時系列でグラフィカルに表示するための主要なコンテナとロジックをセットアップします。
-   **`renderEmpty`** (demo-library/log-visualizer.ts):
    -   役割: ログがない場合に空の視覚化表示を生成します。
    -   機能: ユーザーにログがないことを伝えるメッセージやプレースホルダーを表示します。
-   **`renderFromJson`** (demo-library/log-visualizer.ts):
    -   役割: JSON形式のYM2151ログデータを基に視覚化レンダリングを行います。
    -   機能: パースされたJSONデータを使って、レジスタ書き込みイベントを視覚化ツールに描画します。
-   **`ensureGlobalLane`** (demo-library/log-visualizer.ts):
    -   役割: 全体イベントを表示するためのグローバルレーンが存在することを確認します。
    -   機能: 特定のチャンネルに属さない、全体に影響するイベント（例: テンポ変更）を表示するための領域を確保します。
-   **`setupMmlToSmf`** (demo-library/mml-support.ts):
    -   役割: MMLからSMFへの変換機能をセットアップします。
    -   機能: MMLパーサーを初期化し、MML入力フィールドからの変換要求を処理します。
-   **`setupPlayButton`** (demo-library/pop-noise-demo.ts):
    -   役割: 再生ボタンのイベントリスナーを設定します。
    -   機能: 再生ボタンがクリックされたときに`handlePlay`関数を呼び出すように準備します。
-   **`bootstrap`** (demo-library/pop-noise-demo.ts):
    -   役割: デモアプリケーションの起動処理を実行します。
    -   機能: WASMの初期化、UI要素のセットアップ、イベントリスナーの登録など、デモページの起動に必要な一連のタスクを実行します。
-   **`ensureWasmInitialized`** (demo-library/shared-demo.ts):
    -   役割: WebAssemblyモジュールが初期化済みであることを保証します。
    -   機能: 未初期化であれば初期化を試み、WASM機能の利用前に準備が整っていることを確認します。
-   **`setStatus`** (demo-library/shared-demo.ts):
    -   役割: アプリケーションの現在のステータスをユーザーインターフェースに表示します。
    -   機能: 処理の進捗状況や成功/失敗メッセージなどをステータス表示エリアに更新します。
-   **`setEventCountDisplay`** (demo-library/shared-demo.ts):
    -   役割: 変換されたイベントの数を表示します。
    -   機能: 変換後のYM2151ログのイベント総数などをUIに示します。
-   **`ensureWebYm2151`** (demo-library/shared-demo.ts):
    -   役割: WebAudioベースのYM2151エミュレータが利用可能であることを保証します。
    -   機能: YM2151音源エミュレータのインスタンスを生成または取得し、再生準備が整っていることを確認します。
-   **`clearWebYmAudioCache`** (demo-library/shared-demo.ts):
    -   役割: WebAudio YM2151エミュレータのオーディオキャッシュをクリアします。
    -   機能: 再生リソースを解放し、メモリ使用量を最適化します。
-   **`updateOutput`** (demo-library/shared-demo.ts):
    -   役割: メインの出力領域を更新します。
    -   機能: 変換結果のJSONやエラーメッセージなどを表示する共通のインターフェースを提供します。
-   **`parseAttachmentField`** (demo-library/shared-demo.ts):
    -   役割: アタッチメント入力フィールドからデータを解析します。
    -   機能: ユーザーがテキストとして入力したカスタム音色JSONなどのデータを構造化された形式に変換します。
-   **`cleanup`** (demo-library/shared-demo.ts):
    -   役割: デモアプリケーションのリソースをクリーンアップします。
    -   機能: 不要になったWebAudioコンテキストやイベントリスナーなどを解放し、メモリリークを防ぎます。
-   **`buildEventsFromCompact`** (demo-library/tone-json-attachment.ts):
    -   役割: コンパクトな形式で記述された音色設定からYM2151イベントを構築します。
    -   機能: ユーザーが簡潔に記述したカスタム音色定義を、YM2151レジスタ書き込みイベントの形式に展開します。
-   **`normalizeAttachmentText`** (demo-library/tone-json-attachment.ts):
    -   役割: アタッチメントテキストの改行コードを正規化します。
    -   機能: 異なるOS環境からの入力でも一貫したテキスト処理を保証します。
-   **`convertMmlToSmf`** (demo-library/tone-json-demo.ts):
    -   役割: MML文字列をStandard MIDI File (SMF) 形式に変換します。
    -   機能: MMLパーサーを利用してMMLコードをSMFバイトデータに変換し、次の処理段階へ渡します。
-   **`getMmlParser`** (demo-library/tone-json-mml.ts):
    -   役割: MMLを解析するためのパーサーインスタンスを取得します。
    -   機能: MML文字列から構文ツリーを生成するために必要なパーサーを初期化または返します。
-   **`getParseTreeJsonToSmf`** (demo-library/tone-json-mml.ts):
    -   役割: 解析されたMML構文ツリーをSMF形式のJSONに変換する関数を取得します。
    -   機能: MML構文ツリーを中間的なJSON表現に変換するためのコンバータを提供します。
-   **`treeToJson`** (demo-library/tone-json-mml.ts):
    -   役割: MMLの構文ツリーをJSON形式に変換します。
    -   機能: MMLパーサーによって生成された抽象構文ツリーを、SMF変換に適したJSON構造にシリアライズします。
-   **`ensureMmlRuntime`** (demo-library/tone-json-mml.ts):
    -   役割: MML変換に必要なランタイムが初期化済みであることを保証します。
    -   機能: MMLパーサーや関連ライブラリがロードされ、利用可能な状態であることを確認します。

## 関数呼び出し階層ツリー
```
- main (各デモの主要エントリポイント)
  - initializeWasm (WebAssemblyモジュールの初期化)
    - ensureWasmInitialized (WASM初期化の保証)
  - setupAttachmentEditor (カスタム音色エディタの設定)
    - parseAttachmentField (アタッチメントフィールドの解析)
    - buildEventsFromCompact (コンパクトな設定からイベント構築)
      - normalizeAttachmentText (アタッチメントテキストの正規化)
  - setupMmlInput (MML入力フィールドの設定)
    - setupMmlToSmf (MMLからSMFへの変換設定)
      - getMmlParser (MMLパーサーの取得)
      - getParseTreeJsonToSmf (パースツリーからSMF JSONへの変換関数取得)
      - treeToJson (MML構文ツリーをJSONに変換)
      - ensureMmlRuntime (MMLランタイムの保証)
  - setupMidiInput (MIDIファイル入力フィールドの設定)
  - bootstrapWebYm (WebAudio YM2151エミュレータの初期化)
    - ensureWebYm2151 (WebAudio YM2151の保証)
  - updateOutputWithState (状態に基づいた出力更新)
  - updatePlayButtonState (再生ボタンの状態更新)
  - readAttachmentBytes (アタッチメントファイルのバイト読み込み)
  - runConversion (MIDIからYM2151ログへの変換実行)
    - setStatus (ステータス表示の更新)
    - setEventCountDisplay (イベント数表示の更新)
    - updateOutput (出力領域の更新)
    - convertMmlToSmf (MMLからSMFへの変換, MML入力時)
  - handlePlay (再生処理)
    - playAudioWithOverlay (オーバーレイ付きオーディオ再生)
    - createLogVisualizer (ログ視覚化コンポーネントの作成)
    - renderFromJson (JSONデータからの視覚化レンダリング)
      - parseHexByte (16進数バイト解析)
      - detectChannel (チャンネル検出)
      - normalizeEvents (イベントの正規化)
      - laneColor (レーンカラーの取得)
      - createLane (レーンの作成)
      - computeTrackWidth (トラック幅の計算)
      - ensureGlobalLane (グローバルレーンの保証)
      - renderEmpty (空のレンダリング)
  - nextRequestId (リクエストIDの生成)
  - isLatestRequest (最新リクエストの判定)
  - clearAudioCache (オーディオキャッシュのクリア)
  - clearWebYmAudioCache (WebAudio YM2151キャッシュのクリア)
    - cleanup (リソースのクリーンアップ)

- initWasm (demo-library/library-demo.ts)
  - displayResult (結果の表示)
    - showError (エラーの表示)
  - setupFileInput (ファイル入力の設定)

- bootstrap (demo-library/pop-noise-demo.ts)
  - (main関数と同様の初期化・セットアップ処理)
  - setupPlayButton (再生ボタンの設定)
    - handlePlay (再生処理)

---
Generated at: 2026-03-07 07:11:30 JST

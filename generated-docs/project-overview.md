Last updated: 2026-02-16

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するツールです。
- Rustで実装されており、高いパフォーマンスと型安全性を提供し、コマンドラインツールやライブラリとして利用可能です。
- WebAssembly (WASM) にも対応しており、ブラウザ環境でのMIDIファイル変換とYM2151音源エミュレーションを可能にします。

## 技術スタック
- フロントエンド: **WebAssembly (WASM)** (ブラウザでの実行を可能にする), **TypeScript** (デモUIのロジック), **HTML** (デモページの構造), **CSS** (デモページのスタイリング), **Vite** (デモプロジェクトのビルドツール)
- 音楽・オーディオ: **Standard MIDI Files (SMF)** (入力フォーマット), **YM2151 FM音源チップ** (ターゲット音源チップ), **MIDIプログラムチェンジ** (音色切り替え対応)
- 開発ツール: **Rust** (主要プログラミング言語), **Cargo** (Rustのビルドシステムおよびパッケージマネージャー), **wasm-pack** (RustコードをWASMにビルドするツール)
- テスト: **Cargo test** (ユニットテストおよび統合テストフレームワーク), **cargo tarpaulin** (テストカバレッジ計測ツール)
- ビルドツール: **Cargo** (Rustプロジェクトのビルド), **wasm-pack** (WebAssemblyパッケージ生成), **Vite** (デモ用フロントエンドのビルド)
- 言語機能: **Rust 1.70.0以上** (堅牢なアプリケーション開発), **JSON** (YM2151ログ、中間イベント、カスタム音色ファイルのフォーマット)
- 自動化・CI/CD: (特記事項なし、ただし開発標準ツールはCI/CDで活用可能)
- 開発標準: **cargo fmt** (コードフォーマットの自動化), **cargo clippy** (静的解析によるコード品質向上), **cargo audit** (依存関係のセキュリティ脆弱性チェック)

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
  📘 tone-json-demo.ts
  🌐 tone-json.html
  📊 tsconfig.json
  📘 vite.config.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 105.md
  📖 111.md
  📖 112.md
  📖 114.md
  📖 115.md
  📖 117.md
  📖 122.md
  📖 123.md
  📖 125.md
  📖 126.md
  📖 128.md
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
-   `.gitignore`: Gitバージョン管理システムが無視するファイルやディレクトリを指定します。
-   `Cargo.lock`: Rustプロジェクトの依存関係ツリーと正確なバージョンを記録し、再現可能なビルドを保証します。
-   `Cargo.toml`: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
-   `LICENSE`: 本プロジェクトのライセンス情報が記述されています。
-   `README.ja.md`: プロジェクトの日本語による概要、使い方、開発方法などが記載されています。
-   `README.md`: プロジェクトの英語による概要、使い方、開発方法などが記載されています。
-   `WASM_USAGE.md`: WebAssembly (WASM) としてこのライブラリを使用する方法に関する詳細なガイドです。
-   `_config.yml`: GitHub Pagesのサイト設定ファイルで、主にデモサイトの構成を定義します。
-   `demo-library/`: WebAssembly版ライブラリの使用例を示すデモアプリケーションのソースコード群です。
    -   `delay-vibrato-demo.ts`: 遅延ビブラート効果を伴うYM2151ログ変換デモのTypeScriptロジックです。
    -   `delay-vibrato.html`: 遅延ビブラートデモ用のHTMLページです。
    -   `globals.d.ts`: グローバルに利用される型定義ファイルです。
    -   `index.html`: WebAssembly版ライブラリの基本的な使用方法を示すメインデモページのHTMLです。
    -   `library-demo.ts`: ライブラリとしての基本的な使用例を示すTypeScriptロジックです。
    -   `log-visualizer.ts`: YM2151レジスタ書き込みログを視覚的に表示するためのTypeScriptコードです。
    -   `mml-support.ts`: MML（Music Macro Language）入力からSMFへの変換をサポートするロジックです。
    -   `package-lock.json`: `demo-library`のNode.js依存関係のバージョンを固定するファイルです。
    -   `package.json`: `demo-library`のNode.jsプロジェクト情報と依存関係を定義するファイルです。
    -   `pop-noise-demo.ts`: ポップノイズ関連の処理を示すデモのTypeScriptロジックです。
    -   `pop-noise.html`: ポップノイズデモ用のHTMLページです。
    -   `portamento-soft-lfo-demo.ts`: ポルタメントやソフトLFO（低周波発振器）の効果を示すデモのTypeScriptロジックです。
    -   `portamento-soft-lfo.html`: ポルタメント/ソフトLFOデモ用のHTMLページです。
    -   `shared-demo.ts`: 各デモ間で共通して利用される機能やヘルパー関数を提供するTypeScriptコードです。
    -   `style.css`: デモページのスタイルを定義するCSSファイルです。
    -   `tone-json-demo.ts`: カスタムYM2151音色JSONの適用を示すデモのTypeScriptロジックです。
    -   `tone-json.html`: カスタムYM2151音色JSONデモ用のHTMLページです。
    -   `tsconfig.json`: TypeScriptコンパイラの設定ファイルです。
    -   `vite.config.ts`: Viteビルドツールの設定ファイルで、デモのビルド方法を定義します。
-   `generated-docs/`: `cargo doc`コマンドなどで生成されたAPIドキュメントが格納されるディレクトリです。
-   `googled947dc864c270e07.html`: Google Search Consoleのサイト所有権確認に使用されるファイルです。
-   `issue-notes/`: 開発中の特定の課題や設計メモなどがMarkdown形式で記録されているディレクトリです。
-   `package-lock.json`: プロジェクト全体のNode.js依存関係のバージョンを固定するファイルです。
-   `package.json`: プロジェクト全体のNode.jsプロジェクト情報と依存関係を定義するファイルです。
-   `src/`: Rustの主要なソースコードが格納されるディレクトリです。
    -   `error.rs`: プロジェクト固有のエラー型とエラー処理ロジックを定義します。
    -   `lib.rs`: Rustライブラリクレートの主要なエントリポイントであり、公開APIを定義します。
    -   `main.rs`: コマンドラインアプリケーションのエントリポイントで、MIDIファイル変換処理を調整します。
    -   `midi/`: MIDIファイル解析に関連するモジュールを格納します。
        -   `events.rs`: Standard MIDI File内の様々なMIDIイベントのデータ構造を定義します。
        -   `mod.rs`: `midi`モジュールのルートファイルです。
        -   `parser.rs`: Standard MIDI File (SMF) を読み込み、内部表現のMIDIイベントにパースするロジックを実装します。
        -   `utils.rs`: MIDIデータ処理に役立つ汎用ユーティリティ関数を提供します。
    -   `wasm.rs`: WebAssembly (WASM) にエクスポートされるRust関数と、JavaScriptとのインタフェースを定義します。
    -   `ym2151/`: YM2151レジスタログへの変換ロジックを格納するモジュールです。
        -   `channel_allocation.rs`: MIDIチャンネルからYM2151の8つのオペレータチャンネルへの割り当て戦略（和音数ベース、ドラム優先など）を実装します。
        -   `converter/`: YM2151レジスタログ変換の詳細なロジックを格納します。
            -   `pitch_effects.rs`: ピッチベンドやビブラートなどのピッチ関連エフェクトをYM2151レジスタ値に変換するロジックです。
            -   `register_effects.rs`: YM2151の各種レジスタ（例: 音量、エンベロープ）に対するエフェクトや設定を処理します。
            -   `waveform.rs`: YM2151の波形設定に関連する変換ロジックです。
        -   `converter.rs`: MIDIイベントストリームをYM2151レジスタ書き込みログに変換する主要なロジックを実装します。
        -   `converter_tests.rs`: `ym2151::converter`モジュールのユニットテストが含まれます。
        -   `event_processor.rs`: YM2151変換パイプラインにおける個々のMIDIイベントの処理を管理します。
        -   `events.rs`: YM2151レジスタ書き込みイベントや関連する内部データ構造を定義します。
        -   `init.rs`: YM2151チップの初期状態やリセット時のレジスタ設定を定義します。
        -   `mod.rs`: `ym2151`モジュールのルートファイルです。
        -   `note_table.rs`: MIDIノート番号とYM2151の周波数制御値（F-Number、BLOCK）間のマッピングテーブルを提供します。
        -   `tempo_map.rs`: MIDIファイル内のテンポチェンジイベントを管理し、イベントの正確なタイミング計算を可能にします。
        -   `tone.rs`: YM2151の音色データ構造（オペレータパラメータなど）を定義し、外部JSONファイルからの音色ロードをサポートします。
-   `tests/`: プロジェクトの統合テストが格納されるディレクトリです。
    -   `create_test_midi.py`: テスト用のMIDIファイルをプログラムで生成するためのPythonスクリプトです。
    -   `integration_tests.rs`: 複数のモジュールを結合した際の動作を検証する統合テストが含まれます。
    -   `test_data/`: 統合テストやデモで使用されるサンプルMIDIファイルが格納されます。
-   `tones/`: MIDIプログラムチェンジイベントによってロードされるカスタムYM2151音色（JSON形式）が格納されるディレクトリです。
    -   `000.json`: MIDIプログラム0番に対応するYM2151音色定義ファイルです。
    -   `README.md`: `tones`ディレクトリ内のJSON音色ファイルのフォーマットと使用方法に関する説明です。

## 関数詳細説明
-   `if` (demo-library/delay-vibrato-demo.ts): JavaScriptの条件分岐構文。特定の条件が真の場合にコードブロックを実行します。
-   `nextRequestId()` (demo-library/delay-vibrato-demo.ts): 新しい一意のリクエストIDを生成します。
-   `isLatestRequest()` (demo-library/delay-vibrato-demo.ts): 与えられたリクエストIDが現在処理中の最新のリクエストであるかを判定します。
-   `updateOutputWithState()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): デモの出力エリアと現在の状態表示を更新します。
-   `updatePlayButtonState()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): 再生ボタンの有効/無効状態や表示を更新します。
-   `initializeWasm()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): WebAssemblyモジュールを初期化し、Rustで実装された機能を利用可能にします。
-   `readAttachmentBytes()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): デモで添付されるファイル（MIDIやカスタム音色JSONなど）の内容をバイトデータとして読み込みます。
-   `runConversion()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): MIDIファイルをYM2151レジスタログに変換するメイン処理を実行します。
-   `handlePlay()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): 変換されたYM2151ログを再生するための処理をトリガーします。
-   `setupAttachmentEditor()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): 添付ファイル（カスタム音色など）のテキストエディタを設定し、ユーザー入力を処理します。
-   `setupMmlInput()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): MML入力フィールドのイベントリスナーを設定し、MMLからの変換を可能にします。
-   `setupMidiInput()` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): MIDIファイル入力フィールドのイベントリスナーを設定し、ユーザーがMIDIファイルをアップロードできるようにします。
-   `bootstrapWebYm()` (demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): WebYM2151プレイヤーを初期化し、関連するデモ機能を起動します。
-   `main()` (demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts): デモアプリケーションの主要なエントリポイント関数です。
-   `catch()`: JavaScriptの例外処理構文。tryブロック内で発生したエラーを捕捉し、処理します。
-   `addEventListener()`: DOM要素にイベントリスナーを登録し、特定イベント発生時に指定された関数を実行します。
-   `playAudioWithOverlay()` (demo-library/globals.d.ts): 音声再生を行いながら、視覚的なオーバーレイを表示する関数。（定義のみ、実装は外部）
-   `createLogVisualizer()` (demo-library/log-visualizer.ts): YM2151ログを視覚化するためのコンポーネントを初期化し、DOM要素にアタッチします。
-   `renderFromJson()` (demo-library/log-visualizer.ts): 与えられたJSONデータに基づいてログ視覚化をレンダリングします。
-   `setupMmlToSmf()` (demo-library/mml-support.ts): MMLからSMFへの変換機能を設定します。
-   `ensureWasmInitialized()` (demo-library/shared-demo.ts): WebAssemblyモジュールが初期化されていることを確認します。
-   `setStatus()` (demo-library/shared-demo.ts): 画面上のステータス表示を更新します。
-   `setEventCountDisplay()` (demo-library/shared-demo.ts): 変換されたイベントの数を画面に表示します。
-   `ensureWebYm2151()` (demo-library/shared-demo.ts): WebYM2151プレイヤーが利用可能であることを確認し、必要に応じて初期化します。
-   `updateOutput()` (demo-library/shared-demo.ts): デモの出力テキストエリアを更新します。
-   `parseAttachmentField()` (demo-library/shared-demo.ts): 添付ファイル入力フィールドのテキスト内容を解析します。
-   `setupPlayButton()` (demo-library/pop-noise-demo.ts): 再生ボタンに関連するイベントリスナーを設定します。
-   `bootstrap()` (demo-library/pop-noise-demo.ts): ポップノイズデモアプリケーションを初期化し、起動します。
-   `initWasm()` (demo-library/library-demo.ts): WebAssemblyモジュールの初期化を行います。
-   `displayResult()` (demo-library/library-demo.ts): 変換結果を画面上の出力エリアに表示します。
-   `showError()` (demo-library/library-demo.ts): エラーメッセージを画面に表示します。
-   `setupFileInput()` (demo-library/library-demo.ts): ファイル入力要素のイベントリスナーを設定し、ファイル選択時の処理を定義します。
-   `parseHexByte()` (demo-library/log-visualizer.ts): 16進数文字列（例: "C7"）を対応するバイト値に解析します。
-   `detectChannel()` (demo-library/log-visualizer.ts): YM2151ログイベントから関連するチャンネルを検出します。
-   `normalizeEvents()` (demo-library/log-visualizer.ts): YM2151ログイベントデータを視覚化に適した形式に正規化します。
-   `laneColor()` (demo-library/log-visualizer.ts): ログ視覚化の各レーンに割り当てる色を決定します。
-   `createLane()` (demo-library/log-visualizer.ts): ログ視覚化のために新しいレーン（トラック）のDOM要素を作成します。
-   `computeTrackWidth()` (demo-library/log-visualizer.ts): ログ視覚化のトラックの幅を計算します。
-   `renderEmpty()` (demo-library/log-visualizer.ts): ログデータがない場合に空の視覚化をレンダリングします。
-   `ensureGlobalLane()` (demo-library/log-visualizer.ts): グローバルイベント（全チャンネルに影響する）用のレーンが確保されていることを確認します。
-   `for` (demo-library/log-visualizer.ts): JavaScriptのループ構文。指定された回数、またはコレクションの各要素に対してコードブロックを繰り返し実行します。
-   `treeToJson()` (demo-library/mml-support.ts, demo-library/tone-json-demo.ts): 構文木（AST）などのツリー構造データをJSON形式に変換します。
-   `ensureMmlRuntime()` (demo-library/mml-support.ts, demo-library/tone-json-demo.ts): MML（Music Macro Language）のランタイム環境が利用可能であることを確認し、必要に応じて初期化します。
-   `buildEventsFromCompact()` (demo-library/tone-json-demo.ts): コンパクトなイベント定義から完全なイベントオブジェクトを構築します。
-   `normalizeAttachmentText()` (demo-library/tone-json-demo.ts): 添付されたテキストデータ（例: JSON）を正規化します。
-   `convertMmlToSmf()` (demo-library/tone-json-demo.ts): MML文字列をStandard MIDI File (SMF) 形式に変換します。
-   `clearAudioCache()` (demo-library/globals.d.ts): 音声再生システムが持つキャッシュをクリアします。（定義のみ、実装は外部）
-   `clearWebYmAudioCache()` (demo-library/shared-demo.ts): WebYM2151プレイヤーの音声キャッシュをクリアします。
-   `cleanup()` (demo-library/shared-demo.ts): デモのリソース（イベントリスナーなど）をクリーンアップします。

## 関数呼び出し階層ツリー
```
- if (demo-library/delay-vibrato-demo.ts)
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
      - catch ()
      - addEventListener ()
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
    - normalizeEvents ()
    - laneColor ()
    - createLane ()
    - computeTrackWidth ()
    - renderEmpty ()
    - ensureGlobalLane ()
- treeToJson (demo-library/mml-support.ts)
  - ensureMmlRuntime ()
    - buildEventsFromCompact ()
    - normalizeAttachmentText ()
    - convertMmlToSmf ()
- clearAudioCache ()
- clearWebYmAudioCache ()
  - cleanup ()
- for (demo-library/log-visualizer.ts)

---
Generated at: 2026-02-16 07:09:10 JST

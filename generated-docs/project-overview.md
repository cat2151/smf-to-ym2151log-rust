Last updated: 2026-03-15

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) を、ヤマハYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するツールです。
- Rustで実装されており、ネイティブアプリケーション向けライブラリおよびWebブラウザで動作するWebAssembly (WASM) ライブラリとして利用できます。
- MIDIチャンネルの和音数に基づくYM2151チャンネルの静的割り当て、ドラムチャンネル優先処理、プログラムチェンジによるカスタム音色適用などの機能を備えています。

## 技術スタック
- フロントエンド:
  - **TypeScript**: デモWebアプリケーションのロジック開発に使用されるプログラミング言語。
  - **HTML**: デモWebページの構造を定義。
  - **CSS**: デモWebページのスタイルを定義。
  - **WebAssembly (WASM)**: Rustで書かれた変換ロジックをWebブラウザで実行可能にする技術。
  - **Vite**: デモWebアプリケーションの開発サーバーおよびビルドツール。
  - **Biome.js**: JavaScript/TypeScriptコードのフォーマットとリント（静的解析）を行う開発ツール。
- 音楽・オーディオ:
  - **Standard MIDI Files (SMF)**: 入力としてサポートされる標準MIDIファイル形式。
  - **YM2151**: 出力されるレジスタログの対象となるFM音源チップ。
  - **JSON**: YM2151レジスタ書き込みログの出力形式、中間イベント、カスタム音色定義に利用。
  - **WAV**: デモWebアプリケーションで生成されたオーディオデータをエクスポートする形式。
- 開発ツール:
  - **Rust**: プロジェクトの主要なプログラミング言語。パフォーマンスと型安全性を重視。
  - **Cargo**: Rustの公式なビルドシステムとパッケージマネージャー。
  - **wasm-pack**: RustからWebAssemblyパッケージを生成するためのツール。
  - **Python**: テスト用のMIDIファイルを生成するために使用。
- テスト:
  - **Rust標準テストフレームワーク**: `cargo test`コマンドで実行される、Rust組み込みのテスト機能。
  - **cargo tarpaulin**: Rustコードのテストカバレッジを測定するツール。
- ビルドツール:
  - **Cargo**: Rustプロジェクトのコンパイルとパッケージング。
  - **wasm-pack**: WebAssemblyモジュールのビルド。
  - **Vite**: フロントエンドデモのビルド。
- 言語機能:
  - **Rust 1.70.0以上**: プロジェクトのビルドに必要なRustコンパイラの最低バージョン。
  - **TypeScript**: デモWebアプリケーションの静的型付けされたJavaScript開発。
- 自動化・CI/CD: (提供情報からは特定のCI/CDツールは読み取れませんが、開発標準ツールは使用されています。)
- 開発標準:
  - **cargo fmt**: Rustコードの自動フォーマッター。
  - **cargo clippy**: Rustコードのリンター（静的解析ツール）で、一般的なエラーや非効率なコードを検出。
  - **cargo audit**: Rustプロジェクトの依存関係のセキュリティ脆弱性を検査するツール。

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
  📖 208.md
  📖 209.md
  📖 211.md
  📖 212.md
  📖 213.md
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
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定します。
- **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定を定義します。
- **`LICENSE`**: プロジェクトのライセンス情報が含まれています。
- **`README.ja.md`**: プロジェクトの日本語での概要と説明を提供します。
- **`README.md`**: プロジェクトの英語での概要と説明を提供します。
- **`WASM_USAGE.md`**: WebAssembly (WASM) 環境でのライブラリの使用方法を詳細に説明します。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルです。
- **`package-lock.json`**: `demo-library`内のJavaScript/TypeScriptプロジェクトの依存関係のバージョンを固定します。
- **`package.json`**: `demo-library`内のJavaScript/TypeScriptプロジェクトのメタデータと依存関係を定義します。
- **`googled947dc864c270e07.html`**: Googleサイトの認証用ファイルです。
- **`demo-library/`**: WebAssemblyで変換されたYM2151ログを視覚化・再生するためのWebデモアプリケーションのソースコードが含まれています。
    - **`demo-library/.gitignore`**: デモライブラリ固有のGit無視設定です。
    - **`demo-library/biome.json`**: Biome.jsによるコード品質チェックの設定ファイルです。
    - **`demo-library/delay-vibrato-demo.ts`**: 遅延ビブラート効果のデモロジックを実装しています。WASMモジュールとの連携、UI操作、視覚化、オーディオ再生を含みます。
    - **`demo-library/delay-vibrato.html`**: 遅延ビブラートデモのHTMLページで、UI要素とスクリプトの読み込みを定義します。
    - **`demo-library/globals.d.ts`**: グローバルスコープで使用される型定義を提供し、型安全な開発を支援します。
    - **`demo-library/index.html`**: デモWebアプリケーションのメインエントリポイントとなるHTMLページです。
    - **`demo-library/library-demo.ts`**: ライブラリとしての基本的な使用例を示すデモロジックを実装しています。
    - **`demo-library/log-visualizer-lfo.ts`**: YM2151のLFO（低周波発振器）イベントを視覚化するためのロジックを提供します。
    - **`demo-library/log-visualizer-note-segments.ts`**: YM2151ログ内のノートイベントを視覚化するためのセグメントデータを構築します。
    - **`demo-library/log-visualizer-pitch-canvas.ts`**: YM2151ログのピッチ情報をグラフィカルに描画するキャンバスコンポーネントです。
    - **`demo-library/log-visualizer.ts`**: YM2151レジスタ書き込みログ全体をタイムライン形式で視覚化する主要なロジックを実装しています。
    - **`demo-library/mml-support.ts`**: Music Macro Language (MML) からStandard MIDI File (SMF) への変換をサポートするロジックを含みます。
    - **`demo-library/pop-noise-demo.ts`**: ポップノイズ検出機能のデモロジックを実装しています。
    - **`demo-library/pop-noise-detector.ts`**: YM2151音源のオーディオデータからポップノイズの発生を検出するロジックを提供します。
    - **`demo-library/pop-noise.html`**: ポップノイズ検出デモのHTMLページです。
    - **`demo-library/portamento-soft-lfo-demo.ts`**: ポルタメントとソフトLFO（滑らかな音程変化）効果のデモロジックを実装しています。
    - **`demo-library/portamento-soft-lfo.html`**: ポルタメントとソフトLFOデモのHTMLページです。
    - **`demo-library/shared-demo.ts`**: 複数のデモ間で共通して使用されるユーティリティ関数、WASM初期化処理、UIステータス管理などを提供します。
    - **`demo-library/style.css`**: デモWebアプリケーションの全体的なスタイルシートです。
    - **`demo-library/tone-interpolation-demo.ts`**: 音色補間（トーン・インターポレーション）機能のデモロジックを実装しています。
    - **`demo-library/tone-interpolation.html`**: 音色補間デモのHTMLページです。
    - **`demo-library/tone-json-attachment.ts`**: カスタム音色JSONファイルを処理し、WASMモジュールへの添付形式に変換するロジックを含みます。
    - **`demo-library/tone-json-demo.ts`**: プログラムチェンジによるカスタム音色使用のデモロジックを実装しています。
    - **`demo-library/tone-json-mml.ts`**: MMLパーサーを介して音色JSONをSMF変換に利用するロジックです。
    - **`demo-library/tone-json.html`**: プログラムチェンジデモのHTMLページです。
    - **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    - **`demo-library/vite.config.ts`**: Viteビルドツールの設定ファイルです。
    - **`demo-library/wav-exporter.ts`**: 生成されたオーディオデータをWAVファイル形式でエクスポートするためのロジックを提供します。
    - **`demo-library/waveform-canvas.ts`**: 音源の波形データを描画するキャンバスコンポーネントです。
    - **`demo-library/waveform-viewer.ts`**: YM2151から生成された波形を視覚的に表示し、ズームやスクロールなどの操作を提供するコンポーネントです。
    - **`demo-library/ym2151-utils.ts`**: YM2151関連のユーティリティ関数（例：16進数パース）を定義します。
- **`generated-docs/`**: `cargo doc`などによって生成されるAPIドキュメントが配置されるディレクトリです。（現時点では空の場合があります）
- **`issue-notes/`**: 開発中に発生した課題、調査結果、解決策などを記録したMarkdown形式のメモファイル群です。
- **`src/`**: Rustの主要なソースコードが格納されています。
    - **`src/error.rs`**: カスタムエラー型とエラー処理のためのロジックを定義します。
    - **`src/lib.rs`**: このプロジェクトが提供するライブラリクレートのメインエントリポイントです。公開されるAPIを定義します。
    - **`src/main.rs`**: コマンドラインアプリケーションのエントリポイントです。MIDIファイル変換の実行ロジックを含みます。
    - **`src/wasm.rs`**: WebAssemblyに公開される関数と構造体を定義し、Webブラウザからの呼び出しを可能にします。
    - **`src/midi/`**: Standard MIDI File (SMF) の解析に関連するモジュールです。
        - **`src/midi/events.rs`**: MIDIイベントの内部表現と関連するロジックを定義します。
        - **`src/midi/mod.rs`**: `midi`モジュールのエントリポイントです。
        - **`src/midi/parser.rs`**: SMFを読み込み、内部のMIDIイベント表現にパースする主要なロジックを実装しています。
        - **`src/midi/utils.rs`**: MIDIデータ処理に関するユーティリティ関数を提供します。
        - **`src/midi/utils_tests.rs`**: `src/midi/utils.rs`に定義された関数の単体テストです。
    - **`src/ym2151/`**: YM2151レジスタログへの変換に関連するモジュールです。
        - **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのボイスチャンネルに割り当てる戦略（和音数ベース、ドラム優先など）を実装します。
        - **`src/ym2151/converter.rs`**: MIDIイベントをYM2151レジスタ書き込みログに変換する主要なロジックを調整します。
        - **`src/ym2151/event_processor.rs`**: 変換されたYM2151イベントを処理し、最終的なレジスタログを生成します。
        - **`src/ym2151/event_processor_tests.rs`**: `src/ym2151/event_processor.rs`の単体テストです。
        - **`src/ym2151/events.rs`**: YM2151レジスタ書き込みイベントの内部表現と関連するロジックを定義します。
        - **`src/ym2151/init.rs`**: YM2151チップの初期化設定に関するロジックを定義します。
        - **`src/ym2151/mod.rs`**: `ym2151`モジュールのエントリポイントです。
        - **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151の周波数設定値の間のマッピングテーブルを管理します。
        - **`src/ym2151/tempo_map.rs`**: MIDIのテンポイベントを追跡し、ティック時間と実時間の間の変換を管理します。
        - **`src/ym2151/tone.rs`**: YM2151の音色（プログラムチェンジによって切り替わる音色データ）の定義と管理を行います。
        - **`src/ym2151/converter/`**: YM2151変換器の詳細な内部ロジックです。
            - **`src/ym2151/converter/event_accumulator.rs`**: YM2151イベントを蓄積し、レジスタ書き込みのシーケンスを生成します。
            - **`src/ym2151/converter/pitch_effects.rs`**: ピッチベンドやビブラートなどのピッチ関連のMIDIエフェクトをYM2151レジスタ値に変換するロジックです。
            - **`src/ym2151/converter/register_effects.rs`**: ボリュームやエンベロープなどの様々なYM2151レジスタエフェクトを処理します。
            - **`src/ym2151/converter/register_fields.rs`**: YM2151レジスタの個々のフィールド（例：オペレータ設定）の定義と操作ロジックを含みます。
            - **`src/ym2151/converter/waveform.rs`**: YM2151の波形生成に関連するロジックを管理します。
        - **`src/ym2151/converter_tests/`**: YM2151変換器の各機能に関する単体テストです。
            - **`src/ym2151/converter_tests/attachments.rs`**: 添付ファイル（カスタム音色など）のテスト。
            - **`src/ym2151/converter_tests/attachments_change_to_next_tone.rs`**: 次の音色への変更に関連する添付ファイルのテスト。
            - **`src/ym2151/converter_tests/attachments_program_effects.rs`**: プログラムエフェクトに関連する添付ファイルのテスト。
            - **`src/ym2151/converter_tests/basic.rs`**: 基本的な変換機能のテスト。
            - **`src/ym2151/converter_tests/channels.rs`**: チャンネル割り当てロジックのテスト。
            - **`src/ym2151/converter_tests/drums.rs`**: ドラムパート変換のテスト。
            - **`src/ym2151/converter_tests/effects.rs`**: MIDIエフェクト変換のテスト。
            - **`src/ym2151/converter_tests/lfo.rs`**: LFO関連機能のテスト。
            - **`src/ym2151/converter_tests/portamento.rs`**: ポルタメント機能のテスト。
            - **`src/ym2151/converter_tests/programs.rs`**: プログラムチェンジと音色管理のテスト。
- **`tests/`**: 統合テストおよびテストデータが含まれるディレクトリです。
    - **`tests/create_test_midi.py`**: テスト用のMIDIファイルをプログラム的に生成するPythonスクリプトです。
    - **`tests/integration_conversion.rs`**: 変換プロセス全体の統合テストです。
    - **`tests/integration_midi.rs`**: MIDIパース機能の統合テストです。
    - **`tests/integration_multichannel.rs`**: マルチチャンネルMIDIファイルの変換に関する統合テストです。
    - **`tests/integration_program_change.rs`**: プログラムチェンジ機能の統合テストです。
    - **`tests/integration_wasm.rs`**: WebAssemblyインターフェースの統合テストです。
    - **`tests/test_data/`**: 各種統合テストで使用されるMIDIファイルのサンプルデータです。
- **`tones/`**: MIDIプログラムチェンジに対応するカスタムYM2151音色定義のJSONファイルが格納されています。
    - **`tones/000.json`**: MIDIプログラム0番のデフォルト音色（アコースティックグランドピアノ）のYM2151レジスタ設定。
    - **`tones/README.md`**: `tones`ディレクトリ内のJSONファイルフォーマットに関する説明を提供します。

## 関数詳細説明
- **`computeHash`** (demo-library/delay-vibrato-demo.ts): 与えられた入力に基づいてハッシュ値を計算し、リクエストの一意性を識別します。
- **`nextRequestId`** (demo-library/delay-vibrato-demo.ts): 新しい一意のリクエストIDを生成します。
- **`isLatestRequest`** (demo-library/delay-vibrato-demo.ts): 現在処理中のリクエストが最新のリクエストであるかを判断します。
- **`updateOutputWithState`** (demo-library/delay-vibrato-demo.ts): アプリケーションの状態（WASMロード状況、変換結果など）に基づいてUIの出力を更新します。
- **`updatePlayButtonState`** (demo-library/delay-vibrato-demo.ts): オーディオ再生ボタンの有効/無効状態やテキストを更新します。
- **`initializeWasm`** (demo-library/delay-vibrato-demo.ts): WebAssemblyモジュールを初期化し、Rustの機能をJavaScript環境で利用可能にします。
- **`readAttachmentBytes`** (demo-library/delay-vibrato-demo.ts): ファイル入力から添付ファイル（例: カスタム音色JSON）の内容をバイト配列として読み込みます。
- **`runConversion`** (demo-library/delay-vibrato-demo.ts): MIDIまたはMMLデータをYM2151ログに変換する非同期処理を実行します。
- **`handlePlay`** (demo-library/delay-vibrato-demo.ts): 「再生」ボタンがクリックされた際のオーディオ再生処理を開始します。
- **`setupAttachmentEditor`** (demo-library/delay-vibrato-demo.ts): 添付ファイルの内容を編集するためのUIコンポーネントをセットアップします。
- **`setupMmlInput`** (demo-library/delay-vibrato-demo.ts): MML（Music Macro Language）形式の入力を受け付けるテキストエリアをセットアップします。
- **`setupMidiInput`** (demo-library/delay-vibrato-demo.ts): MIDIファイルをアップロードするためのファイル入力要素をセットアップします。
- **`bootstrapWebYm`** (demo-library/delay-vibrato-demo.ts): WebYMオーディオ再生システムを初期化し、デモWebアプリケーションに組み込みます。
- **`main`** (demo-library/delay-vibrato-demo.ts): デモWebアプリケーションの主要なエントリポイントであり、初期設定やイベントリスナーの登録を行います。
- **`playAudioWithOverlay`** (demo-library/globals.d.ts): オーディオを再生し、その上に再生状態を示す視覚的なオーバーレイを表示します。
- **`clearAudioCache`** (demo-library/globals.d.ts): 生成されたオーディオデータのキャッシュをクリアします。
- **`generateAudioFromJson`** (demo-library/globals.d.ts): JSON形式のYM2151レジスタログデータからオーディオを生成します。
- **`initWasm`** (demo-library/library-demo.ts): WebAssemblyモジュールを初期化し、その状態に応じてUIを更新します。
- **`displayResult`** (demo-library/library-demo.ts): 変換結果（成功/失敗）をUIに表示します。
- **`showError`** (demo-library/library-demo.ts): エラーメッセージをUIに表示します。
- **`setupFileInput`** (demo-library/library-demo.ts): ファイル選択入力要素を設定し、ファイルが選択された際の処理を定義します。
- **`resolveRegisterForChannel`** (demo-library/log-visualizer-lfo.ts): 特定のチャンネルのLFO（低周波発振器）レジスタに関する情報を解決します。
- **`collectLfoEvents`** (demo-library/log-visualizer-lfo.ts): YM2151ログからLFOイベントを収集し、視覚化のために準備します。
- **`renderLfoLane`** (demo-library/log-visualizer-lfo.ts): LFOイベントのデータをタイムライン上のレーンとして描画します。
- **`buildNoteSegments`** (demo-library/log-visualizer-note-segments.ts): ノートイベントの開始/終了時間とピッチに基づいて視覚化用のノートセグメントデータを構築します。
- **`notePitch`** (demo-library/log-visualizer-note-segments.ts): 特定のノートのピッチ値を取得します。
- **`computePitchRange`** (demo-library/log-visualizer-note-segments.ts): 視覚化するノートイベントの全体のピッチ範囲を計算します。
- **`noteYPosition`** (demo-library/log-visualizer-note-segments.ts): ノートのピッチに基づいて、視覚化キャンバス上でのY軸位置を決定します。
- **`renderPitchCanvas`** (demo-library/log-visualizer-pitch-canvas.ts): ピッチ情報を描画するための専用キャンバスに、ノートやピッチベンドのデータを表示します。
- **`detectChannel`** (demo-library/log-visualizer.ts): YM2151ログ内のイベントから、各チャンネルの利用状況を検出します。
- **`normalizeEvents`** (demo-library/log-visualizer.ts): YM2151イベントデータを視覚化に適した形式に正規化します。
- **`laneColor`** (demo-library/log-visualizer.ts): 各視覚化レーンに割り当てる色を決定します。
- **`createLane`** (demo-library/log-visualizer.ts): ログ視覚化のために新しいレーン（時間軸上の表示領域）を作成します。
- **`computeTrackWidth`** (demo-library/log-visualizer.ts): 視覚化トラックの全体的な幅を計算します。
- **`formatInactiveChannels`** (demo-library/log-visualizer.ts): 使用されていないYM2151チャンネルの情報を整形して表示します。
- **`createLogVisualizer`** (demo-library/log-visualizer.ts): YM2151レジスタログを視覚化するための主要なコンポーネントを初期化し、設定します。
- **`renderEmpty`** (demo-library/log-visualizer.ts): ログデータがない場合に、視覚化領域を空の状態で描画します。
- **`renderFromJson`** (demo-library/log-visualizer.ts): JSON形式のYM2151ログデータを受け取り、それを視覚化コンポーネントに描画します。
- **`ensureGlobalLane`** (demo-library/log-visualizer.ts): すべてのチャンネルに共通するグローバルな視覚化レーンが存在することを保証します。
- **`setLfoRegisters`** (demo-library/log-visualizer.ts): LFOレジスタの値を設定し、視覚化に反映させます。
- **`setupMmlToSmf`** (demo-library/mml-support.ts): MMLからSMFへの変換機能のUI要素とイベントハンドラーをセットアップします。
- **`detectPopNoise`** (demo-library/pop-noise-detector.ts): YM2151のオーディオ波形データから不快なポップノイズを検出します。
- **`extractLfoRegistersFromAttachment`** (demo-library/portamento-soft-lfo-demo.ts): 添付ファイル（通常はカスタム音色JSON）からLFOレジスタ設定を抽出します。
- **`syncLfoRegisters`** (demo-library/portamento-soft-lfo-demo.ts): LFOレジスタの値をUIと内部状態間で同期させます。
- **`ensureWasmInitialized`** (demo-library/shared-demo.ts): WebAssemblyモジュールがロードされ、初期化されていることを確認します。
- **`setStatus`** (demo-library/shared-demo.ts): デモアプリケーションのステータスメッセージ（例: 処理中、成功）をUIに表示します。
- **`setEventCountDisplay`** (demo-library/shared-demo.ts): 変換されたYM2151イベントの総数をUIに表示します。
- **`ensureWebYm2151`** (demo-library/shared-demo.ts): WebYM2151オーディオエンジンがインスタンス化され、利用可能であることを保証します。
- **`clearWebYmAudioCache`** (demo-library/shared-demo.ts): WebYM2151オーディオエンジンの内部キャッシュをクリアします。
- **`updateOutput`** (demo-library/shared-demo.ts): 指定されたコンテンツで出力表示エリアを更新します。
- **`parseAttachmentField`** (demo-library/shared-demo.ts): 添付ファイル入力フィールドからテキストデータをパースします。
- **`cleanup`** (demo-library/shared-demo.ts): 不要なリソースを解放するクリーンアップ処理を実行します。
- **`buildEventsFromCompact`** (demo-library/tone-json-attachment.ts): コンパクトなJSON形式からYM2151イベント構造を構築します。
- **`serializeWithStatus`** (demo-library/tone-json-attachment.ts): イベントデータをJSON形式でシリアライズし、その過程でステータス情報を更新します。
- **`normalizeAttachmentText`** (demo-library/tone-json-attachment.ts): 添付ファイルとして入力されたテキストデータを正規化（整形）します。
- **`convertMmlToSmf`** (demo-library/tone-json-demo.ts): MML（Music Macro Language）形式のテキストをStandard MIDI File (SMF) データに変換します。
- **`getMmlParser`** (demo-library/tone-json-mml.ts): MMLを解析するためのパーサーインスタンスを取得します。
- **`getParseTreeJsonToSmf`** (demo-library/tone-json-mml.ts): 解析ツリー（Parse Tree）からSMFデータを生成するための変換ロジックを取得します。
- **`treeToJson`** (demo-library/tone-json-mml.ts): 抽象構文木（AST）などのツリー構造をJSON形式に変換します。
- **`ensureMmlRuntime`** (demo-library/tone-json-mml.ts): MML解析に必要なランタイム環境が確保されていることを確認します。
- **`encodeWav`** (demo-library/wav-exporter.ts): 生のオーディオデータをWAVファイルフォーマットにエンコードします。
- **`writeAscii`** (demo-library/wav-exporter.ts): WAVファイルヘッダの一部としてASCII文字列を書き込みます。
- **`downloadWav`** (demo-library/wav-exporter.ts): 生成されたWAVファイルをユーザーのブラウザにダウンロードさせます。
- **`drawEmpty`** (demo-library/waveform-canvas.ts): 波形キャンバスを空の状態で描画します。
- **`drawWaveform`** (demo-library/waveform-canvas.ts): 生成されたオーディオ波形データをキャンバスに描画します。
- **`extractNoteBoundaries`** (demo-library/waveform-viewer.ts): 音源波形からノートの開始と終了の境界を抽出します。
- **`normalizeAmplitude`** (demo-library/waveform-viewer.ts): 波形データの振幅（音量）を正規化し、視覚化に適した範囲に調整します。
- **`createWaveformViewer`** (demo-library/waveform-viewer.ts): 波形を視覚的に表示し、操作するためのビューアコンポーネントを初期化します。
- **`getWindowDurS`** (demo-library/waveform-viewer.ts): 現在表示されている波形ウィンドウの長さを秒単位で取得します。
- **`clampViewStart`** (demo-library/waveform-viewer.ts): 波形ビューの開始位置を有効な範囲内に制限します。
- **`updatePositionLabel`** (demo-library/waveform-viewer.ts): 波形ビューの現在の位置を示すラベルを更新します。
- **`render`** (demo-library/waveform-viewer.ts): 波形ビューアの現在の状態に基づいて、波形を再描画します。
- **`updateBoundariesAndRender`** (demo-library/waveform-viewer.ts): 波形ビューの境界設定を更新し、それに基づいて再描画を行います。
- **`synthesizeAndRender`** (demo-library/waveform-viewer.ts): YM2151ログからオーディオを合成し、その波形をビューアにレンダリングします。
- **`setZoom`** (demo-library/waveform-viewer.ts): 波形ビューのズームレベルを設定します。
- **`clear`** (demo-library/waveform-viewer.ts): 波形ビューアの表示内容をクリアします。
- **`exportWav`** (demo-library/waveform-viewer.ts): 現在表示されている波形をWAVファイルとしてエクスポートします。
- **`parseHexByte`** (demo-library/ym2151-utils.ts): 16進数表記の文字列をパースしてバイト値に変換します。

## 関数呼び出し階層ツリー
```
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
  - setupWavExportButton ()
  - bootstrap ()
  - createWaveformViewer ()
  - exportWav ()
  - setLfoRegisters (demo-library/log-visualizer.ts)
  - extractLfoRegistersFromAttachment ()
  - syncLfoRegisters ()
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
    - normalizeAttachmentText ()
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

---
Generated at: 2026-03-15 07:10:07 JST

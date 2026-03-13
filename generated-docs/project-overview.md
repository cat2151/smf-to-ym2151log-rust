Last updated: 2026-03-14

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- ネイティブアプリケーションとして、またはWebAssembly (WASM) ライブラリとしてブラウザ上で動作し、高度なチャンネル割り当て戦略に対応します。
- カスタム音色定義や2パス処理アーキテクチャ、充実したテストによる堅牢性が特徴です。

## 技術スタック
- フロントエンド: 
  - **TypeScript (TS)**: Webデモのロジック開発に使用されるJavaScriptの型付きスーパーセット。
  - **HTML**: Webデモのページ構造を定義。
  - **CSS**: Webデモの視覚的なスタイルを提供。
  - **WebAssembly (WASM)**: Rustで書かれたコアロジックをWebブラウザで実行可能にする技術。
  - **Vite**: Webデモの高速な開発とビルドを可能にするフロントエンドビルドツール。
- 音楽・オーディオ: 
  - **Standard MIDI Files (SMF)**: 入力となる標準的なMIDIファイル形式 (Format 0および1をサポート)。
  - **YM2151 FM音源チップ**: 出力されるレジスタ書き込みログの対象となるヤマハ製FM音源チップ。
  - **JSON**: YM2151レジスタ書き込みログや中間イベント、カスタム音色定義に使用されるデータ交換フォーマット。
- 開発ツール: 
  - **Rust**: プロジェクトの主要なプログラミング言語。型安全性とパフォーマンスを重視。
  - **Cargo**: Rustプロジェクトのビルドシステムおよびパッケージマネージャー。
  - **wasm-pack**: RustコードをWebAssemblyにビルドし、npmパッケージとして利用可能にするツール。
  - **Git**: ソースコードのバージョン管理システム。
  - **Biome**: JavaScript/TypeScriptコードのフォーマッター兼リンター。
- テスト: 
  - **cargo test**: Rustの組み込みテストフレームワークによるユニットテストおよび統合テスト。
  - **cargo tarpaulin**: Rustコードのテストカバレッジレポート生成ツール。
- ビルドツール: 
  - **Cargo**: Rustアプリケーションおよびライブラリのビルド。
  - **wasm-pack**: Rust for WASMのビルド。
  - **Vite**: `demo-library` のビルド。
- 言語機能: 
  - **Rustの型システム**: コンパイル時の堅牢性とバグの早期発見に貢献。
  - **Rustのパフォーマンス**: ネイティブコンパイルによる高速な処理能力。
- 自動化・CI/CD: 
  - **GitHub Actions (Implicit)**: `shields.io` バッジから推測されるCI/CD環境。`cargo test`, `cargo fmt --check`, `cargo clippy`, `cargo audit` などのコマンドが自動化パイプラインで利用されている可能性。
- 開発標準: 
  - **cargo fmt**: Rustコードの自動フォーマットツール。
  - **cargo clippy**: Rustコードのlintチェックツール。
  - **cargo audit**: Rustプロジェクトの依存関係のセキュリティ脆弱性チェックツール。

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
プロジェクトは主にRustで書かれたコアロジック (`src/` ディレクトリ) と、Webブラウザ上で動作するデモ (`demo-library/` ディレクトリ) で構成されています。

**Rustコア機能関連 (`src/`):**
-   `src/lib.rs`: Rustライブラリのエントリポイント。プロジェクト全体の変換ロジックを統合し、外部クレートから利用可能なAPIを提供します。
-   `src/main.rs`: コマンドラインアプリケーションのエントリポイント。MIDIファイルのパスを受け取り、YM2151ログに変換する処理を実行します。
-   `src/error.rs`: プロジェクト全体で使用されるカスタムエラー型を定義し、エラーハンドリングを統一します。
-   `src/midi/parser.rs`: Standard MIDI Files (SMF) を解析し、MIDIイベントの時系列データを抽出する主要なパーサーロジックを実装しています。
-   `src/midi/events.rs`: MIDIファイルから抽出される様々なイベント（ノートオン/オフ、テンポチェンジなど）のデータ構造を定義します。
-   `src/ym2151/converter.rs`: MIDIイベントの中間表現をYM2151レジスタ書き込みログに変換する主要なロジックを実装しています。2パス処理の「パスB」に該当します。
-   `src/ym2151/channel_allocation.rs`: YM2151の限られた8チャンネルをMIDIチャンネルの和音数に基づいて静的に割り当て、ドラムチャンネル優先のルールを適用するロジックを実装しています。
-   `src/ym2151/tone.rs`: YM2151の音色（ボイス）定義を管理します。MIDIプログラムチェンジイベントに応じて外部JSONファイル (`tones/`) からカスタム音色をロードする機能を提供します。
-   `src/ym2151/event_processor.rs`: YM2151のイベントを処理し、レジスタ書き込みのシーケンスを生成するロジックが含まれています。
-   `src/wasm.rs`: WebAssembly (WASM) のバインディングを提供し、Rustで書かれたコア変換ロジックをJavaScriptから呼び出せるようにします。

**Webデモ関連 (`demo-library/`):**
-   `demo-library/*.html`: 各デモページ（例: `index.html`, `delay-vibrato.html`）のHTML構造を定義し、WebAssemblyモジュールのロードやUI要素を配置します。
-   `demo-library/library-demo.ts`: WebブラウザでRustライブラリ (`smf_to_ym2151log`) を使用する基本的なデモロジックを提供します。
-   `demo-library/log-visualizer.ts`: YM2151レジスタログを時間軸に沿って視覚的に表示するためのコンポーネントを実装しています。チャンネルごとのノートやレジスタの変化をグラフィカルに表現します。
-   `demo-library/mml-support.ts`: MML (Music Macro Language) からStandard MIDI File (SMF) へ変換する機能を提供し、デモでMML入力からの変換を可能にします。
-   `demo-library/shared-demo.ts`: 複数のWebデモ間で共通して使用されるユーティリティ関数やWASM初期化ロジックを含んでいます。
-   `demo-library/wav-exporter.ts`: 生成されたYM2151のオーディオデータをWAV形式のファイルとしてエクスポートする機能を提供します。
-   `demo-library/waveform-viewer.ts`: YM2151ログから合成された音波の波形を視覚化し、再生位置やズーム機能を提供するコンポーネントです。
-   `demo-library/style.css`: デモページのルック＆フィールを定義するスタイルシートです。
-   `demo-library/package.json`, `tsconfig.json`, `vite.config.ts`: WebデモのTypeScriptプロジェクト設定、依存関係、ビルド設定を定義します。

**設定・ドキュメント・テスト等:**
-   `Cargo.toml`: Rustプロジェクトの依存関係、メタデータ、ビルド設定を定義するファイル。
-   `README.md`, `README.ja.md`: プロジェクトの概要、目的、使い方、機能などを説明するドキュメント（英語・日本語）。
-   `WASM_USAGE.md`: WebAssembly利用に関する詳細なドキュメント。
-   `tones/`: MIDIプログラムチェンジに対応するカスタムYM2151音色定義をJSON形式で格納するディレクトリ。
-   `tests/`: プロジェクトの統合テストおよびテストデータ (`test_data/`) を格納するディレクトリ。

## 関数詳細説明
本プロジェクトの主要な関数群は以下の通りです。Rustのコア変換ロジックとWebデモの主要な機能に関連するものを中心に説明します。

**Rustコア変換ロジック関連:**
-   **`smf_to_ym2151log::convert_midi_to_ym2151` (概念的):**
    -   役割: MIDIファイルをYM2151レジスタ書き込みログに変換する、プロジェクトの中心的な処理。
    -   引数: MIDIファイルのバイナリデータまたはパス、変換オプション（例：音色データ）。
    -   戻り値: YM2151レジスタ書き込みログのJSON文字列またはエラー。
    -   機能: MIDI解析、チャンネル割り当て、イベント変換、YM2151レジスタ値の生成を行います。
-   **`midi::parser::parse_smf(midi_data: &[u8])`:**
    -   役割: Standard MIDI File (SMF) のバイナリデータを解析し、構造化されたMIDIイベントのリストを生成します。
    -   引数: MIDIファイルのバイトスライス。
    -   戻り値: パースされたMIDIイベントの中間表現またはエラー。
    -   機能: MIDIヘッダ、トラックチャンクを読み取り、各MIDIイベント（ノートオン/オフ、プログラムチェンジ、テンポなど）を時系列順に抽出します。
-   **`ym2151::converter::convert_events_to_ym2151_log(midi_events: Vec<MidiEvent>, ...)`:**
    -   役割: パースされたMIDIイベントの中間表現を、YM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換します。
    -   引数: MIDIイベントのリスト、チャンネル割り当て情報、音色情報など。
    -   戻り値: YM2151レジスタ書き込みログのJSON表現またはエラー。
    -   機能: MIDIイベントをYM2151の物理的なレジスタ操作にマッピングし、音色適用、ピッチ計算、エフェクト処理などを行います。
-   **`ym2151::channel_allocation::allocate_channels(midi_tracks: &[MidiTrack])`:**
    -   役割: MIDIファイル内の各MIDIチャンネルの和音数（同時発音数）を分析し、YM2151の利用可能な8チャンネルに静的に割り当てます。
    -   引数: MIDIトラックのリスト。
    -   戻り値: 各MIDIチャンネルへのYM2151チャンネル割り当て情報。
    -   機能: 最大和音数に基づいてYM2151チャンネルを割り当て、ドラムチャンネル（MIDI ch9）を優先的に割り当てるロジックを含みます。
-   **`ym2151::tone::load_tone(program_number: u8, custom_tones: &[ToneDefinition])`:**
    -   役割: 指定されたプログラム番号（0-127）に対応するYM2151音色をロードします。カスタム音色ファイルが存在する場合はそれを優先します。
    -   引数: MIDIプログラム番号、カスタム音色の定義リスト。
    -   戻り値: YM2151音色のデータ構造。
    -   機能: `tones/` ディレクトリからJSONファイルを読み込み、音色のFMパラメータを設定します。
-   **`wasm::convert(midi_bytes: &[u8], attachment_json: &str)`:**
    -   役割: WebAssembly経由でJavaScriptから呼び出されるエントリポイント。MIDIバイト列とアタッチメントJSONを受け取り、YM2151ログJSONを返します。
    -   引数: MIDIファイルの内容をバイト配列、アタッチメントデータ（プログラムチェンジ用の音色定義など）をJSON文字列。
    -   戻り値: 変換されたYM2151ログのJSON文字列。
    -   機能: 内部のRust変換ロジックを呼び出し、Webブラウザ環境で利用可能なインターフェースを提供します。

**Webデモ機能関連 (`demo-library/`):**
-   **`demo-library/log-visualizer.ts::createLogVisualizer(container_element: HTMLElement)`:**
    -   役割: YM2151レジスタログを表示するためのUIコンポーネントを初期化し、指定されたHTML要素内に配置します。
    -   引数: 視覚化コンポーネントを埋め込むHTML要素。
    -   戻り値: ログ視覚化オブジェクトのインスタンス。
    -   機能: ログデータに基づいてタイムライン、ノートイベント、レジスタ値の変化などをグラフィカルに描画します。
-   **`demo-library/mml-support.ts::setupMmlToSmf(mml_input_element: HTMLTextAreaElement, ...)`:**
    -   役割: MML (Music Macro Language) をSMF (Standard MIDI File) に変換する機能のセットアップをデモページで行います。
    -   引数: MML入力用のテキストエリア、変換結果を表示する要素など。
    -   戻り値: なし。
    -   機能: MMLパーサーをロードし、ユーザーがMMLを入力すると自動的にSMFに変換して利用できるようにします。
-   **`demo-library/wav-exporter.ts::downloadWav(audio_data: Float32Array, sample_rate: number, filename: string)`:**
    -   役割: 生成された生のオーディオデータをWAVファイル形式にエンコードし、ユーザーにダウンロードさせます。
    -   引数: オーディオデータ（Float32Array）、サンプルレート、ファイル名。
    -   戻り値: なし。
    -   機能: オーディオバッファをWAVヘッダと共にバイナリ形式に変換し、ブラウザのダウンロード機能を通じてファイルとして提供します。
-   **`demo-library/waveform-viewer.ts::synthesizeAndRender(ym2151_log_json: string, ...)`:**
    -   役割: YM2151ログのJSONデータから音波を合成し、その波形をキャンバス上に描画します。
    -   引数: YM2151ログのJSON文字列、表示範囲、ズームレベルなど。
    -   戻り値: なし。
    -   機能: YM2151のレジスタ操作をシミュレートしてオーディオサンプルを生成し、`waveform-canvas` を使用して波形として描画します。

## 関数呼び出し階層ツリー
```
- if (demo-library/delay-vibrato-demo.ts)
  - computeHash()
    - nextRequestId()
    - isLatestRequest()
    - updateOutputWithState()
    - updatePlayButtonState()
    - initializeWasm()
    - readAttachmentBytes()
    - runConversion()
      - handlePlay()
      - playAudioWithOverlay()
      - createLogVisualizer()
        - renderFromJson()
      - setupAttachmentEditor()
      - setupMmlInput()
        - setupMmlToSmf()
      - setupMidiInput()
      - bootstrapWebYm()
      - main()
      - catch()
      - ensureWasmInitialized()
      - setStatus()
      - setEventCountDisplay()
      - ensureWebYm2151()
      - updateOutput()
      - parseAttachmentField()
      - setupPlayButton()
      - setupWavExportButton()
      - bootstrap()
      - createWaveformViewer()
        - exportWav()
      - setLfoRegisters()
      - extractLfoRegistersFromAttachment()
      - syncLfoRegisters()
- initWasm(demo-library/library-demo.ts)
  - displayResult()
    - showError()
    - setupFileInput()
    - clear()
- resolveRegisterForChannel(demo-library/log-visualizer-lfo.ts)
  - collectLfoEvents()
    - renderLfoLane()
      - createLane()
      - parseHexByte()
- buildNoteSegments(demo-library/log-visualizer-note-segments.ts)
  - notePitch()
    - computePitchRange()
    - noteYPosition()
- renderPitchCanvas(demo-library/log-visualizer-pitch-canvas.ts)
- detectChannel(demo-library/log-visualizer.ts)
  - normalizeEvents()
    - laneColor()
    - computeTrackWidth()
    - formatInactiveChannels()
    - renderEmpty()
    - ensureGlobalLane()
- getMmlParser()
  - getParseTreeJsonToSmf()
    - treeToJson()
    - ensureMmlRuntime()
- detectPopNoise(demo-library/pop-noise-detector.ts)
- clearAudioCache()
  - generateAudioFromJson()
- clearWebYmAudioCache()
  - cleanup()
- buildEventsFromCompact(demo-library/tone-json-attachment.ts)
  - serializeWithStatus()
    - normalizeAttachmentText()
- convertMmlToSmf()
- drawEmpty(demo-library/waveform-canvas.ts)
  - drawWaveform()
- downloadWav()
  - encodeWav(demo-library/wav-exporter.ts)
    - writeAscii()
- extractNoteBoundaries(demo-library/waveform-viewer.ts)
  - getWindowDurS()
    - clampViewStart()
    - updatePositionLabel()
    - render()
    - updateBoundariesAndRender()
    - synthesizeAndRender()
    - setZoom()
- endDrag(demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-14 07:12:16 JST

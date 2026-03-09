Last updated: 2026-03-10

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) を、YM2151 FM音源チップ向けのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- WebAssembly (WASM) に対応しており、ブラウザ環境での利用や、他のRustプロジェクトからのライブラリとしての組み込みが可能です。
- 2パス処理、プログラムチェンジ対応、YM2151チャンネル割り当て戦略、型安全性、高パフォーマンスなどの特徴を持ちます。

## 技術スタック
- フロントエンド: TypeScript (デモアプリケーションのロジック), HTML (ユーザーインターフェース), CSS (スタイル設定), Vite (デモアプリケーションのビルドツール), WebAssembly (Rustコードのブラウザ実行)
- 音楽・オーディオ: Standard MIDI Files (SMF) (入力フォーマット), YM2151 FM音源チップ (ターゲット音源チップ), YM2151レジスタ書き込みログ (出力フォーマット), General MIDI (ドラムチャンネル割り当ての考慮)
- 開発ツール: Rust (主要な開発言語), Cargo (Rustのパッケージマネージャーおよびビルドシステム), git (バージョン管理), wasm-pack (RustからWASMへのビルドツール), tarpaulin (Rustテストカバレッジツール)
- テスト: cargo test (Rustのテストフレームワーク), ユニットテスト (Rust内部のモジュールテスト), 統合テスト (エンドツーエンドの変換検証、WASM機能テスト)
- ビルドツール: Cargo (Rustプロジェクトのビルド), wasm-pack (WASMパッケージの生成), Vite (TypeScript/Webデモのビルド)
- 言語機能: Rustの強力な型システム (堅牢なコードの保証), パターンマッチング, ライフタイム管理
- 自動化・CI/CD: cargo install (アプリケーションのインストール), cargo fmt (コードフォーマットの自動化), cargo clippy (Lintチェックによるコード品質向上)
- 開発標準: cargo fmt (Rustコードの自動フォーマット), cargo clippy (Rustコードの静的解析), biome.json (TypeScript/JavaScriptコードのフォーマット・Lint設定)

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
  📘 envelope-generator.ts
  📘 globals.d.ts
  🌐 index.html
  📘 library-demo.ts
  📘 log-visualizer-lfo.ts
  📘 log-visualizer-note-segments.ts
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
  📘 tone-interpolation-demo.ts
  🌐 tone-interpolation.html
  📘 tone-json-attachment.ts
  📘 tone-json-demo.ts
  📘 tone-json-mml.ts
  🌐 tone-json.html
  📊 tsconfig.json
  📘 vite.config.ts
  📘 waveform-canvas.ts
  📘 waveform-simulator.ts
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
      📄 attachments.rs
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
-   **Cargo.toml**: Rustプロジェクトのビルド設定、依存関係、パッケージ情報が記述されたファイル。
-   **README.ja.md / README.md**: プロジェクトの目的、使い方、機能などを説明する多言語対応の概要ドキュメント。
-   **WASM_USAGE.md**: WebAssembly (WASM) 環境でのプロジェクトの利用方法に関する詳細な説明。
-   **src/main.rs**: コマンドラインアプリケーションのエントリーポイント。MIDIファイルを読み込み、YM2151ログに変換して出力する処理を管理します。
-   **src/lib.rs**: `smf-to-ym2151log-rust`クレートのライブラリとしての公開APIを定義。他のRustプロジェクトやWebAssemblyから利用される主要な変換機能を提供します。
-   **src/error.rs**: プロジェクト内で発生するカスタムエラー型を定義し、エラーハンドリングを一元化します。
-   **src/midi/parser.rs**: Standard MIDI Files (SMF) をパースし、内部の中間イベント形式に変換する主要なロジックを実装しています。
-   **src/midi/events.rs**: MIDIイベントの内部表現（構造体やenum）を定義しています。
-   **src/wasm.rs**: WebAssembly (WASM) とRust間のインターフェースを定義。ブラウザ環境からRustの変換ロジックを呼び出せるようにするバインディングを提供します。
-   **src/ym2151/converter.rs**: 中間イベントからYM2151レジスタ書き込みログへの変換を行う主要なロジックを実装。チャンネル割り当てや音色処理、エフェクト適用などが含まれます。
-   **src/ym2151/channel_allocation.rs**: YM2151の8チャンネルをMIDIチャンネルの和音数に基づいて静的に割り当てる戦略を実装。ドラムチャンネル優先の並び替えルールもここに定義されています。
-   **src/ym2151/tone.rs**: MIDIプログラムチェンジイベントに基づいてYM2151の音色（カスタム音色や内蔵デフォルト音色）を管理し、適用する機能を提供します。
-   **src/ym2151/tempo_map.rs**: MIDIファイル内のテンポイベントを解析し、時間軸の変換に利用するテンポマップを管理します。
-   **src/ym2151/event_processor.rs**: YM2151レジスタログイベントの生成と、イベントキューの管理を行う処理を担います。
-   **src/ym2151/converter/pitch_effects.rs**: ピッチベンドやビブラートなど、音の高さに関連するエフェクトの処理ロジックを実装します。
-   **src/ym2151/converter/register_effects.rs**: YM2151のレジスタに直接作用する様々なエフェクト（エンベロープ、LFOなど）の処理ロジックを提供します。
-   **tests/**: プロジェクト全体の統合テストを格納するディレクトリ。変換の正確性やWASM連携などを検証します。
-   **tests/integration_conversion.rs**: MIDIファイルからYM2151ログへのエンドツーエンドの変換プロセスを検証するテスト。
-   **tests/integration_wasm.rs**: WebAssembly版の変換機能がブラウザ環境で正しく動作するかを検証するテスト。
-   **tones/**: プログラムチェンジで使用されるカスタムYM2151音色定義をJSON形式で格納するディレクトリ。
-   **tones/000.json**: MIDIプログラム番号000に対応するカスタムYM2151音色定義ファイル。
-   **demo-library/index.html**: WebAssembly版ライブラリのデモサイトのメインページ。
-   **demo-library/library-demo.ts**: ブラウザ上でMIDIファイル変換を試せる基本的なデモのロジックを実装しています。
-   **demo-library/log-visualizer.ts**: 変換されたYM2151レジスタログイベントをグラフィカルにタイムラインで表示する視覚化コンポーネントです。
-   **demo-library/mml-support.ts**: Music Macro Language (MML) からStandard MIDI File (SMF) への変換をサポートするための機能を提供します。
-   **demo-library/shared-demo.ts**: 複数のデモページ間で共有されるWebAssembly初期化処理、ステータス表示、オーディオキャッシュ管理などの共通ユーティリティ関数を集めたファイルです。
-   **demo-library/waveform-viewer.ts**: YM2151のレジスタ設定に基づいてFM合成される波形をシミュレートし、表示するビューアコンポーネントです。
-   **demo-library/ym2151-utils.ts**: YM2151関連のレジスタ値のパースや計算など、共通のユーティリティ関数を提供します。
-   **demo-library/vite.config.ts**: デモアプリケーションをビルドするためのVite設定ファイルです。

## 関数詳細説明
-   **computeHash (demo-library/delay-vibrato-demo.ts)**: 特定のリクエストやデータセットの一意なハッシュ値を計算します。引数: `string`, 戻り値: `string`。
-   **nextRequestId ()**: 新しい一意のリクエストIDを生成して返します。引数: なし, 戻り値: `number`。
-   **isLatestRequest ()**: 特定のリクエストIDが現在処理中の最新のリクエストであるかを確認します。引数: `number`, 戻り値: `boolean`。
-   **updateOutputWithState ()**: アプリケーションの内部状態（変換結果、エラーなど）に基づいて、Webページ上の出力表示を更新します。引数: `object (state)`, 戻り値: `void`。
-   **updatePlayButtonState ()**: 変換や再生の状態に応じて、再生ボタンの有効/無効状態や表示テキストを更新します。引数: `boolean (isPlaying)`, 戻り値: `void`。
-   **initializeWasm ()**: WebAssemblyモジュールを非同期でロードし、Rustで実装された変換機能がブラウザのJavaScriptから利用できるように初期化します。引数: なし, 戻り値: `Promise<void>`。
-   **readAttachmentBytes ()**: ユーザーが添付ファイルとして提供した内容を非同期で読み込み、バイト配列として返します。引数: `HTMLInputElement`, 戻り値: `Promise<Uint8Array | null>`。
-   **runConversion ()**: 指定されたMIDIファイルデータ（またはMML）とカスタムYM2151音色情報に基づき、RustのWASM関数を呼び出してYM2151レジスタログへの変換を実行します。引数: `object (conversionOptions)`, 戻り値: `Promise<object>`。
-   **handlePlay ()**: 変換されたYM2151ログをWebオーディオAPIなどを利用して再生するための処理を開始します。再生状態の管理も行います。引数: なし, 戻り値: `void`。
-   **setupAttachmentEditor ()**: カスタムYM2151音色JSONなどの添付ファイルを編集するためのUI要素（テキストエリア、ファイル入力など）をセットアップします。引数: なし, 戻り値: `void`。
-   **setupMmlInput ()**: MML（Music Macro Language）入力フィールドと、MML変更時に変換処理をトリガーするイベントリスナーをセットアップします。引数: なし, 戻り値: `void`。
-   **setupMidiInput ()**: MIDIファイル入力フィールドと、MIDIファイルが選択された際に変換処理をトリガーするイベントリスナーをセットアップします。引数: なし, 戻り値: `void`。
-   **bootstrapWebYm ()**: WebAssemblyベースのYM2151エミュレータまたはプレーヤーを初期化し、オーディオ再生に必要なコンテキストを設定します。引数: なし, 戻り値: `Promise<void>`。
-   **main ()**: デモアプリケーションの主要なエントリーポイント。ページロード時の初期化、UIイベントリスナーの設定、WASMモジュールのロードなど、全体を統括します。引数: なし, 戻り値: `void`。
-   **kcToFrequency (demo-library/envelope-generator.ts)**: MIDIキーコード（ノート番号）をYM2151の内部で使われる周波数値（F-Numberなど）に変換します。引数: `number (key_code)`, 戻り値: `number`。
-   **ampStepPerSample ()**: YM2151のエンベロープジェネレータが1サンプルあたりに進む振幅のステップ値を計算します。引数: `number (decay_rate), number (sample_rate)`, 戻り値: `number`。
-   **initWasm (demo-library/library-demo.ts)**: WebAssemblyモジュールの初期化プロセスを開始し、その完了を待ちます。エラーハンドリングも含まれます。引数: なし, 戻り値: `Promise<void>`。
-   **displayResult ()**: 変換が成功した場合、YM2151ログの視覚化やダウンロードリンクなどをWebページに表示します。引数: `object (result)`, 戻り値: `void`。
-   **showError ()**: 変換プロセス中に発生したエラーメッセージをユーザーインターフェースに表示します。引数: `string | Error`, 戻り値: `void`。
-   **resolveRegisterForChannel (demo-library/log-visualizer-lfo.ts)**: 特定のYM2151チャンネルに紐づくLFO関連のレジスタアドレス（例: LFO周波数、振幅）を解決します。引数: `number (channel_index), string (register_name)`, 戻り値: `string (hex_address)`。
-   **collectLfoEvents ()**: YM2151レジスタログからLFO (Low Frequency Oscillator) の設定変更に関連するイベントを収集し、視覚化に適した形式で返します。引数: `object (ym2151_log_events)`, 戻り値: `Array<object>`。
-   **renderLfoLane ()**: LFOイベントをタイムライン上に視覚化するレーンを描画します。引数: `HTMLElement (lane_container), Array<object> (lfo_events)`, 戻り値: `void`。
-   **buildNoteSegments (demo-library/log-visualizer-note-segments.ts)**: YM2151ログ内のノートオン/オフイベントから、視覚化のためのノートセグメント（開始時刻、終了時刻、ピッチ、チャンネルなど）の情報を構築します。引数: `object (ym2151_log)`, 戻り値: `Array<object>`。
-   **notePitch ()**: 特定のノートイベントのピッチ情報を抽出し、視覚化に適した形式で返します。引数: `object (note_event)`, 戻り値: `number`。
-   **computePitchRange ()**: 視覚化するノートのピッチの最小値と最大値を計算し、表示スケールを決定します。引数: `Array<object> (note_segments)`, 戻り値: `object ({ minPitch, maxPitch })`。
-   **noteYPosition ()**: ノートのピッチに基づいて、視覚化上のY軸位置を計算します。引数: `number (pitch), number (min_pitch), number (max_pitch), number (height)`, 戻り値: `number`。
-   **detectChannel (demo-library/log-visualizer.ts)**: YM2151ログ内のレジスタアドレスに基づいて、そのイベントがどのYM2151チャンネルに関連するかを検出します。引数: `string (register_address)`, 戻り値: `number | null`。
-   **normalizeEvents ()**: 視覚化のために、YM2151ログイベントのタイムスタンプや値などを正規化・整形します。引数: `Array<object> (raw_events)`, 戻り値: `Array<object>`。
-   **laneColor ()**: 視覚化レーンのチャネル番号に基づいて、一貫した色を返します。引数: `number (channel_index)`, 戻り値: `string (css_color)`。
-   **createLane ()**: 視覚化のタイムライン上に新しいレーン（HTML要素）を作成し、適切なスタイルを適用します。引数: `string (label), string (color)`, 戻り値: `HTMLElement`。
-   **computeTrackWidth ()**: 視覚化トラック全体の幅を、ログの総時間と設定されたスケールに基づいて計算します。引数: `number (total_time), number (scale_factor)`, 戻り値: `number`。
-   **createLogVisualizer ()**: YM2151ログを視覚化するためのコンポーネント全体を初期化し、DOMに描画します。引数: `HTMLElement (parent_element)`, 戻り値: `object (visualizer_instance)`。
-   **renderEmpty ()**: ログデータが空の場合に、視覚化エリアにメッセージなどを表示します。引数: なし, 戻り値: `void`。
-   **renderFromJson ()**: JSON形式のYM2151ログデータを受け取り、それを解析してタイムライン視覚化を描画します。引数: `string (json_log_data)`, 戻り値: `void`。
-   **ensureGlobalLane ()**: 全体的なイベントやコントローラー情報を表示するためのグローバルな視覚化レーンが存在することを確認し、必要であれば作成します。引数: なし, 戻り値: `void`。
-   **setLfoRegisters (demo-library/log-visualizer.ts)**: LFOに関連するYM2151レジスタの値を設定し、それに応じて視覚化を更新します。引数: `object (lfo_register_values)`, 戻り値: `void`。
-   **setupMmlToSmf ()**: MMLからSMFへの変換ライブラリをセットアップし、変換機能を提供するための準備を行います。引数: なし, 戻り値: `Promise<void>`。
-   **ensureWasmInitialized ()**: WebAssemblyが初期化されていることを確認し、未初期化の場合は初期化処理をトリガーします。引数: なし, 戻り値: `Promise<void>`。
-   **setStatus ()**: ユーザーインターフェース上のステータス表示領域にメッセージを更新します。引数: `string (message)`, 戻り値: `void`。
-   **setEventCountDisplay ()**: 処理されたイベントの総数をWebページ上に表示します。引数: `number (count)`, 戻り値: `void`。
-   **ensureWebYm2151 ()**: WebYM2151オーディオエンジンがロードされ、利用可能であることを確認します。引数: なし, 戻り値: `Promise<void>`。
-   **updateOutput ()**: 主要な出力表示領域（例: 変換結果のJSON、ログメッセージ）を更新します。引数: `HTMLElement (output_element), string (content)`, 戻り値: `void`。
-   **parseAttachmentField ()**: UI上の添付ファイル入力フィールドからテキストコンテンツを解析し、構造化されたデータとして返します。引数: `HTMLInputElement`, 戻り値: `object | null`。
-   **cleanup ()**: デモ環境をクリーンアップし、リソース（例: オーディオキャッシュ）を解放します。引数: なし, 戻り値: `void`。
-   **clearWebYmAudioCache ()**: WebYM2151オーディオエンジンの内部キャッシュをクリアし、メモリを解放します。引数: なし, 戻り値: `void`。
-   **buildEventsFromCompact (demo-library/tone-json-attachment.ts)**: コンパクトな形式のYM2151レジスタ設定データから、詳細なイベントオブジェクトの配列を構築します。引数: `object (compact_data)`, 戻り値: `Array<object>`。
-   **serializeWithStatus ()**: データと現在のステータス情報をJSON文字列にシリアライズし、特にデバッグや表示のために使用します。引数: `object (data), string (status_message)`, 戻り値: `string`。
-   **normalizeAttachmentText ()**: 添付ファイルとして提供されたテキストデータを正規化（例: 改行コードの統一、不要な空白の除去）します。引数: `string (raw_text)`, 戻り値: `string`。
-   **convertMmlToSmf ()**: MML文字列をStandard MIDI File (SMF) 形式のデータに変換します。引数: `string (mml_text)`, 戻り値: `Promise<Uint8Array>`。
-   **drawEmpty (demo-library/waveform-canvas.ts)**: 波形表示用のHTML Canvas要素に、波形データがない場合の空の状態（例: グリッド線のみ）を描画します。引数: `CanvasRenderingContext2D, number (width), number (height)`, 戻り値: `void`。
-   **drawWaveform ()**: YM2151のシミュレートされた波形データを受け取り、Canvas上にその波形を描画します。引数: `CanvasRenderingContext2D, Array<number> (waveform_data), number (width), number (height)`, 戻り値: `void`。
-   **simulateWaveform (demo-library/waveform-simulator.ts)**: 与えられたYM2151のオペレーター設定とレジスタ値に基づいて、FM合成の波形を数学的にシミュレートします。引数: `object (ym2151_registers), number (duration_seconds), number (sample_rate)`, 戻り値: `Array<number> (waveform_samples)`。
-   **applyOp ()**: 波形シミュレーション中に、YM2151の各オペレーター（キャリアまたはモジュレーター）の計算（位相累積、エンベロープ適用、出力変調など）を適用します。引数: `object (operator_state), number (current_phase), number (modulator_input)`, 戻り値: `number (operator_output)`。
-   **createWaveformViewer ()**: YM2151波形をインタラクティブに表示するためのビューアコンポーネントを初期化し、作成します。ズーム、パン機能を含みます。引数: `HTMLElement (parent_element)`, 戻り値: `object (viewer_instance)`。
-   **getWindowDurS ()**: 波形ビューアの現在表示されている範囲の継続時間（秒単位）を取得します。引数: なし, 戻り値: `number`。
-   **clampViewStart ()**: ユーザーが操作したビューの開始位置が、有効な波形データの範囲内に収まるように制限します。引数: `number (requested_start_time), number (max_time)`, 戻り値: `number`。
-   **updatePositionLabel ()**: 波形ビューアの現在表示されている時間範囲を示すラベルを更新します。引数: `string (start_time_label), string (end_time_label)`, 戻り値: `void`。
-   **render ()**: 波形ビューアの表示内容を更新（再描画）します。ズームやパン操作後に呼び出されます。引数: なし, 戻り値: `void`。
-   **rebuildAndRender ()**: 基となる波形データを再計算（必要であれば）し、ビューア全体を再描画します。レジスタ設定変更時などに使用されます。引数: `object (ym2151_registers)`, 戻り値: `void`。
-   **setZoom ()**: 波形ビューアのズームレベルを設定し、表示を更新します。引数: `number (zoom_factor)`, 戻り値: `void`。
-   **endDrag ()**: 波形ビューア上でのドラッグ操作が終了した際の処理（例: 位置の確定、レンダリングの最適化）を行います。引数: なし, 戻り値: `void`。
-   **parseHexByte (demo-library/ym2151-utils.ts)**: 2桁の16進数文字列をパースし、対応するバイト値（0-255）を返します。引数: `string (hex_string)`, 戻り値: `number`。

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
      - setupPlayButton ()
      - bootstrap ()
      - createWaveformViewer ()
      - setLfoRegisters (demo-library/log-visualizer.ts)
      - extractLfoRegistersFromAttachment ()
      - syncLfoRegisters ()
  - kcToFrequency (demo-library/envelope-generator.ts)
    - ampStepPerSample ()
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
  - detectChannel (demo-library/log-visualizer.ts)
    - normalizeEvents ()
      - laneColor ()
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
  - drawEmpty (demo-library/waveform-canvas.ts)
    - drawWaveform ()
  - simulateWaveform (demo-library/waveform-simulator.ts)
    - applyOp ()
  - getWindowDurS ()
    - clampViewStart ()
      - updatePositionLabel ()
      - render ()
      - rebuildAndRender ()
      - setZoom ()
- switch (demo-library/envelope-generator.ts)
- for (demo-library/log-visualizer-lfo.ts)
- while (demo-library/waveform-simulator.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-10 07:12:06 JST

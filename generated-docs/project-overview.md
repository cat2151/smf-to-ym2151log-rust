Last updated: 2026-03-13

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRustライブラリです。
- ネイティブアプリケーションとして、またWebAssembly (WASM) を介してブラウザでの利用をサポートしています。
- 和音数ベースの静的チャンネル割り当て、ドラムチャンネル優先、およびプログラムチェンジによるカスタム音色適用機能を備えています。

## 技術スタック
- フロントエンド: HTML, CSS, TypeScript (WASMライブラリのデモUI開発に利用), Vite (デモアプリケーションのビルドツール), WebAssembly (Rustコードをブラウザで実行可能にする技術)
- 音楽・オーディオ: Standard MIDI Files (SMF) (入力フォーマット), YM2151 FM音源チップ (ターゲットとなる音源), JSON形式 (YM2151レジスタ書き込みログの出力フォーマット)
- 開発ツール: Rust (主要な開発言語), Cargo (Rustのビルドシステムおよびパッケージマネージャー), wasm-pack (RustからWebAssemblyパッケージを生成するツール), git (バージョン管理)
- テスト: cargo test (ユニットテスト、統合テストの実行), cargo tarpaulin (テストカバレッジレポート生成)
- ビルドツール: Cargo (Rustプロジェクトのビルド), wasm-pack (WASMビルド), Vite (TypeScriptデモプロジェクトのビルド)
- 言語機能: Rustの型安全性 (堅牢なコード記述を支援), 高パフォーマンス (ネイティブコンパイルによる高速な処理)
- 自動化・CI/CD: (特記事項なし - プロジェクト情報からは直接確認できません)
- 開発標準: cargo fmt (コードフォーマットの自動適用), cargo clippy (コード品質・潜在的バグの検出), cargo audit (依存関係のセキュリティ脆弱性チェック)

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

*   **README.ja.md**: プロジェクトの日本語版概要ドキュメント。
*   **README.md**: プロジェクトの英語版概要ドキュメント。
*   **WASM_USAGE.md**: WebAssembly (WASM) としてこのライブラリを使用する方法を説明するドキュメント。
*   **Cargo.toml**: Rustプロジェクトの設定ファイル。依存関係、パッケージ情報、ビルド設定などを定義。
*   **Cargo.lock**: ビルドに使用された全ての依存ライブラリの正確なバージョンを記録。
*   **LICENSE**: プロジェクトのライセンス情報。
*   **_config.yml**: GitHub Pagesなどのサイト設定ファイル。
*   **src/error.rs**: プロジェクト全体のエラー処理に関する定義。
*   **src/lib.rs**: Rustライブラリのエントリポイント。外部から呼び出される主要な機能を含む。
*   **src/main.rs**: コマンドラインアプリケーションのエントリポイント。MIDIファイルの変換を実行。
*   **src/midi/**: MIDIファイル解析に関連するモジュール群。
    *   **src/midi/events.rs**: MIDIイベントの定義。
    *   **src/midi/mod.rs**: MIDIモジュールのルート。
    *   **src/midi/parser.rs**: Standard MIDI Files (SMF) をパースするロジック。
    *   **src/midi/utils.rs**: MIDI関連のユーティリティ関数。
    *   **src/midi/utils_tests.rs**: `utils.rs` の単体テスト。
*   **src/wasm.rs**: RustコードをWebAssemblyにバインドするためのロジック。
*   **src/ym2151/**: YM2151レジスタログ変換に関連するモジュール群。
    *   **src/ym2151/channel_allocation.rs**: YM2151のチャンネル割り当て戦略に関するロジック。
    *   **src/ym2151/converter.rs**: MIDIイベントをYM2151レジスタログに変換する主要なロジック。
    *   **src/ym2151/converter/**: `converter.rs` で使用される変換関連のサブモジュール。
        *   **src/ym2151/converter/pitch_effects.rs**: ピッチに関するエフェクト処理。
        *   **src/ym2151/converter/register_effects.rs**: レジスタ値に関するエフェクト処理。
        *   **src/ym2151/converter/waveform.rs**: 波形生成に関する処理。
    *   **src/ym2151/converter_tests/**: `converter.rs` および関連モジュールの統合テスト。様々な変換シナリオを検証。
    *   **src/ym2151/event_processor.rs**: イベントを処理し、YM2151ログイベントに変換するロジック。
    *   **src/ym2151/event_processor_tests.rs**: `event_processor.rs` の単体テスト。
    *   **src/ym2151/events.rs**: YM2151レジスタログのイベント定義。
    *   **src/ym2151/init.rs**: YM2151の初期化設定に関するロジック。
    *   **src/ym2151/mod.rs**: YM2151モジュールのルート。
    *   **src/ym2151/note_table.rs**: MIDIノートとYM2151の周波数関係を管理するテーブル。
    *   **src/ym2151/tempo_map.rs**: テンポ変化を管理するマップ。
    *   **src/ym2151/tone.rs**: YM2151の音色に関する定義。
*   **tests/**: プロジェクト全体の統合テストおよびテスト用データ。
    *   **tests/create_test_midi.py**: テスト用MIDIファイルを生成するPythonスクリプト。
    *   **tests/integration_conversion.rs**: 変換プロセスの統合テスト。
    *   **tests/integration_midi.rs**: MIDIパースに関する統合テスト。
    *   **tests/integration_multichannel.rs**: マルチチャンネルMIDIの統合テスト。
    *   **tests/integration_program_change.rs**: プログラムチェンジ機能の統合テスト。
    *   **tests/integration_wasm.rs**: WebAssemblyバインディングの統合テスト。
    *   **tests/test_data/**: 統合テストで使用されるMIDIファイル群。
*   **tones/**: MIDIプログラムチェンジに対応するYM2151カスタム音色定義（JSON形式）を格納。
    *   **tones/000.json**: プログラム000番のカスタム音色データ。
    *   **tones/README.md**: 音色ファイル形式に関する説明ドキュメント。
*   **demo-library/**: WebAssembly版ライブラリの機能を示すためのデモアプリケーションのソースコード。
    *   **demo-library/biome.json**: Biome (コードフォーマッター/リンター) の設定ファイル。
    *   **demo-library/*.ts**: デモページのスクリプトファイル。WASMライブラリの呼び出し、UI更新、視覚化、オーディオ再生、MMLサポートなどの機能を提供する。
    *   **demo-library/*.html**: 各デモのHTMLページ。
    *   **demo-library/globals.d.ts**: グローバルな型定義ファイル。
    *   **demo-library/package.json**: デモアプリケーションのNode.js/npmパッケージ設定ファイル。
    *   **demo-library/package-lock.json**: デモアプリケーションの依存関係のロックファイル。
    *   **demo-library/style.css**: デモアプリケーションのスタイルシート。
    *   **demo-library/tsconfig.json**: TypeScriptコンパイラの設定ファイル。
    *   **demo-library/vite.config.ts**: Viteビルドツールの設定ファイル。
    *   **demo-library/wav-exporter.ts**: WAV形式でオーディオデータをエクスポートする機能。
    *   **demo-library/waveform-canvas.ts**: 波形を描画するためのCanvasコンポーネント。
    *   **demo-library/waveform-viewer.ts**: 波形を視覚的に表示・操作するビューア。
    *   **demo-library/ym2151-utils.ts**: YM2151関連のユーティリティ関数（デモ用）。
*   **generated-docs/**: `cargo doc` などで生成されるAPIドキュメントの出力先（通常は公開されない）。
*   **googled947dc864c270e07.html**: Googleサイト検証用ファイル。
*   **issue-notes/**: 開発中の課題や調査結果に関するメモ（来訪者向けには通常非公開）。

## 関数詳細説明

*   **computeHash**: 与えられたデータからハッシュ値を計算する関数。
*   **nextRequestId**: 新しいリクエストIDを生成し、一意性を保つための関数。
*   **isLatestRequest**: 特定のリクエストIDが現在の最新リクエストであるかを判定する関数。
*   **updateOutputWithState**: UI上の出力表示を、現在のアプリケーション状態に基づいて更新する関数。
*   **updatePlayButtonState**: 再生ボタンの表示状態（有効/無効など）を更新する関数。
*   **initializeWasm**: WebAssemblyモジュールを初期化し、Rustで書かれたコア機能を利用可能にする関数。
*   **readAttachmentBytes**: 添付ファイルや設定データの内容をバイト列として読み込む関数。
*   **runConversion**: MIDIからYM2151ログへの変換処理全体を実行する関数。
*   **handlePlay**: オーディオ再生開始のリクエストを処理し、必要な変換や準備を行う関数。
*   **setupAttachmentEditor**: カスタム設定（音色など）を編集するためのUI要素を設定する関数。
*   **setupMmlInput**: MML（Music Macro Language）形式の入力を受け付けるUI要素を設定する関数。
*   **setupMidiInput**: MIDIファイル入力を受け付けるUI要素を設定する関数。
*   **bootstrapWebYm**: Webブラウザ環境でYM2151エミュレーションをブートストラップする関数。
*   **main**: アプリケーションの主要なエントリーポイント関数。
*   **playAudioWithOverlay**: 特定のUIオーバーレイと共に音声を再生する関数。
*   **clearAudioCache**: 以前生成されたオーディオデータをキャッシュからクリアする関数。
*   **generateAudioFromJson**: JSON形式のYM2151ログデータからオーディオデータを生成する関数。
*   **initWasm**: WebAssemblyモジュールを初期化し、結果をUIに表示する関数。
*   **displayResult**: 変換や処理の結果をユーザーインターフェースに表示する関数。
*   **showError**: 発生したエラーメッセージをユーザーに表示する関数。
*   **setupFileInput**: ファイル選択用のUIコンポーネントを設定する関数。
*   **resolveRegisterForChannel**: 特定のYM2151チャンネルに対応するレジスタアドレスを解決する関数。
*   **collectLfoEvents**: LFO (Low Frequency Oscillator) に関連するイベントを収集する関数。
*   **renderLfoLane**: LFOの活動を視覚的に表示するレーンを描画する関数。
*   **buildNoteSegments**: MIDIノートイベントから視覚化用のノートセグメントデータを構築する関数。
*   **notePitch**: 特定のノートイベントのピッチ情報を計算する関数。
*   **computePitchRange**: 視覚化するピッチの範囲を計算する関数。
*   **noteYPosition**: ノートの表示位置（Y座標）を計算する関数。
*   **renderPitchCanvas**: ピッチ情報を表示するためのCanvas要素に描画を行う関数。
*   **detectChannel**: オーディオイベントのチャンネルを検出する関数。
*   **normalizeEvents**: イベントデータを標準化された形式に整形する関数。
*   **laneColor**: 視覚化レーンの色を決定する関数。
*   **createLane**: 視覚化用の新しいレーン要素を作成する関数。
*   **computeTrackWidth**: 視覚化トラックの幅を計算する関数。
*   **formatInactiveChannels**: 使用されていないチャンネルの表示を整形する関数。
*   **createLogVisualizer**: YM2151ログを視覚化するためのコンポーネントを初期化する関数。
*   **renderEmpty**: 空の（データがない）状態の視覚化表示をレンダリングする関数。
*   **renderFromJson**: JSONデータからログビジュアライザーをレンダリングする関数。
*   **ensureGlobalLane**: 視覚化に必要なグローバルなレーンが確実に存在するようにする関数。
*   **setLfoRegisters**: LFO関連のレジスタ値を設定する関数。
*   **setupMmlToSmf**: MMLからSMFへの変換機能のセットアップを行う関数。
*   **ensureWasmInitialized**: WebAssemblyモジュールが確実に初期化されているかをチェックし、必要に応じて初期化する関数。
*   **setStatus**: アプリケーションの現在のステータスをUIに表示する関数。
*   **setEventCountDisplay**: 処理されたイベント数をUIに表示する関数。
*   **ensureWebYm2151**: Webブラウザ向けのYM2151エミュレータが利用可能であることを確認する関数。
*   **clearWebYmAudioCache**: Webブラウザ版YM2151のオーディオキャッシュをクリアする関数。
*   **updateOutput**: UI上の出力エリアの内容を更新する関数。
*   **parseAttachmentField**: 添付ファイルとして入力されたフィールドの内容を解析する関数。
*   **cleanup**: 不要なリソースを解放するクリーンアップ処理を行う関数。
*   **mod**: 剰余（モジュロ）演算を実行する関数。
*   **setupPlayButton**: オーディオ再生用のボタンを設定する関数。
*   **setupWavExportButton**: 生成されたオーディオをWAV形式でエクスポートするためのボタンを設定する関数。
*   **bootstrap**: アプリケーションの初期設定と起動処理を行う関数。
*   **extractLfoRegistersFromAttachment**: 添付データからLFOレジスタの設定値を抽出する関数。
*   **syncLfoRegisters**: LFOレジスタの設定値を同期させる関数。
*   **buildEventsFromCompact**: コンパクトなデータ形式からYM2151イベントを構築する関数。
*   **serializeWithStatus**: 処理ステータスを含めてデータをシリアライズ（直列化）する関数。
*   **normalizeAttachmentText**: 添付ファイルとして扱われるテキストデータを正規化する関数。
*   **getMmlParser**: MMLを解析するためのパーサーインスタンスを取得する関数。
*   **getParseTreeJsonToSmf**: MMLのパースツリーJSONからStandard MIDI Fileデータを生成する関数。
*   **treeToJson**: 抽象構文木のようなツリー構造をJSON形式に変換する関数。
*   **ensureMmlRuntime**: MML処理に必要なランタイム環境が整っていることを確認する関数。
*   **convertMmlToSmf**: MML形式の入力をStandard MIDI File形式に変換する関数。
*   **drawEmpty**: Canvas上に何も描画されていない状態を描写する関数。
*   **drawWaveform**: Canvas上に波形データを描画する関数。
*   **encodeWav**: 生のオーディオデータをWAVファイル形式にエンコードする関数。
*   **writeAscii**: ASCII文字列をバイト配列として書き込む関数。
*   **downloadWav**: 生成されたWAVファイルをユーザーのデバイスにダウンロードさせる関数。
*   **extractNoteBoundaries**: オーディオログからノートイベントの開始・終了時間などの境界情報を抽出する関数。
*   **createWaveformViewer**: 波形表示ビューアのインスタンスを作成する関数。
*   **getWindowDurS**: 現在表示中のウィンドウの持続時間（秒単位）を取得する関数。
*   **clampViewStart**: 波形ビューの開始位置が有効な範囲に収まるように調整する関数。
*   **updatePositionLabel**: 波形ビューの現在位置を示すラベルを更新する関数。
*   **render**: 波形ビューアの描画処理全体を実行する関数。
*   **updateBoundariesAndRender**: 境界情報を更新し、それに基づいて波形ビューを再描画する関数。
*   **synthesizeAndRender**: YM2151ログから音声を合成し、波形ビューにレンダリングする関数。
*   **setZoom**: 波形ビューのズームレベルを設定する関数。
*   **endDrag**: ユーザーが波形ビューをドラッグする操作を終了した際に呼び出される関数。
*   **parseHexByte**: 16進数文字列をバイト値にパース（解析）する関数。

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
Generated at: 2026-03-13 07:09:43 JST

Last updated: 2026-03-17

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- ネイティブアプリケーション向けライブラリとして、またWebAssemblyとしてブラウザでも動作し、YM2151音源のシミュレーションや再生に利用されます。
- MIDIの和音数に基づくYM2151チャンネルの静的割り当てや、外部JSONファイルによるカスタム音色（プログラムチェンジ）に対応しています。

## 技術スタック
- フロントエンド: TypeScript (デモUIロジック), JavaScript (WebAssembly連携), HTML/CSS (デモページ), Vite (Web開発サーバーおよびバンドラー)
- 音楽・オーディオ: Standard MIDI Files (SMF), YM2151 FM音源 (対象ハードウェア), JSON (YM2151ログおよびカスタム音色定義)
- 開発ツール: Rust (主要開発言語), Cargo (Rustのビルド・パッケージマネージャー), wasm-pack (WebAssemblyパッケージ生成), npm (JavaScriptパッケージマネージャー), Biome (TypeScript/JavaScriptコードのフォーマッター・リンター), Python (テストデータ生成スクリプト)
- テスト: `cargo test` (Rustのユニット/統合テスト), `cargo tarpaulin` (テストカバレッジレポート生成), `cargo audit` (依存関係のセキュリティ監査)
- ビルドツール: Cargo (Rustプロジェクトビルド), wasm-pack (WebAssemblyモジュールビルド), Vite (Webアセットのバンドル)
- 言語機能: Rustの強力な型システム (堅牢なコードベースの実現)
- 自動化・CI/CD: `wasm-pack`を用いたWebAssemblyモジュールの自動生成は、継続的インテグレーション/デプロイメントの文脈で利用されます。
- 開発標準: `cargo fmt` (Rustコードフォーマッター), `cargo clippy` (Rustコードリンター), Biome (TypeScript/JavaScriptコード品質チェック)

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
-   **`README.ja.md`**: プロジェクトの日本語版説明書。
-   **`README.md`**: プロジェクトの英語版説明書。
-   **`WASM_USAGE.md`**: WebAssembly (WASM) としてこのライブラリを使用する方法を説明するドキュメント。
-   **`demo-library/`**: Webブラウザ上でライブラリの機能を示すためのデモアプリケーションのソースコードを格納するディレクトリ。
    -   **`demo-library/biome.json`**: Biomeという開発ツールの設定ファイルで、TypeScript/JavaScriptコードのフォーマットとリンティングルールを定義します。
    -   **`demo-library/delay-vibrato-demo.ts`**: ディレイビブラート機能のデモページのロジックを実装したTypeScriptファイル。
    -   **`demo-library/delay-vibrato.html`**: ディレイビブラートデモのウェブページ構造を定義するHTMLファイル。
    -   **`demo-library/globals.d.ts`**: グローバルに定義される型やインターフェースを宣言するTypeScriptの型定義ファイル。
    -   **`demo-library/index.html`**: デモライブラリのトップページとなるHTMLファイル。
    -   **`demo-library/library-demo.ts`**: ライブラリの基本的な使用方法を示すデモロジックを記述したTypeScriptファイル。
    -   **`demo-library/log-visualizer-lfo.ts`**: YM2151のLFO (Low Frequency Oscillator) イベントを視覚化するためのロジック。
    -   **`demo-library/log-visualizer-note-segments.ts`**: ノートイベントを視覚化用のセグメントに変換するロジック。
    -   **`demo-library/log-visualizer-pitch-canvas.ts`**: ピッチ情報をキャンバスに描画するロジック。
    -   **`demo-library/log-visualizer.ts`**: YM2151のレジスタログをグラフィカルに視覚化するための主要なロジック。
    -   **`demo-library/mml-support.ts`**: MML (Music Macro Language) からSMFへの変換をサポートする機能。
    -   **`demo-library/package-lock.json`**: Node.jsパッケージの依存関係とそのバージョンを固定するファイル。
    -   **`demo-library/package.json`**: Node.jsプロジェクトのメタデータ、スクリプト、依存関係を定義するファイル。
    -   **`demo-library/pop-noise-demo.ts`**: ポップノイズ検出機能のデモロジックを実装したTypeScriptファイル。
    -   **`demo-library/pop-noise-detector.ts`**: 生成されたオーディオデータからポップノイズを検出するロジック。
    -   **`demo-library/pop-noise.html`**: ポップノイズデモのウェブページ構造を定義するHTMLファイル。
    -   **`demo-library/portamento-soft-lfo-demo.ts`**: ポルタメントやソフトLFO機能のデモロジックを実装したTypeScriptファイル。
    -   **`demo-library/portamento-soft-lfo.html`**: ポルタメント/ソフトLFOデモのウェブページ構造を定義するHTMLファイル。
    -   **`demo-library/shared-demo.ts`**: 複数のデモページで共通して使用されるユーティリティ関数やロジック。
    -   **`demo-library/style.css`**: デモページのスタイルを定義するCSSファイル。
    -   **`demo-library/tone-interpolation-demo.ts`**: 音色補間機能のデモロジックを実装したTypeScriptファイル。
    -   **`demo-library/tone-interpolation.html`**: 音色補間デモのウェブページ構造を定義するHTMLファイル。
    -   **`demo-library/tone-json-attachment.ts`**: JSON形式の音色データを扱うためのロジック。
    -   **`demo-library/tone-json-demo.ts`**: JSON音色機能のデモロジックを実装したTypeScriptファイル。
    -   **`demo-library/tone-json-mml.ts`**: JSON形式の音色データとMMLを連携させるためのロジック。
    -   **`demo-library/tone-json.html`**: JSON音色デモのウェブページ構造を定義するHTMLファイル。
    -   **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイル。
    -   **`demo-library/vite.config.ts`**: Vite開発サーバーとバンドラーの設定ファイル。
    -   **`demo-library/wav-exporter.ts`**: 生成されたオーディオデータをWAVファイル形式でエクスポートする機能。
    -   **`demo-library/waveform-canvas.ts`**: 波形をキャンバスに描画するためのユーティリティ。
    -   **`demo-library/waveform-viewer.ts`**: YM2151のオーディオ波形を視覚的に表示・操作するためのビューアロジック。
    -   **`demo-library/ym2151-utils.ts`**: YM2151関連のユーティリティ関数。
-   **`generated-docs/`**: `cargo doc`コマンドなどで生成されるAPIドキュメントの出力先ディレクトリ。
-   **`googled947dc864c270e07.html`**: Googleサイトの所有権確認に使用されるHTMLファイル。
-   **`issue-notes/`**: 開発中の特定の課題や検討事項について記録されたメモ群。
-   **`package-lock.json`**: ルートディレクトリ直下のNode.jsプロジェクトの依存関係を固定するファイル。
-   **`package.json`**: ルートディレクトリ直下のNode.jsプロジェクトのメタデータとスクリプトを定義するファイル。
-   **`src/`**: Rustのソースコードを格納するディレクトリ。
    -   **`src/error.rs`**: プロジェクト全体で使用されるカスタムエラータイプを定義します。
    -   **`src/lib.rs`**: プロジェクトの主要なライブラリクレートのエントリポイント。MIDI解析とYM2151変換のコアロジックを公開します。
    -   **`src/main.rs`**: コマンドラインインターフェース (CLI) のエントリポイント。ファイル入出力と変換プロセスを制御します。
    -   **`src/midi/`**: MIDIファイルの解析に関連するモジュール群。
        -   **`src/midi/events.rs`**: MIDIイベントの内部表現（構造体やenum）を定義します。
        -   **`src/midi/mod.rs`**: `midi`モジュールのルートファイル。サブモジュールを公開します。
        -   **`src/midi/parser.rs`**: Standard MIDI File (SMF) を解析し、イベントストリームに変換するロジックを実装します。
        -   **`src/midi/utils.rs`**: MIDIデータ処理に役立つユーティリティ関数を提供します。
        -   **`src/midi/utils_tests.rs`**: `src/midi/utils.rs`で定義された関数の単体テスト。
    -   **`src/wasm.rs`**: WebAssembly (WASM) のバインディングを提供するためのモジュール。ブラウザからRustのコア機能を呼び出すためのインターフェースを定義します。
    -   **`src/ym2151/`**: YM2151 FM音源のレジスタ書き込みログへの変換に関連するモジュール群。
        -   **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのチャンネルに割り当てる戦略（和音数ベース、ドラム優先など）を実装します。
        -   **`src/ym2151/converter/`**: YM2151レジスタログ変換の詳細な内部処理を扱うサブモジュール群。
            -   **`src/ym2151/converter/event_accumulator.rs`**: 変換プロセス中にYM2151イベントを蓄積し、タイムスタンプや最適化を処理します。
            -   **`src/ym2151/converter/pitch_effects.rs`**: ピッチベンドやビブラートなどのピッチ関連エフェクトをYM2151レジスタ値に変換するロジック。
            -   **`src/ym2151/converter/register_effects.rs`**: YM2151レジスタに直接影響を与える様々なエフェクトを処理します。
            -   **`src/ym2151/converter/register_fields.rs`**: YM2151レジスタの個々のビットフィールドを定義し、操作するための構造体やヘルパー。
            -   **`src/ym2151/converter/waveform.rs`**: YM2151の波形設定に関連するロジック。
        -   **`src/ym2151/converter.rs`**: MIDIイベントストリームをYM2151レジスタ書き込みログのJSON形式に変換する主要なロジックを実装します。
        -   **`src/ym2151/converter_tests/`**: YM2151コンバータの様々な側面（プログラムチェンジ、チャンネル、エフェクトなど）をテストするためのモジュール群。
        -   **`src/ym2151/converter_tests.rs`**: `ym2151/converter.rs`に関連するユニットテスト。
        -   **`src/ym2151/event_processor.rs`**: YM2151イベントを処理し、最終的なログを生成するロジック。
        -   **`src/ym2151/event_processor_tests.rs`**: `event_processor.rs`の単体テスト。
        -   **`src/ym2151/events.rs`**: YM2151レジスタ書き込みイベントの内部表現を定義します。
        -   **`src/ym2151/init.rs`**: YM2151チップの初期状態を設定するためのレジスタ値。
        -   **`src/ym2151/mod.rs`**: `ym2151`モジュールのルートファイル。サブモジュールを公開します。
        -   **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151のFM音源レジスタ値（周波数など）のマッピングテーブル。
        -   **`src/ym2151/tempo_map.rs`**: MIDIファイル内のテンポ変更イベントを解析し、正確なタイミング情報を提供します。
        -   **`src/ym2151/tone.rs`**: YM2151の音色（プログラム）を管理し、外部JSONファイルからの読み込みやデフォルト音色の処理を行います。
-   **`tests/`**: プロジェクトの統合テストやテストデータに関連するファイル群。
    -   **`tests/create_test_midi.py`**: テストに使用する特定のMIDIファイルをプログラムで生成するためのPythonスクリプト。
    -   **`tests/integration_conversion.rs`**: 変換プロセスの全体的な統合テスト。
    -   **`tests/integration_midi.rs`**: MIDIファイルの解析に関する統合テスト。
    -   **`tests/integration_multichannel.rs`**: 複数のMIDIチャンネルが正しく変換されるかの統合テスト。
    -   **`tests/integration_program_change.rs`**: プログラムチェンジイベントが正しく処理されるかの統合テスト。
    -   **`tests/integration_wasm.rs`**: WebAssembly版の機能に関する統合テスト。
    -   **`tests/test_data/`**: 統合テストで使用される実際のMIDIファイル。
-   **`tones/`**: MIDIプログラムチェンジイベントに対応するYM2151のカスタム音色定義（JSON形式）を格納するディレクトリ。
    -   **`tones/000.json`**: MIDIプログラム番号000に対応するYM2151の音色定義。
    -   **`tones/README.md`**: カスタム音色JSONファイルのフォーマットに関する説明。
-   **`Cargo.lock`**: `Cargo.toml`で定義された依存関係の正確なバージョンを記録し、ビルドの再現性を保証するファイル。
-   **`Cargo.toml`**: Rustプロジェクトのメタデータ、依存関係、ビルド設定を定義するマニフェストファイル。
-   **`LICENSE`**: プロジェクトのライセンス情報。
-   **`.gitignore`**: Gitのバージョン管理から除外するファイルやディレクトリを指定するファイル。
-   **`_config.yml`**: GitHub PagesのJekyll設定ファイル。

## 関数詳細説明
-   **`computeHash` (demo-library/delay-vibrato-demo.ts)**: 入力データのハッシュ値を計算し、処理のキャッシュや変更検出に使用されます。
-   **`nextRequestId` (demo-library/delay-vibrato-demo.ts)**: 非同期処理の連続するリクエストに対して、一意のIDを生成し、古いリクエストの結果が誤って適用されないようにします。
-   **`isLatestRequest` (demo-library/delay-vibrato-demo.ts)**: 特定のリクエストIDが現在アクティブな最新のリクエストであるかを判断します。
-   **`updateOutputWithState` (demo-library/delay-vibrato-demo.ts)**: 変換処理の現在の状態や最終結果を、ウェブページの指定された出力エリアに表示します。
-   **`updatePlayButtonState` (demo-library/delay-vibrato-demo.ts)**: オーディオ再生ボタンの有効/無効状態を、変換の進行状況や再生可能な状態に基づいて更新します。
-   **`initializeWasm` (demo-library/delay-vibrato-demo.ts)**: WebAssemblyモジュールをロードし、初期化して、ブラウザ環境で利用できる状態にします。
-   **`readAttachmentBytes` (demo-library/delay-vibrato-demo.ts)**: ユーザーが提供したカスタム音色などの添付ファイルのバイトデータを読み込みます。
-   **`runConversion` (demo-library/delay-vibrato-demo.ts)**: MIDIファイルをYM2151レジスタログに変換するコア処理を実行します。
-   **`handlePlay` (demo-library/delay-vibrato-demo.ts)**: 生成されたYM2151レジスタログを基に、YM2151音源のサウンドをブラウザで再生する処理を開始します。
-   **`setupAttachmentEditor` (demo-library/delay-vibrato-demo.ts)**: 添付ファイルを編集するためのUI要素を初期設定します。
-   **`setupMmlInput` (demo-library/delay-vibrato-demo.ts)**: MML (Music Macro Language) 形式の音楽データを入力するためのUI要素を設定します。
-   **`setupMidiInput` (demo-library/delay-vibrato-demo.ts)**: MIDIファイルをウェブページにアップロードするための入力UIを設定します。
-   **`bootstrapWebYm` (demo-library/delay-vibrato-demo.ts)**: WebYM2151オーディオ再生環境の初期設定と起動を行います。
-   **`main` (demo-library/delay-vibrato-demo.ts)**: デモアプリケーションの主要な初期化ロジックを実行するエントリポイントです。
-   **`playAudioWithOverlay` (demo-library/globals.d.ts)**: オーディオの再生を開始し、必要に応じて視覚的なオーバーレイを表示します。
-   **`clearAudioCache` (demo-library/globals.d.ts)**: キャッシュされたオーディオデータをクリアし、メモリを解放します。
-   **`generateAudioFromJson` (demo-library/globals.d.ts)**: JSON形式のYM2151レジスタログから、実際に再生可能なオーディオデータを生成します。
-   **`initWasm` (demo-library/library-demo.ts)**: WebAssemblyモジュールの初期化を行います。
-   **`displayResult` (demo-library/library-demo.ts)**: 変換結果をウェブページに表示します。
-   **`showError` (demo-library/library-demo.ts)**: エラーメッセージをユーザーに表示します。
-   **`setupFileInput` (demo-library/library-demo.ts)**: ファイル入力（アップロード）のUI要素を設定します。
-   **`resolveRegisterForChannel` (demo-library/log-visualizer-lfo.ts)**: 特定のYM2151チャンネルに対するレジスタの情報を解決・取得します。
-   **`collectLfoEvents` (demo-library/log-visualizer-lfo.ts)**: YM2151ログからLFO (Low Frequency Oscillator) に関連するイベントを収集します。
-   **`renderLfoLane` (demo-library/log-visualizer-lfo.ts)**: 視覚化ツールでLFOの動きを示すレーンを描画します。
-   **`buildNoteSegments` (demo-library/log-visualizer-note-segments.ts)**: YM2151ログのノートイベントから、視覚化に適したセグメントデータを構築します。
-   **`notePitch` (demo-library/log-visualizer-note-segments.ts)**: 特定のノートのピッチ情報を取得します。
-   **`computePitchRange` (demo-library/log-visualizer-note-segments.ts)**: 視覚化するピッチの表示範囲（最小から最大）を計算します。
-   **`noteYPosition` (demo-library/log-visualizer-note-segments.ts)**: 視覚化においてノートが描画されるY軸上の位置を計算します。
-   **`renderPitchCanvas` (demo-library/log-visualizer-pitch-canvas.ts)**: ピッチの変化を表現するためのキャンバスを描画します。
-   **`detectChannel` (demo-library/log-visualizer.ts)**: YM2151のレジスタログからアクティブなチャンネルを検出します。
-   **`normalizeEvents` (demo-library/log-visualizer.ts)**: YM2151イベントデータを視覚化に適した形式に正規化します。
-   **`laneColor` (demo-library/log-visualizer.ts)**: 視覚化レーンに割り当てる色を決定します。
-   **`createLane` (demo-library/log-visualizer.ts)**: 視覚化表示のための新しいレーン（トラック）要素を作成します。
-   **`computeTrackWidth` (demo-library/log-visualizer.ts)**: 視覚化トラックの表示幅を計算します。
-   **`formatInactiveChannels` (demo-library/log-visualizer.ts)**: 使用されていないチャンネルの表示をフォーマットします。
-   **`createLogVisualizer` (demo-library/log-visualizer.ts)**: YM2151レジスタログ視覚化ツールを初期化し、生成します。
-   **`renderEmpty` (demo-library/log-visualizer.ts)**: ログデータがない場合に、空の視覚化表示をレンダリングします。
-   **`renderFromJson` (demo-library/log-visualizer.ts)**: JSON形式のYM2151ログデータに基づいて視覚化表示をレンダリングします。
-   **`ensureGlobalLane` (demo-library/log-visualizer.ts)**: グローバルな視覚化レーンが存在し、準備ができていることを確認します。
-   **`setLfoRegisters` (demo-library/log-visualizer.ts)**: LFOに関連するレジスタ値を設定します。
-   **`setupMmlToSmf` (demo-library/mml-support.ts)**: MMLをSMFに変換する機能のセットアップを行います。
-   **`detectPopNoise` (demo-library/pop-noise-detector.ts)**: 生成されたオーディオ波形データ内のポップノイズの発生を検出します。
-   **`extractLfoRegistersFromAttachment` (demo-library/portamento-soft-lfo-demo.ts)**: 添付されたカスタム音色データからLFOレジスタの設定を抽出します。
-   **`syncLfoRegisters` (demo-library/portamento-soft-lfo-demo.ts)**: LFOレジスタの設定を同期し、UI表示などに反映させます。
-   **`ensureWasmInitialized` (demo-library/shared-demo.ts)**: WebAssemblyモジュールが確実に初期化されていることを確認します。
-   **`setStatus` (demo-library/shared-demo.ts)**: 処理の現在のステータスをウェブページのステータス表示エリアに設定します。
-   **`setEventCountDisplay` (demo-library/shared-demo.ts)**: 処理されたイベントの総数をウェブページに表示します。
-   **`ensureWebYm2151` (demo-library/shared-demo.ts)**: WebYM2151オーディオエンジンが利用可能であることを確認し、必要であれば初期化します。
-   **`clearWebYmAudioCache` (demo-library/shared-demo.ts)**: WebYM2151オーディオエンジンの内部キャッシュをクリアします。
-   **`updateOutput` (demo-library/shared-demo.ts)**: ウェブページの主要な出力エリアの内容を更新します。
-   **`parseAttachmentField` (demo-library/shared-demo.ts)**: 添付ファイル入力フィールドのデータを解析します。
-   **`cleanup` (demo-library/shared-demo.ts)**: アプリケーションのリソースを解放するなど、後処理を実行します。
-   **`mod` (demo-library/shared-demo.ts)**: モジュロ演算を行います。
-   **`getToneEditorGenerator` (demo-library/tone-interpolation-demo.ts)**: 音色エディタのインターフェースを生成するための関数を取得します。
-   **`buildRandomAttachment` (demo-library/tone-interpolation-demo.ts)**: ランダムなYM2151音色設定を生成し、添付ファイルとして利用できるようにします。
-   **`buildEventsFromCompact` (demo-library/tone-json-attachment.ts)**: 簡潔な形式で記述された音色設定からYM2151イベントデータを構築します。
-   **`serializeWithStatus` (demo-library/tone-json-attachment.ts)**: データと現在の処理ステータスを合わせてシリアライズ（直列化）します。
-   **`normalizeAttachmentText` (demo-library/tone-json-attachment.ts)**: 添付ファイルとして扱われるテキストデータを標準的な形式に整形します。
-   **`convertMmlToSmf` (demo-library/tone-json-demo.ts)**: MML形式の音楽データをSMF (Standard MIDI File) 形式に変換します。
-   **`getMmlParser` (demo-library/tone-json-mml.ts)**: MMLテキストを解析するためのパーサー関数を取得します。
-   **`getParseTreeJsonToSmf` (demo-library/tone-json-mml.ts)**: MMLのパースツリー（解析木）をJSON経由でSMFに変換する関数を取得します。
-   **`treeToJson` (demo-library/tone-json-mml.ts)**: 抽象構文木のようなツリー構造をJSON形式に変換します。
-   **`ensureMmlRuntime` (demo-library/tone-json-mml.ts)**: MMLの処理に必要なランタイム環境が準備されていることを確認します。
-   **`encodeWav` (demo-library/wav-exporter.ts)**: 生のオーディオデータをWAVファイル形式にエンコードします。
-   **`writeAscii` (demo-library/wav-exporter.ts)**: ASCII文字列をバイナリデータとして書き込みます。
-   **`downloadWav` (demo-library/wav-exporter.ts)**: 生成されたWAVファイルをユーザーのブラウザにダウンロードさせます。
-   **`drawEmpty` (demo-library/waveform-canvas.ts)**: 空の波形表示キャンバスを描画します。
-   **`drawWaveform` (demo-library/waveform-canvas.ts)**: 与えられた波形データをキャンバス上に描画します。
-   **`parseHexByte` (demo-library/ym2151-utils.ts)**: 16進数文字列を1バイトの数値にパースします。
-   **`extractNoteBoundaries` (demo-library/waveform-viewer.ts)**: オーディオ波形からノートの開始と終了の境界を特定します。
-   **`normalizeAmplitude` (demo-library/waveform-viewer.ts)**: 波形の振幅を正規化し、表示に適したスケールに調整します。
-   **`createWaveformViewer` (demo-library/waveform-viewer.ts)**: 波形表示ビューアコンポーネントを初期化し、生成します。
-   **`getWindowDurS` (demo-library/waveform-viewer.ts)**: 波形ビューアの現在の表示ウィンドウの秒単位の期間を取得します。
-   **`clampViewStart` (demo-library/waveform-viewer.ts)**: 波形ビューアの表示開始位置を有効な範囲内に制限します。
-   **`updatePositionLabel` (demo-library/waveform-viewer.ts)**: 波形ビューア上の現在の再生位置や表示位置を示すラベルを更新します。
-   **`render` (demo-library/waveform-viewer.ts)**: 波形ビューアの全体的な表示を更新し、描画します。
-   **`updateBoundariesAndRender` (demo-library/waveform-viewer.ts)**: 波形の境界情報を更新し、ビューアを再描画します。
-   **`synthesizeAndRender` (demo-library/waveform-viewer.ts)**: YM2151ログからオーディオを合成し、その波形をビューアにレンダリングします。
-   **`setZoom` (demo-library/waveform-viewer.ts)**: 波形ビューアのズームレベルを設定します。
-   **`endDrag` (demo-library/waveform-viewer.ts)**: 波形ビューアでのドラッグ操作が終了した際の処理を行います。

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
    - getToneEditorGenerator (demo-library/tone-interpolation-demo.ts)
    - buildRandomAttachment ()
    - normalizeAttachmentText ()
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
- mod (demo-library/shared-demo.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-17 07:15:57 JST

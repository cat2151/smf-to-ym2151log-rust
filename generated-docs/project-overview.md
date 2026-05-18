Last updated: 2026-05-19

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のライブラリです。
- WebAssembly (WASM) にも対応しており、Webブラウザ上でMIDI再生や音色編集のデモを実行できます。
- 独自のYM2151チャンネル割り当て戦略とプログラムチェンジによるカスタム音色管理により、高精度な変換と音質向上を実現しています。

## 技術スタック
- フロントエンド: TypeScript (型安全なJavaScript開発), HTML, CSS (デモページの構築とスタイリング), Vite (開発サーバーおよびビルドツール), WebAssembly (WASM: Rustコードをブラウザで実行可能にする技術)
- 音楽・オーディオ: Standard MIDI Files (SMF: 音楽データを格納する標準フォーマット), YM2151 (FM音源チップ), JSON (YM2151レジスタ書き込みログの出力形式), WAV (デモでのオーディオエクスポート形式)
- 開発ツール: Git (バージョン管理), Cargo (Rustのビルドシステムおよびパッケージマネージャー), wasm-pack (RustからWASMパッケージを生成するツール), npm (JavaScript/TypeScriptプロジェクトのパッケージマネージャー)
- テスト: `cargo test` (Rustユニット/統合テストフレームワーク), `cargo tarpaulin` (Rustコードのテストカバレッジ計測ツール)
- ビルドツール: Cargo (Rustプロジェクトのビルド), wasm-pack (WASMビルド), Vite (デモ用フロントエンドのビルド)
- 言語機能: Rust (堅牢性、高パフォーマンス、型安全なシステムプログラミング), TypeScript (型安全なJavaScript開発)
- 自動化・CI/CD: 明示的なCI/CDツールは記述されていませんが、`cargo` コマンド群によりビルド、テスト、品質チェックの自動化が可能です。
- 開発標準: `cargo fmt` (Rustコードフォーマッター), `cargo clippy` (Rustリンター), `cargo audit` (Rustセキュリティ監査), Biome (TypeScript/JavaScriptのフォーマッター兼リンター)

## ファイル階層ツリー
```
📄 .gitattributes
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
- **`.gitattributes`**: Gitがファイルを扱う際の属性（例: 改行コードの自動変換）を定義する設定ファイル。
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定する設定ファイル。
- **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録し、ビルドの再現性を保証するファイル。
- **`Cargo.toml`**: Rustプロジェクトのメタデータ（プロジェクト名、バージョン、依存クレートなど）を定義するマニフェストファイル。
- **`LICENSE`**: プロジェクトのライセンス情報（著作権、利用条件など）が記載されたファイル。
- **`README.ja.md`**, **`README.md`**: プロジェクトの目的、使い方、特徴などを説明するドキュメント（日本語と英語）。
- **`WASM_USAGE.md`**: WebAssembly (WASM) を使用してプロジェクトをブラウザで利用する方法に関する詳細な説明。
- **`_config.yml`**: GitHub Pagesのサイト生成に関する設定ファイル。
- **`demo-library/`**: WebAssembly版ライブラリのデモアプリケーションおよび関連アセットを格納するディレクトリ。
    - **`.gitignore`**: `demo-library` ディレクトリ内でGitが無視するファイルを指定。
    - **`biome.json`**: JavaScript/TypeScriptコードのフォーマットやリンティングに関する設定ファイル。
    - **`delay-vibrato-demo.ts`**: ディレイビブラート機能のデモ用TypeScriptコード。
    - **`delay-vibrato.html`**: ディレイビブラート機能のデモを表示するためのHTMLページ。
    - **`globals.d.ts`**: TypeScriptプロジェクトでグローバルに利用される型定義。
    - **`index.html`**: WebAssemblyデモのメインエントリポイントとなるHTMLページ。
    - **`library-demo.ts`**: WebAssemblyライブラリをブラウザで利用する基本的なデモを示すTypeScriptコード。
    - **`log-visualizer-lfo.ts`**: YM2151のLFO (低周波発振器) 関連イベントを視覚化するためのTypeScriptコード。
    - **`log-visualizer-note-segments.ts`**: YM2151ログからノートイベントのセグメントを抽出し、視覚化するために利用されるTypeScriptコード。
    - **`log-visualizer-pitch-canvas.ts`**: 音程（ピッチ）の変化をキャンバスに描画する視覚化コンポーネントのTypeScriptコード。
    - **`log-visualizer.ts`**: YM2151レジスタ書き込みログ全体をグラフィカルに視覚化するメインのTypeScriptコード。
    - **`mml-support.ts`**: MML (Music Macro Language) からSMFへの変換をサポートするためのTypeScriptコード。
    - **`package-lock.json`**: `demo-library` 内のNode.jsプロジェクトの依存関係の正確なバージョンを記録。
    - **`package.json`**: `demo-library` 内のNode.jsプロジェクトのメタデータやスクリプトを定義。
    - **`pop-noise-demo.ts`**: ポップノイズ検出機能のデモ用TypeScriptコード。
    - **`pop-noise-detector.ts`**: YM2151のレジスタ書き込みログからポップノイズの発生パターンを検出するTypeScriptコード。
    - **`pop-noise.html`**: ポップノイズ検出機能のデモを表示するためのHTMLページ。
    - **`portamento-soft-lfo-demo.ts`**: ポルタメントやソフトLFO（音色変更時の滑らかな変化）機能のデモ用TypeScriptコード。
    - **`portamento-soft-lfo.html`**: ポルタメント/ソフトLFO機能のデモを表示するためのHTMLページ。
    - **`random-tone.ts`**: ランダムなYM2151音色パラメータを生成し、アタッチメントとして適用するTypeScriptコード。
    - **`shared-demo.ts`**: WebAssemblyデモ全体で共通して利用されるユーティリティ関数やヘルパー関数。
    - **`style.css`**: `demo-library` 内のHTMLページのスタイリングを定義するCSSファイル。
    - **`tone-interpolation-demo.ts`**: 音色補間機能のデモ用TypeScriptコード。
    - **`tone-interpolation.html`**: 音色補間機能のデモを表示するためのHTMLページ。
    - **`tone-json-attachment.ts`**: JSON形式の音色アタッチメントのパースやシリアライズを行うTypeScriptコード。
    - **`tone-json-demo.ts`**: JSON形式のカスタム音色アタッチメント機能のデモ用TypeScriptコード。
    - **`tone-json-mml.ts`**: JSON形式の音色定義をMMLに関連付けるためのTypeScriptコード。
    - **`tone-json.html`**: JSON形式のカスタム音色アタッチメント機能のデモを表示するためのHTMLページ。
    - **`tsconfig.json`**: TypeScriptコンパイラのオプションや設定を定義するファイル。
    - **`vite.config.ts`**: Vite (フロントエンドビルドツール) の設定ファイル。
    - **`wav-exporter.ts`**: デモで生成されたオーディオをWAVファイルとしてエクスポートするTypeScriptコード。
    - **`waveform-canvas.ts`**: 波形描画のためのキャンバスコンポーネントのTypeScriptコード。
    - **`waveform-viewer.ts`**: 音声波形を視覚的に表示し、操作するビューアコンポーネントのTypeScriptコード。
    - **`ym2151-utils.ts`**: YM2151関連のユーティリティ関数（例：16進数パース）。
- **`generated-docs/`**: `cargo doc` などによって生成されたAPIドキュメントやその他のドキュメントを格納するディレクトリ。
- **`googled947dc864c270e07.html`**: Googleサイト認証のために使用されるHTMLファイル。
- **`issue-notes/`**: 開発中に発生した問題や検討事項、課題などを記録したMarkdown形式のノート群。
- **`package-lock.json`**, **`package.json`**: プロジェクト全体のNode.js依存関係を管理するファイル（`demo-library` とは別のトップレベルのもの）。
- **`src/`**: Rustの主要なソースコードを格納するディレクトリ。
    - **`api.rs`**: 外部から利用されるパブリックなAPIを定義。
    - **`error.rs`**: カスタムエラー型とエラーハンドリングロジック。
    - **`lib.rs`**: Rustライブラリクレートのエントリポイント。
    - **`main.rs`**: コマンドラインツール（バイナリクレート）のエントリポイント。
    - **`midi/`**: MIDIファイルのパースに関連するモジュール。
        - **`events.rs`**: MIDIイベントのデータ構造の定義。
        - **`mod.rs`**: `midi`モジュールのルートファイル。
        - **`parser.rs`**: Standard MIDI Fileを読み込み、MIDIイベントをパースするロジック。
        - **`utils.rs`**: MIDIイベント処理に役立つユーティリティ関数。
        - **`utils_tests.rs`**: `midi/utils.rs`に含まれるユーティリティ関数のユニットテスト。
    - **`options/`**: 変換オプションや効果に関連するモジュール。
        - **`attachments.rs`**: カスタム音色ファイルなどの「アタッチメント」を処理するロジック。
        - **`effects.rs`**: YM2151のLFO、ポルタメントなどの効果に関する定義。
        - **`mod.rs`**: `options`モジュールのルートファイル。
        - **`tests.rs`**: `options`モジュールのテスト。
    - **`wasm.rs`**: WebAssembly (WASM) のバインディング定義。RustコードをJavaScriptから呼び出せるようにする。
    - **`ym2151/`**: YM2151レジスタ書き込みログへの変換ロジックを格納するモジュール。
        - **`channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのオペレータチャンネルに割り当てるためのロジック。
        - **`converter/`**: MIDIイベントからYM2151レジスタログへの変換の核心部分。
            - **`event_accumulator.rs`**: タイムライン上のイベントを蓄積し、順次処理するためのロジック。
            - **`pitch_effects.rs`**: ピッチベンドやビブラートなどの音程変化をYM2151レジスタに変換するロジック。
            - **`register_effects/`**: 特定のYM2151レジスタ効果を管理するサブモジュール。
                - **`common.rs`**: 共通のレジスタ操作や効果。
                - **`mod.rs`**: `register_effects`モジュールのルートファイル。
                - **`pop_noise.rs`**: YM2151のON/OFF時に発生しがちなポップノイズを軽減するためのレジスタ操作ロジック。
                - **`register_lfo.rs`**: YM2151のLFO (低周波発振器) レジスタを制御するロジック。
                - **`state_cache.rs`**: YM2151の現在のレジスタ状態をキャッシュし、不要な書き込みを防ぐ。
                - **`tone_interpolation.rs`**: 音色変化時にレジスタ値を滑らかに補間するロジック。
            - **`register_fields.rs`**: YM2151の各レジスタフィールドのビット構成と意味を定義。
            - **`waveform.rs`**: YM2151のFM合成で利用される波形に関する定義と処理。
        - **`converter.rs`**: YM2151ログ変換のメインロジックを統合するファイル。
        - **`converter_tests/`**: YM2151変換ロジックの各側面をテストするサブディレクトリ。
            - **`attachments.rs`**: アタッチメント（カスタム音色など）の適用に関するテスト。
            - **`attachments_change_to_next_tone/`**: アタッチメントが次の音色に切り替わる際の動作テスト（`guards.rs`, `interpolation.rs`, `keep_fields.rs` で詳細テスト）。
            - **`attachments_program_effects.rs`**: アタッチメント内のプログラムエフェクトに関するテスト。
            - **`basic.rs`**: 基本的なMIDIからYM2151への変換テスト。
            - **`channels.rs`**: YM2151チャンネル割り当てロジックのテスト。
            - **`drums.rs`**: ドラムチャンネルの優先割り当てに関するテスト。
            - **`effects.rs`**: LFOやピッチベンドなどのエフェクト変換テスト。
            - **`lfo.rs`**: LFO機能の変換テスト。
            - **`portamento.rs`**: ポルタメント効果の変換テスト。
            - **`programs.rs`**: MIDIプログラムチェンジによる音色切り替えのテスト。
        - **`converter_tests.rs`**: YM2151変換ロジックに関する一般的なテスト。
        - **`event_processor.rs`**: YM2151イベントの処理と状態管理を行うロジック。
        - **`event_processor_tests.rs`**: `event_processor.rs`のユニットテスト。
        - **`events.rs`**: YM2151レジスタイベントのデータ構造定義。
        - **`init.rs`**: YM2151チップの初期化に関するレジスタ設定。
        - **`mod.rs`**: `ym2151`モジュールのルートファイル。
        - **`note_table.rs`**: MIDIノート番号とYM2151の周波数設定（FT/KC）のマッピングテーブル。
        - **`tempo_map.rs`**: MIDIのテンポ変化イベントを処理し、時間軸を管理するロジック。
        - **`tone.rs`**: YM2151の音色（プログラム）定義のデータ構造。
- **`tests/`**: 統合テストを格納するディレクトリ。
    - **`create_test_midi.py`**: テスト目的で特定のMIDIファイルを生成するためのPythonスクリプト。
    - **`integration_conversion.rs`**: MIDIファイル変換全体の統合テスト。
    - **`integration_midi.rs`**: MIDIファイルのパース機能に関する統合テスト。
    - **`integration_multichannel.rs`**: 複数MIDIチャンネルの変換に関する統合テスト。
    - **`integration_program_change.rs`**: MIDIプログラムチェンジイベントの処理に関する統合テスト。
    - **`integration_public_api.rs`**: ライブラリの公開APIが正しく機能するかどうかの統合テスト。
    - **`integration_wasm.rs`**: WebAssemblyバインディングの機能に関する統合テスト。
    - **`test_data/`**: 統合テストで利用されるサンプルMIDIファイル群。
- **`tones/`**: カスタムYM2151音色をJSON形式で保存するためのディレクトリ。
    - **`000.json`**: プログラムチェンジ0番に対応するデフォルトのYM2151音色定義。
    - **`README.md`**: `tones`ディレクトリとその中のJSONファイルフォーマットに関する説明。

## 関数詳細説明
- **`computeHash`**: 与えられた入力データからハッシュ値を計算し、データの一意性を確認または変更検出に利用します。
- **`nextRequestId`**: 非同期処理において、新しいユニークなリクエストIDを生成し、複数のリクエストが同時に実行される際の識別子として機能します。
- **`isLatestRequest`**: 現在処理中のリクエストが、そのタイプの最新のリクエストであるかを判断します。これにより、古いリクエストの処理結果が新しいリクエストの結果を上書きするのを防ぎます。
- **`updateOutputWithState`**: アプリケーションの現在の状態に基づいて、ユーザーインターフェース上の出力表示を更新します。
- **`updatePlayButtonState`**: オーディオ再生ボタンの有効/無効状態を、再生準備状況などに基づいて更新します。
- **`initializeWasm`**: WebAssembly (WASM) モジュールを初期化し、Rustで書かれた変換ロジックをJavaScriptから利用可能にします。
- **`readAttachmentBytes`**: カスタム音色データなどのアタッチメントファイルの内容をバイト配列として読み込みます。
- **`runConversion`**: 入力されたMIDIファイルをYM2151レジスタ書き込みログに変換する主要な処理を実行します。
- **`handlePlay`**: 変換されたYM2151ログデータを使用して、オーディオを再生するイベントを処理します。
- **`setupAttachmentEditor`**: カスタム音色アタッチメントを編集するためのUI要素を初期化および設定します。
- **`setupMmlInput`**: MML (Music Macro Language) 形式の音楽データを入力するためのUI要素を初期化および設定します。
- **`setupMidiInput`**: MIDIファイルをアップロードするためのUI要素を初期化および設定します。
- **`bootstrapWebYm`**: Webブラウザ環境でYM2151ライブラリが適切に動作するための初期設定と準備を行います。
- **`applyRandomToneToAttachment`**: 現在のアタッチメントにランダムに生成されたYM2151音色パラメータを適用します。
- **`setupRandomToneButton`**: ランダム音色生成ボタンのUIを初期化および設定し、クリックイベントを処理します。
- **`main`**: JavaScriptデモアプリケーションのエントリポイント、またはRustバイナリのエントリポイントです。アプリケーションの主要なロジックを呼び出します。
- **`if`**: プログラミング言語の条件分岐構造であり、特定の条件が真の場合にコードブロックを実行します。
- **`catch`**: プログラミング言語の例外処理メカニズムであり、`try`ブロック内で発生したエラーを捕捉して処理します。
- **`initWasm`**: `initializeWasm`と同様に、WebAssemblyモジュールを初期化する関数です。
- **`displayResult`**: 変換結果や処理のステータスをユーザーインターフェースに表示します。
- **`showError`**: 発生したエラーメッセージをユーザーに分かりやすく表示します。
- **`setupFileInput`**: ファイル入力フォームのUIをセットアップし、ユーザーがファイルを選択できるようにします。
- **`resolveRegisterForChannel`**: 特定のYM2151チャンネルに対して、対応するレジスタアドレスを解決します。
- **`collectLfoEvents`**: YM2151のLFOに関するイベントデータを収集します。
- **`renderLfoLane`**: LFOの動作を視覚的に表示するレーンを描画します。
- **`keyOnTimeKey`**: ノートオンイベントのタイミングを特定するためのユニークなキーを生成します。
- **`buildNoteSegments`**: MIDIノートイベントから、視覚化に適した時間のセグメント構造を構築します。
- **`notePitch`**: 特定のノートのピッチ（音の高さ）を計算します。
- **`computePitchRange`**: 音楽データ全体のピッチの最小値と最大値を計算します。
- **`noteYPosition`**: ノートが視覚化されるキャンバス上での垂直位置（Y座標）を計算します。
- **`renderPitchCanvas`**: 音程の変化をグラフィカルに表現するキャンバスを描画します。
- **`for`**: プログラミング言語のループ構造であり、コレクションの各要素に対して、または指定された回数だけコードブロックを繰り返し実行します。
- **`while`**: プログラミング言語のループ構造であり、特定の条件が真である限りコードブロックを繰り返し実行します。
- **`detectChannel`**: MIDIイベントからチャンネル情報を検出します。
- **`normalizeEvents`**: MIDIイベントデータを視覚化や処理のために標準化された形式に変換します。
- **`laneColor`**: 視覚化レーンに適用する色を決定します。
- **`createLane`**: ログ視覚化のために新しいレーン（トラック）を作成します。
- **`computeTrackWidth`**: 視覚化トラックの幅を計算します。
- **`formatInactiveChannels`**: 再生中に使用されていないチャンネルの表示を整形します。
- **`createLogVisualizer`**: YM2151ログの視覚化ツール全体を生成します。
- **`renderEmpty`**: データがない場合に、空の視覚化コンポーネントを描画します。
- **`renderFromJson`**: JSON形式のログデータを受け取り、それを視覚化コンポーネントに描画します。
- **`ensureGlobalLane`**: ログ視覚化に必要となるグローバルなレーンが存在することを確認し、必要であれば作成します。
- **`setLfoRegisters`**: YM2151のLFOレジスタを設定する値を適用します。
- **`setupMmlToSmf`**: MMLをStandard MIDI Fileに変換するためのセットアップを行います。
- **`getMmlParser`**: MMLを解析するためのパーサーオブジェクトを取得します。
- **`getParseTreeJsonToSmf`**: 解析ツリーをSMF形式のJSONデータに変換する関数を取得します。
- **`treeToJson`**: 抽象構文木（AST）のようなツリー構造をJSON形式にシリアライズします。
- **`ensureMmlRuntime`**: MML変換に必要なランタイム環境が利用可能であることを確認します。
- **`detectPopNoise`**: YM2151レジスタ書き込みログの中から、ポップノイズが発生しうるパターンを検出します。
- **`getToneEditorGenerator`**: 音色エディタ用の生成ロジックを取得します。
- **`generateRandomToneRegisters`**: YM2151の音色を構成するレジスタにランダムな値を生成します。
- **`generateRandomInterpolationPairRegisters`**: 音色補間用のランダムなレジスタペアを生成します。
- **`parseAttachmentEntries`**: アタッチメントファイル内のエントリを解析し、構造化されたデータに変換します。
- **`validateRandomToneAttachment`**: 生成されたランダム音色アタッチメントが有効であるか検証します。
- **`upsertEntryRegisters`**: 指定されたエントリのレジスタ値を更新または新規挿入します。
- **`upsertAttachmentRegisters`**: アタッチメント全体のレジスタ値を更新または挿入します。
- **`upsertInterpolationAttachmentRegisters`**: 補間アタッチメントのレジスタ値を更新または挿入します。
- **`buildRandomInterpolationAttachment`**: ランダムな音色補間アタッチメントを構築します。
- **`ensureWasmInitialized`**: WebAssemblyが既に初期化されていることを確認し、されていなければ初期化をトリガーします。
- **`setStatus`**: ユーザーインターフェース上のステータス表示を更新します。
- **`setEventCountDisplay`**: 処理されたイベントの数を表示するUIを更新します。
- **`ensureWebYm2151`**: Web環境でYM2151再生ライブラリが利用可能であることを確認し、必要であれば初期化します。
- **`clearWebYmAudioCache`**: WebYM2151オーディオのキャッシュをクリアし、メモリを解放します。
- **`updateOutput`**: 一般的な出力表示を更新する汎用関数です。
- **`parseAttachmentField`**: アタッチメント内の特定のフィールドを解析します。
- **`cleanup`**: リソースの解放や一時ファイルの削除など、後処理を実行します。
- **`mod`**: プログラミング言語の剰余（モジュロ）演算子です。
- **`buildEventsFromCompact`**: コンパクトな形式のデータから、詳細なイベントオブジェクトを構築します。
- **`serializeWithStatus`**: データに処理ステータス情報を含めてシリアライズ（直列化）します。
- **`normalizeAttachmentText`**: アタッチメントとして提供されるテキストデータを標準的な形式に整形します。
- **`convertMmlToSmf`**: MML (Music Macro Language) 形式のデータをStandard MIDI File形式に変換します。
- **`drawEmpty`**: キャンバスに何も描画されていない空の状態を描画します。
- **`drawWaveform`**: 指定されたデータに基づいて、キャンバスに音声波形を描画します。
- **`encodeWav`**: オーディオデータをWAV (Waveform Audio File Format) 形式にエンコードします。
- **`writeAscii`**: ファイルやストリームにASCII文字列を書き込みます。
- **`downloadWav`**: 生成されたWAVファイルをユーザーのブラウザにダウンロードさせます。
- **`extractNoteBoundaries`**: 音声データからノートの開始と終了の境界を抽出します。
- **`normalizeAmplitude`**: 音声データの振幅を正規化し、音量レベルを均一化します。
- **`createWaveformViewer`**: 音声波形を表示するためのビューアコンポーネントを生成します。
- **`getWindowDurS`**: 現在の表示ウィンドウの持続時間（秒単位）を取得します。
- **`clampViewStart`**: ビューの開始位置が有効な範囲内にあるように制限（クランプ）します。
- **`updatePositionLabel`**: 波形ビューアの現在の再生位置を示すラベルを更新します。
- **`render`**: グラフィカルな要素やビューを画面に描画する汎用的な関数です。
- **`updateBoundariesAndRender`**: データの境界を更新し、それに合わせてビューを再描画します。
- **`synthesizeAndRender`**: 音声データを合成し、その結果を波形ビューアに描画します。
- **`setZoom`**: 波形ビューアのズームレベルを設定します。
- **`endDrag`**: ユーザーインターフェース要素のドラッグ操作が終了した際の処理を実行します。
- **`parseHexByte`**: 16進数表記のバイト文字列を解析し、数値に変換します。
- **`playAudioWithOverlay`**: オーバーレイ表示を伴ってオーディオを再生します。
- **`clearAudioCache`**: オーディオデータのキャッシュをクリアし、メモリを解放します。
- **`generateAudioFromJson`**: JSON形式のデータからオーディオを生成します。
- **`updateRegisterReflectionStatus`**: YM2151レジスタの状態反映に関するステータスを更新します。
- **`countRegisterNormalizationTargets`**: レジスタ正規化の対象となる項目数をカウントします。
- **`setupPlayButton`**: 再生ボタンのUIコンポーネントを初期化および設定します。
- **`setupWavExportButton`**: WAVエクスポートボタンのUIコンポーネントを初期化および設定します。
- **`bootstrap`**: アプリケーションの初期起動処理全体を調整します。
- **`validateRandomToneAttachment`**: ランダムに生成された音色アタッチメントの有効性を検証します。
- **`clear`**: 特定のコンポーネントや表示の内容をクリアします。
- **`exportWav`**: 現在のオーディオデータをWAVファイルとしてエクスポートします。
- **`extractLfoRegistersFromAttachment`**: アタッチメントデータからLFO関連のレジスタ情報を抽出します。
- **`syncLfoRegisters`**: LFOレジスタの状態を同期します。

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
  - keyOnTimeKey (demo-library/log-visualizer-note-segments.ts)
    - buildNoteSegments ()
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
Generated at: 2026-05-19 07:27:14 JST

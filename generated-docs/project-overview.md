Last updated: 2026-02-15

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツール/ライブラリです。
- ネイティブアプリケーション向けライブラリとして、またWebAssemblyを介したブラウザ向けライブラリとして利用可能です。
- 高度なチャンネル割り当て戦略とプログラムチェンジ対応により、忠実なYM2151音源エミュレーションを実現します。

## 技術スタック
- フロントエンド:
    - **TypeScript**: WebAssemblyデモアプリケーションのロジック実装に使用されているプログラミング言語です。
    - **Vite**: WebAssemblyデモアプリケーションの高速な開発サーバーとビルドツールとして利用されています。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: 入力として扱われる、デジタル楽器間の演奏情報を記録するための標準ファイル形式です。
    - **YM2151 FM音源チップ**: 出力フォーマットとして対象となる、日本のヤマハが開発したFM音源チップです。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語であり、型安全性とパフォーマンスを重視しています。
    - **Cargo**: Rustプロジェクトのビルドシステムおよびパッケージマネージャーです。
    - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、JavaScriptから利用可能なパッケージを生成するためのツールです。
    - **Python**: テスト用のMIDIファイルを生成するためのスクリプトに使用されています。
- テスト:
    - **cargo test**: Rustの組み込みテストフレームワークで、ユニットテストと統合テストの実行に使用されます。
    - **cargo tarpaulin**: Rustプロジェクトのテストカバレッジを測定し、レポートを生成するためのツールです。
- ビルドツール:
    - **Cargo**: Rustアプリケーションおよびライブラリのビルドと依存関係管理を行います。
    - **wasm-pack**: WebAssemblyターゲット向けにRustコードをビルドします。
    - **Vite**: WebAssemblyデモアプリケーションをビルドします。
- 言語機能:
    - **Rustの型システム**: コンパイル時に多くのエラーを捕捉し、堅牢なコードを保証します。
    - **高パフォーマンス**: ネイティブコードにコンパイルされるため、高速な処理が可能です。
- 自動化・CI/CD:
    - **cargo install**: CLIツールとしてプロジェクトをシステムにインストールし、利用可能にするためのコマンドです。
- 開発標準:
    - **cargo fmt**: Rustコードのフォーマットを自動的にチェック・修正し、コードスタイルを統一します。
    - **cargo clippy**: Rustコードの一般的な誤りや非効率的なコードパターンを検出する静的解析ツールです。
    - **cargo audit**: プロジェクトの依存関係に存在する既知のセキュリティ脆弱性をチェックします。

## ファイル階層ツリー
```
.gitignore
Cargo.lock
Cargo.toml
LICENSE
README.ja.md
README.md
WASM_USAGE.md
_config.yml
demo-library/
  .gitignore
  delay-vibrato-demo.ts
  delay-vibrato.html
  globals.d.ts
  index.html
  library-demo.ts
  log-visualizer.ts
  mml-support.ts
  package-lock.json
  package.json
  pop-noise-demo.ts
  pop-noise.html
  portamento-soft-lfo-demo.ts
  portamento-soft-lfo.html
  shared-demo.ts
  style.css
  tone-json-demo.ts
  tone-json.html
  tsconfig.json
  vite.config.ts
generated-docs/
googled947dc864c270e07.html
package-lock.json
package.json
src/
  error.rs
  lib.rs
  main.rs
  midi/
    events.rs
    mod.rs
    parser.rs
    utils.rs
  wasm.rs
  ym2151/
    channel_allocation.rs
    converter/
      pitch_effects.rs
      register_effects.rs
      waveform.rs
    converter.rs
    converter_tests.rs
    event_processor.rs
    events.rs
    init.rs
    mod.rs
    note_table.rs
    tempo_map.rs
    tone.rs
tests/
  create_test_midi.py
  integration_tests.rs
  test_data/
    multi_channel.mid
    multi_track.mid
    program_change.mid
    simple_melody.mid
    tempo_change.mid
tones/
  000.json
  README.md
```

## ファイル詳細説明
- **README.ja.md / README.md**: プロジェクトの概要、使い方、開発方法などを記述したドキュメント（日本語版/英語版）。
- **WASM_USAGE.md**: WebAssembly (WASM) を使用したブラウザでの利用方法に関する詳細なドキュメント。
- **Cargo.toml**: Rustプロジェクトのメタデータ、依存関係、ビルド設定を定義するファイル。
- **Cargo.lock**: ビルド時に実際に使用された依存関係の正確なバージョンを記録するファイル。
- **LICENSE**: プロジェクトのライセンス情報。
- **src/main.rs**: コマンドラインインターフェース (CLI) アプリケーションのエントリポイントで、SMFからYM2151ログへの変換処理を調整します。
- **src/lib.rs**: このプロジェクトが提供するライブラリクレートのメインファイルで、公開APIを定義し、変換ロジックをカプセル化します。
- **src/error.rs**: プロジェクト固有のエラー型を定義し、エラーハンドリングを一元化します。
- **src/wasm.rs**: WebAssemblyバインディングを提供し、Rustで実装された変換ロジックをブラウザ環境から呼び出せるようにします。
- **src/midi/events.rs**: MIDIイベントのデータ構造（ノートオン/オフ、テンポチェンジなど）を定義します。
- **src/midi/parser.rs**: Standard MIDI File (SMF) をパースし、内部的なMIDIイベントシーケンスに変換するロジックを実装します。
- **src/midi/utils.rs**: MIDI関連のユーティリティ関数やヘルパーを定義します。
- **src/ym2151/channel_allocation.rs**: YM2151チップの限られたチャンネルを、MIDIチャンネルの和音数やドラム優先度に基づいて割り当てる戦略を実装します。
- **src/ym2151/converter.rs**: MIDIイベントをYM2151レジスタ書き込みログに変換する主要なロジックを含みます。
- **src/ym2151/converter_tests.rs**: YM2151変換ロジックに関する単体テストを定義します。
- **src/ym2151/event_processor.rs**: YM2151イベントのタイムライン処理や、レジスタ操作に関するロジックを管理します。
- **src/ym2151/events.rs**: YM2151レジスタ書き込みイベントのデータ構造を定義します。
- **src/ym2151/init.rs**: YM2151チップの初期化シーケンスやデフォルト設定を定義します。
- **src/ym2151/note_table.rs**: MIDIノート番号からYM2151の周波数設定値への変換テーブルを提供します。
- **src/ym2151/tempo_map.rs**: MIDIファイル内のテンポチェンジイベントに基づいて、時間軸マッピングを管理します。
- **src/ym2151/tone.rs**: MIDIプログラムチェンジイベントに対応するYM2151音色（インストゥルメント）のロードと管理を行います。
- **src/ym2151/converter/pitch_effects.rs**: ピッチベンドやビブラートなどのピッチ関連エフェクトのYM2151レジスタ変換ロジックを扱います。
- **src/ym2151/converter/register_effects.rs**: YM2151レジスタに特定の効果（エンベロープ、LFOなど）を適用するための変換ロジックを扱います。
- **src/ym2151/converter/waveform.rs**: YM2151の波形設定に関する変換ロジックを扱います。
- **tests/integration_tests.rs**: プロジェクト全体の機能が正しく連携するかを検証する統合テストを定義します。
- **tests/create_test_midi.py**: 統合テストで使用されるサンプルMIDIファイルをプログラムで生成するためのPythonスクリプト。
- **tests/test_data/**: 統合テストで利用されるMIDIファイルデータが格納されています。
- **tones/000.json ... 127.json**: MIDIプログラムチェンジに対応するYM2151カスタム音色設定をJSON形式で定義したファイル群。
- **tones/README.md**: カスタム音色JSONファイルのフォーマットと使用方法について説明します。
- **demo-library/index.html**: WebAssemblyデモのメインページとなるHTMLファイルです。
- **demo-library/library-demo.ts**: WebAssemblyライブラリの基本的な使用方法を示すTypeScriptデモのスクリプト。
- **demo-library/log-visualizer.ts**: 生成されたYM2151レジスタログを視覚的に表示するためのTypeScriptスクリプト。
- **demo-library/mml-support.ts**: MML (Music Macro Language) をSMFに変換するサポート機能を提供するTypeScriptスクリプト。
- **demo-library/shared-demo.ts**: 複数のWebAssemblyデモで共通して使用されるユーティリティ関数やロジックを含むTypeScriptスクリプト。
- **demo-library/style.css**: WebAssemblyデモのUIデザインを定義するスタイルシート。
- **demo-library/delay-vibrato-demo.ts / delay-vibrato.html**: ディレイビブラート機能のデモに特化したTypeScriptスクリプトとHTMLページ。
- **demo-library/pop-noise-demo.ts / pop-noise.html**: ポップノイズ対策機能のデモに特化したTypeScriptスクリプトとHTMLページ。
- **demo-library/portamento-soft-lfo-demo.ts / portamento-soft-lfo.html**: ポルタメントやソフトLFO機能のデモに特化したTypeScriptスクリプトとHTMLページ。
- **demo-library/tone-json-demo.ts / tone-json.html**: カスタム音色JSONファイルの利用デモに特化したTypeScriptスクリプトとHTMLページ。
- **demo-library/globals.d.ts**: デモで使用されるグローバルな型定義ファイル。
- **demo-library/package.json / package-lock.json**: デモアプリケーションの依存関係とスクリプトを定義するNode.jsのパッケージファイル。
- **demo-library/tsconfig.json**: TypeScriptコンパイラの設定ファイル。
- **demo-library/vite.config.ts**: Viteビルドツールの設定ファイル。

## 関数詳細説明
- **nextRequestId (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts)**: 非同期リクエストの一意なIDを生成し、最新のリクエストを追跡するために使用されます。
- **isLatestRequest (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts)**: 現在のリクエストIDが最新のものであるかを確認します。
- **updateOutputWithState (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: アプリケーションの状態に基づいて、ユーザーインターフェースの出力表示を更新します。
- **updatePlayButtonState (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: 再生ボタンの状態（有効/無効）を更新します。
- **initializeWasm (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: WebAssemblyモジュールを初期化し、Rustの機能をブラウザから利用できるようにします。
- **readAttachmentBytes (demo-library/delay-vibrato-demo.ts, library-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: ファイル入力または添付フィールドからバイトデータを読み取ります。
- **runConversion (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: MIDIファイルをYM2151ログに変換する処理を実行します。
- **handlePlay (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: 再生ボタンがクリックされたときのイベントを処理し、オーディオ再生を開始します。
- **setupAttachmentEditor (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: 添付ファイルエディタのUI要素を設定します。
- **setupMmlInput (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: MML入力フィールドのUI要素を設定します。
- **setupMidiInput (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: MIDIファイル入力のUI要素を設定します。
- **bootstrapWebYm (demo-library/delay-vibrato-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: WebYM2151オーディオプレイヤーを初期化し、デモアプリケーションに統合します。
- **main (demo-library/delay-vibrato-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: デモアプリケーションのエントリポイントで、初期設定やイベントリスナーの登録を行います。
- **if (demo-library/delay-vibrato-demo.ts, library-demo.ts, log-visualizer.ts, mml-support.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, shared-demo.ts, tone-json-demo.ts)**: 条件分岐を定義する一般的な制御フロー構造です。
- **catch (demo-library/delay-vibrato-demo.ts, library-demo.ts, mml-support.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, shared-demo.ts, tone-json-demo.ts)**: Try-catchブロックのエラーハンドリング部分で、例外を捕捉して処理します。
- **addEventListener (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-json-demo.ts)**: DOM要素にイベントリスナーを登録し、ユーザーインタラクションに対応します。
- **initWasm (demo-library/library-demo.ts)**: WebAssemblyモジュールの初期化を担当します。
- **displayResult (demo-library/library-demo.ts)**: 変換結果をユーザーインターフェースに表示します。
- **showError (demo-library/library-demo.ts)**: エラーメッセージをユーザーインターフェースに表示します。
- **setupFileInput (demo-library/library-demo.ts)**: ファイル入力要素のイベントハンドラーを設定します。
- **parseHexByte (demo-library/log-visualizer.ts)**: 16進数文字列をバイト値にパースします。
- **detectChannel (demo-library/log-visualizer.ts)**: ログイベントからYM2151チャンネルを検出します。
- **normalizeEvents (demo-library/log-visualizer.ts)**: ログイベントデータを視覚化に適した形式に正規化します。
- **laneColor (demo-library/log-visualizer.ts)**: 各チャンネルの視覚化レーンの色を決定します。
- **createLane (demo-library/log-visualizer.ts)**: ログ視覚化のためのHTMLレーン要素を作成します。
- **computeTrackWidth (demo-library/log-visualizer.ts)**: 視覚化トラックの幅を計算します。
- **createLogVisualizer (demo-library/log-visualizer.ts)**: YM2151ログの視覚化コンポーネントを初期化し、構築します。
- **renderEmpty (demo-library/log-visualizer.ts)**: ログデータがない場合に、空の視覚化表示をレンダリングします。
- **renderFromJson (demo-library/log-visualizer.ts)**: JSON形式のログデータを受け取り、視覚化をレンダリングします。
- **ensureGlobalLane (demo-library/log-visualizer.ts)**: グローバルな視覚化レーンが存在することを保証します。
- **for (demo-library/log-visualizer.ts, mml-support.ts, tone-json-demo.ts)**: 繰り返し処理を定義する一般的な制御フロー構造です。
- **treeToJson (demo-library/mml-support.ts, tone-json-demo.ts)**: MMLの構文木やその他のツリー構造をJSON形式に変換します。
- **ensureMmlRuntime (demo-library/mml-support.ts, tone-json-demo.ts)**: MMLを処理するためのランタイムが利用可能であることを保証します。
- **setupMmlToSmf (demo-library/mml-support.ts)**: MMLからSMFへの変換機能を設定します。
- **setupPlayButton (demo-library/pop-noise-demo.ts)**: 再生ボタンの初期設定とイベントリスナーをセットアップします。
- **bootstrap (demo-library/pop-noise-demo.ts)**: デモアプリケーションの起動処理を実行します。
- **ensureWasmInitialized (demo-library/shared-demo.ts)**: WebAssemblyが確実に初期化されているかをチェックします。
- **setStatus (demo-library/shared-demo.ts)**: アプリケーションのステータスメッセージを更新します。
- **setEventCountDisplay (demo-library/shared-demo.ts)**: 処理されたイベントの数を表示します。
- **ensureWebYm2151 (demo-library/shared-demo.ts)**: WebYM2151プレイヤーがロードされ、利用可能であることを保証します。
- **clearWebYmAudioCache (demo-library/shared-demo.ts)**: WebYM2151オーディオキャッシュをクリアします。
- **updateOutput (demo-library/shared-demo.ts)**: 汎用的な出力領域を更新します。
- **parseAttachmentField (demo-library/shared-demo.ts)**: 添付フィールドから内容をパースします。
- **cleanup (demo-library/shared-demo.ts)**: リソースの解放や後処理を行います。
- **buildEventsFromCompact (demo-library/tone-json-demo.ts)**: コンパクトなデータ形式からイベントオブジェクトを構築します。
- **normalizeAttachmentText (demo-library/tone-json-demo.ts)**: 添付フィールドのテキストデータを正規化します。
- **convertMmlToSmf (demo-library/tone-json-demo.ts)**: MMLデータをStandard MIDI File (SMF) 形式に変換します。
- **playAudioWithOverlay (demo-library/globals.d.ts)**: オーバーレイ付きでオーディオを再生します。
- **clearAudioCache (demo-library/globals.d.ts)**: オーディオキャッシュをクリアします。

- **Rust側 (src/)の主要な機能関数 (具体的な関数名は情報にないため、役割に基づく推測)**:
    - **`main` (src/main.rs)**: CLIアプリケーションのエントリポイント。ファイルの読み込み、変換ロジックの呼び出し、結果の出力などのトップレベルの処理を調整します。
    - **`convert_smf_to_ym2151_log` (src/lib.rs)**: ライブラリの主要な公開関数。Standard MIDI Files (SMF) データをYM2151レジスタ書き込みログに変換します。
    - **`parse_midi_file` (src/midi/parser.rs)**: MIDIファイルフォーマットを解析し、内部表現のMIDIイベントリストを生成します。
    - **`convert_events_to_ym2151_log` (src/ym2151/converter.rs)**: パースされたMIDIイベントをYM2151チップのレジスタ操作ログに変換するコアロジックを実装します。
    - **`allocate_channels` (src/ym2151/channel_allocation.rs)**: YM2151の利用可能な8チャンネルを、入力MIDIチャンネルの和音数やドラム優先度に基づいて割り当てます。
    - **`load_tone` (src/ym2151/tone.rs)**: プログラムチェンジイベントに対応するカスタムYM2151音色（JSONファイル）をロードし、適用します。
    - **`wasm_convert` (src/wasm.rs)**: WebAssemblyから呼び出されるためのラッパー関数。MIDIバイトデータを入力として受け取り、YM2151ログをJSON文字列として返します。

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
Generated at: 2026-02-15 07:08:34 JST

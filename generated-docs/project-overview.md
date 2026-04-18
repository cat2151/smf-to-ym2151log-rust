Last updated: 2026-04-19

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- 高度なYM2151チャンネル割り当て戦略と、外部ファイルからのカスタム音色読み込み機能により、豊かなFM音源表現を可能にします。
- ネイティブアプリケーションとしてだけでなく、WebAssembly (WASM) を通じてWebブラウザ上でも動作し、多様な環境での利用が可能です。

## 技術スタック
- フロントエンド: WebAssembly (WASM) とTypeScriptを基盤に、ブラウザ向けデモUIを構築しています。HTMLとCSSで見た目を整え、Viteをビルドツールとして採用しています。
- 音楽・オーディオ: Standard MIDI Files (SMF) を入力とし、ヤマハYM2151 FM音源チップの特性に合わせたレジスタ操作ログを生成します。MIDIプログラムチェンジによる音色切り替えに対応しています。
- 開発ツール: 主にRust言語で開発されており、RustのビルドシステムとパッケージマネージャーであるCargoを利用しています。WebAssemblyへのコンパイルにはwasm-packを使用し、Gitでバージョン管理を行っています。
- テスト: Rustの標準テストフレームワークによる包括的なユニットテストと統合テストを実施しており、テストカバレッジ測定にはcargo tarpaulinを使用しています。
- ビルドツール: RustプロジェクトのビルドにはCargoを、WebAssemblyのビルドにはwasm-packを、デモサイトのビルドにはViteを使用しています。
- 言語機能: Rust 1.70.0以上のバージョンを使用し、堅牢でパフォーマンスの高いコードを記述しています。WebAssembly (WASM) により、ウェブブラウザでの実行もサポートされています。
- 自動化・CI/CD: (提供された情報から特定のCI/CDツールは特定できませんが、テスト駆動開発と品質チェックの記述から自動テストは行われていると推測されます。)
- 開発標準: コードの品質を維持するため、cargo fmtによるフォーマットチェック、cargo clippyによる静的解析、cargo auditによる依存関係のセキュリティ監査を定期的に実施しています。

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
- **`.gitattributes`**: Gitがファイルを扱う際の属性（例: 行末変換）を定義するファイル。
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定するファイル。
- **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録するファイル。
- **`Cargo.toml`**: Rustプロジェクトの設定ファイル。プロジェクト名、バージョン、依存関係などが記述されています。
- **`LICENSE`**: プロジェクトのライセンス情報。
- **`README.ja.md`**: プロジェクトの日本語版説明書。
- **`README.md`**: プロジェクトの英語版説明書。
- **`WASM_USAGE.md`**: WebAssembly (WASM) を使用する際の詳細な手順や情報が記載されたドキュメント。
- **`_config.yml`**: GitHub Pagesなどの静的サイトジェネレーターの設定ファイル。
- **`demo-library/`**: WebAssembly版ライブラリのデモアプリケーションが含まれるディレクトリ。
    - **`demo-library/biome.json`**: JavaScript/TypeScriptコードのフォーマットとリンティングを行うBiomeツールの設定ファイル。
    - **`demo-library/delay-vibrato-demo.ts`**: ディレイビブラート機能のデモのためのTypeScriptスクリプト。
    - **`demo-library/delay-vibrato.html`**: ディレイビブラートデモ用のHTMLページ。
    - **`demo-library/globals.d.ts`**: デモで使用されるグローバルな型定義ファイル。
    - **`demo-library/index.html`**: WebAssemblyデモのメインページ。
    - **`demo-library/library-demo.ts`**: WebAssemblyライブラリの基本的な使用方法を示すTypeScriptデモスクリプト。
    - **`demo-library/log-visualizer-lfo.ts`**: YM2151ログ内のLFO（低周波発振器）イベントを視覚化するためのスクリプト。
    - **`demo-library/log-visualizer-note-segments.ts`**: ノートイベントのセグメントを構築し、視覚化に利用するためのスクリプト。
    - **`demo-library/log-visualizer-pitch-canvas.ts`**: ピッチ情報をキャンバスに描画する視覚化スクリプト。
    - **`demo-library/log-visualizer.ts`**: YM2151レジスタ書き込みログを視覚的に表示するためのメインスクリプト。
    - **`demo-library/mml-support.ts`**: MML (Music Macro Language) からStandard MIDI File (SMF) への変換をサポートするスクリプト。
    - **`demo-library/package-lock.json`**: `package.json`の依存関係の正確なツリー構造を記録するファイル。
    - **`demo-library/package.json`**: デモアプリケーションのNode.js/npmパッケージ設定ファイル。
    - **`demo-library/pop-noise-demo.ts`**: ポップノイズ検出と対策機能のデモのためのTypeScriptスクリプト。
    - **`demo-library/pop-noise-detector.ts`**: YM2151のレジスタログからポップノイズの発生を検出するスクリプト。
    - **`demo-library/pop-noise.html`**: ポップノイズデモ用のHTMLページ。
    - **`demo-library/portamento-soft-lfo-demo.ts`**: ポルタメントやソフトLFO機能のデモのためのTypeScriptスクリプト。
    - **`demo-library/portamento-soft-lfo.html`**: ポルタメント/ソフトLFOデモ用のHTMLページ。
    - **`demo-library/random-tone.ts`**: ランダムなYM2151音色設定を生成するためのスクリプト。
    - **`demo-library/shared-demo.ts`**: 複数のデモ間で共通して利用されるユーティリティ関数や設定をまとめたスクリプト。
    - **`demo-library/style.css`**: デモページのスタイルシート。
    - **`demo-library/tone-interpolation-demo.ts`**: 音色補間機能のデモのためのTypeScriptスクリプト。
    - **`demo-library/tone-interpolation.html`**: 音色補間デモ用のHTMLページ。
    - **`demo-library/tone-json-attachment.ts`**: カスタム音色JSONファイルを添付データとして扱うためのユーティリティスクリプト。
    - **`demo-library/tone-json-demo.ts`**: カスタム音色JSONの適用方法を示すTypeScriptデモスクリプト。
    - **`demo-library/tone-json-mml.ts`**: MMLと音色JSONを組み合わせて使用するためのスクリプト。
    - **`demo-library/tone-json.html`**: カスタム音色JSONデモ用のHTMLページ。
    - **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイル。
    - **`demo-library/vite.config.ts`**: Viteビルドツールの設定ファイル。デモのビルド方法を定義しています。
    - **`demo-library/wav-exporter.ts`**: 生成されたオーディオデータをWAVファイル形式でエクスポートするスクリプト。
    - **`demo-library/waveform-canvas.ts`**: オーディオ波形を描画するためのキャンバス操作スクリプト。
    - **`demo-library/waveform-viewer.ts`**: 波形表示ビューアのコンポーネント。ズームや再生位置表示などを担当します。
    - **`demo-library/ym2151-utils.ts`**: YM2151関連のユーティリティ関数（例: 16進数パース）をまとめたスクリプト。
- **`generated-docs/`**: ドキュメント自動生成ツールによって出力されるドキュメントが格納されるディレクトリ。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイル。
- **`issue-notes/`**: 開発中の課題や検討事項を記録したMarkdownファイル群。
- **`package-lock.json`**: プロジェクト全体のNode.js/npm依存関係のロックファイル。
- **`package.json`**: プロジェクト全体のNode.js/npmパッケージ設定ファイル。
- **`src/`**: Rustのソースコードが格納されているディレクトリ。
    - **`src/api.rs`**: パブリックAPIの定義。他のRustプロジェクトからライブラリとして利用される際のインターフェースを提供します。
    - **`src/error.rs`**: エラー型とその処理ロジックを定義します。
    - **`src/lib.rs`**: Rustライブラリクレートのエントリポイント。プロジェクトの主要な機能がここに集約されています。
    - **`src/main.rs`**: コマンドラインツールのエントリポイント。SMFファイルを読み込み、YM2151ログに変換する処理を実装しています。
    - **`src/midi/`**: MIDIファイルのパースとイベント処理に関するモジュール。
        - **`src/midi/events.rs`**: Standard MIDI File (SMF) の各種イベント（ノートオン、オフ、テンポなど）の構造体を定義します。
        - **`src/midi/mod.rs`**: `midi`モジュールのエントリポイント。
        - **`src/midi/parser.rs`**: SMFバイナリデータを解析し、MIDIイベントを抽出するロジックを実装します。
        - **`src/midi/utils.rs`**: MIDI関連のヘルパー関数やユーティリティを提供します。
        - **`src/midi/utils_tests.rs`**: `midi/utils.rs`のテストコード。
    - **`src/options/`**: プログラムの実行オプションや設定を扱うモジュール。
        - **`src/options/attachments.rs`**: 外部から提供されるカスタムデータ（例: 音色ファイル）の処理に関するロジック。
        - **`src/options/effects.rs`**: YM2151出力に適用される様々なエフェクトに関するオプション。
        - **`src/options/mod.rs`**: `options`モジュールのエントリポイント。
        - **`src/options/tests.rs`**: `options`モジュールのテストコード。
    - **`src/wasm.rs`**: WebAssembly (WASM) とRustコード間のインターフェースを定義し、ブラウザからの呼び出しを可能にします。
    - **`src/ym2151/`**: YM2151 FM音源チップに特化した変換ロジックとデータ構造。
        - **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのオペレータチャンネルに割り当てるための複雑なロジック（和音数ベース、ドラム優先など）。
        - **`src/ym2151/converter/`**: SMFイベントをYM2151レジスタ書き込みログに変換する詳細なサブモジュール。
            - **`src/ym2151/converter/event_accumulator.rs`**: MIDIイベントを一時的に蓄積し、YM2151イベントに変換するための準備を行います。
            - **`src/ym2151/converter/pitch_effects.rs`**: ピッチベンドやビブラートなどのピッチ関連エフェクトをYM2151レジスタ値に変換するロジック。
            - **`src/ym2151/converter/register_effects/`**: YM2151のレジスタ設定に影響を与える特殊なエフェクト処理。
                - **`src/ym2151/converter/register_effects/common.rs`**: レジスタエフェクト共通のヘルパー関数。
                - **`src/ym2151/converter/register_effects/mod.rs`**: `register_effects`モジュールのエントリポイント。
                - **`src/ym2151/converter/register_effects/pop_noise.rs`**: 音色切り替え時のポップノイズを軽減するためのレジスタ操作ロジック。
                - **`src/ym2151/converter/register_effects/register_lfo.rs`**: YM2151のLFO（低周波発振器）レジスタを制御するロジック。
                - **`src/ym2151/converter/register_effects/state_cache.rs`**: YM2151のレジスタ状態をキャッシュし、不要な書き込みを防ぐための最適化。
                - **`src/ym2151/converter/register_effects/tone_interpolation.rs`**: 音色間を滑らかに補間するためのレジスタ操作ロジック。
            - **`src/ym2151/converter/register_fields.rs`**: YM2151の各レジスタフィールドのビット定義とマッピング。
            - **`src/ym2151/converter/waveform.rs`**: YM2151がサポートする波形の種類とそれらの設定。
        - **`src/ym2151/converter.rs`**: SMFイベントをYM2151レジスタ書き込みログに変換する主要なロジック（パスB）。
        - **`src/ym2151/converter_tests/`**: YM2151変換ロジックに関する様々なテストケース。
        - **`src/ym2151/event_processor.rs`**: YM2151イベントを時間順に処理し、レジスタ操作に変換するプロセッサ。
        - **`src/ym2151/event_processor_tests.rs`**: `event_processor.rs`のテストコード。
        - **`src/ym2151/events.rs`**: YM2151レジスタ書き込みイベントの構造体を定義します。
        - **`src/ym2151/init.rs`**: YM2151チップの初期化に関するレジスタ設定。
        - **`src/ym2151/mod.rs`**: `ym2151`モジュールのエントリポイント。
        - **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151の周波数設定値とのマッピングテーブル。
        - **`src/ym2151/tempo_map.rs`**: MIDIファイルのテンポ変更イベントを管理し、正確なタイミングを計算します。
        - **`src/ym2151/tone.rs`**: YM2151の音色（プログラムチェンジによって切り替わる音色設定）を定義・管理します。
- **`tests/`**: 統合テストが格納されているディレクトリ。
    - **`tests/create_test_midi.py`**: 統合テストで使用されるサンプルMIDIファイルをプログラム的に生成するためのPythonスクリプト。
    - **`tests/integration_conversion.rs`**: SMFからYM2151ログへの変換プロセス全体の統合テスト。
    - **`tests/integration_midi.rs`**: MIDIパースに関する統合テスト。
    - **`tests/integration_multichannel.rs`**: 複数のMIDIチャンネルが使用された場合の変換の統合テスト。
    - **`tests/integration_program_change.rs`**: プログラムチェンジによる音色切り替えが正しく行われるかの統合テスト。
    - **`tests/integration_public_api.rs`**: 公開されているAPIが正しく機能するかの統合テスト。
    - **`tests/integration_wasm.rs`**: WebAssemblyモジュールが正しく動作するかの統合テスト。
    - **`tests/test_data/`**: 統合テストで使用されるサンプルMIDIファイル群。
- **`tones/`**: カスタムYM2151音色定義をJSON形式で格納するディレクトリ。
    - **`tones/000.json`**: プログラムチェンジ0番（アコースティックグランドピアノ）のデフォルト音色定義。
    - **`tones/README.md`**: カスタム音色JSONファイルのフォーマットと使用方法に関する説明。

## 関数詳細説明
- **`applyRandomToneToAttachment`**: 添付ファイルにランダムなYM2151音色設定を適用します。
- **`bootstrap`**: デモアプリケーションの起動処理を行い、主要なコンポーネントを初期化します。
- **`bootstrapWebYm`**: WebYm2151デモ環境の初期化を実行します。
- **`buildEventsFromCompact`**: コンパクトな形式のデータからイベントオブジェクトのリストを構築します。
- **`buildNoteSegments`**: ノートイベントに基づいて、視覚化用のノートセグメントを作成します。
- **`buildRandomInterpolationAttachment`**: ランダムな音色補間設定を含む添付ファイルを生成します。
- **`cleanup`**: リソースの後処理やクリーンアップを行います。
- **`clampViewStart`**: ビューの開始位置を有効な範囲内に制限します。
- **`clearAudioCache`**: オーディオデータのキャッシュをクリアします。
- **`clearWebYmAudioCache`**: WebYm2151で使用されるオーディオキャッシュをクリアします。
- **`collectLfoEvents`**: YM2151レジスタログからLFO関連のイベントを収集します。
- **`computeHash`**: 与えられたデータのハッシュ値を計算します。
- **`computePitchRange`**: 視覚化のためにピッチの表示範囲を計算します。
- **`computeTrackWidth`**: 視覚化におけるトラックの表示幅を計算します。
- **`countRegisterNormalizationTargets`**: レジスタの正規化対象の数を数えます。
- **`convertMmlToSmf`**: MML (Music Macro Language) のテキストをStandard MIDI File (SMF) データに変換します。
- **`createLane`**: ログ視覚化における個々のレーン（トラック）要素を作成します。
- **`createLogVisualizer`**: YM2151レジスタログを視覚化するためのコンポーネントを生成します。
- **`createWaveformViewer`**: 波形表示ビューアのインスタンスを作成します。
- **`detectChannel`**: MIDIイベントからチャンネル情報を検出します。
- **`detectPopNoise`**: YM2151レジスタログからポップノイズが発生しうる箇所を検出します。
- **`displayResult`**: 変換や処理の結果をユーザーインターフェースに表示します。
- **`downloadWav`**: 生成されたWAVファイルをユーザーのデバイスにダウンロードします。
- **`drawEmpty`**: 波形キャンバスに何も描画されていない状態（空の状態）を描画します。
- **`drawWaveform`**: オーディオ波形をキャンバスに描画します。
- **`encodeWav`**: 生のオーディオデータをWAVファイル形式にエンコードします。
- **`endDrag`**: ドラッグ操作が終了した際のイベントを処理します。
- **`ensureGlobalLane`**: ログ視覚化にグローバルなレーンが存在することを確認します。
- **`ensureMmlRuntime`**: MMLの処理に必要なランタイムが利用可能であることを確認します。
- **`ensureWasmInitialized`**: WebAssemblyモジュールが初期化されていることを確認します。
- **`ensureWebYm2151`**: WebYm2151オーディオエンジンが利用可能であることを確認します。
- **`extractLfoRegistersFromAttachment`**: 添付ファイルデータからLFO関連のレジスタ設定を抽出します。
- **`extractNoteBoundaries`**: ノートイベントの開始・終了時刻から境界情報を抽出します。
- **`formatInactiveChannels`**: 非アクティブなチャンネルの表示を整形します。
- **`for`**: 繰り返し処理（ループ）の制御構造。
- **`generateAudioFromJson`**: YM2151レジスタログのJSONデータからオーディオを生成します。
- **`generateRandomInterpolationPairRegisters`**: 音色補間用のランダムなレジスタペアを生成します。
- **`generateRandomToneRegisters`**: ランダムなYM2151音色レジスタ設定を生成します。
- **`getMmlParser`**: MML (Music Macro Language) を解析するためのパーサーを取得します。
- **`getParseTreeJsonToSmf`**: パースツリーのJSONからSMFへの変換ロジックを取得します。
- **`getToneEditorGenerator`**: 音色エディタ用の設定ジェネレーターを取得します。
- **`getWindowDurS`**: 波形ビューアの表示ウィンドウの秒単位の長さを取得します。
- **`handlePlay`**: 再生ボタンが押された際の処理ロジック。
- **`if`**: 条件分岐の制御構造。
- **`initWasm`**: WebAssemblyモジュールの初期化を行います。
- **`initializeWasm`**: WebAssemblyモジュールを初期化します。
- **`isLatestRequest`**: 現在のリクエストが最新のものであるかを確認します。
- **`laneColor`**: ログ視覚化におけるレーンの色を決定します。
- **`main`**: メインのエントリポイント関数。デモの起動や主要な処理を管理します。
- **`mod`**: モジュロ演算を行います。
- **`nextRequestId`**: 新しいユニークなリクエストIDを生成します。
- **`normalizeAmplitude`**: オーディオ波形の振幅を正規化します。
- **`normalizeAttachmentText`**: 添付ファイルとして読み込まれたテキストデータを正規化します。
- **`normalizeEvents`**: MIDIイベントやYM2151イベントを標準化された形式に変換します。
- **`notePitch`**: ノートのピッチ値を取得します。
- **`noteYPosition`**: ノートが描画されるY座標を計算します。
- **`parseAttachmentEntries`**: 添付ファイルのエントリを解析し、構造化されたデータに変換します。
- **`parseAttachmentField`**: 添付ファイルに関するフィールドを解析します。
- **`parseHexByte`**: 16進数文字列をバイト値にパースします。
- **`playAudioWithOverlay`**: 音声の再生を開始し、同時にオーバーレイ表示を行います。
- **`readAttachmentBytes`**: 指定された添付ファイルのバイトデータを読み込みます。
- **`render`**: 波形ビューアの現在の状態をキャンバスに描画します。
- **`renderEmpty`**: ログ視覚化コンポーネントを空の状態として描画します。
- **`renderFromJson`**: JSONデータに基づいてログ視覚化コンポーネントや波形ビューアを描画します。
- **`renderLfoLane`**: LFOの情報を視覚化レーンに描画します。
- **`runConversion`**: MIDIからYM2151ログへの変換処理を実行します。
- **`serializeWithStatus`**: 処理ステータスを含めてデータをシリアライズします。
- **`setEventCountDisplay`**: 表示されるイベントの数を更新します。
- **`setLfoRegisters`**: YM2151のLFOレジスタを設定します。
- **`setZoom`**: 波形ビューアのズームレベルを設定します。
- **`setupAttachmentEditor`**: 添付ファイルを編集するためのUIコンポーネントを初期化します。
- **`setupFileInput`**: ファイル入力UIをセットアップします。
- **`setupMidiInput`**: MIDIファイルのアップロード入力UIをセットアップします。
- **`setupMmlInput`**: MMLテキスト入力UIをセットアップします。
- **`setupMmlToSmf`**: MMLからSMFへの変換機能をセットアップします。
- **`setupPlayButton`**: 再生ボタンのイベントハンドラなどを設定します。
- **`setupRandomToneButton`**: ランダム音色生成ボタンのイベントハンドラなどを設定します。
- **`setupWavExportButton`**: WAVエクスポートボタンのイベントハンドラなどを設定します。
- **`showError`**: エラーメッセージをユーザーインターフェースに表示します。
- **`setStatus`**: ステータスメッセージをユーザーインターフェースに設定します。
- **`syncLfoRegisters`**: LFOレジスタの設定を同期します。
- **`synthesizeAndRender`**: YM2151ログから音声を合成し、波形として描画します。
- **`treeToJson`**: 構文解析ツリーのような構造をJSON形式に変換します。
- **`updateBoundariesAndRender`**: 波形ビューアの境界を更新し、再描画します。
- **`updateOutput`**: ユーザーインターフェースの出力表示を更新します。
- **`updateOutputWithState`**: アプリケーションの現在の状態に基づいて出力表示を更新します。
- **`updatePlayButtonState`**: 再生ボタンの有効/無効状態などを更新します。
- **`updatePositionLabel`**: 波形ビューアの再生位置を示すラベルを更新します。
- **`updateRegisterReflectionStatus`**: レジスタ反映のステータス表示を更新します。
- **`upsertAttachmentRegisters`**: 添付ファイル内のレジスタ設定を挿入または更新します。
- **`upsertEntryRegisters`**: 特定のエントリのレジスタ設定を挿入または更新します。
- **`upsertInterpolationAttachmentRegisters`**: 補間添付ファイル内のレジスタ設定を挿入または更新します。
- **`validateRandomToneAttachment`**: ランダム音色設定の添付ファイルが有効であるかを検証します。
- **`while`**: 条件が真である間、繰り返し処理を行う制御構造。
- **`writeAscii`**: データビューにASCII文字列を書き込みます。

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
Generated at: 2026-04-19 07:13:45 JST

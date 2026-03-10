Last updated: 2026-03-11

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製ライブラリおよびツールです。
- ネイティブアプリケーションとWebAssembly (WASM) 対応のブラウザアプリケーションの両方で利用可能で、高度なチャンネル割り当て戦略とカスタム音色に対応しています。
- YM2151の特性を考慮した2パス変換処理により、デバッグ用中間イベントと最終YM2151レジスタログを生成し、高精度な音源再現を目指します。

## 技術スタック
- フロントエンド:
    - **TypeScript**: WebAssemblyデモのロジック開発に使用されるプログラミング言語。
    - **HTML**: デモページの構造を定義するためのマークアップ言語。
    - **CSS**: デモページのスタイリングに使用されるスタイルシート言語。
    - **Vite**: フロントエンドの高速な開発体験を提供するビルドツール。
    - **WebAssembly (WASM)**: Rustで書かれたコアロジックをウェブブラウザで実行可能にするためのバイナリフォーマット。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: デジタル楽器やシーケンサー間で音楽情報を交換するための標準フォーマット。
    - **YM2151**: ヤマハ製のFM音源チップで、多くのレトロゲームやPCに搭載されていました。
    - **MIDIプログラムチェンジ**: MIDIメッセージの一種で、音色（パッチ）を切り替えるために使用されます。
- 開発ツール:
    - **Rust**: プロジェクトのコアロジックを実装するために使用されるシステムプログラミング言語。
    - **Cargo**: Rustのビルドシステムおよびパッケージマネージャ。
    - **wasm-pack**: RustコードをWebAssemblyにビルドし、JavaScriptと連携させるためのツール。
    - **git**: バージョン管理システム。
    - **Biome**: コードのフォーマットとリンティングを統合的に行うツール。
- テスト:
    - **cargo test**: Rustの組み込みテストフレームワーク。
    - **cargo tarpaulin**: Rustプロジェクトのコードカバレッジ計測ツール。
- ビルドツール:
    - **Cargo**: Rustプロジェクトのビルド管理。
    - **wasm-pack**: WebAssemblyビルドの実行。
    - **Vite**: フロントエンドデモのビルドおよび開発サーバー。
- 言語機能:
    - **Rustの型システム**: コンパイル時の強力な型チェックにより、堅牢で安全なコードを保証します。
- 自動化・CI/CD:
    - **_config.yml**: GitHub Pagesのサイト設定ファイルで、デモページのデプロイに利用されます。
    - **cargo audit**: Rustプロジェクトの依存関係における既知の脆弱性をチェックするセキュリティツール。
    - **cargo fmt**: Rustコードのフォーマットを自動化するツール。
    - **cargo clippy**: Rustコードの潜在的なバグや非効率なコードを検出するリンター。
- 開発標準:
    - **cargo fmt --check**: コードフォーマットの遵守を確認。
    - **cargo clippy -- -D warnings**: リンターによる警告をエラーとして扱い、コード品質を厳しく保つ。
    - **Biome**: TypeScript/JavaScriptコードのフォーマットとリンティングの標準化。

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
  📖 180.md
  📖 181.md
  📖 183.md
  📖 184.md
  📖 185.md
  📖 186.md
  📖 187.md
  📖 188.md
  📖 189.md
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
- **.gitignore**: Gitが追跡しないファイルやディレクトリを指定する設定ファイル。
- **Cargo.lock**: Rustプロジェクトの依存関係の正確なバージョンを記録するファイル。
- **Cargo.toml**: Rustプロジェクトのビルド設定、依存関係、メタデータを定義するファイル。
- **LICENSE**: プロジェクトのライセンス情報が記載されたファイル。
- **README.ja.md**: プロジェクトの概要、使い方、開発方法などを日本語で説明するファイル。
- **README.md**: プロジェクトの概要、使い方、開発方法などを英語で説明するファイル。
- **WASM_USAGE.md**: WebAssembly版の使用方法と例を説明するファイル。
- **_config.yml**: GitHub Pagesのサイト設定ファイル。
- **demo-library/**: WebAssembly版ライブラリのデモアプリケーションが含まれるディレクトリ。
    - **biome.json**: Biomeツールによるコードフォーマットとリンティングの設定ファイル。
    - **delay-vibrato-demo.ts**: ディレイビブラート機能のデモ用TypeScriptコード。
    - **delay-vibrato.html**: ディレイビブラートデモのHTMLページ。
    - **envelope-generator.ts**: YM2151のエンベロープ生成に関連するユーティリティコード。
    - **globals.d.ts**: グローバル変数や関数の型定義ファイル。
    - **index.html**: デモのメインエントリポイントとなるHTMLページ。
    - **library-demo.ts**: ライブラリの基本的な使用方法を示すデモ用TypeScriptコード。
    - **log-visualizer-lfo.ts**: LFO（低周波発振器）イベントのログ可視化を担当するTypeScriptコード。
    - **log-visualizer-note-segments.ts**: ノートセグメントのログ可視化を担当するTypeScriptコード。
    - **log-visualizer.ts**: YM2151レジスタログを視覚的に表示する機能のTypeScriptコード。
    - **mml-support.ts**: MML (Music Macro Language) からSMFへの変換をサポートするTypeScriptコード。
    - **package-lock.json**: `npm`または`yarn`が使用する依存関係の正確なバージョンを記録するファイル。
    - **package.json**: デモプロジェクトの依存関係、スクリプト、メタデータを定義するファイル。
    - **pop-noise-demo.ts**: ポップノイズ現象のデモ用TypeScriptコード。
    - **pop-noise.html**: ポップノイズデモのHTMLページ。
    - **portamento-soft-lfo-demo.ts**: ポルタメントやソフトLFOのデモ用TypeScriptコード。
    - **portamento-soft-lfo.html**: ポルタメント・ソフトLFOデモのHTMLページ。
    - **shared-demo.ts**: デモアプリケーション間で共有される共通ロジックやユーティリティコード。
    - **style.css**: デモページのスタイルを定義するCSSファイル。
    - **tone-interpolation-demo.ts**: 音色補間機能のデモ用TypeScriptコード。
    - **tone-interpolation.html**: 音色補間デモのHTMLページ。
    - **tone-json-attachment.ts**: 音色JSONアタッチメントの処理に関連するTypeScriptコード。
    - **tone-json-demo.ts**: 音色JSONのデモ用TypeScriptコード。
    - **tone-json-mml.ts**: MMLからの音色JSON生成をサポートするTypeScriptコード。
    - **tone-json.html**: 音色JSONデモのHTMLページ。
    - **tsconfig.json**: TypeScriptコンパイラの設定ファイル。
    - **vite.config.ts**: Viteビルドツールの設定ファイル。
    - **waveform-canvas.ts**: 波形描画のためのキャンバス操作を扱うTypeScriptコード。
    - **waveform-simulator.ts**: YM2151の波形をシミュレートするTypeScriptコード。
    - **waveform-viewer.ts**: 波形ビューアのUIとロジックを扱うTypeScriptコード。
    - **ym2151-utils.ts**: YM2151関連の汎用ユーティリティ関数を提供するTypeScriptコード。
- **generated-docs/**: プロジェクトの自動生成されたドキュメントが格納されるディレクトリ。
- **googled947dc864c270e07.html**: Googleサイト認証用のファイル。
- **package-lock.json**: ルートディレクトリにあるJavaScriptプロジェクトの依存関係ロックファイル。
- **package.json**: ルートディレクトリにあるJavaScriptプロジェクトのメタデータと依存関係ファイル。
- **src/**: Rustのソースコードが格納されるディレクトリ。
    - **error.rs**: カスタムエラー型とエラー処理ロロジック。
    - **lib.rs**: ライブラリのエントリーポイントと公開API。
    - **main.rs**: コマンドラインツールのメインエントリポイント。
    - **midi/**: MIDIファイル解析に関連するモジュール。
        - **events.rs**: MIDIイベントのデータ構造を定義。
        - **mod.rs**: MIDIモジュールのエントリポイント。
        - **parser.rs**: Standard MIDI Filesを解析するロジック。
        - **utils.rs**: MIDI関連のユーティリティ関数。
        - **utils_tests.rs**: `utils.rs`の単体テスト。
    - **wasm.rs**: WebAssemblyバインディングのためのRustコード。
    - **ym2151/**: YM2151関連の変換ロジックが格納されるモジュール。
        - **channel_allocation.rs**: YM2151チャンネル割り当て戦略の実装。
        - **converter/**: YM2151レジスタログ変換の詳細ロジック。
            - **pitch_effects.rs**: ピッチ関連のエフェクト処理。
            - **register_effects.rs**: レジスタエフェクト処理。
            - **waveform.rs**: YM2151波形生成に関連するロジック。
        - **converter.rs**: MIDIイベントからYM2151レジスタログへの変換を司る主要ロジック。
        - **converter_tests/**: YM2151コンバータのテストコードが格納されるディレクトリ。
            - **attachments.rs**: アタッチメント（カスタム音色）関連のテスト。
            - **basic.rs**: 基本的な変換機能のテスト。
            - **channels.rs**: チャンネル割り当てのテスト。
            - **drums.rs**: ドラムチャンネル処理のテスト。
            - **effects.rs**: YM2151エフェクトのテスト。
            - **lfo.rs**: LFO処理のテスト。
            - **portamento.rs**: ポルタメント処理のテスト。
            - **programs.rs**: プログラムチェンジ処理のテスト。
        - **converter_tests.rs**: `converter.rs`のテストコード。
        - **event_processor.rs**: YM2151イベントの処理ロジック。
        - **event_processor_tests.rs**: `event_processor.rs`のテストコード。
        - **events.rs**: YM2151イベントのデータ構造を定義。
        - **init.rs**: YM2151の初期化設定に関するロジック。
        - **mod.rs**: YM2151モジュールのエントリポイント。
        - **note_table.rs**: MIDIノートからYM2151の周波数情報への変換テーブル。
        - **tempo_map.rs**: テンポ変更イベントを管理するマップ構造。
        - **tone.rs**: YM2151の音色データ構造と関連ロジック。
- **tests/**: プロジェクトの統合テストが格納されるディレクトリ。
    - **create_test_midi.py**: テスト用のMIDIファイルを生成するPythonスクリプト。
    - **integration_conversion.rs**: 変換の統合テスト。
    - **integration_midi.rs**: MIDI解析の統合テスト。
    - **integration_multichannel.rs**: マルチチャンネルMIDIの統合テスト。
    - **integration_program_change.rs**: プログラムチェンジの統合テスト。
    - **integration_wasm.rs**: WebAssembly統合テスト。
    - **test_data/**: テスト用のMIDIファイルが格納されるディレクトリ。
- **tones/**: カスタムYM2151音色定義のJSONファイルが格納されるディレクトリ。
    - **000.json**: プログラム番号000に対応するYM2151音色定義。
    - **README.md**: `tones`ディレクトリ内のJSONファイルのフォーマットと使い方を説明するファイル。

## 関数詳細説明
- **computeHash** (demo-library/delay-vibrato-demo.ts): 入力データのハッシュ値を計算し、リクエストの識別に使用する。
- **nextRequestId** (demo-library/delay-vibrato-demo.ts): 次のリクエストIDを生成する。
- **isLatestRequest** (demo-library/delay-vibrato-demo.ts): 現在のリクエストが最新のものであるかを確認する。
- **updateOutputWithState** (demo-library/delay-vibrato-demo.ts): UIの状態に応じて出力表示を更新する。
- **updatePlayButtonState** (demo-library/delay-vibrato-demo.ts): プレイボタンの有効/無効状態を更新する。
- **initializeWasm** (demo-library/delay-vibrato-demo.ts): WebAssemblyモジュールを初期化する。
- **readAttachmentBytes** (demo-library/delay-vibrato-demo.ts): アタッチメント（カスタム音色など）のバイトデータを読み込む。
- **runConversion** (demo-library/delay-vibrato-demo.ts): MIDIからYM2151ログへの変換処理を実行する。
- **handlePlay** (demo-library/delay-vibrato-demo.ts): 変換結果のオーディオ再生を処理する。
- **setupAttachmentEditor** (demo-library/delay-vibrato-demo.ts): アタッチメント編集用のUIを設定する。
- **setupMmlInput** (demo-library/delay-vibrato-demo.ts): MML入力フィールドのイベントハンドラを設定する。
- **setupMidiInput** (demo-library/delay-vibrato-demo.ts): MIDIファイル入力フィールドのイベントハンドラを設定する。
- **bootstrapWebYm** (demo-library/delay-vibrato-demo.ts): WebYm2151ライブラリの初期化と設定を行う。
- **main** (demo-library/delay-vibrato-demo.ts): デモアプリケーションの主要な初期化ロジック。
- **kcToFrequency** (demo-library/envelope-generator.ts): YM2151のKey Code (KC) から周波数に変換する。
- **ampStepPerSample** (demo-library/envelope-generator.ts): アンプリチュードのサンプルごとのステップ値を計算する。
- **initWasm** (demo-library/library-demo.ts): WebAssemblyモジュールを初期化する。
- **displayResult** (demo-library/library-demo.ts): 変換結果をUIに表示する。
- **showError** (demo-library/library-demo.ts): エラーメッセージをUIに表示する。
- **setupFileInput** (demo-library/library-demo.ts): ファイル入力要素のイベントリスナーを設定する。
- **resolveRegisterForChannel** (demo-library/log-visualizer-lfo.ts): 特定チャンネルのレジスタアドレスを解決する。
- **collectLfoEvents** (demo-library/log-visualizer-lfo.ts): LFOイベントを収集する。
- **renderLfoLane** (demo-library/log-visualizer-lfo.ts): LFOの視覚化レーンを描画する。
- **buildNoteSegments** (demo-library/log-visualizer-note-segments.ts): ノートイベントから表示用のセグメントを構築する。
- **notePitch** (demo-library/log-visualizer-note-segments.ts): ノートのピッチを計算する。
- **computePitchRange** (demo-library/log-visualizer-note-segments.ts): 表示するピッチの範囲を計算する。
- **noteYPosition** (demo-library/log-visualizer-note-segments.ts): ノートの表示Y座標を計算する。
- **detectChannel** (demo-library/log-visualizer.ts): YM2151レジスタイベントからチャンネルを検出する。
- **normalizeEvents** (demo-library/log-visualizer.ts): イベントデータを正規化し、可視化に適した形式に変換する。
- **laneColor** (demo-library/log-visualizer.ts): 各チャンネルのレーン色を決定する。
- **createLane** (demo-library/log-visualizer.ts): 可視化用のレーン要素を作成する。
- **computeTrackWidth** (demo-library/log-visualizer.ts): トラックの幅を計算する。
- **createLogVisualizer** (demo-library/log-visualizer.ts): YM2151ログ可視化コンポーネントを生成する。
- **renderEmpty** (demo-library/log-visualizer.ts): 空のログ可視化ビューを描画する。
- **renderFromJson** (demo-library/log-visualizer.ts): JSONデータからログ可視化ビューを描画する。
- **ensureGlobalLane** (demo-library/log-visualizer.ts): グローバルイベント用のレーンが確実に存在するようにする。
- **setLfoRegisters** (demo-library/log-visualizer.ts): LFOレジスタの値を設定する。
- **setupMmlToSmf** (demo-library/mml-support.ts): MMLからSMFへの変換機能を設定する。
- **setupPlayButton** (demo-library/pop-noise-demo.ts): 再生ボタンのイベントハンドラを設定する。
- **bootstrap** (demo-library/pop-noise-demo.ts): デモの起動処理を行う。
- **extractLfoRegistersFromAttachment** (demo-library/portamento-soft-lfo-demo.ts): アタッチメントからLFOレジスタ情報を抽出する。
- **syncLfoRegisters** (demo-library/portamento-soft-lfo-demo.ts): LFOレジスタをUIと同期させる。
- **ensureWasmInitialized** (demo-library/shared-demo.ts): WebAssemblyが初期化されていることを確認する。
- **setStatus** (demo-library/shared-demo.ts): ステータスメッセージをUIに表示する。
- **setEventCountDisplay** (demo-library/shared-demo.ts): イベント数をUIに表示する。
- **ensureWebYm2151** (demo-library/shared-demo.ts): WebYm2151インスタンスが確実に利用可能であることを確認する。
- **clearWebYmAudioCache** (demo-library/shared-demo.ts): WebYm2151のオーディオキャッシュをクリアする。
- **updateOutput** (demo-library/shared-demo.ts): 主要な出力エリアを更新する。
- **parseAttachmentField** (demo-library/shared-demo.ts): アタッチメントフィールドの値を解析する。
- **cleanup** (demo-library/shared-demo.ts): リソースのクリーンアップを行う。
- **buildEventsFromCompact** (demo-library/tone-json-attachment.ts): コンパクトな形式からイベントオブジェクトを構築する。
- **serializeWithStatus** (demo-library/tone-json-attachment.ts): 状態メッセージ付きでデータをシリアライズする。
- **normalizeAttachmentText** (demo-library/tone-json-attachment.ts): アタッチメントのテキストデータを正規化する。
- **convertMmlToSmf** (demo-library/tone-json-demo.ts): MMLをSMFに変換する。
- **getMmlParser** (demo-library/tone-json-mml.ts): MMLパーサーを取得する。
- **getParseTreeJsonToSmf** (demo-library/tone-json-mml.ts): パースツリーJSONからSMFへの変換関数を取得する。
- **treeToJson** (demo-library/tone-json-mml.ts): パースツリーをJSON形式に変換する。
- **ensureMmlRuntime** (demo-library/tone-json-mml.ts): MMLランタイムが利用可能であることを確認する。
- **drawEmpty** (demo-library/waveform-canvas.ts): 空の波形キャンバスを描画する。
- **drawWaveform** (demo-library/waveform-canvas.ts): 波形データをキャンバスに描画する。
- **simulateWaveform** (demo-library/waveform-simulator.ts): YM2151のレジスタ設定に基づいて波形をシミュレートする。
- **applyOp** (demo-library/waveform-simulator.ts): 波形シミュレーション中のオペレーションを適用する。
- **createWaveformViewer** (demo-library/waveform-viewer.ts): 波形ビューアコンポーネントを作成する。
- **getWindowDurS** (demo-library/waveform-viewer.ts): 表示ウィンドウの持続時間（秒）を取得する。
- **clampViewStart** (demo-library/waveform-viewer.ts): ビューの開始位置を有効な範囲にクランプする。
- **updatePositionLabel** (demo-library/waveform-viewer.ts): ポジション表示ラベルを更新する。
- **render** (demo-library/waveform-viewer.ts): 波形ビューアを再描画する。
- **rebuildAndRender** (demo-library/waveform-viewer.ts): 波形ビューアを再構築し、描画する。
- **setZoom** (demo-library/waveform-viewer.ts): ズームレベルを設定する。
- **endDrag** (demo-library/waveform-viewer.ts): ドラッグ操作の終了を処理する。
- **clear** (demo-library/waveform-viewer.ts): 波形ビューアの内容をクリアする。
- **parseHexByte** (demo-library/ym2151-utils.ts): 16進数文字列をバイト値に解析する。
- **playAudioWithOverlay** (demo-library/globals.d.ts): オーバーレイ付きでオーディオを再生する。
- **clearAudioCache** (demo-library/globals.d.ts): オーディオキャッシュをクリアする。

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
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-11 07:10:45 JST

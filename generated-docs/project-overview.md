Last updated: 2026-03-09

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- 独自の2パス処理アーキテクチャとYM2151チャンネル割り当て戦略により、高精度な音色変換と同時発音管理を実現します。
- ネイティブアプリケーションとWebAssembly（ブラウザ）の両方に対応し、幅広い用途でSMFのYM2151音源変換を可能にします。

## 技術スタック
- フロントエンド: HTML (デモページの構造), CSS (デモページのスタイル), TypeScript (デモアプリケーションのロジック), Vite (デモアプリケーションの高速開発ツール)
- 音楽・オーディオ: Standard MIDI Files (SMF) (入力形式), YM2151 FM音源チップ (ターゲット音源), JSON (YM2151レジスタ書き込みログの出力形式、音色定義)
- 開発ツール: Rust (主要な開発言語), Cargo (Rustのビルドシステムとパッケージマネージャー), wasm-pack (RustをWebAssemblyにビルドするツール)
- テスト: Rustの組み込みテストフレームワーク (単体・結合テスト), Tarpaulin (Rustコードのテストカバレッジ測定)
- ビルドツール: Cargo (Rustプロジェクトのビルド), wasm-pack (WebAssemblyパッケージの生成)
- 言語機能: Rustの強力な型システム (堅牢なコードのため), WebAssembly (WASM) (ブラウザでの実行を可能にする技術)
- 自動化・CI/CD: (プロジェクト情報には明示されていませんが、一般的に`cargo fmt`, `clippy`, `audit`はCI/CDプロセスに組み込まれます。)
- 開発標準: cargo fmt (コードフォーマッター), cargo clippy (コード品質リンター), cargo audit (依存関係のセキュリティ脆弱性チェック)

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
  📖 171.md
  📖 172.md
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
- **Cargo.toml**: Rustプロジェクトの構成ファイル。依存関係、パッケージ情報、機能フラグなどを定義します。
- **LICENSE**: プロジェクトのライセンス情報が記載されています。
- **README.ja.md / README.md**: プロジェクトの目的、機能、使用方法などが日本語および英語で説明されています。
- **WASM_USAGE.md**: WebAssembly版ライブラリの具体的な使用方法と例が記述されています。
- **_config.yml**: GitHub Pagesなどの設定ファイル。
- **src/error.rs**: プロジェクト固有のエラー型を定義し、エラーハンドリングを標準化します。
- **src/lib.rs**: このクレートのメインライブラリコード。SMFからYM2151ログへの変換機能の公開APIを提供します。
- **src/main.rs**: コマンドラインアプリケーションのエントリポイント。`lib.rs`の機能を利用してMIDIファイルを処理し、JSON形式のYM2151ログを出力します。
- **src/wasm.rs**: WebAssembly (WASM) バインディングを提供します。JavaScriptからRustのコア変換ロジックを呼び出すためのインターフェースを定義します。
- **src/midi/events.rs**: MIDIイベントの内部表現とデータ構造を定義します。
- **src/midi/mod.rs**: `src/midi`モジュール全体の公開定義を管理します。
- **src/midi/parser.rs**: Standard MIDI File (SMF) をパースし、内部的なMIDIイベントシーケンスに変換する主要なロジックを実装します。
- **src/midi/utils.rs**: MIDI関連の処理に共通して使用されるユーティリティ関数を提供します。
- **src/midi/utils_tests.rs**: `src/midi/utils.rs`で定義されたユーティリティ関数の単体テストが含まれています。
- **src/ym2151/channel_allocation.rs**: MIDIチャンネルをYM2151の限られたチャンネルに効率的に割り当てるための戦略（和音数ベース、ドラム優先など）を実装します。
- **src/ym2151/converter.rs**: MIDIイベントからYM2151レジスタ書き込みログへの変換処理の主要ロジックを担います。
- **src/ym2151/converter/pitch_effects.rs**: YM2151特有のピッチ関連のエフェクト（ポルタメントなど）の変換ロジックを扱います。
- **src/ym2151/converter/register_effects.rs**: YM2151のレジスタ操作に関する詳細な変換ロジックを扱います。
- **src/ym2151/converter/waveform.rs**: YM2151の波形生成に関連する変換ロジックを扱います。
- **src/ym2151/converter_tests.rs**: `src/ym2151/converter.rs`の主要なユニットテストが含まれています。
- **src/ym2151/converter_tests/*.rs**: YM2151変換ロジックに関する様々な側面（アタッチメント、基本変換、チャンネル、ドラム、エフェクト、プログラムチェンジ）の詳細なユニットテストが含まれています。
- **src/ym2151/event_processor.rs**: 変換されたYM2151イベントを最終的なレジスタログ形式に整形・処理します。
- **src/ym2151/event_processor_tests.rs**: `src/ym2151/event_processor.rs`のユニットテストが含まれています。
- **src/ym2151/events.rs**: YM2151レジスタログイベントのデータ構造を定義します。
- **src/ym2151/init.rs**: YM2151チップの初期化状態や初期設定に関する定義が含まれています。
- **src/ym2151/mod.rs**: `src/ym2151`モジュール全体の公開定義を管理します。
- **src/ym2151/note_table.rs**: MIDIノート番号とYM2151の周波数制御値（KL/KC）のマッピングテーブルを管理します。
- **src/ym2151/tempo_map.rs**: MIDIファイルのテンポイベントを解析し、タイムマップ（時間軸上のイベント位置）を生成するロジックを扱います。
- **src/ym2151/tone.rs**: YM2151音色（パッチ）のデータ構造と、外部JSONファイルからの読み込み、管理ロジックを定義します。
- **tests/*.rs**: 統合テストファイル群。MIDIパース、変換、プログラムチェンジ、WASM統合など、エンドツーエンドの動作検証を行います。
- **tests/create_test_midi.py**: 統合テストで使用するサンプルMIDIファイルをプログラムで生成するためのPythonスクリプト。
- **tests/test_data/*.mid**: 統合テストで使用される実際のMIDIファイルサンプル。
- **tones/*.json**: MIDIプログラムチェンジに対応するカスタムYM2151音色データ（JSON形式）を格納します。
- **tones/README.md**: カスタムYM2151音色ファイルのJSONフォーマットに関する説明です。
- **demo-library/biome.json**: Biome (JavaScript/TypeScriptリンター、フォーマッター) の設定ファイル。
- **demo-library/delay-vibrato-demo.ts / .html**: ディレイビブラート機能のデモページとそのTypeScriptロジック。
- **demo-library/envelope-generator.ts**: YM2151のエンベロープ生成に関連する計算ロジック。
- **demo-library/globals.d.ts**: グローバルに利用される型定義ファイル。
- **demo-library/index.html**: WebAssembly版ライブラリのメインデモページ。
- **demo-library/library-demo.ts**: WebAssembly版ライブラリの基本的な使用例を示すTypeScriptロジック。
- **demo-library/log-visualizer.ts**: YM2151レジスタ書き込みログを視覚的に表示するためのUIコンポーネント。
- **demo-library/mml-support.ts**: MML (Music Macro Language) からSMFへの変換（`mmlabc-to-smf-rust`との連携）をサポートするロジック。
- **demo-library/package.json / package-lock.json**: `demo-library`のJavaScript/TypeScript依存関係とパッケージ管理ファイル。
- **demo-library/pop-noise-demo.ts / .html**: ポップノイズ防止機能のデモページとそのTypeScriptロジック。
- **demo-library/portamento-soft-lfo-demo.ts / .html**: ポルタメントとソフトLFO機能のデモページとそのTypeScriptロジック。
- **demo-library/shared-demo.ts**: デモアプリケーション間で共通して使用されるユーティリティ関数（WASM初期化、ステータス表示など）。
- **demo-library/style.css**: デモページのスタイルシート。
- **demo-library/tone-interpolation-demo.ts / .html**: 音色補間機能のデモページとそのTypeScriptロジック。
- **demo-library/tone-json-attachment.ts**: YM2151音色JSONのアタッチメント処理ロジック。
- **demo-library/tone-json-demo.ts / .html**: カスタム音色JSONの読み込みと適用を示すデモページとそのTypeScriptロジック。
- **demo-library/tone-json-mml.ts**: 音色JSONとMMLの連携に関するロジック。
- **demo-library/tsconfig.json**: TypeScriptコンパイラの設定ファイル。
- **demo-library/vite.config.ts**: Viteビルドツールの設定ファイル。
- **demo-library/waveform-canvas.ts**: 波形描画用のHTML Canvas操作ロジック。
- **demo-library/waveform-simulator.ts**: YM2151の音源波形をシミュレートするロジック。
- **demo-library/waveform-viewer.ts**: 波形ビューアのUIとロジック。
- **demo-library/ym2151-utils.ts**: YM2151関連の数値計算やユーティリティ関数。

## 関数詳細説明
- **computeHash()** (demo-library/delay-vibrato-demo.ts): 与えられたデータからハッシュ値を計算し、リクエストの一意性を識別するために使用されます。
- **nextRequestId()**: 次のリクエストIDを生成し、非同期処理の管理に利用されます。
- **isLatestRequest()**: 特定のリクエストIDが最新のものであるかをチェックし、古い処理結果によるUIの更新を防ぎます。
- **updateOutputWithState()**: 現在のアプリケーションの状態に基づいてUIの出力エリアを更新します。
- **updatePlayButtonState()**: 音声再生ボタンの有効/無効状態を制御します。
- **initializeWasm()**: WebAssemblyモジュールを非同期で初期化し、Rustで実装された機能をブラウザで使用可能にします。
- **readAttachmentBytes()**: ユーザーがアップロードしたファイルや入力されたデータ（例えばYM2151音色JSON）をバイト配列として読み込みます。
- **runConversion()**: MIDIファイルまたはMMLデータからYM2151レジスタ書き込みログへの変換プロセスを実行します。
- **handlePlay()**: 変換されたYM2151ログデータを用いて音声を再生する処理を開始します。
- **setupAttachmentEditor()**: 添付ファイル（カスタム音色JSONなど）の入力・編集UIをセットアップします。
- **setupMmlInput()**: MML（Music Macro Language）入力フィールドを初期化し、MMLの入力イベントを処理します。
- **setupMidiInput()**: MIDIファイル入力フィールドを初期化し、MIDIファイルのアップロードイベントを処理します。
- **bootstrapWebYm()**: WebYmオーディオプレイヤーを初期化し、デモアプリケーション全体の起動処理を担います。
- **main()**: デモアプリケーションの主要なエントリポイント関数で、初期設定やイベントリスナーの登録を行います。
- **if**, **catch**: JavaScript/TypeScriptの制御構造。`if`は条件分岐、`catch`はエラーハンドリングに使用されます。
- **kcToFrequency(kc: number): number** (demo-library/envelope-generator.ts): YM2151のキーコード (KC) 値から対応する周波数をヘルツ (Hz) で計算します。
- **ampStepPerSample(rate: number, decayRate: number, sampleRate: number): number** (demo-library/envelope-generator.ts): サンプルレートと減衰率に基づき、YM2151エンベロープの振幅ステップを計算します。
- **switch**: JavaScript/TypeScriptの制御構造。複数の条件分岐を簡潔に記述します。
- **playAudioWithOverlay()** (demo-library/globals.d.ts): グローバルに利用可能な関数で、変換されたオーディオを再生し、必要に応じてオーバーレイUIを表示します。
- **clearAudioCache()**: グローバルに利用可能な関数で、WebYmオーディオプレイヤーのキャッシュをクリアします。
- **initWasm()** (demo-library/library-demo.ts): WebAssemblyモジュールを初期化し、ロードが完了した際に結果を処理します。
- **displayResult()**: 変換処理の結果（成功メッセージや出力データ）をUIに表示します。
- **showError(message: string)**: 指定されたエラーメッセージをUIのエラー表示エリアに表示します。
- **setupFileInput()**: MIDIファイルなどのファイル入力要素を初期化し、ファイル選択イベントを処理します。
- **resolveRegisterForChannel(ch: number, addr: number): string** (demo-library/log-visualizer.ts): 特定のチャンネルとアドレスからYM2151レジスタの名前や意味を解決し、表示可能な文字列を返します。
- **collectLfoEvents(events: LogEvent[]): LfoEvent[]**: YM2151ログイベントからLFO（低周波発振器）に関連するイベントを抽出します。
- **renderLfoLane(context: CanvasRenderingContext2D, lfoEvents: LfoEvent[], width: number, height: number)**: Canvas上にLFOの動きを示すレーンを描画します。
- **detectChannel(addr: number): number | undefined**: YM2151のレジスタアドレスから関連するチャンネル番号を推測します。
- **buildNoteSegments(events: LogEvent[]): NoteSegment[]**: YM2151ログイベントからノートオン/オフのシーケンスを解析し、音符のセグメントを構築します。
- **notePitch(kc: number, kf: number): number**: YM2151のKC/KF値からノートのピッチ（MIDIノート番号など）を計算します。
- **computePitchRange(segments: NoteSegment[]): { min: number, max: number }**: 全てのノートセグメントからピッチの最小値と最大値を計算します。
- **noteYPosition(note: number, minPitch: number, maxPitch: number, height: number): number**: ノートのピッチに基づいて、Canvas上のY座標を計算します。
- **normalizeEvents(events: LogEvent[], maxTime: number): LogEvent[]**: ログイベントのタイムスタンプを正規化します。
- **laneColor(channel: number): string**: 指定されたチャンネルに対応する色を返します。
- **createLane(channel: number, index: number): HTMLElement**: ログビジュアライザー内で各チャンネルのレーン（表示領域）をDOM要素として作成します。
- **computeTrackWidth(maxTime: number, pixelsPerSecond: number): number**: ログデータの最大時間に基づいて、ビジュアライザーのトラックの幅を計算します。
- **createLogVisualizer(container: HTMLElement): LogVisualizer**: ログを視覚化するためのビジュアライザーコンポーネントを生成し、指定されたコンテナに追加します。
- **renderEmpty()**: ビジュアライザーを空の状態（データなし）でレンダリングします。
- **renderFromJson(json: any)**: JSON形式のYM2151ログデータを受け取り、それを解析してビジュアライザー上に描画します。
- **ensureGlobalLane()**: 全体的な情報表示のための「グローバルレーン」がUIに存在することを確認します。
- **setLfoRegisters(lfoEvents: LfoEvent[])**: 抽出されたLFOイベントに基づいて、UI上のLFOレジスタ表示を更新します。
- **for**: JavaScript/TypeScriptの制御構造。ループ処理に使用されます。
- **setupMmlToSmf()** (demo-library/mml-support.ts): MML-to-SMF変換機能のセットアップを行います。
- **setupPlayButton()** (demo-library/pop-noise-demo.ts): 音声再生ボタンのイベントリスナーを設定します。
- **bootstrap()**: `pop-noise-demo.ts`のデモアプリケーションを起動します。
- **extractLfoRegistersFromAttachment()** (demo-library/portamento-soft-lfo-demo.ts): 添付ファイル（例えばYM2151音色JSON）からLFOレジスタ設定を抽出します。
- **syncLfoRegisters(lfoRegisters: LfoRegisters)**: 抽出されたLFOレジスタ設定を、UI上の対応するコントロールと同期させます。
- **ensureWasmInitialized()** (demo-library/shared-demo.ts): WebAssemblyモジュールが確実に初期化されているかをチェックします。
- **setStatus(message: string)**: ユーザーインターフェース上のステータス表示エリアにメッセージを設定します。
- **setEventCountDisplay(count: number)**: 処理されたイベントの数をUIに表示します。
- **ensureWebYm2151()**: WebYm2151オーディオプレイヤーのインスタンスが利用可能であることを確認します。
- **clearWebYmAudioCache()**: WebYm2151オーディオプレイヤーの内部キャッシュをクリアします。
- **updateOutput(output: string)**: 指定された文字列でUIの出力エリアを更新します。
- **parseAttachmentField(field: HTMLTextAreaElement | HTMLInputElement): Promise<Uint8Array | undefined>**: フォームフィールドから添付データ（バイト配列）を解析して取得します。
- **cleanup()**: デモアプリケーションが使用したリソース（例: オーディオコンテキスト）を解放します。
- **buildEventsFromCompact()** (demo-library/tone-json-attachment.ts): コンパクトな形式で記述された音色設定から、より詳細なYM2151イベント構造を構築します。
- **serializeWithStatus()**: データのシリアライズ処理を実行し、そのステータス（成功/失敗）を返します。
- **normalizeAttachmentText()**: 添付されたテキストデータ（例えばJSON）を正規化し、解析に適した形式にします。
- **convertMmlToSmf()** (demo-library/tone-json-demo.ts): MML文字列をStandard MIDI File (SMF) 形式に変換します。
- **getMmlParser()** (demo-library/tone-json-mml.ts): MMLを解析するためのパーサーインスタンスを取得します。
- **getParseTreeJsonToSmf()**: MMLのパースツリーからSMFを生成するためのJSONデータ構造を取得します。
- **treeToJson()**: 構文解析ツリーのような構造をJSON形式に変換します。
- **ensureMmlRuntime()**: MMLの変換に必要なランタイム環境が整っていることを確認します。
- **drawEmpty(context: CanvasRenderingContext2D, width: number, height: number)** (demo-library/waveform-canvas.ts): 指定されたCanvasコンテキスト上に、空の波形ビューを描画します。
- **drawWaveform(context: CanvasRenderingContext2D, waveform: number[], width: number, height: number)**: 指定されたCanvasコンテキスト上に、YM2151の波形データを描画します。
- **simulateWaveform(registers: YM2151Registers, durationSamples: number, sampleRate: number): number[]** (demo-library/waveform-simulator.ts): YM2151のレジスタ設定に基づき、指定された期間の音源波形をシミュレートしてサンプルデータの配列を生成します。
- **applyOp()**: 波形シミュレーションにおいて、YM2151のオペレーターの作用を計算します。
- **while**: JavaScript/TypeScriptの制御構造。条件が真である限りループ処理を継続します。
- **createWaveformViewer(container: HTMLElement): WaveformViewer** (demo-library/waveform-viewer.ts): 波形を表示・操作するためのビューアコンポーネントを生成し、指定されたコンテナに追加します。
- **getWindowDurS(): number**: 現在の波形ビューアの表示ウィンドウの秒単位の長さを取得します。
- **clampViewStart(viewStart: number): number**: 波形ビューの開始位置が有効な範囲内にあるように調整（クランプ）します。
- **updatePositionLabel(positionS: number)**: 現在の波形表示位置を示すラベルを更新します。
- **render()**: 波形ビューアの描画を更新します。
- **rebuildAndRender()**: 波形シミュレーションを再構築し、ビューアを再描画します。
- **setZoom(zoomLevel: number)**: 波形ビューアのズームレベルを設定します。
- **endDrag()**: 波形ビューアのドラッグ操作が終了した際の処理を実行します。
- **clear()**: 波形ビューアの内容をクリアします。
- **parseHexByte(hex: string): number** (demo-library/ym2151-utils.ts): 16進数形式の文字列（例: "C7"）をパースして、対応する数値（バイト値）を返します。

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
  - resolveRegisterForChannel (demo-library/log-visualizer.ts)
    - collectLfoEvents ()
      - renderLfoLane ()
      - detectChannel ()
      - buildNoteSegments ()
      - notePitch ()
      - computePitchRange ()
      - noteYPosition ()
      - normalizeEvents ()
      - laneColor ()
      - createLane ()
      - computeTrackWidth ()
      - renderEmpty ()
      - ensureGlobalLane ()
      - parseHexByte ()
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
- for (demo-library/log-visualizer.ts)
- while (demo-library/waveform-simulator.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-09 07:09:05 JST

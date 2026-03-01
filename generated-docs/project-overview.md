Last updated: 2026-03-02

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をヤマハYM2151 FM音源のレジスタ操作ログ（JSON形式）に変換するRust製ライブラリです。
- ネイティブアプリケーションとWebAssembly (WASM) 対応のWebブラウザ環境の両方で利用可能です。
- 高度なチャンネル割り当て戦略とプログラムチェンジ対応により、忠実なYM2151エミュレーションをサポートします。

## 技術スタック
- フロントエンド: HTML, CSS, TypeScript (WebデモのUI構築、インタラクション記述)、Vite (Webデモのビルドおよび開発サーバー)
- 音楽・オーディオ: Standard MIDI Files (SMF) (入力フォーマット)、YM2151 (出力対象のFM音源チップのアーキテクチャ)
- 開発ツール: Rust (主要な開発言語)、Cargo (Rustプロジェクトのビルドシステムおよびパッケージマネージャー)、wasm-pack (RustからWebAssemblyをビルドするツール)
- テスト: cargo test (ユニットテストおよび統合テストの実行)、cargo tarpaulin (テストカバレッジレポート生成)
- ビルドツール: Cargo (Rustクレートのビルド)、Vite (TypeScript/JavaScriptベースのWebデモのビルド)
- 言語機能: Rustの型システム (堅牢なコードを保証するための型安全な設計)
- 自動化・CI/CD: (明確なCI/CDツール名の言及はありませんが、開発標準に挙げられるツールはCI/CDワークフローに組み込むことが可能です。)
- 開発標準: cargo fmt (コードの自動フォーマット)、cargo clippy (RustコードのLinter、潜在的なバグや非効率を検出)、cargo audit (Rustプロジェクトの依存関係の脆弱性チェック)

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
  📘 delay-vibrato-demo.ts
  🌐 delay-vibrato.html
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
  📘 tone-json-demo.ts
  🌐 tone-json.html
  📊 tsconfig.json
  📘 vite.config.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
📁 issue-notes/
  📖 105.md
  📖 111.md
  📖 112.md
  📖 114.md
  📖 115.md
  📖 117.md
  📖 122.md
  📖 123.md
  📖 125.md
  📖 126.md
  📖 128.md
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
  📄 wasm.rs
  📁 ym2151/
    📄 channel_allocation.rs
    📁 converter/
      📄 pitch_effects.rs
      📄 register_effects.rs
      📄 waveform.rs
    📄 converter.rs
    📄 converter_tests.rs
    📄 event_processor.rs
    📄 events.rs
    📄 init.rs
    📄 mod.rs
    📄 note_table.rs
    📄 tempo_map.rs
    📄 tone.rs
📁 tests/
  📄 create_test_midi.py
  📄 integration_tests.rs
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
- **.gitignore**: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
- **Cargo.lock**: Rustプロジェクトの依存クレートとその正確なバージョンを記録し、ビルドの再現性を保証します。
- **Cargo.toml**: Rustプロジェクトのマニフェストファイル。プロジェクトのメタデータ、依存クレート、ビルド設定などを定義します。
- **LICENSE**: プロジェクトの配布ライセンス情報（著作権や利用条件）を記述します。
- **README.ja.md**: プロジェクトの日本語版の概要、機能、使い方、開発方法などを説明するドキュメントです。
- **README.md**: プロジェクトの英語版の概要、機能、使い方、開発方法などを説明するドキュメントです。
- **WASM_USAGE.md**: WebAssembly (WASM) 環境でこのライブラリを使用する方法について詳細に説明したドキュメントです。
- **_config.yml**: GitHub Pagesなどの静的サイトジェネレーターの設定ファイルです。
- **demo-library/**: WebAssembly版ライブラリの動作を示すデモンストレーションアプリケーションのソースコードと関連ファイル群を格納するディレクトリです。
    - **delay-vibrato-demo.ts**: ディレイビブラート機能のデモに特化したTypeScriptコードです。
    - **delay-vibrato.html**: ディレイビブラートデモ用のHTMLページです。
    - **globals.d.ts**: グローバル変数や型定義を宣言するTypeScriptの宣言ファイルです。
    - **index.html**: デモアプリケーションのメインHTMLページです。
    - **library-demo.ts**: WASMライブラリの基本的な使用方法を示すTypeScriptコードです。
    - **log-visualizer.ts**: YM2151のレジスタログを視覚的に表示するためのTypeScriptコードです。
    - **mml-support.ts**: Music Macro Language (MML) からStandard MIDI File (SMF) への変換をサポートするTypeScriptコードです。
    - **package-lock.json**: `demo-library`のNode.jsパッケージ依存関係の正確なバージョンを記録し、再現可能なインストールを保証します。
    - **package.json**: `demo-library`のNode.jsプロジェクトのメタデータと依存関係を定義します。
    - **pop-noise-demo.ts**: ポップノイズに関する機能のデモに特化したTypeScriptコードです。
    - **pop-noise.html**: ポップノイズデモ用のHTMLページです。
    - **portamento-soft-lfo-demo.ts**: ポルタメントとソフトLFO機能のデモに特化したTypeScriptコードです。
    - **portamento-soft-lfo.html**: ポルタメントとソフトLFOデモ用のHTMLページです。
    - **shared-demo.ts**: デモアプリケーション間で共有される共通のユーティリティ関数やロジックを含むTypeScriptコードです。
    - **style.css**: デモアプリケーションのスタイル定義を提供するCSSファイルです。
    - **tone-json-demo.ts**: カスタム音色JSONの読み込みと適用に関するデモに特化したTypeScriptコードです。
    - **tone-json.html**: カスタム音色JSONデモ用のHTMLページです。
    - **tsconfig.json**: TypeScriptコンパイラの設定ファイルです。
    - **vite.config.ts**: Viteビルドツールの設定ファイルで、デモアプリケーションのビルド方法を定義します。
- **generated-docs/**: `cargo doc`コマンドによって生成されるRustのAPIドキュメントが格納されるディレクトリです。
- **googled947dc864c270e07.html**: Googleサイト認証のために使用されるHTMLファイルです。
- **issue-notes/**: 開発中に検討された課題や決定事項、メモなどを記録したMarkdown形式のノートファイル群です。
- **package-lock.json**: プロジェクトルートのNode.jsパッケージ依存関係の正確なバージョンを記録し、再現可能なインストールを保証します。
- **package.json**: プロジェクトルートのNode.jsプロジェクトのメタデータと依存関係を定義します。
- **src/**: Rustのソースコードを格納するメインディレクトリです。
    - **error.rs**: プロジェクト内で使用されるカスタムエラー型を定義します。
    - **lib.rs**: Rustライブラリクレートのエントリポイントです。公開モジュール、機能、WASMバインディングのエクスポートなどを管理します。
    - **main.rs**: コマンドラインインターフェース (CLI) ツールのエントリポイントです。`smf-to-ym2151log-rust`コマンドの主要なロジックを実装します。
    - **midi/**: Standard MIDI File (SMF) の解析と関連データ構造を扱うモジュール群です。
        - **events.rs**: MIDIイベントの様々な種類とそれらのデータ構造を定義します。
        - **mod.rs**: `midi`モジュールのルートファイルで、サブモジュールを公開します。
        - **parser.rs**: MIDIファイルを解析し、内部のMIDIイベント表現に変換するロジックを実装します。
        - **utils.rs**: MIDI関連のヘルパー関数やユーティリティを提供します。
    - **wasm.rs**: WebAssembly (WASM) へのバインディングを提供し、JavaScriptからRust関数を呼び出せるようにするWASM公開関数を定義します。
    - **ym2151/**: YM2151 FM音源チップのレジスタ書き込みログへの変換ロジックを扱うモジュール群です。
        - **channel_allocation.rs**: MIDIチャンネルをYM2151の8つのチャンネルに効率的に割り当てるための戦略（和音数ベース、ドラム優先など）を実装します。
        - **converter/**: YM2151固有の音源変換ロジックのサブモジュール群です。
            - **pitch_effects.rs**: ピッチベンドやビブラートといったピッチ関連のMIDI効果をYM2151レジスタ操作に変換するロジックを実装します。
            - **register_effects.rs**: その他の一般的なMIDIコントローラーイベント（ボリューム、パンなど）をYM2151レジスタ書き込みに変換するロジックを実装します。
            - **waveform.rs**: YM2151のオペレーター波形や関連パラメータを処理するロジックを実装します。
        - **converter.rs**: MIDIイベントストリームをYM2151のレジスタ書き込みログに変換する主要なロジックを実装します。
        - **converter_tests.rs**: `ym2151/converter.rs`モジュールの単体テストが含まれています。
        - **event_processor.rs**: 変換過程でYM2151イベントを処理・管理するロジックを実装します。
        - **events.rs**: YM2151のレジスタ書き込みログの各イベントのデータ構造を定義します。
        - **init.rs**: YM2151チップの初期化状態やリセット時のレジスタ値を定義します。
        - **mod.rs**: `ym2151`モジュールのルートファイルで、サブモジュールを公開します。
        - **note_table.rs**: MIDIノート番号とYM2151の周波数設定値（FN/BLOCK）とのマッピングテーブルを提供します。
        - **tempo_map.rs**: MIDIのテンポ変更イベントに基づいて、時間軸を管理するためのテンポマップを構築するロジックを実装します。
        - **tone.rs**: YM2151の音色（プログラムチェンジによって切り替えられる音色定義）の読み込み、解析、適用を扱います。
- **tests/**: 統合テストのファイル群です。
    - **create_test_midi.py**: 統合テストのために使用されるテスト用MIDIファイルをプログラム的に生成するPythonスクリプトです。
    - **integration_tests.rs**: プロジェクト全体の主要な機能に対する統合テストを定義します。
    - **test_data/**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
- **tones/**: MIDIプログラムチェンジに対応するカスタムYM2151音色定義をJSON形式で格納するディレクトリです。
    - **000.json**: MIDIプログラム番号0番（グランドピアノなど）に対応するYM2151音色定義のJSONファイルです。
    - **README.md**: `tones`ディレクトリ内のカスタム音色JSONファイルのフォーマットと使用方法について説明するドキュメントです。

## 関数詳細説明
- **bootstrap(wasm, attachmentField, mmlField, midiField, playButton, mmlToSmfApi, webYmApi, outputElement, statusElement, eventCountElement)**: デモアプリケーションの起動処理を行い、WASMモジュール、UI要素、APIなどを初期化・設定します。
- **bootstrapWebYm(webYmApi, outputElement, statusElement, eventCountElement)**: WebYM2151関連のコンポーネントを初期化し、UI要素と連携させます。
- **buildEventsFromCompact(compactData)**: コンパクトなデータ構造からイベントリストを構築します。
- **clearAudioCache()**: オーディオキャッシュをクリアし、再生関連のリソースを解放します。
- **clearWebYmAudioCache(webYmApi)**: WebYM2151のオーディオキャッシュをクリアします。
- **cleanup(webYmApi)**: アプリケーションの終了時やリセット時に必要なクリーンアップ処理を実行します。
- **computeTrackWidth(events, totalTimeMs, scale)**: ログ視覚化のためにトラックの幅を計算します。
- **convertMmlToSmf(mmlApi, mmlText)**: MMLテキストをStandard MIDI File (SMF) データに変換します。
- **createLane(channel)**: ログ視覚化において、特定のチャンネルに対応する表示レーンを作成します。
- **createLogVisualizer(parent, width, height, totalTimeMs)**: ログを視覚化するための新しいコンポーネントを生成します。
- **detectChannel(event)**: イベントデータから関連するチャンネルを検出します。
- **displayResult(result, outputElement, statusElement, eventCountElement)**: 変換結果をUIに表示し、ステータスやイベント数を更新します。
- **ensureGlobalLane(visualizer, channel)**: ログ視覚化において、指定されたチャンネルのグローバルレーンが存在することを確認します。
- **ensureMmlRuntime(mmlApi)**: Music Macro Language (MML) ランタイムが利用可能であることを確認します。
- **ensureWasmInitialized(wasm)**: WebAssemblyモジュールが初期化されていることを確認します。
- **ensureWebYm2151(webYmApi)**: WebYM2151ライブラリが利用可能であることを確認します。
- **handlePlay(wasm, mmlToSmfApi, webYmApi, outputElement, statusElement, eventCountElement, mmlInput, midiInput, attachmentField)**: 再生ボタンクリック時のイベントを処理し、変換とオーディオ再生を開始します。
- **initWasm()**: WebAssemblyモジュールを初期化します。
- **initializeWasm()**: WebAssemblyモジュールを初期化し、Rustの関数をJavaScriptから利用可能にします。
- **isLatestRequest(requestId, latestRequestId)**: 特定のリクエストIDが最新のものであるかを確認します。
- **laneColor(channel)**: ログ視覚化のために、指定されたチャンネルの色を決定します。
- **main()**: デモアプリケーションの主要な実行ロジックを含むエントリポイント関数です。
- **nextRequestId()**: 新しいユニークなリクエストIDを生成します。
- **normalizeAttachmentText(text)**: 添付ファイルの内容を表すテキストデータを正規化します。
- **normalizeEvents(events)**: イベントデータを標準的な形式に正規化します。
- **parseAttachmentField(field)**: 添付ファイル入力フィールドの値を解析します。
- **parseHexByte(hexString)**: 16進数表記の文字列をバイト値にパースします。
- **playAudioWithOverlay(data, volume, webYmApi, outputElement)**: オーディオデータ（YM2151ログ）を再生し、必要に応じてオーバーレイ表示を伴います。
- **readAttachmentBytes(element)**: HTML要素から添付ファイル（MIDIなど）のバイトデータを読み込みます。
- **renderEmpty(visualizer)**: ログ視覚化コンポーネントを空の状態でレンダリングします。
- **renderFromJson(visualizer, ym2151Log)**: JSON形式のYM2151ログデータに基づいてログ視覚化コンポーネントをレンダリングします。
- **runConversion(wasm, inputBytes, attachmentBytes, outputElement, statusElement, eventCountElement, requestId)**: MIDIデータをYM2151ログに変換する処理を実行します。
- **setEventCountDisplay(element, count)**: UI上のイベント数表示を更新します。
- **setStatus(element, message, isError)**: UI上のステータスメッセージを設定します。エラーメッセージの場合は特別なスタイルを適用します。
- **setupAttachmentEditor(element, updateCallback)**: 添付ファイル編集用のUI要素を設定します。
- **setupFileInput(element, handler)**: ファイル入力UI要素を設定し、ファイル選択時のハンドラーを登録します。
- **setupMidiInput(element, updateCallback)**: MIDIファイル入力用のUI要素を設定します。
- **setupMmlInput(element, updateCallback)**: MML入力用のUI要素を設定します。
- **setupMmlToSmf(mmlToSmfApi, element, output)**: MMLからSMFへの変換機能をUIに設定します。
- **setupPlayButton(element, handler)**: 再生ボタンを設定し、クリックイベントハンドラーを登録します。
- **showError(message, outputElement, statusElement)**: エラーメッセージをUIに表示します。
- **treeToJson(tree)**: ツリー構造のデータをJSON形式に変換します。
- **updateOutput(element, message, isError)**: 出力エリアの表示内容を更新します。
- **updateOutputWithState(state, outputElement, statusElement, eventCountElement)**: 特定の状態に基づいて出力エリアを更新します。
- **updatePlayButtonState(state, button)**: 再生ボタンの表示状態（有効/無効、テキストなど）を更新します。

## 関数呼び出し階層ツリー
```
- main (demo-library/delay-vibrato-demo.ts)
  - bootstrapWebYm (demo-library/delay-vibrato-demo.ts)
  - initializeWasm (demo-library/delay-vibrato-demo.ts)
  - setupAttachmentEditor (demo-library/delay-vibrato-demo.ts)
  - setupMmlInput (demo-library/delay-vibrato-demo.ts)
  - setupMidiInput (demo-library/delay-vibrato-demo.ts)
  - handlePlay (demo-library/delay-vibrato-demo.ts)
    - nextRequestId (demo-library/delay-vibrato-demo.ts)
    - readAttachmentBytes (demo-library/delay-vibrato-demo.ts)
    - runConversion (demo-library/delay-vibrato-demo.ts)
      - isLatestRequest (demo-library/delay-vibrato-demo.ts)
      - updateOutputWithState (demo-library/delay-vibrato-demo.ts)
      - updatePlayButtonState (demo-library/delay-vibrato-demo.ts)
      - playAudioWithOverlay (demo-library/globals.d.ts)
      - createLogVisualizer (demo-library/log-visualizer.ts)
      - renderFromJson (demo-library/log-visualizer.ts)
      - setupMmlToSmf (demo-library/mml-support.ts)
      - ensureWasmInitialized (demo-library/shared-demo.ts)
      - setStatus (demo-library/shared-demo.ts)
      - setEventCountDisplay (demo-library/shared-demo.ts)
      - ensureWebYm2151 (demo-library/shared-demo.ts)
      - updateOutput (demo-library/shared-demo.ts)
      - parseAttachmentField (demo-library/shared-demo.ts)
- initWasm (demo-library/library-demo.ts)
  - displayResult (demo-library/library-demo.ts)
    - showError (demo-library/library-demo.ts)
    - setupFileInput (demo-library/library-demo.ts)
- parseHexByte (demo-library/log-visualizer.ts)
- detectChannel (demo-library/log-visualizer.ts)
- normalizeEvents (demo-library/log-visualizer.ts)
- laneColor (demo-library/log-visualizer.ts)
- createLane (demo-library/log-visualizer.ts)
- computeTrackWidth (demo-library/log-visualizer.ts)
- renderEmpty (demo-library/log-visualizer.ts)
- ensureGlobalLane (demo-library/log-visualizer.ts)
- treeToJson (demo-library/mml-support.ts)
- ensureMmlRuntime (demo-library/mml-support.ts)
  - buildEventsFromCompact (demo-library/tone-json-demo.ts)
  - normalizeAttachmentText (demo-library/tone-json-demo.ts)
  - convertMmlToSmf (demo-library/tone-json-demo.ts)
- clearAudioCache (demo-library/globals.d.ts)
- clearWebYmAudioCache (demo-library/shared-demo.ts)
  - cleanup (demo-library/shared-demo.ts)
- setupPlayButton (demo-library/pop-noise-demo.ts)
- bootstrap (demo-library/pop-noise-demo.ts)

---
Generated at: 2026-03-02 07:09:26 JST

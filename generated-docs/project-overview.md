Last updated: 2026-03-18

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRustライブラリです。
- ネイティブアプリケーションとWebAssembly (WASM) の両方で利用可能で、2パス処理とプログラムチェンジに対応しています。
- 高いパフォーマンスと型安全性を持ち、包括的なテスト駆動開発によって堅牢性が確保されています。

## 技術スタック
- フロントエンド:
    - **WebAssembly (WASM)**: RustコードをWebブラウザで実行可能にするための技術。
    - **TypeScript**: JavaScriptに静的型チェックを追加した言語で、デモ用WebUIのロジック開発に使用されています。
    - **HTML/CSS**: Webデモのユーザーインターフェース構造とスタイリングに使用されています。
    - **Vite**: フロントエンドの高速な開発体験を提供するビルドツールです。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: デジタル楽器やシーケンサー間で音楽データを交換するための標準フォーマットです。
    - **YM2151**: ヤマハ製のFM音源チップで、このプロジェクトではそのレジスタ書き込みログを生成します。
    - **JSON**: YM2151レジスタ書き込みログやカスタム音色ファイルのデータ形式として使用されています。
- 開発ツール:
    - **Rust**: 高性能かつ安全なシステムプログラミング言語で、プロジェクトの主要な実装言語です。
    - **Cargo**: Rustの公式なビルドシステムとパッケージマネージャーです。
    - **git**: ソースコードのバージョン管理システムです。
    - **wasm-pack**: Rustで書かれたWASMバイナリをWebで利用可能なパッケージにビルドするためのツールです。
- テスト:
    - **`cargo test`**: Rustの組み込みテストフレームワークで、ユニットテストと統合テストを実行します。
    - **Tarpaulin**: Rustプロジェクトのコードカバレッジレポートを生成するツールです。
- ビルドツール:
    - **Cargo**: Rustプロジェクトのビルド、依存関係管理、テスト実行などを行います。
    - **wasm-pack**: RustからWebAssemblyへのコンパイルとパッケージ化を行います。
    - **Vite**: デモ用Webアプリケーションのビルドと開発サーバーを提供します。
- 言語機能:
    - **Rustの型システム**: コンパイル時に多くのエラーを検出する強力な型安全性を提供し、堅牢なコード開発を支援します。
- 自動化・CI/CD:
    - (プロジェクト情報に明示的なCI/CDパイプラインの記述はありませんが、以下の開発標準はCI/CDに組み込まれる可能性があります。)
- 開発標準:
    - **`cargo fmt`**: Rustコードの自動フォーマットツールで、コードスタイルの一貫性を保ちます。
    - **`cargo clippy`**: Rustのリンターツールで、一般的な間違いや非効率なコードを指摘し、品質向上に貢献します。
    - **`cargo audit`**: Rustプロジェクトの依存関係のセキュリティ脆弱性をチェックするツールです。
    - **Biome**: JavaScript/TypeScriptコードのフォーマッターとリンターで、デモ関連コードの品質を維持します。

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
  📖 211.md
  📖 219.md
  📖 22.md
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
- **.gitignore**: Gitが追跡しないファイルやディレクトリを指定する設定ファイルです。
- **Cargo.lock**: Cargoが依存関係を解決した結果の正確なバージョンを記録し、ビルドの再現性を保証します。
- **Cargo.toml**: Rustプロジェクトの設定ファイルで、プロジェクト名、バージョン、依存関係などが定義されています。
- **LICENSE**: プロジェクトのライセンス情報が記載されています。
- **README.ja.md**: プロジェクトの日本語版概要と使い方が記載されたドキュメントです。
- **README.md**: プロジェクトの英語版概要と使い方が記載されたドキュメントです。
- **WASM_USAGE.md**: WebAssembly (WASM) でのライブラリ利用方法に関する詳細なドキュメントです。
- **_config.yml**: GitHub Pagesなどのサイト設定ファイルです。
- **demo-library/**: WebAssembly版ライブラリのデモアプリケーションが含まれるディレクトリです。
    - **biome.json**: Biomeというツール用の設定ファイルで、TypeScript/JavaScriptコードのフォーマットやリンティングルールを定義します。
    - **delay-vibrato-demo.ts**: ディレイビブラート機能のWebデモを実現するためのTypeScriptコードです。
    - **delay-vibrato.html**: ディレイビブラート機能のデモ用Webページです。
    - **globals.d.ts**: グローバルに定義される型宣言ファイルです。
    - **index.html**: メインのWebデモページです。
    - **library-demo.ts**: ライブラリの基本的なWebデモ動作を制御するTypeScriptコードです。
    - **log-visualizer-lfo.ts**: YM2151ログのLFO（低周波発振器）イベントを視覚化するロジックが含まれています。
    - **log-visualizer-note-segments.ts**: YM2151ログのノートセグメント（音符の開始と終了）を視覚化するロジックが含まれています。
    - **log-visualizer-pitch-canvas.ts**: ピッチの変化をキャンバスに描画して視覚化するロジックが含まれています。
    - **log-visualizer.ts**: YM2151レジスタログを視覚的に表示するための共通ロジックです。
    - **mml-support.ts**: MML (Music Macro Language) からSMFへの変換をサポートする機能が含まれています。
    - **package-lock.json**: Node.jsプロジェクトの依存関係の正確なバージョンを固定するファイルです。
    - **package.json**: Node.jsプロジェクトのメタデータと依存関係を定義するファイルです。
    - **pop-noise-demo.ts**: ポップノイズ検出機能のWebデモを実現するためのTypeScriptコードです。
    - **pop-noise-detector.ts**: YM2151音源のポップノイズを検出するロジックが含まれています。
    - **pop-noise.html**: ポップノイズ検出機能のデモ用Webページです。
    - **portamento-soft-lfo-demo.ts**: ポルタメントとソフトLFO機能のWebデモを実現するためのTypeScriptコードです。
    - **portamento-soft-lfo.html**: ポルタメントとソフトLFO機能のデモ用Webページです。
    - **shared-demo.ts**: 複数のデモページで共有される共通のTypeScriptロジックです。
    - **style.css**: Webデモページのスタイル定義（CSS）です。
    - **tone-interpolation-demo.ts**: 音色補間機能のWebデモを実現するためのTypeScriptコードです。
    - **tone-interpolation.html**: 音色補間機能のデモ用Webページです。
    - **tone-json-attachment.ts**: 音色JSONデータの添付と処理に関するロジックが含まれています。
    - **tone-json-demo.ts**: 音色JSON機能のWebデモを実現するためのTypeScriptコードです。
    - **tone-json-mml.ts**: MMLからの音色JSON生成に関するロジックが含まれています。
    - **tone-json.html**: 音色JSON機能のデモ用Webページです。
    - **tsconfig.json**: TypeScriptコンパイラの設定ファイルです。
    - **vite.config.ts**: Viteビルドツールの設定ファイルです。
    - **wav-exporter.ts**: ウェーブファイル（WAV）をエクスポートする機能を提供します。
    - **waveform-canvas.ts**: 波形をキャンバスに描画するロジックが含まれています。
    - **waveform-viewer.ts**: 生成された音源の波形を表示・操作するためのビューア機能です。
    - **ym2151-utils.ts**: YM2151関連のユーティリティ関数が含まれています。
- **generated-docs/**: `cargo doc`などによって生成されるドキュメントが格納されるディレクトリです。
- **googled947dc864c270e07.html**: Googleサイト認証用のファイルです。
- **issue-notes/**: 開発中に記録された課題や検討事項に関するMarkdown形式のメモです。
- **package-lock.json**: ルートディレクトリのNode.jsプロジェクトの依存関係の正確なバージョンを固定するファイルです。
- **package.json**: ルートディレクトリのNode.jsプロジェクトのメタデータと依存関係を定義するファイルです。
- **src/**: Rustのソースコードが格納されるディレクトリです。
    - **error.rs**: エラーハンドリングのためのカスタムエラー型と関連ロジックを定義します。
    - **lib.rs**: プロジェクトのライブラリクレートのメインエントリポイントです。
    - **main.rs**: コマンドラインツールとしての実行エントリポイントです。
    - **midi/**: MIDIファイルの解析と処理に関するモジュールです。
        - **events.rs**: MIDIイベントのデータ構造を定義します。
        - **mod.rs**: `midi`モジュールのルートファイルです。
        - **parser.rs**: Standard MIDI Files (SMF) を解析し、中間イベントに変換するロジックを提供します。
        - **utils.rs**: MIDI関連のユーティリティ関数が含まれています。
        - **utils_tests.rs**: MIDIユーティリティ関数のユニットテストです。
    - **wasm.rs**: WebAssemblyバインディングを提供し、RustコードをJavaScriptから呼び出せるようにします。
    - **ym2151/**: YM2151 FM音源チップ関連の変換ロジックとデータ構造を定義するモジュールです。
        - **channel_allocation.rs**: YM2151チャンネルをMIDIチャンネルに割り当てるためのロジックが含まれています。
        - **converter/**: YM2151レジスタログ変換の内部ロジックを構成するサブモジュールです。
            - **event_accumulator.rs**: イベントを累積し、YM2151レジスタイベントに変換するための状態を管理します。
            - **pitch_effects.rs**: ピッチに関するエフェクト（ビブラートなど）を処理するロジックです。
            - **register_effects.rs**: YM2151レジスタに書き込む各種エフェクトを処理するロジックです。
            - **register_fields.rs**: YM2151レジスタの各フィールドに関する定義と操作ロジックです。
            - **waveform.rs**: YM2151の波形生成に関するロジックです。
        - **converter.rs**: MIDIイベントをYM2151レジスタログに変換する主要なロジックです。
        - **converter_tests/**: YM2151コンバータの詳細なユニットテストが格納されています。
            - **attachments.rs**: 添付機能に関するテストです。
            - **attachments_change_to_next_tone.rs**: 次の音色への変更に関する添付テストです。
            - **attachments_program_effects.rs**: プログラムエフェクトに関する添付テストです。
            - **basic.rs**: 基本的な変換機能のテストです。
            - **channels.rs**: チャンネル割り当てに関するテストです。
            - **drums.rs**: ドラムチャンネルの処理に関するテストです。
            - **effects.rs**: 各種エフェクト変換のテストです。
            - **lfo.rs**: LFO（低周波発振器）変換のテストです。
            - **portamento.rs**: ポルタメント変換のテストです。
            - **programs.rs**: プログラムチェンジ処理のテストです。
        - **converter_tests.rs**: YM2151コンバータのテストのメインエントリポイントです。
        - **event_processor.rs**: MIDIイベントをYM2151特有のイベントに加工するロジックです。
        - **event_processor_tests.rs**: イベントプロセッサのテストです。
        - **events.rs**: YM2151レジスタイベントのデータ構造を定義します。
        - **init.rs**: YM2151の初期化に関するロジックです。
        - **mod.rs**: `ym2151`モジュールのルートファイルです。
        - **note_table.rs**: MIDIノート番号とYM2151の周波数設定値間のマッピングを定義します。
        - **tempo_map.rs**: MIDIのテンポ変更イベントを管理し、時間のマッピングを行います。
        - **tone.rs**: YM2151音色（トーン）のデータ構造と関連ロジックを定義します。
- **tests/**: 統合テストが格納されるディレクトリです。
    - **create_test_midi.py**: テスト用のMIDIファイルを生成するためのPythonスクリプトです。
    - **integration_conversion.rs**: 変換プロセスの統合テストです。
    - **integration_midi.rs**: MIDIパーサーの統合テストです。
    - **integration_multichannel.rs**: マルチチャンネルMIDIファイルの変換テストです。
    - **integration_program_change.rs**: プログラムチェンジ機能の統合テストです。
    - **integration_wasm.rs**: WebAssemblyインターフェースの統合テストです。
    - **test_data/**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
- **tones/**: プログラムチェンジで使用されるカスタムYM2151音色ファイル（JSON形式）が格納されるディレクトリです。
    - **000.json**: プログラム0番用のデフォルト音色データです。
    - **README.md**: 音色ファイルのフォーマットに関する説明ドキュメントです。

## 関数詳細説明
- **computeHash** (demo-library/delay-vibrato-demo.ts):
    - 役割: 入力データのハッシュ値を計算し、処理の重複を防ぐための識別子を生成します。
    - 機能: 提供されたテキストやデータを元に一意のハッシュを生成し、デモの状態管理に利用されます。
- **nextRequestId** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 非同期リクエストのための一意のIDを生成します。
    - 機能: 連続するリクエスト間で、どの処理が最新のものかを識別するために使用されます。
- **isLatestRequest** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 現在のリクエストIDが最新のものであるかを確認します。
    - 機能: 古い非同期処理の結果が誤ってUIに適用されるのを防ぎます。
- **updateOutputWithState** (demo-library/delay-vibrato-demo.ts他):
    - 役割: デモの状態に基づいて出力表示を更新します。
    - 機能: 変換結果やエラーメッセージなどをWebUI上に反映させます。
- **updatePlayButtonState** (demo-library/delay-vibrato-demo.ts他):
    - 役割: オーディオ再生ボタンの状態（有効/無効）を更新します。
    - 機能: 変換が完了し、再生可能な状態になったときにボタンを有効化します。
- **initializeWasm** (demo-library/delay-vibrato-demo.ts他):
    - 役割: WebAssemblyモジュールを初期化します。
    - 機能: Rustで書かれたコアロジックをWebブラウザ環境で利用可能にします。
- **readAttachmentBytes** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 添付ファイル（音色JSONなど）の内容を読み込みます。
    - 機能: ユーザーが指定したカスタム音色データをWASMモジュールに渡す準備をします。
- **runConversion** (demo-library/delay-vibrato-demo.ts他):
    - 役割: MIDIファイルまたはMMLをYM2151ログに変換する処理を実行します。
    - 機能: WASMモジュールを呼び出し、変換処理を実行し、その結果を取得します。
- **handlePlay** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 生成されたYM2151ログを基にオーディオ再生を開始します。
    - 機能: `web-ym2151`などのオーディオ再生ライブラリを呼び出して音源を鳴らします。
- **setupAttachmentEditor** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 添付ファイル（音色JSON）の編集UIを設定します。
    - 機能: ユーザーがブラウザ上で音色データを直接編集できるようにします。
- **setupMmlInput** (demo-library/delay-vibrato-demo.ts他):
    - 役割: MML入力フィールドのイベントハンドラを設定します。
    - 機能: ユーザーがMMLを入力した際に変換処理をトリガーします。
- **setupMidiInput** (demo-library/delay-vibrato-demo.ts他):
    - 役割: MIDIファイル入力フィールドのイベントハンドラを設定します。
    - 機能: ユーザーがMIDIファイルをアップロードした際に変換処理をトリガーします。
- **bootstrapWebYm** (demo-library/delay-vibrato-demo.ts他):
    - 役割: WebYM2151オーディオエンジンを初期化します。
    - 機能: ブラウザでのYM2151エミュレーションによる音声出力のための準備を行います。
- **main** (demo-library/delay-vibrato-demo.ts他):
    - 役割: デモアプリケーションのエントリポイントです。
    - 機能: アプリケーションの初期化、イベントリスナーの設定、最初の変換実行などを行います。
- **catch** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 非同期処理中のエラーを捕捉し、適切に処理します。
    - 機能: エラーメッセージをユーザーに表示するなどして、ユーザー体験を損なわないようにします。
- **if** (demo-library/delay-vibrato-demo.ts他):
    - 役割: 条件分岐を制御します。
    - 機能: 特定の条件が満たされた場合にのみコードブロックを実行します。
- **playAudioWithOverlay** (demo-library/globals.d.ts):
    - 役割: 音源を再生し、再生中にオーバーレイを表示します。
    - 機能: デモUI上でオーディオ再生の状態を視覚的に示します。
- **clearAudioCache** (demo-library/globals.d.ts):
    - 役割: オーディオキャッシュをクリアします。
    - 機能: 新しいオーディオデータを生成する前に、以前のデータを破棄します。
- **generateAudioFromJson** (demo-library/globals.d.ts):
    - 役割: JSON形式のYM2151ログからオーディオデータを生成します。
    - 機能: `web-ym2151`などのライブラリを呼び出して、YM2151エミュレーションによる音源を生成します。
- **initWasm** (demo-library/library-demo.ts):
    - 役割: WebAssemblyモジュールの初期化を行います。
    - 機能: `initializeWasm`を呼び出し、結果をUIに表示します。
- **displayResult** (demo-library/library-demo.ts):
    - 役割: 変換結果をWebUIに表示します。
    - 機能: 成功または失敗のメッセージ、生成されたログの視覚化などを行います。
- **showError** (demo-library/library-demo.ts):
    - 役割: エラーメッセージをWebUIに表示します。
    - 機能: 変換中に問題が発生した場合、ユーザーにその内容を通知します。
- **setupFileInput** (demo-library/library-demo.ts):
    - 役割: ファイル入力要素のイベントハンドラを設定します。
    - 機能: ユーザーがMIDIファイルを選択した際に、そのファイルを読み込み、変換処理に渡します。
- **resolveRegisterForChannel** (demo-library/log-visualizer-lfo.ts):
    - 役割: 特定のチャンネルにおけるレジスタアドレスを解決します。
    - 機能: YM2151のレジスタマッピングに基づき、チャンネルごとのレジスタを特定します。
- **collectLfoEvents** (demo-library/log-visualizer-lfo.ts):
    - 役割: YM2151ログからLFO関連のイベントを収集します。
    - 機能: 視覚化のためにLFOレジスタの変更履歴を抽出します。
- **renderLfoLane** (demo-library/log-visualizer-lfo.ts):
    - 役割: LFOイベントを視覚化するためのレーンを描画します。
    - 機能: LFOの変化を時間軸に沿ってグラフィカルに表示します。
- **for** (demo-library/log-visualizer-lfo.ts他):
    - 役割: ループ処理を制御します。
    - 機能: コレクションの各要素に対して操作を実行します。
- **buildNoteSegments** (demo-library/log-visualizer-note-segments.ts):
    - 役割: ノートイベントから視覚化用のセグメントを構築します。
    - 機能: 音符の開始、継続、終了をグラフィカルに表現するためのデータ構造を作成します。
- **notePitch** (demo-library/log-visualizer-note-segments.ts):
    - 役割: ノートのピッチ値を取得します。
    - 機能: 視覚化のためにノートの音高情報を抽出します。
- **computePitchRange** (demo-library/log-visualizer-note-segments.ts):
    - 役割: 視覚化するノートのピッチ範囲を計算します。
    - 機能: 表示領域に合わせたピッチスケールを決定します。
- **noteYPosition** (demo-library/log-visualizer-note-segments.ts):
    - 役割: ノートのY軸位置を計算します。
    - 機能: キャンバス上でのノートの垂直方向の位置を決定します。
- **renderPitchCanvas** (demo-library/log-visualizer-pitch-canvas.ts):
    - 役割: ピッチ視覚化用のキャンバスを描画します。
    - 機能: 音高の変化を時間軸に沿ってグラフィカルに表示します。
- **while** (demo-library/log-visualizer-pitch-canvas.ts):
    - 役割: 条件が真である間、ループ処理を制御します。
    - 機能: 特定の条件が続く限り、コードブロックを繰り返し実行します。
- **detectChannel** (demo-library/log-visualizer.ts):
    - 役割: YM2151ログ内のイベントから関連するチャンネルを検出します。
    - 機能: 特定のイベントがどのYM2151チャンネルに影響を与えるかを特定します。
- **normalizeEvents** (demo-library/log-visualizer.ts):
    - 役割: YM2151ログイベントを視覚化しやすい形式に正規化します。
    - 機能: 視覚化のためのデータ整形を行います。
- **laneColor** (demo-library/log-visualizer.ts):
    - 役割: 各YM2151チャンネルに割り当てる色を決定します。
    - 機能: 視覚的な識別を容易にするための色情報を返します。
- **createLane** (demo-library/log-visualizer.ts):
    - 役割: 視覚化用のレーン要素を作成します。
    - 機能: 各チャンネルや特定のイベントタイプに対応する表示領域をDOMに生成します。
- **computeTrackWidth** (demo-library/log-visualizer.ts):
    - 役割: トラック全体の幅を計算します。
    - 機能: ログイベントの時間軸全体に基づいて表示領域の幅を決定します。
- **formatInactiveChannels** (demo-library/log-visualizer.ts):
    - 役割: 非アクティブなYM2151チャンネルの表示をフォーマットします。
    - 機能: 未使用のチャンネルに関する情報をUIに表示します。
- **createLogVisualizer** (demo-library/log-visualizer.ts):
    - 役割: YM2151ログ視覚化コンポーネントを初期化します。
    - 機能: ログデータの解析と表示を行うためのインスタンスを生成します。
- **renderEmpty** (demo-library/log-visualizer.ts):
    - 役割: 空のログ状態を表示します。
    - 機能: ログデータがない場合に、適切なメッセージや表示を行います。
- **renderFromJson** (demo-library/log-visualizer.ts):
    - 役割: JSON形式のYM2151ログを基に視覚化をレンダリングします。
    - 機能: 変換されたレジスタログをグラフィカルに表示します。
- **ensureGlobalLane** (demo-library/log-visualizer.ts):
    - 役割: グローバルなイベント（全チャンネルに影響する）を表示するレーンを確保します。
    - 機能: LFOなどのグローバルなYM2151イベントを表示するための領域を作成します。
- **setLfoRegisters** (demo-library/log-visualizer.ts):
    - 役割: LFOレジスタの表示状態を設定します。
    - 機能: LFOイベントの視覚化に関する特定のレジスタ値を扱います。
- **setupMmlToSmf** (demo-library/mml-support.ts):
    - 役割: MMLからSMFへの変換機能を設定します。
    - 機能: 別のRustライブラリをWASM経由で呼び出し、MML入力をSMFデータに変換します。
- **detectPopNoise** (demo-library/pop-noise-detector.ts):
    - 役割: YM2151ログからポップノイズの発生を検出します。
    - 機能: 音源再生時に発生しうるノイズの原因となるレジスタ操作パターンを特定します。
- **setupPlayButton** (demo-library/pop-noise-demo.ts):
    - 役割: 再生ボタンのイベントハンドラを設定します。
    - 機能: ボタンクリックでオーディオ再生を開始します。
- **setupWavExportButton** (demo-library/pop-noise-demo.ts):
    - 役割: WAVエクスポートボタンのイベントハンドラを設定します。
    - 機能: ボタンクリックで生成された音源をWAVファイルとしてダウンロードできるようにします。
- **getToneEditorGenerator** (demo-library/pop-noise-demo.ts他):
    - 役割: 音色エディタのUI要素を生成する関数を取得します。
    - 機能: ユーザーが音色パラメータを調整するためのインタフェースを提供します。
- **applyRandomToneToAttachment** (demo-library/pop-noise-demo.ts):
    - 役割: 添付データにランダムなYM2151音色を適用します。
    - 機能: テストやデモンストレーションのために、多様な音色を自動生成して適用します。
- **setupRandomToneButton** (demo-library/pop-noise-demo.ts):
    - 役割: ランダム音色生成ボタンのイベントハンドラを設定します。
    - 機能: ボタンクリックでランダムな音色を生成し、デモに適用します。
- **bootstrap** (demo-library/pop-noise-demo.ts):
    - 役割: デモアプリケーションの初期起動処理を行います。
    - 機能: WASM初期化、UIセットアップ、イベントリスナー登録など、全体のセットアップを実行します。
- **extractLfoRegistersFromAttachment** (demo-library/portamento-soft-lfo-demo.ts):
    - 役割: 添付された音色データからLFO関連のレジスタ設定を抽出します。
    - 機能: カスタム音色に含まれるLFOパラメータを分離して取得します。
- **syncLfoRegisters** (demo-library/portamento-soft-lfo-demo.ts):
    - 役割: LFOレジスタの設定を同期させます。
    - 機能: 異なるUI要素や内部状態間でLFOパラメータの一貫性を保ちます。
- **ensureWasmInitialized** (demo-library/shared-demo.ts):
    - 役割: WebAssemblyモジュールが確実に初期化されていることを確認します。
    - 機能: WASMが未初期化の場合、初期化処理をトリガーします。
- **setStatus** (demo-library/shared-demo.ts):
    - 役割: デモのステータスメッセージをWebUIに表示します。
    - 機能: 現在の処理状況やエラーなどをユーザーに伝えます。
- **setEventCountDisplay** (demo-library/shared-demo.ts):
    - 役割: 変換されたイベントの数をWebUIに表示します。
    - 機能: 処理されたMIDIイベントやYM2151イベントの総数をユーザーに示します。
- **ensureWebYm2151** (demo-library/shared-demo.ts):
    - 役割: WebYM2151オーディオエンジンが確実にロードされていることを確認します。
    - 機能: オーディオ再生前にエンジンが利用可能であることを保証します。
- **clearWebYmAudioCache** (demo-library/shared-demo.ts):
    - 役割: WebYM2151オーディオエンジンの内部キャッシュをクリアします。
    - 機能: 以前生成されたオーディオデータを破棄し、新しい生成に備えます。
- **updateOutput** (demo-library/shared-demo.ts):
    - 役割: デモの出力表示を更新します。
    - 機能: 変換結果や視覚化データをWebUIにレンダリングします。
- **parseAttachmentField** (demo-library/shared-demo.ts):
    - 役割: 添付入力フィールドの値を解析します。
    - 機能: ユーザーが入力した添付JSON文字列などを処理します。
- **cleanup** (demo-library/shared-demo.ts):
    - 役割: リソースのクリーンアップ処理を行います。
    - 機能: オーディオキャッシュのクリアなど、デモ終了時やリセット時に必要な処理を実行します。
- **mod** (demo-library/shared-demo.ts):
    - 役割: 剰余演算を実行します。
    - 機能: 数値が特定の範囲内に収まるように調整する際などに使用されます。
- **buildRandomAttachment** (demo-library/tone-interpolation-demo.ts):
    - 役割: ランダムなYM2151音色添付データを生成します。
    - 機能: テストやデモ目的で、様々な音色を動的に作成します。
- **buildEventsFromCompact** (demo-library/tone-json-attachment.ts):
    - 役割: コンパクトなJSON形式からYM2151イベントリストを構築します。
    - 機能: 簡略化された音色定義から、完全なレジスタイベントシーケンスを生成します。
- **serializeWithStatus** (demo-library/tone-json-attachment.ts):
    - 役割: データをシリアライズし、その処理ステータスを返します。
    - 機能: JSONオブジェクトなどを文字列に変換し、成功/失敗の情報を付加します。
- **normalizeAttachmentText** (demo-library/tone-json-attachment.ts):
    - 役割: 添付テキストデータを正規化します。
    - 機能: ユーザー入力されたテキストの整形や検証を行います。
- **convertMmlToSmf** (demo-library/tone-json-demo.ts):
    - 役割: MML (Music Macro Language) をStandard MIDI File (SMF) に変換します。
    - 機能: MML入力からMIDIデータを生成し、さらにYM2151ログ変換の入力として利用します。
- **getMmlParser** (demo-library/tone-json-mml.ts):
    - 役割: MMLパーサーを取得します。
    - 機能: MML文字列を解析し、構文木を生成するためのパーサーを提供します。
- **getParseTreeJsonToSmf** (demo-library/tone-json-mml.ts):
    - 役割: 構文木（JSON形式）をSMFに変換する機能を取得します。
    - 機能: MMLパーサーが出力した構文木をMIDIファイルデータに変換します。
- **treeToJson** (demo-library/tone-json-mml.ts):
    - 役割: 構文木をJSON形式に変換します。
    - 機能: 構文解析結果を標準的なデータ形式で表現します。
- **ensureMmlRuntime** (demo-library/tone-json-mml.ts):
    - 役割: MMLランタイム環境が確実に初期化されていることを確認します。
    - 機能: MML変換に必要な依存関係がロードされていることを保証します。
- **encodeWav** (demo-library/wav-exporter.ts):
    - 役割: 音声データをWAVファイル形式にエンコードします。
    - 機能: 生のオーディオサンプルデータからWAVヘッダーとデータを結合したバイト列を生成します。
- **writeAscii** (demo-library/wav-exporter.ts):
    - 役割: 文字列をASCIIバイト列として書き込みます。
    - 機能: WAVファイルのチャンクIDなどをバイト列としてエンコードするために使用されます。
- **downloadWav** (demo-library/wav-exporter.ts):
    - 役割: 生成されたWAVデータをユーザーにダウンロードさせます。
    - 機能: ブラウザのダウンロード機能をトリガーし、ファイルを保存します。
- **drawEmpty** (demo-library/waveform-canvas.ts):
    - 役割: 波形キャンバスを空の状態（何も描画されていない状態）で初期化します。
    - 機能: 以前の波形をクリアしたり、デフォルトの空白表示を行ったりします。
- **drawWaveform** (demo-library/waveform-canvas.ts):
    - 役割: 音声波形をキャンバスに描画します。
    - 機能: 提供されたオーディオデータに基づき、時間軸に沿った波形を視覚化します。
- **extractNoteBoundaries** (demo-library/waveform-viewer.ts):
    - 役割: 波形データからノートの境界（開始・終了点）を抽出します。
    - 機能: YM2151ログに基づいて、再生されている音符の区切りを特定します。
- **normalizeAmplitude** (demo-library/waveform-viewer.ts):
    - 役割: 波形データの振幅を正規化します。
    - 機能: 表示に適したレンジにオーディオデータの音量を調整します。
- **createWaveformViewer** (demo-library/waveform-viewer.ts):
    - 役割: 波形ビューアコンポーネントを初期化します。
    - 機能: 音声波形を表示・操作するためのインタフェースを生成します。
- **getWindowDurS** (demo-library/waveform-viewer.ts):
    - 役割: 現在表示されているウィンドウの再生時間（秒）を取得します。
    - 機能: ズームレベルに基づいて、表示範囲の長さを計算します。
- **clampViewStart** (demo-library/waveform-viewer.ts):
    - 役割: ビューの開始位置を有効な範囲内に制限します。
    - 機能: ユーザーが波形ビューをスクロールしすぎないように調整します。
- **updatePositionLabel** (demo-library/waveform-viewer.ts):
    - 役割: 現在の再生位置を示すラベルを更新します。
    - 機能: 波形ビューア上で時間情報を表示します。
- **render** (demo-library/waveform-viewer.ts):
    - 役割: 波形ビューアの表示をレンダリングします。
    - 機能: 内部状態（ズーム、スクロール位置など）に基づいて波形を再描画します。
- **updateBoundariesAndRender** (demo-library/waveform-viewer.ts):
    - 役割: 波形境界を更新し、ビューアを再レンダリングします。
    - 機能: 新しいオーディオデータがロードされた際などに、表示範囲を調整して描画します。
- **synthesizeAndRender** (demo-library/waveform-viewer.ts):
    - 役割: オーディオを合成し、その結果をビューアにレンダリングします。
    - 機能: YM2151ログから音源を生成し、その波形を表示します。
- **setZoom** (demo-library/waveform-viewer.ts):
    - 役割: 波形ビューアのズームレベルを設定します。
    - 機能: ユーザー操作に応じて波形の拡大・縮小を行います。
- **endDrag** (demo-library/waveform-viewer.ts):
    - 役割: ドラッグ操作の終了を処理します。
    - 機能: ユーザーが波形をスクロールした際のドラッグ終了時のロジックを実行します。
- **clear** (demo-library/waveform-viewer.ts):
    - 役割: 波形ビューアの表示内容をクリアします。
    - 機能: 以前の波形表示を消去し、初期状態に戻します。
- **exportWav** (demo-library/waveform-viewer.ts):
    - 役割: 現在表示されている波形をWAVファイルとしてエクスポートします。
    - 機能: 表示されているオーディオデータをファイルに保存できるようにします。
- **parseHexByte** (demo-library/ym2151-utils.ts):
    - 役割: 16進数文字列をバイト値に解析します。
    - 機能: YM2151のレジスタアドレスやデータ値の文字列表現を数値に変換します。

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
      - getToneEditorGenerator ()
      - applyRandomToneToAttachment ()
      - setupRandomToneButton ()
      - bootstrap ()
      - createWaveformViewer ()
      - exportWav ()
      - setLfoRegisters (demo-library/log-visualizer.ts)
      - extractLfoRegistersFromAttachment ()
      - syncLfoRegisters ()
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
- for (demo-library/log-visualizer-lfo.ts)
- while (demo-library/log-visualizer-pitch-canvas.ts)
- mod (demo-library/shared-demo.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-18 07:15:17 JST

Last updated: 2026-05-17

# Project Overview

## プロジェクト概要
- Standard MIDIファイルをYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- ネイティブアプリケーションとして利用できる他、WebAssemblyとしてWebブラウザ上で動作させることも可能です。
- MIDIチャンネルの和音数に応じたYM2151チャンネル割り当てや、外部定義によるプログラムチェンジ（音色切り替え）に対応しています。

## 技術スタック
- フロントエンド: 
  - **TypeScript**: WebAssemblyデモのロジック開発に使用されています。
  - **HTML**: WebAssemblyデモのユーザーインターフェース構造を定義しています。
  - **CSS**: WebAssemblyデモのスタイルを定義しています。
  - **JavaScript**: WebAssemblyモジュールの読み込みと連携、オーディオ再生関連（WASMデモ）に使用されます。
  - **Vite**: WebAssemblyデモの高速な開発サーバーおよびビルドツールとして使用されています。
- 音楽・オーディオ: 
  - **Standard MIDI Files (SMF)**: プロジェクトの入力となる音楽データ形式です。
  - **YM2151 FM音源**: プロジェクトの出力ターゲットとなる音源チップです。
- 開発ツール: 
  - **Rust**: プロジェクトの主要なプログラミング言語です。
  - **Cargo**: Rustプロジェクトのビルド、依存関係管理、テスト実行に使用される標準ツールです。
  - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、JavaScriptから利用可能なパッケージを生成するために使用されます。
  - **Git**: バージョン管理システムとして使用されています。
- テスト: 
  - **Cargo test**: Rustの組み込みテストフレームワークにより、ユニットテストと統合テストが実行されます。
  - **Cargo tarpaulin**: テストカバレッジの測定に使用されます。
- ビルドツール: 
  - **Cargo**: Rustプロジェクトのネイティブビルドを管理します。
  - **wasm-pack**: WebAssemblyモジュールのビルドを管理します。
  - **Vite**: WebAssemblyデモのフロントエンドビルドを管理します。
- 言語機能: 
  - **Rustの型システム**: コードの堅牢性と安全性を保証するために活用されています。
- 自動化・CI/CD: 
  - (特になし。プロジェクト情報からは読み取れません)
- 開発標準: 
  - **Cargo fmt**: Rustコードの自動フォーマットに使用され、コードスタイルの一貫性を保ちます。
  - **Cargo clippy**: RustコードのLintチェックに使用され、潜在的なバグや非効率なコードを検出します。
  - **Cargo audit**: 依存関係のセキュリティ脆弱性をチェックします。
  - **Biome**: `demo-library`ディレクトリにおけるTypeScript/JavaScriptコードのフォーマットとリンティングに使用されます。

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
- **.gitattributes**: Gitがファイルをどのように扱うかを定義する設定ファイルです。
- **.gitignore**: Gitが追跡しないファイルやディレクトリを指定する設定ファイルです。
- **Cargo.lock**: Rustプロジェクトの依存関係の正確なバージョンを記録するファイルです。
- **Cargo.toml**: Rustプロジェクトのメタデータ（名前、バージョン、依存関係など）を定義するマニフェストファイルです。
- **LICENSE**: プロジェクトのライセンス情報が記載されています。
- **README.ja.md**: プロジェクトの日本語での説明、使い方、特徴などが記載されています。
- **README.md**: プロジェクトの英語での説明、使い方、特徴などが記載されています。
- **WASM_USAGE.md**: WebAssembly (WASM) の使用方法に関する詳細なドキュメントです。
- **_config.yml**: GitHub Pagesなどのサイト設定ファイルです。
- **googled947dc864c270e07.html**: Googleサイト認証用のファイルです。
- **package-lock.json**: Node.jsプロジェクトの依存関係の正確なバージョンを記録するファイルです。
- **package.json**: Node.jsプロジェクトのメタデータと依存関係を定義するファイルです。
- **demo-library/**: WebAssembly版のライブラリデモアプリケーションのソースコードと関連ファイルが格納されています。
    - **.gitignore**: `demo-library`ディレクトリ内のGit追跡対象外ファイルを指定します。
    - **biome.json**: Biome (JavaScript/TypeScriptのLinter/Formatter) の設定ファイルです。
    - **delay-vibrato-demo.ts**: ディレイビブラートのデモロジックを実装したTypeScriptファイルです。
    - **delay-vibrato.html**: ディレイビブラートのデモページのHTML構造を定義します。
    - **globals.d.ts**: グローバルに定義される型宣言ファイルです。
    - **index.html**: メインのライブラリデモページのHTML構造を定義します。
    - **library-demo.ts**: ライブラリデモの主要なロジックを実装したTypeScriptファイルです。
    - **log-visualizer-lfo.ts**: LFO（低周波発振器）イベントのログを可視化するロジックを実装しています。
    - **log-visualizer-note-segments.ts**: ノートセグメント（音符の開始から終了まで）のログを可視化するロジックを実装しています。
    - **log-visualizer-pitch-canvas.ts**: ピッチ情報のキャンバス描画ロジックを実装しています。
    - **log-visualizer.ts**: YM2151レジスタ書き込みログ全体の可視化ロジックを実装しています。
    - **mml-support.ts**: MML（Music Macro Language）関連のサポート機能を提供します。
    - **package-lock.json**: Node.jsプロジェクトの依存関係の正確なバージョンを記録するファイルです。
    - **package.json**: Node.jsプロジェクトのメタデータと依存関係を定義するファイルです。
    - **pop-noise-demo.ts**: ポップノイズ対策のデモロジックを実装したTypeScriptファイルです。
    - **pop-noise-detector.ts**: ポップノイズの検出ロジックを実装したTypeScriptファイルです。
    - **pop-noise.html**: ポップノイズ対策のデモページのHTML構造を定義します。
    - **portamento-soft-lfo-demo.ts**: ポルタメントとソフトLFOのデモロジックを実装したTypeScriptファイルです。
    - **portamento-soft-lfo.html**: ポルタメントとソフトLFOのデモページのHTML構造を定義します。
    - **random-tone.ts**: ランダムなYM2151音色を生成・操作するロジックを実装しています。
    - **shared-demo.ts**: デモアプリケーション間で共有される共通のユーティリティ機能を提供します。
    - **style.css**: デモページのスタイルシートです。
    - **tone-interpolation-demo.ts**: 音色補間のデモロジックを実装したTypeScriptファイルです。
    - **tone-interpolation.html**: 音色補間のデモページのHTML構造を定義します。
    - **tone-json-attachment.ts**: 音色JSONの添付データ処理に関するロジックを実装しています。
    - **tone-json-demo.ts**: 音色JSONの使用デモロジックを実装したTypeScriptファイルです。
    - **tone-json-mml.ts**: 音色JSONとMMLの連携に関するロジックを実装しています。
    - **tone-json.html**: 音色JSONの使用デモページのHTML構造を定義します。
    - **tsconfig.json**: TypeScriptコンパイラの設定ファイルです。
    - **vite.config.ts**: Viteビルドツールの設定ファイルです。
    - **wav-exporter.ts**: WAVファイルをエクスポートするロジックを実装しています。
    - **waveform-canvas.ts**: 波形描画キャンバスのロジックを実装しています。
    - **waveform-viewer.ts**: 波形ビューアのロジックを実装しています。
    - **ym2151-utils.ts**: YM2151関連のユーティリティ関数を提供します。
- **generated-docs/**: rustdocによって生成されたAPIドキュメントなどが格納されるディレクトリです。
- **issue-notes/**: 開発中の課題や検討事項に関するメモが格納されています。
- **src/**: Rustの主要なソースコードが格納されているディレクトリです。
    - **api.rs**: プロジェクトのパブリックAPIを定義しています。
    - **error.rs**: カスタムエラー型とエラーハンドリングロジックを定義しています。
    - **lib.rs**: Rustライブラリのエントリポイントであり、主要なモジュールを公開しています。
    - **main.rs**: コマンドラインアプリケーションのエントリポイントです。
    - **wasm.rs**: WebAssemblyにエクスポートされる関数とロジックを定義しています。
    - **midi/**: MIDIファイル解析に関連するモジュール群です。
        - **events.rs**: MIDIイベントのデータ構造を定義しています。
        - **mod.rs**: `midi`モジュールの定義ファイルです。
        - **parser.rs**: Standard MIDI Files (SMF) をパースするロジックを実装しています。
        - **utils.rs**: MIDIデータ処理に関するユーティリティ関数を提供しています。
        - **utils_tests.rs**: `midi/utils.rs`のテストコードです。
    - **options/**: 変換オプションに関するモジュール群です。
        - **attachments.rs**: カスタム音色などの添付データを処理するロジックを実装しています。
        - **effects.rs**: 変換時に適用されるエフェクトに関連するオプションを定義しています。
        - **mod.rs**: `options`モジュールの定義ファイルです。
        - **tests.rs**: `options`モジュール内のテストコードです。
    - **ym2151/**: YM2151レジスタログ変換に関連するモジュール群です。
        - **channel_allocation.rs**: MIDIチャンネルからYM2151チャンネルへの割り当てロジックを実装しています。
        - **converter.rs**: YM2151レジスタログへの変換を行う主要なロジックを実装しています。
        - **event_processor.rs**: YM2151イベントを処理するロジックを実装しています。
        - **events.rs**: YM2151関連のイベントデータ構造を定義しています。
        - **init.rs**: YM2151の初期化設定に関するロジックを実装しています。
        - **mod.rs**: `ym2151`モジュールの定義ファイルです。
        - **note_table.rs**: YM2151のノート周波数テーブルを定義しています。
        - **tempo_map.rs**: テンポ変更を含む時間管理ロジックを実装しています。
        - **tone.rs**: YM2151音色のデータ構造と関連ロジックを定義しています。
        - **converter/**: YM2151変換の詳細なサブモジュール群です。
            - **event_accumulator.rs**: YM2151イベントの蓄積と処理に関するロジックを実装しています。
            - **pitch_effects.rs**: ピッチ関連のエフェクト処理ロジックを実装しています。
            - **register_fields.rs**: YM2151レジスタの各フィールド定義を扱っています。
            - **waveform.rs**: YM2151の波形生成に関連するロジックです。
            - **register_effects/**: YM2151レジスタへの様々なエフェクト適用ロジックです。
                - **common.rs**: 共通のエフェクト処理を提供します。
                - **mod.rs**: `register_effects`モジュールの定義ファイルです。
                - **pop_noise.rs**: ポップノイズ対策のためのレジスタ操作ロジックです。
                - **register_lfo.rs**: LFOレジスタの制御ロジックを実装しています。
                - **state_cache.rs**: レジスタの状態キャッシュ管理ロジックです。
                - **tone_interpolation.rs**: 音色補間に関するロジックです。
        - **converter_tests/**: YM2151コンバータのテストケース群です。
            - **attachments.rs**: 添付データ（カスタム音色など）に関するテストです。
            - **attachments_change_to_next_tone/**: 複雑な添付データ変更に関するテストのサブディレクトリです。
                - **guards.rs**: 添付データ変更時のガード条件に関するテストです。
                - **interpolation.rs**: 添付データ変更時の補間に関するテストです。
                - **keep_fields.rs**: 添付データ変更時にフィールドを保持する挙動に関するテストです。
                - **mod.rs**: `attachments_change_to_next_tone`モジュールの定義ファイルです。
            - **attachments_program_effects.rs**: プログラムチェンジとエフェクトに関するテストです。
            - **basic.rs**: 基本的な変換機能のテストです。
            - **channels.rs**: チャンネル割り当てに関するテストです。
            - **drums.rs**: ドラムチャンネルの処理に関するテストです。
            - **effects.rs**: 一般的なエフェクト処理に関するテストです。
            - **lfo.rs**: LFO処理に関するテストです。
            - **portamento.rs**: ポルタメント処理に関するテストです。
            - **programs.rs**: プログラムチェンジ（音色切り替え）に関するテストです。
        - **converter_tests.rs**: `ym2151`モジュールのコンバータの総合テストファイルです。
        - **event_processor_tests.rs**: YM2151イベントプロセッサのテストコードです。
- **tests/**: 統合テストとテストデータが格納されています。
    - **create_test_midi.py**: テスト用のMIDIファイルを生成するためのPythonスクリプトです。
    - **integration_conversion.rs**: 変換処理全体の統合テストです。
    - **integration_midi.rs**: MIDIパーサーの統合テストです。
    - **integration_multichannel.rs**: マルチチャンネルMIDIの統合テストです。
    - **integration_program_change.rs**: プログラムチェンジ機能の統合テストです。
    - **integration_public_api.rs**: 公開APIの統合テストです。
    - **integration_wasm.rs**: WebAssembly版の統合テストです。
    - **test_data/**: 統合テストで使用されるMIDIサンプルデータが格納されています。
- **tones/**: カスタムYM2151音色定義のJSONファイルが格納されています。
    - **000.json**: プログラム0番のデフォルト音色定義です。
    - **README.md**: `tones`ディレクトリ内のカスタム音色ファイルのフォーマットに関する説明です。

## 関数詳細説明
- **computeHash (demo-library/delay-vibrato-demo.ts)**: 入力データからハッシュ値を計算する機能を提供します。
- **nextRequestId ()**: 次のリクエストIDを生成し、一意な操作を識別します。
- **isLatestRequest ()**: 現在のリクエストが最新のものであるかを確認し、古いリクエストの結果が適用されないようにします。
- **updateOutputWithState ()**: 現在の状態（変換結果など）に基づいてWeb UIの表示を更新します。
- **updatePlayButtonState ()**: 再生ボタンの有効/無効状態や表示テキストを更新します。
- **initializeWasm ()**: WebAssemblyモジュールを初期化し、Rustで書かれた機能をJavaScriptから利用できるようにします。
- **readAttachmentBytes ()**: UIから提供された添付ファイル（例: カスタム音色JSON）のデータをバイト列として読み込みます。
- **runConversion ()**: MIDIファイルをYM2151レジスタログに変換するコア処理を実行します。
- **handlePlay ()**: UIからの再生イベントを処理し、変換結果のオーディオ再生を開始します。
- **setupAttachmentEditor ()**: 添付データ（音色JSONなど）を編集するためのUIコンポーネントを初期化・設定します。
- **setupMmlInput ()**: MML（Music Macro Language）を入力するためのUIフィールドを初期化・設定します。
- **setupMidiInput ()**: MIDIファイルをアップロードするためのUI入力要素を初期化・設定します。
- **bootstrapWebYm ()**: WebAssemblyベースのYM2151関連機能（オーディオエンジンなど）を起動します。
- **applyRandomToneToAttachment ()**: ランダムに生成された音色パラメータを添付データに適用します。
- **setupRandomToneButton ()**: ランダム音色生成ボタンのUIを初期化・設定します。
- **main ()**: デモアプリケーションの主要なエントリポイントとして、初期設定やイベントハンドラを設定します。
- **playAudioWithOverlay (globals.d.ts)**: オーバーレイ表示を伴いながらオーディオを再生する機能を提供します。
- **clearAudioCache (globals.d.ts)**: 再生用にキャッシュされたオーディオデータをクリアします。
- **generateAudioFromJson (globals.d.ts)**: YM2151レジスタログのJSONデータからオーディオ波形を生成します。
- **initWasm (demo-library/library-demo.ts)**: WebAssemblyモジュールの初期化を行います。
- **displayResult (demo-library/library-demo.ts)**: 変換や処理の結果をWeb UIに表示します。
- **showError (demo-library/library-demo.ts)**: エラーメッセージをWeb UIに表示します。
- **setupFileInput (demo-library/library-demo.ts)**: ファイル入力（`input type="file"`）要素のイベントハンドラを設定します。
- **resolveRegisterForChannel (demo-library/log-visualizer-lfo.ts)**: 特定のYM2151チャンネルに対応するレジスタアドレスを解決します。
- **collectLfoEvents (demo-library/log-visualizer-lfo.ts)**: YM2151ログからLFO（低周波発振器）関連のイベントを収集します。
- **renderLfoLane (demo-library/log-visualizer-lfo.ts)**: ログビジュアライザー内でLFOの活動を示すレーンを描画します。
- **keyOnTimeKey (demo-library/log-visualizer-note-segments.ts)**: ノートオンイベントの時間に基づいたキーを生成します。
- **buildNoteSegments (demo-library/log-visualizer-note-segments.ts)**: 音楽的なノートの開始と終了を示すセグメントを構築します。
- **notePitch (demo-library/log-visualizer-note-segments.ts)**: ノートのピッチ情報を取得します。
- **computePitchRange (demo-library/log-visualizer-note-segments.ts)**: 表示するピッチの範囲を計算します。
- **noteYPosition (demo-library/log-visualizer-note-segments.ts)**: ノートの表示Y座標を計算します。
- **renderPitchCanvas (demo-library/log-visualizer-pitch-canvas.ts)**: ピッチ情報をグラフィカルに表示するためのキャンバスを描画します。
- **detectChannel (demo-library/log-visualizer.ts)**: YM2151ログイベントから使用されているチャンネルを検出します。
- **normalizeEvents (demo-library/log-visualizer.ts)**: イベントデータを可視化に適した形式に正規化します。
- **laneColor (demo-library/log-visualizer.ts)**: 各レーン（チャンネル）に対応する色を決定します。
- **createLane (demo-library/log-visualizer.ts)**: ログビジュアライザー内の個々のレーンを作成します。
- **computeTrackWidth (demo-library/log-visualizer.ts)**: 可視化トラック全体の幅を計算します。
- **formatInactiveChannels (demo-library/log-visualizer.ts)**: 非アクティブなチャンネルの表示をフォーマットします。
- **createLogVisualizer (demo-library/log-visualizer.ts)**: YM2151レジスタログを視覚的に表示するコンポーネントを生成します。
- **renderEmpty (demo-library/log-visualizer.ts)**: 空のログビジュアライザーを描画します。
- **renderFromJson (demo-library/log-visualizer.ts)**: YM2151ログのJSONデータからビジュアライザーを描画します。
- **ensureGlobalLane (demo-library/log-visualizer.ts)**: グローバルな可視化レーンが確実に存在するようにします。
- **setLfoRegisters (demo-library/log-visualizer.ts)**: LFOに関連するレジスタの値を設定します。
- **setupMmlToSmf (demo-library/mml-support.ts)**: MMLをSMFに変換する機能のセットアップを行います。
- **updateRegisterReflectionStatus (demo-library/pop-noise-demo.ts)**: レジスタ反映のステータス表示を更新します。
- **countRegisterNormalizationTargets (demo-library/pop-noise-demo.ts)**: レジスタ正規化の対象となる数をカウントします。
- **setupPlayButton (demo-library/pop-noise-demo.ts)**: 再生ボタンのUIとイベントハンドラを設定します。
- **setupWavExportButton (demo-library/pop-noise-demo.ts)**: WAVエクスポートボタンのUIとイベントハンドラを設定します。
- **bootstrap (demo-library/pop-noise-demo.ts)**: アプリケーションの初期起動と主要なコンポーネントのセットアップを行います。
- **detectPopNoise (demo-library/pop-noise-detector.ts)**: 生成された波形データからポップノイズの発生を検出します。
- **extractLfoRegistersFromAttachment (demo-library/portamento-soft-lfo-demo.ts)**: 添付データからLFOレジスタの設定を抽出します。
- **syncLfoRegisters (demo-library/portamento-soft-lfo-demo.ts)**: 複数のLFOレジスタ設定を同期させます。
- **getToneEditorGenerator (demo-library/random-tone.ts)**: 音色エディタ用のランダム音色生成機能を提供します。
- **generateRandomToneRegisters (demo-library/random-tone.ts)**: YM2151のランダムな音色レジスタ設定を生成します。
- **generateRandomInterpolationPairRegisters (demo-library/random-tone.ts)**: 音色補間用のランダムなレジスタペアを生成します。
- **parseAttachmentEntries (demo-library/random-tone.ts)**: 添付データ内のエントリをパース（解析）します。
- **validateRandomToneAttachment (demo-library/random-tone.ts)**: ランダム音色添付データの有効性を検証します。
- **upsertEntryRegisters (demo-library/random-tone.ts)**: 特定のエントリのレジスタ設定を挿入または更新します。
- **upsertAttachmentRegisters (demo-library/random-tone.ts)**: 添付データ全体にレジスタ設定を挿入または更新します。
- **upsertInterpolationAttachmentRegisters (demo-library/random-tone.ts)**: 補間添付データにレジスタ設定を挿入または更新します。
- **buildRandomInterpolationAttachment (demo-library/random-tone.ts)**: ランダムな音色補間添付データを構築します。
- **ensureWasmInitialized (demo-library/shared-demo.ts)**: WebAssemblyモジュールが初期化されていることを確認し、必要に応じて初期化します。
- **setStatus (demo-library/shared-demo.ts)**: アプリケーションのステータスメッセージを更新します。
- **setEventCountDisplay (demo-library/shared-demo.ts)**: 処理されたイベントの数を表示するUIを更新します。
- **ensureWebYm2151 (demo-library/shared-demo.ts)**: WebYM2151のインスタンスが利用可能であることを確認し、必要に応じて準備します。
- **clearWebYmAudioCache (demo-library/shared-demo.ts)**: WebYM2151のオーディオ再生キャッシュをクリアします。
- **updateOutput (demo-library/shared-demo.ts)**: 変換結果やログなどの出力エリアを更新します。
- **parseAttachmentField (demo-library/shared-demo.ts)**: UI上の添付データ入力フィールドの内容を解析します。
- **cleanup (demo-library/shared-demo.ts)**: リソースの解放など、クリーンアップ処理を実行します。
- **mod (demo-library/shared-demo.ts)**: モジュロ演算（剰余計算）を行います。
- **buildEventsFromCompact (demo-library/tone-json-attachment.ts)**: コンパクトな形式のJSONデータからYM2151イベントを構築します。
- **serializeWithStatus (demo-library/tone-json-attachment.ts)**: 現在のステータス情報を含めてデータをシリアライズ（直列化）します。
- **normalizeAttachmentText (demo-library/tone-json-attachment.ts)**: 添付テキストの形式を正規化します。
- **convertMmlToSmf (demo-library/tone-json-demo.ts)**: MMLデータをStandard MIDI File (SMF) に変換します。
- **getMmlParser (demo-library/tone-json-mml.ts)**: MMLパーサーのインスタンスを取得します。
- **getParseTreeJsonToSmf (demo-library/tone-json-mml.ts)**: パースツリーのJSONからSMFへの変換ロジックを取得します。
- **treeToJson (demo-library/tone-json-mml.ts)**: MMLのパースツリーをJSON形式に変換します。
- **ensureMmlRuntime (demo-library/tone-json-mml.ts)**: MMLランタイムが利用可能であることを確認し、必要に応じてロードします。
- **encodeWav (demo-library/wav-exporter.ts)**: 生のオーディオデータをWAVファイル形式にエンコードします。
- **writeAscii (demo-library/wav-exporter.ts)**: ASCII文字列をバイナリデータとして書き込みます。
- **downloadWav (demo-library/wav-exporter.ts)**: 生成されたWAVファイルをユーザーのデバイスにダウンロードします。
- **drawEmpty (demo-library/waveform-canvas.ts)**: 波形表示キャンバスに何も表示されていない状態を描画します。
- **drawWaveform (demo-library/waveform-canvas.ts)**: オーディオ波形データをキャンバスに描画します。
- **parseHexByte (demo-library/ym2151-utils.ts)**: 16進数文字列をバイト値にパースします。
- **extractNoteBoundaries (demo-library/waveform-viewer.ts)**: 波形データからノートの開始と終了の境界を抽出します。
- **normalizeAmplitude (demo-library/waveform-viewer.ts)**: 波形の振幅を正規化し、表示に適した範囲に調整します。
- **createWaveformViewer (demo-library/waveform-viewer.ts)**: オーディオ波形を視覚的に表示するビューアコンポーネントを生成します。
- **getWindowDurS (demo-library/waveform-viewer.ts)**: 現在表示されているウィンドウの持続時間を秒単位で取得します。
- **clampViewStart (demo-library/waveform-viewer.ts)**: 表示開始位置が有効な範囲に収まるように調整（クランプ）します。
- **updatePositionLabel (demo-library/waveform-viewer.ts)**: 波形ビューアの現在位置を示すラベルを更新します。
- **render (demo-library/waveform-viewer.ts)**: 波形ビューアの現在の状態を描画します。
- **updateBoundariesAndRender (demo-library/waveform-viewer.ts)**: 境界情報を更新し、それに基づいて波形ビューアを再描画します。
- **synthesizeAndRender (demo-library/waveform-viewer.ts)**: 音声を合成し、その結果を波形ビューアに描画します。
- **setZoom (demo-library/waveform-viewer.ts)**: 波形ビューアのズームレベルを設定します。
- **endDrag (demo-library/waveform-viewer.ts)**: 波形ビューアでのドラッグ操作が終了した際の処理を行います。
- **exportWav (demo-library/waveform-viewer.ts)**: 現在表示されている波形データをWAVファイルとしてエクスポートします。

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
    - applyRandomToneToAttachment ()
    - setupRandomToneButton ()
    - main ()
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
- mod (demo-library/shared-demo.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-05-17 07:21:43 JST

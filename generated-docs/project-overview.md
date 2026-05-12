Last updated: 2026-05-13

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製ツールです。
- ネイティブアプリケーションとWebAssembly（ブラウザ）環境の両方で、FM音源の音を制御するための強力なライブラリとして機能します。
- 高度なチャンネル割り当て戦略とプログラムチェンジ対応により、MIDIデータのYM2151向け変換を効率的かつ正確に行います。

## 技術スタック
- フロントエンド:
    - **HTML/CSS/TypeScript**: デモライブラリのウェブUIとロジックを構築するための基本技術。
    - **Vite**: デモライブラリの高速な開発体験と最適化されたビルドを提供するフロントエンドビルドツール。
    - **WebAssembly (WASM)**: Rustで書かれた変換ロジックをウェブブラウザ上で実行可能にする技術。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: プロジェクトの入力フォーマットとして、標準的なMIDIデータを扱います。
    - **YM2151**: ターゲットとなるFM音源チップ。そのレジスタ設定をJSON形式で出力します。
    - **JSON**: YM2151レジスタ書き込みログの出力形式、およびカスタム音色定義ファイルの形式として使用されます。
- 開発ツール:
    - **Rust**: メインの開発言語であり、型安全性と高性能を特徴とします。
    - **Cargo**: Rustの公式なビルドシステムおよびパッケージマネージャー。依存関係管理、ビルド、テスト、ドキュメント生成などを行います。
    - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、JavaScriptとの連携を容易にするためのツール。
    - **Biome**: デモライブラリのTypeScript/JavaScriptコードに対して、フォーマットとリンティングを適用し、コード品質を統一します。
- テスト:
    - **`cargo test`**: Rustに組み込まれた単体テストおよび統合テストの実行フレームワーク。
    - **`cargo tarpaulin`**: Rustコードのテストカバレッジを測定し、レポートを生成するツール。
- ビルドツール:
    - **Cargo**: Rustプロジェクト全体のビルドを管理します。
    - **wasm-pack**: WebAssemblyモジュールをビルドします。
    - **Vite**: デモライブラリ（JavaScript/TypeScript）のビルドと開発サーバーを提供します。
- 言語機能:
    - **Rust**: 高いパフォーマンス、メモリ安全性、並行性を保証するシステムプログラミング言語。その堅牢な型システムがプロジェクトの信頼性を高めます。
- 自動化・CI/CD:
    - (特筆すべき自動化・CI/CDツールは明示されていません)
- 開発標準:
    - **`cargo fmt`**: Rustコードの自動フォーマッター。コードの一貫性を保ちます。
    - **`cargo clippy`**: Rustのリンター。一般的なエラーや非効率なコードパターンを指摘し、品質向上を支援します。
    - **`cargo audit`**: Rustプロジェクトの依存関係における既知のセキュリティ脆弱性をチェックします。

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
- **`.gitattributes`**: Gitリポジトリで特定のファイルタイプに対する属性（例: 改行コードの扱い）を定義するファイルです。
- **`.gitignore`**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定するファイルです。
- **`Cargo.lock`**: RustのパッケージマネージャーCargoが、ビルドに使用される依存クレートの正確なバージョンとチェックサムを記録するファイルです。
- **`Cargo.toml`**: Rustプロジェクトのビルド設定、メタデータ、および依存関係を定義するマニフェストファイルです。
- **`LICENSE`**: プロジェクトのライセンス情報が記述されています。
- **`README.ja.md`**: プロジェクトの日本語での概要と使用方法を説明するドキュメントです。
- **`README.md`**: プロジェクトの英語での概要と使用方法を説明するドキュメントです。
- **`WASM_USAGE.md`**: WebAssembly (WASM) としてこのライブラリを使用する方法に関する詳細な説明ドキュメントです。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルで、デモページの生成に使用されます。
- **`demo-library/`**: WebAssemblyで変換されたYM2151ログをブラウザで視覚化・再生するためのデモコードを格納するディレクトリです。
    - **`biome.json`**: TypeScript/JavaScriptコードのフォーマットとリンティングルールを定義する設定ファイルです。
    - **`delay-vibrato-demo.ts`**: ディレイビブラート機能の動作を示すデモロジックを実装したTypeScriptファイルです。
    - **`delay-vibrato.html`**: ディレイビブラートデモのユーザーインターフェースを提供するウェブページです。
    - **`globals.d.ts`**: デモで使用されるグローバルな型定義（例: WebオーディオAPI関連）を宣言するファイルです。
    - **`index.html`**: メインのライブラリデモのウェブページです。MIDIファイル入力からYM2151ログへの変換、可視化までを体験できます。
    - **`library-demo.ts`**: メインライブラリデモのJavaScriptロジックで、ファイル入力の処理、WASMモジュールの初期化、結果の表示を担当します。
    - **`log-visualizer-lfo.ts`**: YM2151ログ内のLFO（低周波発振器）イベントを抽出し、可視化するためのロジックを提供します。
    - **`log-visualizer-note-segments.ts`**: MIDIノートイベントからYM2151のピッチ情報を分析し、ノートの表示セグメントを構築するロジックです。
    - **`log-visualizer-pitch-canvas.ts`**: ピッチ情報をグラフィカルに描画するためのキャンバスレンダリングロジックです。
    - **`log-visualizer.ts`**: YM2151レジスタ書き込みログを視覚的に表示するメインの可視化ロジックが含まれています。
    - **`mml-support.ts`**: MML（Music Macro Language）形式の入力をStandard MIDI File（SMF）に変換する機能をサポートするためのコードです。
    - **`package-lock.json`**: Node.jsプロジェクトの依存関係ツリーとバージョンを正確に記録するファイルです。
    - **`package.json`**: Node.jsプロジェクトのメタデータ、依存関係、スクリプトを定義するマニフェストファイルです。
    - **`pop-noise-demo.ts`**: ポップノイズ検出と対策に関するデモのロジックを実装したTypeScriptファイルです。
    - **`pop-noise-detector.ts`**: YM2151ログイベントからオーディオのポップノイズを引き起こす可能性のあるパターンを検出するロジックです。
    - **`pop-noise.html`**: ポップノイズデモのユーザーインターフェースを提供するウェブページです。
    - **`portamento-soft-lfo-demo.ts`**: ポルタメントソフトLFO（滑らかなピッチ変化とLFO）機能のデモロジックを実装したTypeScriptファイルです。
    - **`portamento-soft-lfo.html`**: ポルタメントソフトLFOデモのユーザーインターフェースを提供するウェブページです。
    - **`random-tone.ts`**: ランダムなYM2151音色を生成し、デモに適用するためのロジックを提供します。
    - **`shared-demo.ts`**: 複数のデモページ間で共有されるユーティリティ関数やWASMモジュールの初期化ロジックなどを集約したファイルです。
    - **`style.css`**: デモページの全体的なレイアウトとデザインを定義するスタイルシートです。
    - **`tone-interpolation-demo.ts`**: YM2151音色の補間機能の動作を示すデモロジックを実装したTypeScriptファイルです。
    - **`tone-interpolation.html`**: 音色補間デモのユーザーインターフェースを提供するウェブページです。
    - **`tone-json-attachment.ts`**: コンパクトなJSON形式の音色定義をYM2151レジスタイベントに変換したり、シリアライズしたりするロジックです。
    - **`tone-json-demo.ts`**: JSON形式の音色定義を扱うデモのロジックを実装したTypeScriptファイルです。
    - **`tone-json-mml.ts`**: MMLからJSON形式の音色定義を生成するためのパーサー関連ロジックです。
    - **`tone-json.html`**: JSON音色デモのユーザーインターフェースを提供するウェブページです。
    - **`tsconfig.json`**: TypeScriptコンパイラのオプションとプロジェクトの設定を定義するファイルです。
    - **`vite.config.ts`**: Viteビルドツール専用の設定ファイルで、デモプロジェクトのビルド挙動を制御します。
    - **`wav-exporter.ts`**: 生成されたYM2151オーディオデータをWAVファイル形式にエンコードし、ダウンロードするためのロジックです。
    - **`waveform-canvas.ts`**: 音声波形をHTML Canvas要素に描画するための低レベルなロジックを提供します。
    - **`waveform-viewer.ts`**: 合成されたYM2151オーディオの波形を表示するための高レベルなビューアコンポーネントを実装しています。
    - **`ym2151-utils.ts`**: YM2151関連の一般的なユーティリティ関数（例: 16進数文字列のパース）を定義します。
- **`generated-docs/`**: `cargo doc`コマンドによって生成されたAPIドキュメントなどが格納されるディレクトリです。
- **`googled947dc864c270e07.html`**: Googleサイト認証のために使用されるファイルです。
- **`issue-notes/`**: プロジェクト開発中に検討された課題や技術的なメモが記録されているディレクトリです。
- **`src/`**: Rustのソースコードが格納されている主要なディレクトリです。
    - **`api.rs`**: このライブラリの公開APIインターフェースを定義するファイルです。
    - **`error.rs`**: プロジェクト内で使用されるカスタムエラータイプを定義します。
    - **`lib.rs`**: Rustライブラリクレートのエントリポイントで、モジュールの宣言や公開アイテムを管理します。
    - **`main.rs`**: コマンドラインアプリケーションとしての実行エントリポイントです。
    - **`midi/`**: MIDIファイルのパースと処理に関連するモジュールです。
        - **`events.rs`**: MIDIイベントのデータ構造（ノートオン/オフ、テンポチェンジなど）を定義します。
        - **`mod.rs`**: `midi`モジュールのルートファイルです。
        - **`parser.rs`**: Standard MIDI Files (SMF) を読み込み、その内容を内部表現にパースするロジックを含みます。
        - **`utils.rs`**: MIDIデータ処理に役立つ各種ユーティリティ関数を提供します。
        - **`utils_tests.rs`**: `midi/utils.rs`で定義されたユーティリティ関数の単体テストコードです。
    - **`options/`**: YM2151変換のオプションやエフェクト定義に関連するモジュールです。
        - **`attachments.rs`**: 外部ファイルから提供されるカスタム音色データなどの「添付」情報を処理するロジックです。
        - **`effects.rs`**: YM2151ログ変換中に適用されるLFOやポルタメントなどのエフェクトの定義を含みます。
        - **`mod.rs`**: `options`モジュールのルートファイルです。
        - **`tests.rs`**: `options`モジュール内の機能の単体テストコードです。
    - **`wasm.rs`**: WebAssembly (WASM) バインディングを実装しており、Rustの関数をJavaScriptから呼び出せるようにします。
    - **`ym2151/`**: YM2151 FM音源チップ固有のロジック、変換、および関連イベントを管理するモジュールです。
        - **`channel_allocation.rs`**: YM2151の限られた8チャンネルをMIDIチャンネルの和音数に基づいて割り当てるための複雑な戦略を実装します。
        - **`converter/`**: MIDIイベントをYM2151レジスタ書き込みログに変換する中核ロジックです。
            - **`event_accumulator.rs`**: MIDIイベントをタイムラインに沿って蓄積し、YM2151イベントに変換するための中間データ構造を扱います。
            - **`pitch_effects.rs`**: ピッチベンドやLFOによるピッチ調整など、YM2151のピッチ関連レジスタに影響を与えるエフェクトを処理します。
            - **`register_effects/`**: YM2151の特定レジスタに作用する低レベルなエフェクトロジックを格納します。
                - **`common.rs`**: レジスタエフェクトで共通して使用されるユーティリティ関数を提供します。
                - **`mod.rs`**: `register_effects`モジュールのルートファイルです。
                - **`pop_noise.rs`**: 音切れ時などに発生しうるポップノイズを軽減するためのレジスタ操作ロジックを含みます。
                - **`register_lfo.rs`**: YM2151のLFOレジスタを制御し、ビブラートなどの効果を生成するロジックです。
                - **`state_cache.rs`**: YM2151のレジスタ値をキャッシュし、不要なレジスタ書き込みを省略して効率化を図ります。
                - **`tone_interpolation.rs`**: 2つのYM2151音色間でパラメータを補間し、滑らかな音色変化を実現するロジックです。
            - **`register_fields.rs`**: YM2151の各レジスタフィールドのビット構成と意味を定義し、読み書きを抽象化します。
            - **`waveform.rs`**: YM2151の波形生成に関連するロジックです。
        - **`converter.rs`**: YM2151ログへの変換処理の中心となるファイルで、全体のワークフローを調整します。
        - **`converter_tests/`**: YM2151コンバータの機能に関する詳細な単体テストが格納されています。
            - **`attachments.rs`**: 音色アタッチメント機能に関するテストです。
            - **`attachments_change_to_next_tone/`**: 音色変更時のアタッチメント挙動に関するテストのサブディレクトリです。
                - **`guards.rs`**: 音色変更時のガードロジックのテストです。
                - **`interpolation.rs`**: 音色補間に関するテストです。
                - **`keep_fields.rs`**: 音色変更時に特定のフィールドが保持されるかのテストです。
                - **`mod.rs`**: `attachments_change_to_next_tone`モジュールのルートファイルです。
            - **`attachments_program_effects.rs`**: プログラムチェンジとエフェクトのアタッチメントテストです。
            - **`basic.rs`**: コンバータの基本的な変換ロジックに関するテストです。
            - **`channels.rs`**: チャンネル割り当て戦略に関するテストです。
            - **`drums.rs`**: ドラムチャンネルの優先割り当てなど、ドラム関連のテストです。
            - **`effects.rs`**: 各種YM2151エフェクトの適用に関するテストです。
            - **`lfo.rs`**: LFO（低周波発振器）機能に関するテストです。
            - **`portamento.rs`**: ポルタメント（滑らかなピッチ移動）機能に関するテストです。
            - **`programs.rs`**: MIDIプログラムチェンジイベントによる音色切り替えに関するテストです。
        - **`converter_tests.rs`**: `ym2151`モジュール全体のコンバータテストを統合するファイルです。
        - **`event_processor.rs`**: YM2151のレジスタ書き込みイベントを処理し、YM2151チップの状態をシミュレートするロジックです。
        - **`event_processor_tests.rs`**: `event_processor.rs`の単体テストコードです。
        - **`events.rs`**: YM2151のレジスタイベントや関連するデータ構造を定義します。
        - **`init.rs`**: YM2151チップの初期状態設定に関するロジックです。
        - **`mod.rs`**: `ym2151`モジュールのルートファイルです。
        - **`note_table.rs`**: MIDIノート番号とYM2151の周波数設定値（FN/BLOCK）のマッピングテーブルを定義します。
        - **`tempo_map.rs`**: MIDIファイル内のテンポイベントを管理し、イベント時刻をティックから実時間へ変換する機能を提供します。
        - **`tone.rs`**: YM2151の音色データ構造とその処理ロジックを定義します。
- **`tests/`**: プロジェクト全体の統合テストが格納されているディレクトリです。
    - **`create_test_midi.py`**: 統合テストで使用するためのテスト用MIDIファイルを生成するPythonスクリプトです。
    - **`integration_conversion.rs`**: MIDIからYM2151ログへの変換プロセス全体の統合テストです。
    - **`integration_midi.rs`**: MIDIファイルパーシング機能の統合テストです。
    - **`integration_multichannel.rs`**: マルチチャンネルMIDI入力が正しく処理されるかを確認する統合テストです。
    - **`integration_program_change.rs`**: プログラムチェンジ機能が統合された状態で正しく動作するかを確認するテストです。
    - **`integration_public_api.rs`**: ライブラリの公開APIが期待通りに機能するかを確認する統合テストです。
    - **`integration_wasm.rs`**: WebAssemblyビルドが機能的に正しく動作するかを確認する統合テストです。
    - **`test_data/`**: 統合テストで使用されるサンプルMIDIファイルなどのデータが格納されています。
- **`tones/`**: カスタムYM2151音色定義をJSON形式で格納するディレクトリです。プログラムチェンジイベントでロードされます。
    - **`000.json`**: プログラム番号0番に割り当てられたデフォルトのYM2151音色定義です。
    - **`README.md`**: `tones`ディレクトリ内のJSON音色ファイルのフォーマットと使用方法について説明するドキュメントです。

## 関数詳細説明
- **`computeHash(data: string)`**: (demo-library/delay-vibrato-demo.ts) 入力文字列のハッシュ値を計算します。主にリクエストの識別に使用されます。
- **`nextRequestId()`**: (demo-library/delay-vibrato-demo.ts) デモ内で非同期リクエストを追跡するためのユニークなリクエストIDを生成します。
- **`isLatestRequest(requestId: number)`**: (demo-library/delay-vibrato-demo.ts) 指定されたリクエストIDが現在アクティブな最新のリクエストであるかを判定します。
- **`updateOutputWithState(state: any)`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) デモのUI出力領域を、現在の状態オブジェクトに基づいて更新します。
- **`updatePlayButtonState(isPlaying: boolean)`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) UI上の再生ボタンの有効/無効状態や表示テキストを、オーディオ再生状況に応じて更新します。
- **`initializeWasm()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) WebAssemblyモジュールを非同期でロードし、初期化します。
- **`readAttachmentBytes(attachmentId: string)`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) 指定されたIDの添付ファイル（音色データなど）の内容をバイト配列として読み込みます。
- **`runConversion(midiBytes: Uint8Array, attachmentBytes: Uint8Array | null)`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) 提供されたMIDIバイトデータとオプションの添付ファイルバイトデータを使用して、YM2151レジスタログへの変換処理を実行します。
- **`handlePlay()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) UI上の再生ボタンが押された際に、MIDIデータの変換からYM2151ログの生成、そしてそのオーディオ再生までの一連の処理を調整します。
- **`setupAttachmentEditor()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) カスタム音色定義などの添付ファイルを編集するためのUI要素をセットアップします。
- **`setupMmlInput()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) MML（Music Macro Language）形式の音楽データを入力するためのUI要素をセットアップします。
- **`setupMidiInput()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) MIDIファイルをアップロードするためのUI要素とイベントハンドラをセットアップします。
- **`bootstrapWebYm()`**: (demo-library/delay-vibrato-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) WebAssemblyで動作するYM2151サウンドエンジンを初期化し、デモページの起動に必要な処理を行います。
- **`applyRandomToneToAttachment()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) 添付ファイルエディタに、ランダムに生成されたYM2151音色（レジスタ設定）を適用します。
- **`setupRandomToneButton()`**: (demo-library/delay-vibrato-demo.ts, pop-noise-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) ランダム音色生成ボタンのイベントリスナーを登録し、UIをセットアップします。
- **`main()`**: (demo-library/delay-vibrato-demo.ts, portamento-soft-lfo-demo.ts, tone-interpolation-demo.ts, tone-json-demo.ts) 各デモページの主要なエントリポイントとなる関数で、初期化処理やイベントハンドラの登録を行います。
- **`initWasm()`**: (demo-library/library-demo.ts) メインデモページでWebAssemblyモジュールを初期化する処理です。
- **`displayResult(result: any)`**: (demo-library/library-demo.ts) 変換結果やYM2151ログをUIに表示します。
- **`showError(error: Error)`**: (demo-library/library-demo.ts) UI上の指定された領域にエラーメッセージを表示します。
- **`setupFileInput()`**: (demo-library/library-demo.ts) ファイル入力要素を設定し、ユーザーがMIDIファイルをアップロードした際の処理を定義します。
- **`resolveRegisterForChannel(channel: number)`**: (demo-library/log-visualizer-lfo.ts) YM2151の指定されたチャンネルに対応するLFO関連レジスタアドレスを解決します。
- **`collectLfoEvents(events: any[])`**: (demo-library/log-visualizer-lfo.ts) YM2151ログイベントの中からLFO（低周波発振器）に関するイベントを収集します。
- **`renderLfoLane(context: CanvasRenderingContext2D, laneRect: DOMRect, lfoEvents: any[], maxTime: number)`**: (demo-library/log-visualizer-lfo.ts) LFOイベントのデータをキャンバス上に視覚的な「レーン」として描画します。
- **`buildNoteSegments(logEvents: any[])`**: (demo-library/log-visualizer-note-segments.ts) YM2151ログイベントを解析し、各ノートの開始・終了時刻、ピッチなどの情報をまとめたセグメントを構築します。
- **`notePitch(register: number, data: number)`**: (demo-library/log-visualizer-note-segments.ts) YM2151のレジスタアドレスとデータ値から、対応するノートのピッチ（MIDIノート番号相当）を計算します。
- **`computePitchRange(segments: any[])`**: (demo-library/log-visualizer-note-segments.ts) ノートセグメントのリスト全体から、最低ピッチと最高ピッチを算出してピッチ範囲を決定します。
- **`noteYPosition(pitch: number, minPitch: number, maxPitch: number, height: number)`**: (demo-library/log-visualizer-note-segments.ts) 指定されたピッチが、与えられたピッチ範囲と描画高さの中でどのY座標に位置するかを計算します。
- **`renderPitchCanvas(canvas: HTMLCanvasElement, segments: any[], minPitch: number, maxPitch: number)`**: (demo-library/log-visualizer-pitch-canvas.ts) ノートセグメントのピッチ情報をHTML Canvas要素に描画し、視覚化します。
- **`detectChannel(addr: number)`**: (demo-library/log-visualizer.ts) YM2151のレジスタアドレスから、それがどの音源チャンネル（オペレータ）に属するかを検出します。
- **`normalizeEvents(events: any[])`**: (demo-library/log-visualizer.ts) YM2151ログイベントのデータを視覚化に適した形式に正規化します。
- **`laneColor(channel: number)`**: (demo-library/log-visualizer.ts) 指定されたYM2151チャンネルに対応する、可視化用の色を返します。
- **`createLane(width: number, height: number, backgroundColor: string)`**: (demo-library/log-visualizer.ts) ログ可視化のための単一の「レーン」（チャンネルごとの表示エリア）となるHTML要素を作成します。
- **`computeTrackWidth(maxTime: number)`**: (demo-library/log-visualizer.ts) YM2151ログの総再生時間に基づいて、可視化トラックの最適な幅を計算します。
- **`formatInactiveChannels(channels: Set<number>)`**: (demo-library/log-visualizer.ts) ログ中にイベントがない（非アクティブな）YM2151チャンネルのリストを整形して表示します。
- **`createLogVisualizer(containerId: string)`**: (demo-library/log-visualizer.ts) 指定されたHTMLコンテナ内にYM2151ログ可視化ツールを初期化し、インスタンスを返します。
- **`renderEmpty()`**: (demo-library/log-visualizer.ts) ログデータがない場合に、空の可視化ビューをレンダリングします。
- **`renderFromJson(json: string, events?: any)`**: (demo-library/log-visualizer.ts, waveform-viewer.ts) JSON形式のYM2151ログデータを解析し、可視化または波形合成・描画を行います。
- **`ensureGlobalLane(channel: number)`**: (demo-library/log-visualizer.ts) LFOなど、特定のチャンネルに紐づかないグローバルなイベント表示レーンが存在することを確認し、必要に応じて作成します。
- **`setLfoRegisters(registers: string[])`**: (demo-library/log-visualizer.ts) 可視化ツールがLFOとして認識すべきレジスタアドレスのリストを設定します。
- **`updateRegisterReflectionStatus(enabled: boolean)`**: (demo-library/pop-noise-demo.ts) UI上のレジスタ反映（ノーマライズ）機能の有効/無効状態を更新します。
- **`countRegisterNormalizationTargets(attachmentText: string)`**: (demo-library/pop-noise-demo.ts) 添付ファイルテキスト内の、レジスタ正規化の対象となる設定の数をカウントします。
- **`setupPlayButton()`**: (demo-library/pop-noise-demo.ts) ポップノイズデモページの再生ボタンのUIとイベントハンドラをセットアップします。
- **`setupWavExportButton()`**: (demo-library/pop-noise-demo.ts) ポップノイズデモページのWAVエクスポートボタンのUIとイベントハンドラをセットアップします。
- **`bootstrap()`**: (demo-library/pop-noise-demo.ts) ポップノイズデモページの起動時に必要な初期化処理全般を実行します。
- **`detectPopNoise(events: any[], threshold: number)`**: (demo-library/pop-noise-detector.ts) YM2151ログイベントから、特定の閾値に基づいてポップノイズが発生しうる箇所を検出します。
- **`extractLfoRegistersFromAttachment(attachmentBytes: Uint8Array)`**: (demo-library/portamento-soft-lfo-demo.ts) 添付ファイルからLFOに関連するレジスタ設定を抽出し、解析します。
- **`syncLfoRegisters(lfoRegisters: string[])`**: (demo-library/portamento-soft-lfo-demo.ts) 抽出されたLFOレジスタの設定をデモの状態と同期させ、UIに反映します。
- **`getToneEditorGenerator(attachmentText: string)`**: (demo-library/random-tone.ts) 添付ファイルテキストを基に、音色エディタ用のランダム音色生成ロジックを取得します。
- **`generateRandomToneRegisters()`**: (demo-library/random-tone.ts) 新しいランダムなYM2151音色のレジスタ設定を生成します。
- **`generateRandomInterpolationPairRegisters()`**: (demo-library/random-tone.ts) 音色補間デモのために、ランダムな開始・終了音色のレジスタペアを生成します。
- **`parseAttachmentEntries(attachmentText: string)`**: (demo-library/random-tone.ts) 添付ファイルとして提供されるJSONテキストを解析し、音色のエントリーリストを構築します。
- **`validateRandomToneAttachment(attachmentText: string)`**: (demo-library/random-tone.ts) ランダム音色添付ファイルのJSON構造が有効であるかを検証します。
- **`upsertEntryRegisters(entries: any[], entryName: string, registers: any[])`**: (demo-library/random-tone.ts) 既存の音色エントリーリストに対して、指定されたエントリー名でレジスタ設定を挿入または更新します。
- **`upsertAttachmentRegisters(attachmentText: string, toneId: string, registers: any[])`**: (demo-library/random-tone.ts) 添付ファイルのJSONテキスト内の特定の音色IDに対して、レジスタ設定を挿入または更新し、更新されたテキストを返します。
- **`upsertInterpolationAttachmentRegisters(attachmentText: string, registers: any[])`**: (demo-library/random-tone.ts) 添付ファイルテキスト内の補間音色設定に対して、レジスタを挿入または更新します。
- **`buildRandomInterpolationAttachment(attachmentText: string)`**: (demo-library/random-tone.ts) 既存の添付ファイルテキストにランダムな音色補間設定を組み込んで、新しい添付ファイルテキストを生成します。
- **`ensureWasmInitialized()`**: (demo-library/shared-demo.ts) WebAssemblyモジュールがロードされ、初期化されていることを保証します。
- **`setStatus(message: string)`**: (demo-library/shared-demo.ts) UI上のステータス表示領域にメッセージを設定します。
- **`setEventCountDisplay(count: number)`**: (demo-library/shared-demo.ts) 処理されたYM2151イベントの数をUIに表示します。
- **`ensureWebYm2151()`**: (demo-library/shared-demo.ts) WebYM2151サウンドエンジンが利用可能であり、初期化されていることを保証します。
- **`clearWebYmAudioCache()`**: (demo-library/shared-demo.ts) WebYM2151サウンドエンジンが持つオーディオキャッシュをクリアします。
- **`updateOutput(data: string)`**: (demo-library/shared-demo.ts) 一般的なUI出力領域を更新します。
- **`parseAttachmentField(field: string)`**: (demo-library/shared-demo.ts) UI上の添付ファイル入力フィールドの値を解析します。
- **`cleanup()`**: (demo-library/shared-demo.ts) デモの実行終了時や状態リセット時に、リソースの解放などを行うクリーンアップ処理です。
- **`mod(n: number, m: number)`**: (demo-library/shared-demo.ts) JavaScriptの負の数にも対応したモジュロ演算（剰余計算）を行います。
- **`buildEventsFromCompact(compactTone: string)`**: (demo-library/tone-json-attachment.ts) コンパクトなJSON形式の音色定義から、詳細なYM2151レジスタイベントのリストを構築します。
- **`serializeWithStatus(data: any)`**: (demo-library/tone-json-attachment.ts) データをJSON文字列にシリアライズし、その処理の状態をUIに表示します。
- **`normalizeAttachmentText(text: string)`**: (demo-library/tone-json-attachment.ts) 添付ファイルとして提供されるJSONテキストからコメントなどを除去し、正規化します。
- **`convertMmlToSmf()`**: (demo-library/tone-json-demo.ts) MML入力データをStandard MIDI File (SMF) 形式に変換します。
- **`getMmlParser()`**: (demo-library/tone-json-mml.ts) MML（Music Macro Language）のテキストを解析するためのパーサーインスタンスを取得します。
- **`getParseTreeJsonToSmf()`**: (demo-library/tone-json-mml.ts) MMLのパースツリーからSMFを生成するためのロジックを取得します。
- **`treeToJson(tree: any)`**: (demo-library/tone-json-mml.ts) MMLのパースツリーをJSON形式に変換します。
- **`ensureMmlRuntime()`**: (demo-library/tone-json-mml.ts) MMLパーサーを実行するために必要なランタイム環境が準備されていることを保証します。
- **`encodeWav(samples: Float32Array, sampleRate: number)`**: (demo-library/wav-exporter.ts) 浮動小数点形式の音声サンプルデータとサンプルレートから、WAVファイル形式のバイナリデータを生成します。
- **`writeAscii(view: DataView, offset: number, s: string)`**: (demo-library/wav-exporter.ts) `DataView`オブジェクトの指定オフセットにASCII文字列を書き込みます。
- **`downloadWav(buffer: ArrayBuffer, filename: string)`**: (demo-library/wav-exporter.ts) 生成されたWAVファイルの`ArrayBuffer`データを受け取り、指定されたファイル名でユーザーにダウンロードを促します。
- **`drawEmpty(context: CanvasRenderingContext2D, width: number, height: number)`**: (demo-library/waveform-canvas.ts) HTML Canvasの指定されたコンテキストとサイズで、空の（初期状態の）波形キャンバスを描画します。
- **`drawWaveform(context: CanvasRenderingContext2D, samples: Float32Array, width: number, height: number, color: string)`**: (demo-library/waveform-canvas.ts) 浮動小数点形式の音声サンプルデータから、HTML Canvas上に波形を実際の形状で描画します。
- **`parseHexByte(hex: string)`**: (demo-library/ym2151-utils.ts) 2桁の16進数文字列（例: "C7"）をパースし、対応するバイト値（0-255）を返します。
- **`extractNoteBoundaries(logEvents: any[])`**: (demo-library/waveform-viewer.ts) YM2151ログイベントから、各ノートの開始時刻と終了時刻を抽出し、境界データを生成します。
- **`normalizeAmplitude(samples: Float32Array)`**: (demo-library/waveform-viewer.ts) 浮動小数点形式の音声サンプルデータの振幅を、可視化に適した範囲に正規化します。
- **`createWaveformViewer(containerId: string)`**: (demo-library/waveform-viewer.ts) 指定されたHTMLコンテナ内に波形ビューアコンポーネントを初期化し、そのインスタンスを返します。
- **`getWindowDurS(zoomLevel: number)`**: (demo-library/waveform-viewer.ts) 現在のズームレベルに基づいて、波形ビューアの表示ウィンドウの持続時間（秒単位）を計算します。
- **`clampViewStart(viewStart: number)`**: (demo-library/waveform-viewer.ts) 波形ビューの開始位置が有効な範囲内（オーディオの先頭から末尾まで）に収まるように調整します。
- **`updatePositionLabel(viewStartS: number, viewEndS: number)`**: (demo-library/waveform-viewer.ts) 波形ビューアの現在表示されている時間範囲を示すUIラベルを更新します。
- **`render()`**: (demo-library/waveform-viewer.ts) 波形ビューアの現在の状態（ズーム、スクロール位置など）に基づいて、波形を再描画します。
- **`updateBoundariesAndRender(logEvents: any[])`**: (demo-library/waveform-viewer.ts) 新しいYM2151ログイベントに基づいてノート境界を更新し、波形ビューアを再レンダリングします。
- **`synthesizeAndRender(logEvents: any[])`**: (demo-library/waveform-viewer.ts) YM2151ログイベントからオーディオ波形を合成し、その結果を波形ビューアに描画します。
- **`setZoom(zoomLevel: number)`**: (demo-library/waveform-viewer.ts) 波形ビューアのズームレベルを設定し、表示を更新します。
- **`endDrag()`**: (demo-library/waveform-viewer.ts) 波形ビューア上でのドラッグ操作が終了した際に、位置の確定やUIの更新を行います。

## 関数呼び出し階層ツリー
```
- main() (demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)
    - initializeWasm()
    - setupMmlInput()
        - setupMmlToSmf() (demo-library/mml-support.ts)
    - setupMidiInput()
    - setupAttachmentEditor()
    - setupRandomToneButton()
        - applyRandomToneToAttachment()
            - generateRandomToneRegisters() (demo-library/random-tone.ts)
            - upsertAttachmentRegisters() (demo-library/random-tone.ts)
            - generateRandomInterpolationPairRegisters() (demo-library/random-tone.ts)
            - upsertInterpolationAttachmentRegisters() (demo-library/random-tone.ts)
            - buildRandomInterpolationAttachment() (demo-library/random-tone.ts)
    - bootstrapWebYm()
        - ensureWasmInitialized() (demo-library/shared-demo.ts)
        - ensureWebYm2151() (demo-library/shared-demo.ts)
    - handlePlay()
        - nextRequestId()
        - isLatestRequest()
        - readAttachmentBytes()
        - runConversion()
        - updateOutputWithState()
        - updatePlayButtonState()
        - playAudioWithOverlay() (globals.d.ts)
        - createLogVisualizer() (demo-library/log-visualizer.ts)
            - renderFromJson() (demo-library/log-visualizer.ts)
                - normalizeEvents()
                - laneColor()
                - createLane()
                - computeTrackWidth()
                - formatInactiveChannels()
                - renderEmpty()
                - ensureGlobalLane()
                - setLfoRegisters()
        - clearWebYmAudioCache() (demo-library/shared-demo.ts)
        - parseAttachmentField() (demo-library/shared-demo.ts)
        - cleanup() (demo-library/shared-demo.ts)

- initWasm() (demo-library/library-demo.ts)
    - displayResult()
        - showError()
    - setupFileInput()
        - clear() (demo-library/waveform-viewer.ts)

- resolveRegisterForChannel() (demo-library/log-visualizer-lfo.ts)
    - collectLfoEvents()
        - renderLfoLane()
        - createLane()
        - parseHexByte() (demo-library/ym2151-utils.ts)

- buildNoteSegments() (demo-library/log-visualizer-note-segments.ts)
    - notePitch()
    - computePitchRange()
    - noteYPosition()

- renderPitchCanvas() (demo-library/log-visualizer-pitch-canvas.ts)

- detectChannel() (demo-library/log-visualizer.ts)

- bootstrap() (demo-library/pop-noise-demo.ts)
    - initializeWasm()
    - setupMmlInput()
    - setupMidiInput()
    - setupAttachmentEditor()
    - setupRandomToneButton()
    - setupPlayButton()
    - setupWavExportButton()
    - updateRegisterReflectionStatus()
    - countRegisterNormalizationTargets()
    - validateRandomToneAttachment()
    - createWaveformViewer()
        - renderFromJson()
        - clear()
        - exportWav()
            - downloadWav()
                - encodeWav() (demo-library/wav-exporter.ts)
                    - writeAscii() (demo-library/wav-exporter.ts)

- detectPopNoise() (demo-library/pop-noise-detector.ts)

- extractLfoRegistersFromAttachment() (demo-library/portamento-soft-lfo-demo.ts)
- syncLfoRegisters() (demo-library/portamento-soft-lfo-demo.ts)

- getToneEditorGenerator() (demo-library/random-tone.ts)
    - parseAttachmentEntries()
        - upsertEntryRegisters()

- getMmlParser() (demo-library/tone-json-mml.ts)
    - getParseTreeJsonToSmf()
        - treeToJson()
        - ensureMmlRuntime()

- buildEventsFromCompact() (demo-library/tone-json-attachment.ts)
    - serializeWithStatus()

- convertMmlToSmf() (demo-library/tone-json-demo.ts)

- drawEmpty() (demo-library/waveform-canvas.ts)
    - drawWaveform()

- extractNoteBoundaries() (demo-library/waveform-viewer.ts)
    - normalizeAmplitude()
    - getWindowDurS()
    - clampViewStart()
    - updatePositionLabel()
    - render()
    - updateBoundariesAndRender()
    - synthesizeAndRender()
    - setZoom()
    - endDrag()

- mod() (demo-library/shared-demo.ts)

---
Generated at: 2026-05-13 07:33:26 JST

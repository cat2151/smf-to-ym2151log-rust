Last updated: 2026-03-12

# Project Overview

## プロジェクト概要
- Standard MIDIファイル (SMF) をヤマハYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- ネイティブアプリケーションやWebブラウザ (WebAssembly) で利用可能な変換機能を提供し、FM音源の再生データを生成します。
- 和音数ベースの静的チャンネル割り当てやプログラムチェンジによる音色切り替えに対応し、YM2151の多様な表現を可能にします。

## 技術スタック
- フロントエンド: 
    -   **WebAssembly (WASM)**: Rustで書かれた変換ロジックをWebブラウザ上で実行可能にする技術です。
    -   **TypeScript**: Webデモの実装に用いられる、JavaScriptに型安全性を加えた言語です。
    -   **HTML/CSS**: Webデモのユーザーインターフェースを構築するためのマークアップ言語とスタイルシート言語です。
    -   **Vite**: Webデモの開発サーバーおよびビルドツールとして利用されています。
- 音楽・オーディオ: 
    -   **Standard MIDI Files (SMF)**: 入力として受け付ける標準的なMIDIファイル形式です。
    -   **YM2151**: ヤマハ製のFM音源チップであり、本プロジェクトの変換ターゲットです。
    -   **JSON**: 中間イベントおよび最終的なYM2151レジスタ書き込みログの出力形式です。
- 開発ツール: 
    -   **Rust**: プロジェクトの主要な実装言語であり、高いパフォーマンスと型安全性を提供します。
    -   **Cargo**: Rustプロジェクトのビルド、依存関係管理、テスト実行を統合するツールです。
    -   **wasm-pack**: RustコードをWebAssemblyにコンパイルし、Webで利用可能なパッケージを生成するためのツールです。
- テスト: 
    -   **`cargo test`**: Rustの標準テストフレームワークを活用し、ユニットテストおよび統合テストを実行します。
    -   **`cargo tarpaulin`**: テストカバレッジを測定し、テスト網羅率を評価します。
- ビルドツール: 
    -   **Cargo**: Rustアプリケーションおよびライブラリのビルドを管理します。
    -   **wasm-pack**: WebAssemblyモジュールのビルドとパッケージングを行います。
    -   **Vite**: WebデモのJavaScript/TypeScriptコードのバンドルと最適化を行います。
- 言語機能: 
    -   **Rust 1.70.0 以上**: プロジェクトが要求するRustコンパイラの最低バージョンです。
    -   **型安全性 (Rust)**: Rustの強力な型システムにより、コンパイル時に多くのバグを検出します。
    -   **高パフォーマンス (Rust)**: ネイティブコードへのコンパイルにより、高速な処理性能を実現します。
- 自動化・CI/CD: 
    -   (情報なし: 明示的な自動化・CI/CDツールの記述はありません。)
- 開発標準: 
    -   **`cargo fmt`**: Rustコードの自動フォーマットを行い、コードスタイルの一貫性を保ちます。
    -   **`cargo clippy`**: RustコードのLintチェックを行い、潜在的なバグや非効率なコードを検出します。
    -   **`cargo audit`**: プロジェクトの依存関係に存在する既知のセキュリティ脆弱性をチェックします。

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
  📘 wav-exporter.ts
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
-   `.gitignore`: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
-   `Cargo.lock`: `Cargo.toml` に基づいて解決された依存関係の正確なバージョンを記録し、ビルドの再現性を保証します。
-   `Cargo.toml`: Rustプロジェクトのメタデータ、依存関係、ビルド設定を定義するマニフェストファイルです。
-   `LICENSE`: プロジェクトのライセンス情報（著作権や利用条件）を記述したファイルです。
-   `README.ja.md`: プロジェクトの日本語版概要、使い方、特徴などを説明するドキュメントです。
-   `README.md`: プロジェクトの英語版概要、使い方、特徴などを説明する主要ドキュメントです。
-   `WASM_USAGE.md`: WebAssembly (WASM) を使用してプロジェクトをブラウザで利用する方法の詳細な説明が記載されたドキュメントです。
-   `_config.yml`: GitHub Pagesのサイト設定ファイルです。
-   `demo-library/`: Webブラウザ向けのデモンストレーションページやライブラリ関連のファイル群を格納するディレクトリです。
    -   `demo-library/.gitignore`: `demo-library`固有のGit無視ファイル設定です。
    -   `demo-library/biome.json`: Biome（コードフォーマッター・リンター）の設定ファイルです。
    -   `demo-library/delay-vibrato-demo.ts`: ディレイビブラート機能のデモンストレーションロジックを実装するTypeScriptファイルです。
    -   `demo-library/delay-vibrato.html`: ディレイビブラートデモのHTMLページです。
    -   `demo-library/envelope-generator.ts`: エンベロープジェネレータ関連のロジックを実装するTypeScriptファイルです。音量変化の計算などに使用されます。
    -   `demo-library/globals.d.ts`: グローバルな型定義を宣言するTypeScriptの型定義ファイルです。
    -   `demo-library/index.html`: WebデモのメインページとなるHTMLファイルです。
    -   `demo-library/library-demo.ts`: ライブラリとしての利用例を示すデモンストレーションロジックを実装するTypeScriptファイルです。
    -   `demo-library/log-visualizer-lfo.ts`: LFO (低周波発振器) イベントを視覚化するためのロジックを実装するTypeScriptファイルです。
    -   `demo-library/log-visualizer-note-segments.ts`: ノートイベントを視覚的なセグメントとして構築するためのロジックを実装するTypeScriptファイルです。
    -   `demo-library/log-visualizer.ts`: YM2151レジスタログを視覚的に表示するための主要なロジックを実装するTypeScriptファイルです。
    -   `demo-library/mml-support.ts`: MML (Music Macro Language) からSMFへの変換をサポートするロジックを実装するTypeScriptファイルです。
    -   `demo-library/package-lock.json`: `package.json` に基づく依存関係の正確なツリーを記録し、ビルドの再現性を保証します。
    -   `demo-library/package.json`: `demo-library` のフロントエンド依存関係とスクリプトを定義するマニフェストファイルです。
    -   `demo-library/pop-noise-demo.ts`: ポップノイズ関連のデモンストレーションロジックを実装するTypeScriptファイルです。
    -   `demo-library/pop-noise.html`: ポップノイズデモのHTMLページです。
    -   `demo-library/portamento-soft-lfo-demo.ts`: ポルタメントやソフトLFOのデモンストレーションロジックを実装するTypeScriptファイルです。
    -   `demo-library/portamento-soft-lfo.html`: ポルタメント・ソフトLFOデモのHTMLページです。
    -   `demo-library/shared-demo.ts`: 複数のデモで共通して使用されるユーティリティ関数や設定をまとめたTypeScriptファイルです。
    -   `demo-library/style.css`: デモページのスタイルを定義するCSSファイルです。
    -   `demo-library/tone-interpolation-demo.ts`: 音色補間機能のデモンストレーションロジックを実装するTypeScriptファイルです。
    -   `demo-library/tone-interpolation.html`: 音色補間デモのHTMLページです。
    -   `demo-library/tone-json-attachment.ts`: 外部から提供されるYM2151音色JSONデータを処理するロジックを実装するTypeScriptファイルです。
    -   `demo-library/tone-json-demo.ts`: YM2151音色JSONアタッチメント機能のデモンストレーションロジックを実装するTypeScriptファイルです。
    -   `demo-library/tone-json-mml.ts`: YM2151音色JSONとMMLを組み合わせたデモのロジックを実装するTypeScriptファイルです。
    -   `demo-library/tone-json.html`: YM2151音色JSONデモのHTMLページです。
    -   `demo-library/tsconfig.json`: TypeScriptコンパイラの設定ファイルです。
    -   `demo-library/vite.config.ts`: Viteビルドツール（フロントエンド開発サーバーおよびバンドラー）の設定ファイルです。
    -   `demo-library/wav-exporter.ts`: 生成されたオーディオデータをWAVファイルとしてエクスポートするロジックを実装するTypeScriptファイルです。
    -   `demo-library/waveform-canvas.ts`: 波形を描画するためのCanvas関連のロジックを実装するTypeScriptファイルです。
    -   `demo-library/waveform-simulator.ts`: YM2151の波形生成をシミュレートするロジックを実装するTypeScriptファイルです。
    -   `demo-library/waveform-viewer.ts`: YM2151の波形を視覚的に表示するためのビューアロジックを実装するTypeScriptファイルです。
    -   `demo-library/ym2151-utils.ts`: YM2151関連の汎用ユーティリティ関数を提供するTypeScriptファイルです。
-   `generated-docs/`: `cargo doc` などで生成されたドキュメントが格納されるディレクトリです。
-   `googled947dc864c270e07.html`: Googleサイト認証用のファイルです。
-   `src/`: Rustソースコードが格納される主要なディレクトリです。
    -   `src/error.rs`: エラーハンドリングのためのカスタムエラー型と関連ロジックを定義します。
    -   `src/lib.rs`: プロジェクトのライブラリクレートの入り口ファイルです。主要なAPIやモジュールを公開します。
    -   `src/main.rs`: プロジェクトのバイナリクレートの入り口ファイルです。コマンドラインアプリケーションのエントリポイントです。
    -   `src/midi/`: MIDIファイルのパースおよびイベント処理に関連するモジュールを格納します。
        -   `src/midi/events.rs`: MIDIイベントのデータ構造を定義します。
        -   `src/midi/mod.rs`: `src/midi` ディレクトリのモジュール定義ファイルです。
        -   `src/midi/parser.rs`: Standard MIDI Files (SMF) をパースし、内部表現に変換するロジックを実装します。
        -   `src/midi/utils.rs`: MIDI関連のユーティリティ関数を提供します。
        -   `src/midi/utils_tests.rs`: `src/midi/utils.rs` の単体テストを定義します。
    -   `src/wasm.rs`: WebAssembly (WASM) バインディングを提供し、JavaScriptからRust関数を呼び出せるようにします。
    -   `src/ym2151/`: YM2151レジスタログ変換に関連するモジュールを格納します。
        -   `src/ym2151/channel_allocation.rs`: YM2151のチャンネル割り当て戦略に関するロジックを実装します。
        -   `src/ym2151/converter/`: MIDIイベントからYM2151レジスタイベントへの変換ロジックを格納します。
            -   `src/ym2151/converter/pitch_effects.rs`: ピッチベンドなどのピッチ関連エフェクトの変換ロジックを実装します。
            -   `src/ym2151/converter/register_effects.rs`: 各種レジスタ設定エフェクトの変換ロジックを実装します。
            -   `src/ym2151/converter/waveform.rs`: YM2151の波形設定に関連するロジックを実装します。
        -   `src/ym2151/converter.rs`: MIDIイベントをYM2151レジスタログに変換する主要なロジックを実装します。
        -   `src/ym2151/converter_tests/`: `src/ym2151/converter.rs` の各種単体テストを格納します。
            -   `src/ym2151/converter_tests/attachments.rs`: 音色アタッチメント機能のテストです。
            -   `src/ym2151/converter_tests/basic.rs`: 基本的な変換機能のテストです。
            -   `src/ym2151/converter_tests/channels.rs`: チャンネル割り当て機能のテストです。
            -   `src/ym2151/converter_tests/drums.rs`: ドラムチャンネル処理のテストです。
            -   `src/ym2151/converter_tests/effects.rs`: 各種エフェクト（ピッチベンドなど）のテストです。
            -   `src/ym2151/converter_tests/lfo.rs`: LFO (低周波発振器) 関連機能のテストです。
            -   `src/ym2151/converter_tests/portamento.rs`: ポルタメント機能のテストです。
            -   `src/ym2151/converter_tests/programs.rs`: プログラムチェンジによる音色切り替え機能のテストです。
        -   `src/ym2151/converter_tests.rs`: `src/ym2151/converter.rs` のテストモジュールです（上記のサブディレクトリからのテストを集約する可能性）。
        -   `src/ym2151/event_processor.rs`: YM2151イベントのタイムライン処理や状態管理を行うロジックを実装します。
        -   `src/ym2151/event_processor_tests.rs`: `src/ym2151/event_processor.rs` の単体テストを定義します。
        -   `src/ym2151/events.rs`: YM2151レジスタイベントのデータ構造を定義します。
        -   `src/ym2151/init.rs`: YM2151チップの初期化レジスタ設定に関連するロジックを実装します。
        -   `src/ym2151/mod.rs`: `src/ym2151` ディレクトリのモジュール定義ファイルです。
        -   `src/ym2151/note_table.rs`: ノート番号と周波数、YM2151のキーコード/オクターブ設定のマッピングを管理します。
        -   `src/ym2151/tempo_map.rs`: MIDIテンポイベントから構築されるテンポマップを管理し、時間計算をサポートします。
        -   `src/ym2151/tone.rs`: YM2151の音色データ構造と関連ロジックを定義します。
-   `tests/`: 統合テストや大きな機能テストを格納するディレクトリです。
    -   `tests/create_test_midi.py`: テスト用のMIDIファイルを生成するためのPythonスクリプトです。
    -   `tests/integration_conversion.rs`: 変換プロセスの全体的な統合テストを定義します。
    -   `tests/integration_midi.rs`: MIDIパース機能の統合テストを定義します。
    -   `tests/integration_multichannel.rs`: マルチチャンネルMIDIの変換に関する統合テストを定義します。
    -   `tests/integration_program_change.rs`: プログラムチェンジ機能の統合テストを定義します。
    -   `tests/integration_wasm.rs`: WebAssembly (WASM) 変換パスの統合テストを定義します。
    -   `tests/test_data/`: 統合テストで使用されるサンプルMIDIファイルなどのテストデータを格納するディレクトリです。
        -   `tests/test_data/multi_channel.mid`: マルチチャンネルMIDIテスト用のサンプルファイルです。
        -   `tests/test_data/multi_track.mid`: マルチトラックMIDIテスト用のサンプルファイルです。
        -   `tests/test_data/program_change.mid`: プログラムチェンジテスト用のサンプルファイルです。
        -   `tests/test_data/simple_melody.mid`: シンプルなメロディのMIDIテスト用サンプルファイルです。
        -   `tests/test_data/tempo_change.mid`: テンポチェンジテスト用のサンプルファイルです。
-   `tones/`: プログラムチェンジで使用されるカスタムYM2151音色定義JSONファイル群を格納するディレクトリです。
    -   `tones/000.json`: MIDIプログラム0番に対応するYM2151音色定義ファイルです。
    -   `tones/README.md`: `tones` ディレクトリ内のJSONファイルのフォーマットや使い方を説明するドキュメントです。

## 関数詳細説明
-   **`computeHash` (demo-library/delay-vibrato-demo.ts)**: 入力値からハッシュ値を計算し、リクエストの一意性を確保します。
-   **`nextRequestId` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts)**: 新しいリクエストIDを生成し、非同期処理の管理に使用します。
-   **`isLatestRequest` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts)**: 現在のリクエストが最新のものであるかを確認し、古いリクエストの結果がUIに反映されないようにします。
-   **`updateOutputWithState` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: アプリケーションの状態に基づいて出力表示を更新します。
-   **`updatePlayButtonState` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: オーディオ再生ボタンの状態（有効/無効、表示テキストなど）を更新します。
-   **`initializeWasm` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: WebAssemblyモジュールを初期化し、Rustの機能をJavaScriptから利用可能にします。
-   **`readAttachmentBytes` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: 外部から添付されたファイル（音色JSONなど）の内容をバイトデータとして読み込みます。
-   **`runConversion` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: MIDIファイルをYM2151レジスタログに変換する主要な処理を実行します。
-   **`handlePlay` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: オーディオ再生ボタンがクリックされた際の処理を行い、YM2151ログを基に音声を再生します。
-   **`setupAttachmentEditor` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: 外部音色などの添付ファイルを編集するためのUI要素をセットアップします。
-   **`setupMmlInput` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: MML入力フィールドのイベントハンドラをセットアップし、MML入力時の処理を定義します。
-   **`setupMidiInput` (demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: MIDIファイル入力フィールドのイベントハンドラをセットアップし、MIDIファイル選択時の処理を定義します。
-   **`bootstrapWebYm` (demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: WebYM2151のデモ環境を初期化・起動します。
-   **`main` (demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-interpolation-demo.ts, demo-library/tone-json-demo.ts)**: デモンストレーションのメインエントリポイントとなる関数。ページの初期化処理を呼び出します。
-   **`if` (demo-library/delay-vibrato-demo.ts, 他多数)**: 条件分岐を行う汎用的な制御構造です。
-   **`catch` (demo-library/delay-vibrato-demo.ts, 他多数)**: エラーハンドリングを行う汎用的な制御構造です。
-   **`kcToFrequency` (demo-library/envelope-generator.ts)**: YM2151のキーコードを周波数に変換します。
-   **`ampStepPerSample` (demo-library/envelope-generator.ts)**: サンプルあたりの振幅ステップを計算します。
-   **`switch` (demo-library/envelope-generator.ts)**: 複数の条件分岐を行う汎用的な制御構造です。
-   **`playAudioWithOverlay` (demo-library/globals.d.ts)**: オーディオを再生し、その上にオーバーレイ表示を行うグローバル関数です。
-   **`clearAudioCache` (demo-library/globals.d.ts)**: オーディオキャッシュをクリアするグローバル関数です。
-   **`initWasm` (demo-library/library-demo.ts)**: WebAssemblyモジュールを初期化します。
-   **`displayResult` (demo-library/library-demo.ts)**: 変換結果を画面に表示します。
-   **`showError` (demo-library/library-demo.ts)**: エラーメッセージを画面に表示します。
-   **`setupFileInput` (demo-library/library-demo.ts)**: ファイル入力要素のイベントリスナーをセットアップします。
-   **`resolveRegisterForChannel` (demo-library/log-visualizer-lfo.ts)**: 特定のチャンネルに割り当てられたYM2151レジスタを解決します。
-   **`collectLfoEvents` (demo-library/log-visualizer-lfo.ts)**: LFO関連のイベントを収集します。
-   **`renderLfoLane` (demo-library/log-visualizer-lfo.ts)**: LFOの動きを視覚化するレーンを描画します。
-   **`for` (demo-library/log-visualizer-lfo.ts, 他多数)**: ループ処理を行う汎用的な制御構造です。
-   **`buildNoteSegments` (demo-library/log-visualizer-note-segments.ts)**: ノートイベントから視覚化用のセグメントデータを構築します。
-   **`notePitch` (demo-library/log-visualizer-note-segments.ts)**: ノートのピッチ情報を取得します。
-   **`computePitchRange` (demo-library/log-visualizer-note-segments.ts)**: ピッチの表示範囲を計算します。
-   **`noteYPosition` (demo-library/log-visualizer-note-segments.ts)**: ノートが視覚化されるY座標を計算します。
-   **`detectChannel` (demo-library/log-visualizer.ts)**: イベントから関連するYM2151チャンネルを検出します。
-   **`normalizeEvents` (demo-library/log-visualizer.ts)**: イベントデータを視覚化に適した形式に正規化します。
-   **`laneColor` (demo-library/log-visualizer.ts)**: 視覚化レーンの色を決定します。
-   **`createLane` (demo-library/log-visualizer.ts)**: 視覚化用の新しいレーン要素を作成します。
-   **`computeTrackWidth` (demo-library/log-visualizer.ts)**: 視覚化トラックの幅を計算します。
-   **`createLogVisualizer` (demo-library/log-visualizer.ts)**: ログ視覚化ツール全体を初期化し、生成します。
-   **`renderEmpty` (demo-library/log-visualizer.ts)**: 空の視覚化ビューを描画します。
-   **`renderFromJson` (demo-library/log-visualizer.ts, demo-library/waveform-viewer.ts)**: JSONデータからYM2151レジスタログを読み込み、視覚化・表示します。
-   **`ensureGlobalLane` (demo-library/log-visualizer.ts)**: グローバルな視覚化レーンが存在することを保証します。
-   **`setLfoRegisters` (demo-library/log-visualizer.ts)**: LFOに関連するYM2151レジスタを設定します。
-   **`setupPlayButton` (demo-library/pop-noise-demo.ts)**: 再生ボタンのイベントハンドラをセットアップします。
-   **`setupWavExportButton` (demo-library/pop-noise-demo.ts)**: WAVエクスポートボタンのイベントハンドラをセットアップします。
-   **`bootstrap` (demo-library/pop-noise-demo.ts)**: ポップノイズデモの初期化処理全体を行います。
-   **`extractLfoRegistersFromAttachment` (demo-library/portamento-soft-lfo-demo.ts)**: 添付ファイルからLFOレジスタ設定を抽出します。
-   **`syncLfoRegisters` (demo-library/portamento-soft-lfo-demo.ts)**: LFOレジスタ設定をUIと同期させます。
-   **`ensureWasmInitialized` (demo-library/shared-demo.ts)**: WebAssemblyモジュールが初期化されていることを確認します。
-   **`setStatus` (demo-library/shared-demo.ts)**: アプリケーションのステータス表示を更新します。
-   **`setEventCountDisplay` (demo-library/shared-demo.ts)**: イベント数の表示を更新します。
-   **`ensureWebYm2151` (demo-library/shared-demo.ts)**: WebYM2151インスタンスが利用可能であることを保証します。
-   **`clearWebYmAudioCache` (demo-library/shared-demo.ts)**: WebYM2151のオーディオキャッシュをクリアします。
-   **`updateOutput` (demo-library/shared-demo.ts)**: 一般的な出力表示を更新します。
-   **`parseAttachmentField` (demo-library/shared-demo.ts)**: 添付フィールドからデータをパースします。
-   **`cleanup` (demo-library/shared-demo.ts)**: デモ環境の後処理を行います。
-   **`buildEventsFromCompact` (demo-library/tone-json-attachment.ts)**: コンパクトなデータ形式からYM2151イベントを構築します。
-   **`serializeWithStatus` (demo-library/tone-json-attachment.ts)**: ステータス情報とともにデータをシリアライズします。
-   **`normalizeAttachmentText` (demo-library/tone-json-attachment.ts)**: 添付ファイルのテキスト内容を正規化します。
-   **`convertMmlToSmf` (demo-library/tone-json-demo.ts)**: MMLをStandard MIDIファイル形式に変換します。
-   **`getMmlParser` (demo-library/tone-json-mml.ts)**: MMLパーサーのインスタンスを取得します。
-   **`getParseTreeJsonToSmf` (demo-library/tone-json-mml.ts)**: パースツリーをJSON形式のSMFに変換する関数を取得します。
-   **`treeToJson` (demo-library/tone-json-mml.ts)**: パースツリーをJSON形式に変換します。
-   **`ensureMmlRuntime` (demo-library/tone-json-mml.ts)**: MMLランタイムが利用可能であることを確認します。
-   **`encodeWav` (demo-library/wav-exporter.ts)**: オーディオデータをWAVフォーマットにエンコードします。
-   **`writeAscii` (demo-library/wav-exporter.ts)**: ASCII文字列をバイト配列に書き込みます。
-   **`downloadWav` (demo-library/wav-exporter.ts)**: 生成されたWAVファイルをダウンロードします。
-   **`drawEmpty` (demo-library/waveform-canvas.ts)**: 空の波形キャンバスを描画します。
-   **`drawWaveform` (demo-library/waveform-canvas.ts)**: 指定されたデータに基づいて波形をキャンバスに描画します。
-   **`simulateWaveform` (demo-library/waveform-simulator.ts)**: YM2151のレジスタ操作に基づき、波形をシミュレートします。
-   **`applyOp` (demo-library/waveform-simulator.ts)**: 波形シミュレーション中のオペレーションを適用します。
-   **`while` (demo-library/waveform-simulator.ts)**: ループ処理を行う汎用的な制御構造です。
-   **`createWaveformViewer` (demo-library/waveform-viewer.ts)**: 波形ビューアのインスタンスを生成します。
-   **`getWindowDurS` (demo-library/waveform-viewer.ts)**: 表示ウィンドウの持続時間（秒）を取得します。
-   **`clampViewStart` (demo-library/waveform-viewer.ts)**: ビューの開始位置を有効な範囲にクランプします。
-   **`updatePositionLabel` (demo-library/waveform-viewer.ts)**: 現在の表示位置を示すラベルを更新します。
-   **`render` (demo-library/waveform-viewer.ts)**: 波形ビューアの表示を更新します。
-   **`rebuildAndRender` (demo-library/waveform-viewer.ts)**: 波形データを再構築し、ビューアを再描画します。
-   **`setZoom` (demo-library/waveform-viewer.ts)**: 波形ビューアのズームレベルを設定します。
-   **`endDrag` (demo-library/waveform-viewer.ts)**: ドラッグ操作が終了した際の処理です。
-   **`clear` (demo-library/waveform-viewer.ts)**: 波形ビューアの表示をクリアします。
-   **`exportWav` (demo-library/waveform-viewer.ts)**: 表示されている波形データをWAVとしてエクスポートします。
-   **`parseHexByte` (demo-library/ym2151-utils.ts)**: 16進数文字列をバイト値にパースします。

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
      - bootstrap ()
      - createWaveformViewer ()
      - exportWav ()
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
  - downloadWav ()
    - encodeWav (demo-library/wav-exporter.ts)
      - writeAscii ()
  - getWindowDurS ()
    - clampViewStart ()
      - updatePositionLabel ()
      - render ()
      - rebuildAndRender ()
      - setZoom ()
- switch (demo-library/envelope-generator.ts)
- for (demo-library/log-visualizer-lfo.ts)
- while (demo-library/waveform-simulator.ts)
- endDrag (demo-library/waveform-viewer.ts)

---
Generated at: 2026-03-12 07:11:57 JST

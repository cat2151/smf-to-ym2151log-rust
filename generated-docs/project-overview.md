Last updated: 2026-04-02

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップ向けのレジスタ書き込みログ（JSON）に変換します。
- Rustで実装されており、ネイティブアプリケーションとWebAssemblyによるブラウザでの利用をサポートします。
- 和音数に基づくYM2151チャンネル割り当てや外部音色ファイルの読み込みなど、高度な変換機能を提供します。

## 技術スタック
- フロントエンド:
    - **WebAssembly (WASM)**: RustコードをWebブラウザで実行可能にするバイナリ形式。デモページで利用されています。
    - **TypeScript**: JavaScriptに型安全性を追加するプログラミング言語。デモライブラリのUIロジックやWASMラッパーの開発に使用されています。
    - **Vite**: 高速な開発サーバーとバンドラーを提供するフロントエンドビルドツール。デモライブラリのビルドに利用されています。
    - **HTML/CSS**: デモページの構造とスタイリングに使用される標準技術。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: プロジェクトの入力となる、標準的なMIDIファイルフォーマット。Format 0およびFormat 1をサポートしています。
    - **YM2151 FM音源チップ**: プロジェクトの出力ターゲットとなるヤマハ製のFM音源チップ。
    - **JSON**: YM2151レジスタ書き込みログの出力形式、中間イベントの形式、およびカスタムYM2151音色ファイルの定義に使用されます。
- 開発ツール:
    - **Rust 1.70.0以上**: プロジェクトの主要な開発言語。
    - **Cargo**: Rustの公式ビルドシステムとパッケージマネージャー。プロジェクトのビルド、テスト、依存関係管理に使用されます。
    - **wasm-pack**: RustコードをWebAssemblyにコンパイルし、Web互換パッケージを生成するためのツール。
    - **Git**: ソースコードのバージョン管理システム。
    - **Biome**: デモライブラリのコードフォーマットとリンティングに使用されるツール。
- テスト:
    - **Rust標準テストフレームワーク**: `cargo test`コマンドで実行されるユニットテストと統合テスト。73のテストが含まれています。
    - **cargo tarpaulin**: Rustプロジェクトのテストカバレッジを測定し、レポートを生成するツール。
- ビルドツール:
    - **Cargo**: Rustプロジェクトのビルド。
    - **wasm-pack**: WebAssemblyモジュールのビルド。
    - **Vite**: デモライブラリのビルド。
- 言語機能:
    - **Rust**: 高いパフォーマンスとメモリ安全性を提供するシステムプログラミング言語。強力な型システムにより、堅牢なコードが実現されています。
- 自動化・CI/CD:
    - (プロジェクト情報からは特定のCI/CDツールは明示されていませんが、`cargo fmt --check`, `cargo clippy`, `cargo audit`などの品質チェックはCIプロセスに組み込むことが推奨されます。)
- 開発標準:
    - **cargo fmt**: Rustコードの自動フォーマットツール。コードの一貫性を保ちます。
    - **cargo clippy**: Rustコードの潜在的なバグや非効率性を検出するリンター。
    - **cargo audit**: Rustプロジェクトの依存関係のセキュリティ脆弱性をチェックするツール。

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
  📖 234.md
  📖 235.md
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
  📄 options.rs
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
- **`.gitignore`**: Gitがバージョン管理の対象から除外するファイルやディレクトリを指定します。
- **`Cargo.lock`**: Cargo.tomlに基づいて解決された正確な依存関係のバージョンをロックするファイルです。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクトのメタデータ、依存関係、ビルド設定などを定義します。
- **`LICENSE`**: プロジェクトのライセンス情報が含まれます。
- **`README.ja.md`**: プロジェクトの日本語での説明、使い方、特徴などを記載したドキュメントです。
- **`README.md`**: プロジェクトの英語での説明、使い方、特徴などを記載したドキュメントです。
- **`WASM_USAGE.md`**: WebAssembly (WASM) バージョンのライブラリの使用方法に関する詳細なドキュメントです。
- **`_config.yml`**: GitHub Pagesのサイト設定ファイルで、デモサイトの構成を定義します。

**`demo-library/`**: WebAssembly版ライブラリのデモおよびテスト用のWebアプリケーションコードが含まれています。
    - **`.gitignore`**: `demo-library`内のGit管理から除外するファイルを指定します。
    - **`biome.json`**: Biomeによるコードフォーマットとリンティングの設定ファイルです。
    - **`delay-vibrato-demo.ts`**: ディレイビブラート機能のデモ用TypeScriptロジックです。WASMライブラリの初期化、MIDI/MML入力の処理、オーディオ再生、視覚化を担当します。
    - **`delay-vibrato.html`**: ディレイビブラートのデモ表示用HTMLファイルです。
    - **`globals.d.ts`**: グローバルスコープで利用される型定義ファイルです。
    - **`index.html`**: メインのライブラリデモページです。MIDI変換の基本的な機能を示します。
    - **`library-demo.ts`**: `index.html`で使用される基本的なライブラリデモのTypeScriptロジックです。
    - **`log-visualizer-lfo.ts`**: YM2151のLFO（低周波発振器）レジスタイベントを可視化するためのロジックです。
    - **`log-visualizer-note-segments.ts`**: 音符のセグメントを構築し、YM2151ログ内のノートイベントを視覚的に表現するためのロジックです。
    - **`log-visualizer-pitch-canvas.ts`**: ピッチの動きをグラフィカルに表示するためのキャンバス描画ロジックです。
    - **`log-visualizer.ts`**: YM2151レジスタ書き込みログ全体の視覚化を担当するメインロジックです。
    - **`mml-support.ts`**: MML (Music Macro Language) からSMFへの変換をサポートするためのロジックです。
    - **`package-lock.json`**: `package.json`に基づく依存関係の正確なバージョンをロックするファイルです。
    - **`package.json`**: `demo-library`の依存関係やスクリプトを定義するNode.jsマニフェストファイルです。
    - **`pop-noise-demo.ts`**: ポップノイズ軽減機能のデモ用TypeScriptロジックです。
    - **`pop-noise-detector.ts`**: YM2151ログからポップノイズの発生を検出するためのロジックです。
    - **`pop-noise.html`**: ポップノイズ軽減のデモ表示用HTMLファイルです。
    - **`portamento-soft-lfo-demo.ts`**: ポルタメントとソフトLFO機能のデモ用TypeScriptロジックです。
    - **`portamento-soft-lfo.html`**: ポルタメントとソフトLFOのデモ表示用HTMLファイルです。
    - **`random-tone.ts`**: YM2151のランダムな音色定義を生成するためのロジックです。
    - **`shared-demo.ts`**: 複数のデモで共通して使用されるユーティリティ関数やWASM初期化ロジックが含まれます。
    - **`style.css`**: デモページのスタイル定義を記述したCSSファイルです。
    - **`tone-interpolation-demo.ts`**: 音色補間機能のデモ用TypeScriptロジックです。
    - **`tone-interpolation.html`**: 音色補間のデモ表示用HTMLファイルです。
    - **`tone-json-attachment.ts`**: カスタム音色JSONの添付と正規化に関するロジックです。
    - **`tone-json-demo.ts`**: カスタム音色JSONの読み込みと適用をデモするTypeScriptロジックです。
    - **`tone-json-mml.ts`**: カスタム音色JSONとMMLを組み合わせたデモ用のロジックです。
    - **`tone-json.html`**: カスタム音色JSONのデモ表示用HTMLファイルです。
    - **`tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    - **`vite.config.ts`**: Viteのビルド設定ファイルです。
    - **`wav-exporter.ts`**: 生成されたオーディオデータをWAVファイルとしてエクスポートするためのロジックです。
    - **`waveform-canvas.ts`**: 波形をキャンバスに描画するためのロジックです。
    - **`waveform-viewer.ts`**: YM2151レジスタログから生成された波形を視覚的に表示し、操作するためのビューアロジックです。
    - **`ym2151-utils.ts`**: YM2151関連のユーティリティ関数が含まれます。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルです。
- **`issue-notes/`**: 開発中の課題や調査に関するメモが格納されています。
- **`package-lock.json`**: プロジェクト全体のNode.js依存関係のロックファイルです。
- **`package.json`**: プロジェクト全体のNode.js依存関係を定義するファイルです。

**`src/`**: Rustのコアライブラリおよび実行可能コードが含まれています。
    - **`api.rs`**: ライブラリの公開API定義です。
    - **`error.rs`**: カスタムエラータイプとエラーハンドリングロジックを定義します。
    - **`lib.rs`**: Rustクレートのエントリポイントで、ライブラリの公開インターフェースを提供します。
    - **`main.rs`**: コマンドラインアプリケーションのエントリポイントで、`lib.rs`の機能を利用してファイル変換を実行します。
    - **`midi/`**: MIDIファイルの解析と処理に関連するモジュールです。
        - **`events.rs`**: MIDIイベントのデータ構造を定義します。
        - **`mod.rs`**: MIDIモジュールのエントリポイントです。
        - **`parser.rs`**: Standard MIDI File (SMF) を解析し、内部中間イベント形式に変換するロジックが含まれます。
        - **`utils.rs`**: MIDIデータ処理に関するユーティリティ関数を提供します。
        - **`utils_tests.rs`**: `utils.rs`のテストコードです。
    - **`options.rs`**: コマンドライン引数の解析とプログラムオプションの設定を扱います。
    - **`wasm.rs`**: WebAssembly (WASM) へのバインディングと、ブラウザ環境でRustライブラリを使用するための機能を提供します。
    - **`ym2151/`**: YM2151 FM音源への変換ロジックに関連するモジュールです。
        - **`channel_allocation.rs`**: MIDIチャンネルからYM2151の8つのチャンネルへの割り当て戦略（和音数ベース、ドラム優先など）を実装します。
        - **`converter/`**: YM2151レジスタログへの詳細な変換プロセスを扱います。
            - **`event_accumulator.rs`**: 複数のMIDIイベントからYM2151レジスタイベントを累積し、最終的なログを構築します。
            - **`pitch_effects.rs`**: ピッチベンドやポルタメントなどのピッチ関連エフェクトのYM2151レジスタへの変換ロジックです。
            - **`register_effects/`**: 特定のレジスタエフェクトに関するモジュールです。
                - **`common.rs`**: 共通のレジスタエフェクト処理やユーティリティが含まれます。
                - **`mod.rs`**: レジスタエフェクトモジュールのエントリポイントです。
                - **`pop_noise.rs`**: YM2151のレジスタ操作におけるポップノイズを軽減するためのロジックです。
                - **`register_lfo.rs`**: YM2151のLFOレジスタに関連する処理を実装します。
                - **`state_cache.rs`**: YM2151レジスタの状態をキャッシュし、冗長な書き込みを防ぐことで効率を向上させます。
                - **`tone_interpolation.rs`**: 音色のパラメータを時間的に補間するロジックです。
            - **`register_fields.rs`**: YM2151の個々のレジスタフィールドの定義と操作ロジックです。
            - **`waveform.rs`**: YM2151の波形設定に関するロジックです。
        - **`converter.rs`**: YM2151への変換プロセス全体をオーケストレートするメインロジックです。
        - **`converter_tests/`**: `ym2151/converter`モジュールのテストコード集です。様々な機能（アタッチメント、プログラムチェンジ、チャンネル、ドラム、エフェクト、LFO、ポルタメントなど）のテストが含まれます。
        - **`converter_tests.rs`**: `ym2151/converter`モジュールのテストエントリポイントです。
        - **`event_processor.rs`**: MIDIイベントをYM2151レジスタイベントに変換する高レベルの処理ロジックです。
        - **`event_processor_tests.rs`**: `event_processor.rs`のテストコードです。
        - **`events.rs`**: YM2151のレジスタ書き込みイベントのデータ構造を定義します。
        - **`init.rs`**: YM2151チップの初期化に関連する定数や設定が含まれます。
        - **`mod.rs`**: YM2151モジュールのエントリポイントです。
        - **`note_table.rs`**: MIDIノート番号とYM2151が使用する周波数パラメータとのマッピングテーブルを提供します。
        - **`tempo_map.rs`**: MIDIファイルのテンポイベントを処理し、時間とティックの変換マップを管理します。
        - **`tone.rs`**: YM2151の音色（プログラム）のデータ構造と、それらを管理するロジックを定義します。
- **`tests/`**: プロジェクトの統合テストコードとテストデータが含まれています。
    - **`create_test_midi.py`**: テストで使用するMIDIファイルを自動生成するためのPythonスクリプトです。
    - **`integration_conversion.rs`**: 変換プロセスの統合テストです。
    - **`integration_midi.rs`**: MIDIファイル解析の統合テストです。
    - **`integration_multichannel.rs`**: マルチチャンネルMIDIの変換に関する統合テストです。
    - **`integration_program_change.rs`**: プログラムチェンジイベントの処理に関する統合テストです。
    - **`integration_public_api.rs`**: ライブラリの公開APIの統合テストです。
    - **`integration_wasm.rs`**: WebAssemblyバージョンの統合テストです。
    - **`test_data/`**: 統合テストで使用されるMIDIサンプルデータが格納されています。
- **`tones/`**: カスタムYM2151音色ファイル（JSON形式）を格納するためのディレクトリです。
    - **`000.json`**: プログラム0番（アコースティックグランドピアノ）のデフォルト音色定義です。
    - **`README.md`**: カスタム音色JSONファイルのフォーマットに関する説明ドキュメントです。

## 関数詳細説明
以下に、提供された情報から検出された主要な関数について、その役割と機能を説明します。引数や戻り値の具体的な型は情報に含まれていないため、役割と機能に焦点を当てています。

- **`computeHash` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: 文字列のハッシュ値を計算します。
    - **機能**: 入力文字列に基づいて一意のハッシュ値を生成し、キャッシュの識別などに利用されます。
- **`nextRequestId` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: 次のリクエストIDを生成します。
    - **機能**: 非同期処理において、特定のリクエストが最新のものであるかを追跡するために一意のIDを発行します。
- **`isLatestRequest` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: 指定されたリクエストIDが最新であるかを確認します。
    - **機能**: 複数の非同期リクエストが競合する可能性がある場合に、古いリクエストの結果がUIに反映されないように制御します。
- **`updateOutputWithState` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: アプリケーションの状態に基づいて出力UIを更新します。
    - **機能**: 変換結果やエラーメッセージ、処理状況などを画面に表示します。
- **`updatePlayButtonState` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: オーディオ再生ボタンの状態（有効/無効）を更新します。
    - **機能**: オーディオデータが利用可能か、現在再生中かといった状況に応じてボタンのクリック可否を制御します。
- **`initializeWasm` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: WebAssemblyモジュールを初期化します。
    - **機能**: smf-to-ym2151log-rustのWASMモジュールをWebブラウザ環境でロードし、利用可能な状態にします。
- **`readAttachmentBytes` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: 添付ファイル（カスタム音色JSONなど）のバイトデータを読み込みます。
    - **機能**: ユーザーが提供する外部データファイルをバイナリ形式で取得し、WASMライブラリに渡せるように準備します。
- **`runConversion` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: MIDIファイルまたはMMLの変換処理を実行します。
    - **機能**: 入力された音楽データとオプション（添付ファイルなど）を受け取り、YM2151レジスタログへの変換処理をWASMライブラリ経由で実行し、結果を返します。
- **`handlePlay` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: 変換結果のオーディオ再生を処理します。
    - **機能**: 生成されたYM2151ログを基にオーディオを合成し、ブラウザで再生を開始または停止します。
- **`setupAttachmentEditor` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: カスタム音色JSONなどの添付ファイルを編集するUIを設定します。
    - **機能**: 添付ファイルの入力欄や関連するイベントリスナーを初期化し、ユーザーがカスタム設定を入力できるようにします。
- **`setupMmlInput` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: MML（Music Macro Language）入力フィールドを設定します。
    - **機能**: MMLをSMFに変換し、さらにYM2151ログに変換するフローのための入力インターフェースを提供します。
- **`setupMidiInput` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: MIDIファイル入力フィールドを設定します。
    - **機能**: ユーザーがMIDIファイルをアップロードし、変換処理を開始するためのUIコンポーネントを初期化します。
- **`bootstrapWebYm` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: WebMidiとYM2151関連のデモ環境を初期化します。
    - **機能**: Webブラウザ上でのMIDI入力検出やYM2151ログ再生環境のセットアップを行います。
- **`applyRandomToneToAttachment` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: ランダムに生成された音色を添付ファイルに適用します。
    - **機能**: デモ用途で、ランダムなYM2151音色パラメータを生成し、既存の添付音色定義を更新します。
- **`setupRandomToneButton` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: ランダム音色適用ボタンのイベントリスナーを設定します。
    - **機能**: ボタンクリック時に`applyRandomToneToAttachment`関数を呼び出すように設定します。
- **`main` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: デモアプリケーションの主要なエントリポイントです。
    - **機能**: ページのロード時にすべてのUIコンポーネントとイベントリスナーを初期化し、デモアプリケーションを開始します。
- **`if` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: 条件分岐を処理します。
    - **機能**: 特定の条件が真である場合にコードブロックを実行します。
- **`catch` (demo-library/delay-vibrato-demo.ts)**
    - **役割**: エラーハンドリングを処理します。
    - **機能**: `try`ブロック内で発生した例外を捕捉し、適切なエラー処理を実行します。

（上記は`delay-vibrato-demo.ts`の関数の一部を詳細に説明したものです。他のファイルに定義されている関数についても、同様に役割、引数、戻り値、機能を詳細に説明します。）

- **`initWasm` (demo-library/library-demo.ts)**: WASMモジュールの初期化を担当。
- **`displayResult` (demo-library/library-demo.ts)**: 変換結果をUIに表示する。
- **`showError` (demo-library/library-demo.ts)**: エラーメッセージをUIに表示する。
- **`readAttachmentBytes` (demo-library/library-demo.ts)**: 添付ファイルのバイトデータを読み込む。
- **`setupFileInput` (demo-library/library-demo.ts)**: ファイル入力要素を設定する。
- **`resolveRegisterForChannel` (demo-library/log-visualizer-lfo.ts)**: 特定のチャンネルのレジスタ値を解決する。
- **`collectLfoEvents` (demo-library/log-visualizer-lfo.ts)**: LFOイベントを収集する。
- **`renderLfoLane` (demo-library/log-visualizer-lfo.ts)**: LFOの動きをレーンで描画する。
- **`buildNoteSegments` (demo-library/log-visualizer-note-segments.ts)**: ノートセグメントを構築する。
- **`notePitch` (demo-library/log-visualizer-note-segments.ts)**: ノートのピッチを計算する。
- **`computePitchRange` (demo-library/log-visualizer-note-segments.ts)**: ピッチの範囲を計算する。
- **`noteYPosition` (demo-library/log-visualizer-note-segments.ts)**: ノートのY軸位置を決定する。
- **`renderPitchCanvas` (demo-library/log-visualizer-pitch-canvas.ts)**: ピッチ表示キャンバスを描画する。
- **`detectChannel` (demo-library/log-visualizer.ts)**: チャンネルを検出する。
- **`normalizeEvents` (demo-library/log-visualizer.ts)**: イベントデータを正規化する。
- **`laneColor` (demo-library/log-visualizer.ts)**: レーンの色を決定する。
- **`createLane` (demo-library/log-visualizer.ts)**: 視覚化レーンを作成する。
- **`computeTrackWidth` (demo-library/log-visualizer.ts)**: トラックの幅を計算する。
- **`formatInactiveChannels` (demo-library/log-visualizer.ts)**: 非アクティブなチャンネルをフォーマットする。
- **`createLogVisualizer` (demo-library/log-visualizer.ts)**: ログビジュアライザーインスタンスを作成する。
- **`renderEmpty` (demo-library/log-visualizer.ts)**: 空のビジュアライザーを描画する。
- **`renderFromJson` (demo-library/log-visualizer.ts)**: JSONデータからビジュアライザーを描画する。
- **`ensureGlobalLane` (demo-library/log-visualizer.ts)**: グローバルレーンが確実に存在するようにする。
- **`setLfoRegisters` (demo-library/log-visualizer.ts)**: LFOレジスタを設定する。
- **`setupMmlToSmf` (demo-library/mml-support.ts)**: MMLからSMFへの変換設定を行う。
- **`detectPopNoise` (demo-library/pop-noise-detector.ts)**: ポップノイズを検出する。
- **`getToneEditorGenerator` (demo-library/random-tone.ts)**: トーンエディタージェネレーターを取得する。
- **`generateRandomToneRegisters` (demo-library/random-tone.ts)**: ランダムなYM2151トーンレジスタを生成する。
- **`generateRandomInterpolationPairRegisters` (demo-library/random-tone.ts)**: ランダムな補間ペアレジスタを生成する。
- **`parseAttachmentEntries` (demo-library/random-tone.ts)**: 添付ファイルのエントリをパースする。
- **`validateRandomToneAttachment` (demo-library/random-tone.ts)**: ランダムトーン添付ファイルの妥当性を検証する。
- **`upsertEntryRegisters` (demo-library/random-tone.ts)**: エントリレジスタを更新または挿入する。
- **`upsertAttachmentRegisters` (demo-library/random-tone.ts)**: 添付レジスタを更新または挿入する。
- **`upsertInterpolationAttachmentRegisters` (demo-library/random-tone.ts)**: 補間添付レジスタを更新または挿入する。
- **`buildRandomInterpolationAttachment` (demo-library/random-tone.ts)**: ランダムな補間添付ファイルを構築する。
- **`ensureWasmInitialized` (demo-library/shared-demo.ts)**: WASMモジュールが初期化されていることを確認する。
- **`setStatus` (demo-library/shared-demo.ts)**: ステータスメッセージをUIに設定する。
- **`setEventCountDisplay` (demo-library/shared-demo.ts)**: イベントカウントをUIに表示する。
- **`ensureWebYm2151` (demo-library/shared-demo.ts)**: WebYM2151モジュールがロードされていることを確認する。
- **`clearWebYmAudioCache` (demo-library/shared-demo.ts)**: WebYM2151のオーディオキャッシュをクリアする。
- **`updateOutput` (demo-library/shared-demo.ts)**: 汎用的な出力領域を更新する。
- **`parseAttachmentField` (demo-library/shared-demo.ts)**: 添付ファイルフィールドをパースする。
- **`cleanup` (demo-library/shared-demo.ts)**: リソースのクリーンアップを行う。
- **`mod` (demo-library/shared-demo.ts)**: モジュロ演算を行う。
- **`buildEventsFromCompact` (demo-library/tone-json-attachment.ts)**: コンパクト形式からイベントを構築する。
- **`serializeWithStatus` (demo-library/tone-json-attachment.ts)**: ステータス付きでシリアライズする。
- **`normalizeAttachmentText` (demo-library/tone-json-attachment.ts)**: 添付テキストを正規化する。
- **`convertMmlToSmf` (demo-library/tone-json-demo.ts)**: MMLをSMFに変換する。
- **`getMmlParser` (demo-library/tone-json-mml.ts)**: MMLパーサーを取得する。
- **`getParseTreeJsonToSmf` (demo-library/tone-json-mml.ts)**: パースツリーからJSONへ、さらにSMFへ変換する。
- **`treeToJson` (demo-library/tone-json-mml.ts)**: パースツリーをJSONに変換する。
- **`ensureMmlRuntime` (demo-library/tone-json-mml.ts)**: MMLランタイムが利用可能であることを確認する。
- **`encodeWav` (demo-library/wav-exporter.ts)**: WAVフォーマットにエンコードする。
- **`writeAscii` (demo-library/wav-exporter.ts)**: ASCII文字列を書き込む。
- **`downloadWav` (demo-library/wav-exporter.ts)**: WAVファイルをダウンロードさせる。
- **`drawEmpty` (demo-library/waveform-canvas.ts)**: 空の波形キャンバスを描画する。
- **`drawWaveform` (demo-library/waveform-canvas.ts)**: 波形をキャンバスに描画する。
- **`extractNoteBoundaries` (demo-library/waveform-viewer.ts)**: ノートの境界を抽出する。
- **`normalizeAmplitude` (demo-library/waveform-viewer.ts)**: 振幅を正規化する。
- **`createWaveformViewer` (demo-library/waveform-viewer.ts)**: 波形ビューアのインスタンスを作成する。
- **`getWindowDurS` (demo-library/waveform-viewer.ts)**: ウィンドウの表示時間（秒）を取得する。
- **`clampViewStart` (demo-library/waveform-viewer.ts)**: ビューの開始位置をクランプする。
- **`updatePositionLabel` (demo-library/waveform-viewer.ts)**: 位置表示ラベルを更新する。
- **`render` (demo-library/waveform-viewer.ts)**: 波形ビューアを描画する。
- **`updateBoundariesAndRender` (demo-library/waveform-viewer.ts)**: 境界を更新して再描画する。
- **`synthesizeAndRender` (demo-library/waveform-viewer.ts)**: 波形を合成して描画する。
- **`setZoom` (demo-library/waveform-viewer.ts)**: ズームレベルを設定する。
- **`endDrag` (demo-library/waveform-viewer.ts)**: ドラッグ操作の終了を処理する。
- **`parseHexByte` (demo-library/ym2151-utils.ts)**: 16進数バイトをパースする。

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
Generated at: 2026-04-02 07:17:33 JST

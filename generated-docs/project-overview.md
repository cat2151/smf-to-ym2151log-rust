Last updated: 2026-02-06

# Project Overview

## プロジェクト概要
-   Standard MIDIファイルを、レトロなYM2151 FM音源チップで演奏可能なデータ形式に変換します。
-   ネイティブアプリケーションやWebブラウザ（WebAssembly経由）で利用できる高性能なRustライブラリです。
-   複雑なMIDIイベントやプログラムチェンジにも対応し、カスタム音色定義も可能です。

## 技術スタック
-   フロントエンド: WebAssembly (WASM) によるRustコードのブラウザ実行、TypeScriptでのWebアプリケーションロジック記述、HTML/CSSによるユーザーインターフェース構築、Viteによるモダンなフロントエンド開発。
-   音楽・オーディオ: Standard MIDI Files (SMF) を入力とし、YM2151 FM音源チップ向けのレジスタ書き込みログ（JSON形式）を出力。カスタムYM2151音色定義にはJSONファイルを使用。
-   開発ツール: 主要なプログラミング言語としてRustを使用し、Cargoがビルドと依存関係管理を担います。WebAssemblyへのコンパイルには`wasm-pack`を使用。
-   テスト: Rustの組み込みテストフレームワークと`cargo tarpaulin`によるテストカバレッジ計測、包括的なユニットテストと統合テストを実施。
-   ビルドツール: RustプロジェクトのビルドはCargo、WebAssemblyパッケージのビルドは`wasm-pack`、フロントエンドのアセットビルドはViteを使用。
-   言語機能: Rustの強力な型システムによる堅牢なコードと、所有権システムによるメモリ安全性を確保。
-   自動化・CI/CD: GitHub Pagesの構成ファイル`_config.yml`が含まれており、Webサイトのデプロイ自動化を示唆します。
-   開発標準: `cargo fmt`によるコードフォーマット、`cargo clippy`によるLintチェック、`cargo audit`による依存関係のセキュリティ監査を通じて、高いコード品質を維持。テスト駆動開発を採用。

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📖 DEMO_README.md
📄 LICENSE
📖 MML_INTEGRATION.md
📖 README.ja.md
📖 README.md
📖 WASM_USAGE.md
📄 _config.yml
📁 demo-library/
  📄 .gitignore
  📖 README.md
  🌐 index.html
  📘 library-demo.ts
  📊 package.json
  🎨 style.css
  📊 tsconfig.json
  📘 vite.config.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
🌐 index.html
📁 issue-notes/
  📖 21.md
  📖 22.md
  📖 23.md
  📖 25.md
  📖 28.md
  📖 30.md
  📖 32.md
  📖 33.md
  📖 34.md
  📖 36.md
  📖 38.md
  📖 39.md
  📖 41.md
  📖 43.md
  📖 45.md
  📖 47.md
  📖 49.md
  📖 51.md
  📖 53.md
  📖 55.md
  📖 57.md
  📖 58.md
📊 package.json
📁 src/
  📄 error.rs
  📄 lib.rs
  📄 main.rs
  📘 main.ts
  📁 midi/
    📄 events.rs
    📄 mod.rs
    📄 parser.rs
    📄 utils.rs
  🎨 style.css
  📄 wasm.rs
  📁 ym2151/
    📄 channel_allocation.rs
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
📊 tsconfig.json
📘 vite.config.ts
```

## ファイル詳細説明
-   `.gitignore`: Gitによるバージョン管理から除外するファイルやディレクトリを指定します。
-   `Cargo.lock`: プロジェクトの依存関係の正確なバージョンを固定し、ビルドの再現性を保証します。
-   `Cargo.toml`: Rustプロジェクトの設定ファイルで、プロジェクト名、バージョン、依存クレートなどが記述されています。
-   `DEMO_README.md`: デモに関する追加情報や使用方法を説明するドキュメントです。
-   `LICENSE`: プロジェクトのライセンス情報が記載されています。
-   `MML_INTEGRATION.md`: Music Macro Language (MML) との統合に関する詳細情報が記載されています。
-   `README.ja.md`: プロジェクトの日本語版概要ドキュメントです。
-   `README.md`: プロジェクトの英語版概要ドキュメントです。
-   `WASM_USAGE.md`: WebAssembly (WASM) としてこのライブラリを利用する方法に関する詳細なドキュメントです。
-   `_config.yml`: GitHub Pagesのサイト設定ファイルです。
-   `demo-library/`: WebAssembly版ライブラリの最小限のデモサイト関連ファイル群です。
    -   `demo-library/.gitignore`: デモライブラリ固有のGit除外ファイル指定です。
    -   `demo-library/README.md`: デモライブラリの概要ドキュメントです。
    -   `demo-library/index.html`: デモサイトのメインHTMLファイルです。
    -   `demo-library/library-demo.ts`: デモサイトのTypeScriptロジックで、WASMライブラリの使用例を含みます。
    -   `demo-library/package.json`: デモライブラリのJavaScriptプロジェクト設定ファイルで、依存関係が記述されています。
    -   `demo-library/style.css`: デモサイトのCSSスタイルシートです。
    -   `demo-library/tsconfig.json`: TypeScriptコンパイラの設定ファイルです。
    -   `demo-library/vite.config.ts`: Viteビルドツールの設定ファイルです。
-   `generated-docs/`: プロジェクトの自動生成ドキュメントが格納されるディレクトリです。
-   `googled947dc864c270e07.html`: Googleサイト認証用のファイルです。
-   `index.html`: WebインターフェースデモのメインHTMLファイルです。
-   `issue-notes/`: 開発中の特定の課題や設計上のメモが記録されているディレクトリです。
-   `package.json`: JavaScript/TypeScript関連の依存関係やスクリプトが定義されているファイルです。
-   `src/`: プロジェクトの主要なRustソースコードが格納されているディレクトリです。
    -   `src/error.rs`: エラーハンドリングに関連する定義が含まれています。
    -   `src/lib.rs`: ライブラリのエントリーポイント。主要な公開APIが含まれます。
    -   `src/main.rs`: コマンドラインアプリケーションのエントリーポイントです。
    -   `src/main.ts`: Webインターフェースデモの主要なTypeScriptロジックです。
    -   `src/midi/`: MIDIファイル解析に関連するモジュール群です。
        -   `src/midi/events.rs`: MIDIイベントのデータ構造を定義します。
        -   `src/midi/mod.rs`: `midi`モジュールのルートファイルです。
        -   `src/midi/parser.rs`: Standard MIDIファイルを解析し、中間イベントに変換するロジックが含まれます（パスA）。
        -   `src/midi/utils.rs`: MIDI関連のユーティリティ関数が含まれます。
    -   `src/style.css`: WebインターフェースデモのCSSスタイルシートです。
    -   `src/wasm.rs`: WebAssemblyバインディングのためのコードが含まれています。JavaScriptからRust関数を呼び出すためのインターフェースを提供します。
    -   `src/ym2151/`: YM2151関連の変換ロジックが格納されているモジュール群です。
        -   `src/ym2151/channel_allocation.rs`: YM2151チャンネルの割り当て戦略を実装します。
        -   `src/ym2151/converter.rs`: 中間イベントをYM2151レジスタログに変換する主要なロジックが含まれます（パスB）。
        -   `src/ym2151/converter_tests.rs`: `converter.rs`のテストコードです。
        -   `src/ym2151/event_processor.rs`: YM2151イベントの処理ロジックが含まれます。
        -   `src/ym2151/events.rs`: YM2151レジスタイベントのデータ構造を定義します。
        -   `src/ym2151/init.rs`: YM2151初期化に関連するロジックが含まれます。
        -   `src/ym2151/mod.rs`: `ym2151`モジュールのルートファイルです。
        -   `src/ym2151/note_table.rs`: ノート周波数などのテーブルデータが含まれます。
        -   `src/ym2151/tempo_map.rs`: テンポマップの管理ロジックが含まれます。
        -   `src/ym2151/tone.rs`: YM2151の音色（トーン）に関する定義と読み込みロジックが含まれます。
-   `tests/`: 統合テスト関連のファイル群です。
    -   `tests/create_test_midi.py`: テスト用のMIDIファイルを生成するPythonスクリプトです。
    -   `tests/integration_tests.rs`: プロジェクト全体の統合テストコードです。
    -   `tests/test_data/`: テストに使用されるMIDIファイルなどのデータが格納されています。
-   `tones/`: プログラムチェンジで利用されるカスタムYM2151音色定義JSONファイル群です。
    -   `tones/000.json`: プログラム番号000に対応するYM2151音色定義です。
    -   `tones/README.md`: `tones`ディレクトリ内のJSONファイルのフォーマットと使用方法を説明します。
-   `tsconfig.json`: ルートディレクトリのTypeScriptコンパイラ設定ファイルです。
-   `vite.config.ts`: ルートディレクトリのViteビルドツールの設定ファイルです。

## 関数詳細説明
-   `catch` (demo-library/library-demo.ts, src/main.ts)
    -   **役割**: エラー発生時の処理を捕捉し、適切にハンドリングします。通常、エラーメッセージの表示などを行います。
    -   **引数**: エラーオブジェクト。
    -   **戻り値**: なし。
-   `initWasm` (demo-library/library-demo.ts, src/main.ts)
    -   **役割**: WebAssemblyモジュールを初期化し、Rustでコンパイルされた機能をJavaScript環境で利用可能にします。
    -   **引数**: なし、またはWASMモジュールのパスなど。
    -   **戻り値**: 初期化されたWASMモジュールオブジェクト。
-   `displayResult` (demo-library/library-demo.ts, src/main.ts)
    -   **役割**: 変換結果（YM2151レジスタログJSONなど）をWebインターフェース上に表示します。
    -   **引数**: 変換結果データ。
    -   **戻り値**: なし。
-   `showError` (demo-library/library-demo.ts, src/main.ts)
    -   **役割**: エラーメッセージをWebインターフェース上に表示します。
    -   **引数**: エラーメッセージ文字列。
    -   **戻り値**: なし。
-   `setupFileInput` (demo-library/library-demo.ts, src/main.ts)
    -   **役割**: ファイル入力要素を設定し、ユーザーがMIDIファイルをアップロードした際のイベントハンドリングを定義します。
    -   **引数**: なし。
    -   **戻り値**: なし。
-   `checkMMLWasm` (src/main.ts)
    -   **役割**: Music Macro Language (MML) 関連のWebAssembly機能が利用可能かどうかをチェックします。
    -   **引数**: なし。
    -   **戻り値**: 真偽値（利用可能ならtrue）。
-   `convertMML` (src/main.ts)
    -   **役割**: MML形式の入力をYM2151ログに変換します。
    -   **引数**: MMLデータ文字列。
    -   **戻り値**: 変換されたYM2151ログデータ。
-   `loadMMLExample` (src/main.ts)
    -   **役割**: MMLのサンプルデータをロードし、入力フィールドに表示します。
    -   **引数**: なし。
    -   **戻り値**: なし。
-   `switchTab` (src/main.ts)
    -   **役割**: Webインターフェースのタブを切り替える機能を実装します。
    -   **引数**: 選択されたタブのIDなど。
    -   **戻り値**: なし。
-   `setupEventListeners` (src/main.ts)
    -   **役割**: Webページ内の様々なUI要素（ボタン、入力フィールドなど）に対するイベントリスナーを設定します。
    -   **引数**: なし。
    -   **戻り値**: なし。

## 関数呼び出し階層ツリー
```
- catch (demo-library/library-demo.ts)
  - initWasm (demo-library/library-demo.ts)
    - displayResult ()
      - showError ()
      - setupFileInput ()
      - checkMMLWasm ()
      - convertMML ()
      - loadMMLExample ()
      - switchTab ()
      - setupEventListeners ()

---
Generated at: 2026-02-06 07:11:13 JST

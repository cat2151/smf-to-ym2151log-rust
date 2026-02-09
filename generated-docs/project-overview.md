Last updated: 2026-02-10

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製のツールおよびライブラリです。
- WebAssembly (WASM) に対応しており、ブラウザ環境での利用や、他のRustプロジェクトからのライブラリとしての統合が可能です。
- 和音数ベースの静的チャンネル割り当て戦略とドラムチャンネル優先割り当てにより、YM2151の8チャンネルを効率的に利用し、音質を最適化します。

## 技術スタック
- フロントエンド:
    - **TypeScript**: `demo-library`のロジック開発に使用され、WASMモジュールとの連携を強化します。
    - **HTML/CSS**: `demo-library`のウェブインターフェースの構造とスタイルを定義します。
    - **Vite**: `demo-library`の高速な開発サーバーとビルドツールとして使用されます。
    - **WebAssembly (WASM)**: RustコードをWebブラウザで実行可能にするための技術で、高性能なMIDI変換をブラウザで実現します。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: 入力として扱われる標準的なMIDIファイル形式です（Format 0および1をサポート）。
    - **YM2151**: ヤマハ製のFM音源チップで、本プロジェクトの出力形式であるレジスタログのターゲットです。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語であり、型安全性と高パフォーマンスを実現します。
    - **Cargo**: Rustのビルドシステムおよびパッケージマネージャーで、依存関係の管理、ビルド、テスト、ドキュメント生成を行います。
    - **wasm-pack**: RustからWebAssemblyを生成するためのツールで、WASMのビルドとパッケージングを簡素化します。
    - **Git**: ソースコードのバージョン管理に使用されます。
    - **Python**: テスト用のMIDIファイルを生成するためのスクリプト (`create_test_midi.py`) に使用されます。
- テスト:
    - **cargo test**: Rustプロジェクトのユニットテストおよび統合テストを実行するための標準コマンドです。
    - **cargo tarpaulin**: コードカバレッジを測定し、テスト網羅率を可視化するためのツールです。
- ビルドツール:
    - **Cargo**: Rustコードのコンパイルと実行可能ファイルの生成を行います。
    - **wasm-pack**: WebAssemblyモジュールのビルドとJavaScriptバインディングの生成を行います。
    - **Vite**: `demo-library`のフロントエンド資産をビルドします。
- 言語機能:
    - **Rustの型システム**: 堅牢なアプリケーション開発をサポートし、コンパイル時に多くのエラーを検出します。
    - **Rustのパフォーマンス特性**: ネイティブコンパイルにより高速なMIDI変換処理を実現します。
- 開発標準:
    - **cargo fmt**: Rustコードのフォーマットを自動的にチェックし、統一されたコーディングスタイルを維持します。
    - **cargo clippy**: Rustコードのlintチェックを行い、潜在的なバグや非効率なコードパターンを検出します。
    - **cargo audit**: プロジェクトの依存関係に存在する既知のセキュリティ脆弱性をチェックします。

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
  🌐 index.html
  📘 library-demo.ts
  📊 package-lock.json
  📊 package.json
  🎨 style.css
  📊 tsconfig.json
  📘 vite.config.ts
📁 generated-docs/
🌐 googled947dc864c270e07.html
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
  📖 61.md
  📖 63.md
  📖 65.md
  📖 66-resolution.md
  📖 66.md
  📖 68.md
  📖 70.md
  📖 72.md
  📖 74.md
  📖 75.md
  📖 77.md
  📖 79.md
  📖 81.md
  📖 83.md
  📖 84.md
  📖 88.md
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
-   **.gitignore**: Gitがバージョン管理から無視するファイルやディレクトリのパターンを定義します。
-   **Cargo.lock**: ビルド時に使用されたすべての依存クレートの正確なバージョンとチェックサムを記録し、再現性のあるビルドを保証します。
-   **Cargo.toml**: Rustプロジェクトのメタデータ、依存関係、およびビルド設定を記述するマニフェストファイルです。
-   **LICENSE**: プロジェクトの配布および使用に関するライセンス情報を提供します。
-   **README.ja.md**: プロジェクトの概要、使い方、機能などを日本語で説明する主要なドキュメントです。
-   **README.md**: プロジェクトの概要、使い方、機能などを英語で説明する主要なドキュメントです。
-   **WASM_USAGE.md**: WebAssembly (WASM) としてこのライブラリをブラウザやJavaScript環境で使用するための詳細な手順とサンプルコードを提供します。
-   **_config.yml**: GitHub Pagesのサイト設定ファイルで、静的サイトの生成オプションを定義します。
-   **demo-library/**: プロジェクトのWebAssembly (WASM) 版ライブラリのデモンストレーションを行うためのフロントエンドアプリケーションを格納するディレクトリです。
    -   **.gitignore**: `demo-library`ディレクトリ固有のGit無視ファイル設定です。
    -   **index.html**: `demo-library`のウェブページのルートファイルで、WASMデモのUIとエントリーポイントを提供します。
    -   **library-demo.ts**: `demo-library`の主要なロジックを実装するTypeScriptファイルで、WASMモジュールとの連携、ファイル処理、UI更新を管理します。
    -   **package-lock.json**: `demo-library`のnpm依存関係の正確なツリー構造を記録し、パッケージのバージョン固定を保証します。
    -   **package.json**: `demo-library`のnpmパッケージ定義ファイルで、プロジェクトのメタデータ、スクリプト、依存関係を記述します。
    -   **style.css**: `demo-library`のウェブページのルックアンドフィールを定義するスタイルシートです。
    -   **tsconfig.json**: `demo-library`のTypeScriptコンパイラ設定ファイルで、コンパイルオプションを指定します。
    -   **vite.config.ts**: `demo-library`を開発・ビルドするためのVite設定ファイルです。
-   **generated-docs/**: RustプロジェクトのAPIドキュメントが自動生成されるディレクトリです（`cargo doc`コマンドで生成）。
-   **googled947dc864c270e07.html**: Googleのサイト所有権確認のための認証ファイルです。
-   **issue-notes/**: 開発中に発生した課題、検討事項、設計上のメモなどを記録したMarkdownファイル群です。
-   **package-lock.json**: プロジェクト全体のnpm依存関係の正確なツリー構造を記録し、パッケージのバージョン固定を保証します。`demo-library`のものとは別です。
-   **package.json**: プロジェクト全体のnpmパッケージ定義ファイルで、プロジェクトのメタデータ、スクリプト、依存関係を記述します。`demo-library`のものとは別です。
-   **src/**: Rustのソースコードが配置されているディレクトリです。
    -   **error.rs**: プロジェクト固有のエラー型とエラー処理ロジックを定義します。
    -   **lib.rs**: ライブラリクレートのエントリーポイントで、MIDIファイル解析やYM2151ログ変換といった主要な公開APIを定義します。
    -   **main.rs**: コマンドラインアプリケーションのエントリーポイントで、`lib.rs`のライブラリ機能を利用してファイル変換を実行します。
    -   **midi/**: MIDIファイルに関連するデータ構造、パーサー、ユーティリティ関数を格納するモジュールです。
        -   **events.rs**: Standard MIDI File (SMF) 内の様々なイベント（ノートオン/オフ、テンポチェンジなど）のデータ構造を定義します。
        -   **mod.rs**: `midi`モジュールのルートファイルで、他のサブモジュールを公開し、モジュール構造を定義します。
        -   **parser.rs**: SMFファイルを解析し、内部的なMIDIイベントのシーケンスに変換するロジックを実装します。
        -   **utils.rs**: MIDI関連のヘルパー関数やユーティリティロジックを提供します。
    -   **wasm.rs**: WebAssembly (WASM) 向けにRustの関数をJavaScriptから呼び出せるようにするためのバインディングを定義します。
    -   **ym2151/**: YM2151 FM音源チップに関連する変換ロジック、イベント処理、チャンネル割り当てなどを格納するモジュールです。
        -   **channel_allocation.rs**: MIDIチャンネルの和音数に基づいてYM2151の8チャンネルを割り当てるロジックを実装します。ドラムチャンネル優先などの戦略を含みます。
        -   **converter.rs**: 中間的なMIDIイベントデータからYM2151レジスタ書き込みログへの変換処理全体を管理します。
        -   **converter_tests.rs**: `converter.rs`で実装された変換ロジックのユニットテストを格納します。
        -   **event_processor.rs**: 個々のMIDIイベントをYM2151レジスタ書き込みに変換する具体的な処理を実装します。
        -   **events.rs**: YM2151のレジスタアドレスとデータ、および関連するイベントのデータ構造を定義します。
        -   **init.rs**: YM2151チップの初期化に必要なレジスタ設定データなどを定義します。
        -   **mod.rs**: `ym2151`モジュールのルートファイルで、他のサブモジュールを公開し、モジュール構造を定義します。
        -   **note_table.rs**: MIDIノート番号とYM2151の周波数設定値（オペレーターパラメータ）とのマッピングテーブルを管理します。
        -   **tempo_map.rs**: MIDIファイル内のテンポチェンジイベントを処理し、時間軸（ティックからミリ秒）への変換マップを管理します。
        -   **tone.rs**: MIDIプログラムチェンジに対応するYM2151の音色（トーン）データの読み込みと管理を扱います。
-   **tests/**: プロジェクトの統合テストおよびテスト用データが格納されているディレクトリです。
    -   **create_test_midi.py**: テスト目的で特定のMIDIシーケンスを生成するためのPythonスクリプトです。
    -   **integration_tests.rs**: プロジェクト全体としての機能が正しく連携しているかを検証する統合テストを記述します。
    -   **test_data/**: 統合テストやユニットテストで使用されるサンプルMIDIファイルなどのテスト用データを格納します。
-   **tones/**: MIDIプログラムチェンジイベントに対応するカスタムYM2151音色（トーン）を定義したJSONファイル群を格納するディレクトリです。
    -   **000.json**: MIDIプログラム0番（アコースティックグランドピアノ）に対応するYM2151音色の設定を定義したJSONファイルです。
    -   **README.md**: `tones`ディレクトリ内のJSONファイル形式、カスタム音色の作成方法、および使用方法について説明するドキュメントです。

## 関数詳細説明
-   **initWasm()** (demo-library/library-demo.ts):
    -   **役割**: WebAssemblyモジュールを初期化し、Rustで実装されたMIDI変換機能をブラウザで使用可能にします。
    -   **引数**: なし
    -   **戻り値**: Promise<void> (非同期操作の完了を示す)
    -   **機能**: `smf_to_ym2151log.js`をインポートし、WebAssembly環境をセットアップします。
-   **displayResult()** (demo-library/library-demo.ts):
    -   **役割**: MIDIファイル変換処理の結果をウェブページのUIに表示します。
    -   **引数**: なし
    -   **戻り値**: なし
    -   **機能**: 変換後のJSONデータや処理メッセージを特定のHTML要素にレンダリングします。
-   **showError()** (demo-library/library-demo.ts):
    -   **役割**: エラーメッセージをウェブページのUIに表示します。
    -   **引数**: error: any (表示するエラーオブジェクトまたはメッセージ)
    -   **戻り値**: なし
    -   **機能**: 発生したエラーをユーザーに分かりやすい形で通知するため、エラーメッセージを特定のHTML要素に表示します。
-   **setupFileInput()** (demo-library/library-demo.ts):
    -   **役割**: ファイル入力フィールドのイベントリスナーを設定し、ユーザーがMIDIファイルを選択した際の処理をトリガーします。
    -   **引数**: なし
    -   **戻り値**: なし
    -   **機能**: ファイル入力フィールドへの変更（ファイル選択）を監視し、ファイルが選択されると変換処理を開始します。
-   **catch** (demo-library/library-demo.ts):
    -   **役割**: Promiseチェーンにおけるエラーハンドリングメカニズムの一部として機能します。
    -   **引数**: error: any (前のPromiseで発生したエラー)
    -   **戻り値**: Promise<void> (エラー処理が完了したPromise)
    -   **機能**: 非同期操作中に発生した例外を捕捉し、指定されたエラーハンドラ（例: `showError`）を実行します。
-   **if** (demo-library/library-demo.ts):
    -   **役割**: 条件分岐ロジックの一部として機能します。
    -   **引数**: 条件式 (真偽値に評価される式)
    -   **戻り値**: なし (実行フローを制御する言語構造)
    -   **機能**: 指定された条件式が真の場合にのみ、特定のコードブロックを実行します。

## 関数呼び出し階層ツリー
```
- catch (demo-library/library-demo.ts)
  - initWasm (demo-library/library-demo.ts)
    - displayResult ()
      - showError ()
      - setupFileInput ()
- if (demo-library/library-demo.ts)

---
Generated at: 2026-02-10 07:16:30 JST

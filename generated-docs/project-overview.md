Last updated: 2026-02-09

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) を、ヤマハFM音源チップYM2151のレジスタ書き込みログ（JSON形式）に変換するRust製のツールです。
- ネイティブアプリケーション向けライブラリ、およびWebブラウザ向けのWebAssembly (WASM) ライブラリとして利用可能です。
- MIDIプログラムチェンジに対応し、外部音色ファイルによるカスタムYM2151音色の適用もサポートします。

## 技術スタック
- フロントエンド: HTML (デモUI)、CSS (デモスタイリング)、TypeScript (デモロジック)、Vite (デモビルドツール)、WebAssembly (WASM) (Rustコードのブラウザ実行)
- 音楽・オーディオ: Standard MIDI Files (SMF) (入力フォーマット)、YM2151 FM音源チップ (ターゲット音源)、JSON (中間イベントおよび最終出力ログ形式)
- 開発ツール: Rust (プログラミング言語)、Cargo (Rustのビルドシステムとパッケージマネージャ)、wasm-pack (RustをWASMにビルドするツール)、Git (バージョン管理)
- テスト: Cargo test (Rustの標準テストフレームワーク)、cargo tarpaulin (テストカバレッジ計測ツール)
- ビルドツール: Cargo (Rustプロジェクトのビルド)、wasm-pack (WASMパッケージのビルド)、Vite (デモプロジェクトのビルド)
- 言語機能: Rust (型安全性、高パフォーマンスを実現)、TypeScript (フロントエンド開発における型安全性)
- 自動化・CI/CD: 明示的なCI/CDツールは記述されていませんが、`cargo fmt --check`や`cargo clippy`は自動化された品質チェックに利用されます。
- 開発標準: cargo fmt (コードフォーマッタ)、cargo clippy (Linter)、cargo audit (セキュリティ監査)

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
🌐 googled947dc864c270e07.html
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
- **`.gitignore`**: Gitが追跡しないファイルやディレクトリを指定する設定ファイルです。
- **`Cargo.lock`**: Rustプロジェクトの依存関係の正確なバージョンを記録し、ビルドの一貫性を保証します。
- **`Cargo.toml`**: Rustプロジェクトのマニフェストファイルで、プロジェクト名、バージョン、依存関係、ビルド設定などを定義します。
- **`LICENSE`**: プロジェクトの配布条件を定めるライセンス情報です。
- **`README.ja.md`**: プロジェクトの日本語による概要と使用方法を説明するドキュメントです。
- **`README.md`**: プロジェクトの英語による概要と使用方法を説明するドキュメントです。
- **`WASM_USAGE.md`**: WebAssembly (WASM) としてこのライブラリを使用するための詳細な手順と例が記載されています。
- **`_config.yml`**: GitHub Pages の設定ファイルであり、Webサイトのビルド方法やテーマなどを定義します。
- **`demo-library/`**: WebAssembly版ライブラリの動作を確認できるデモアプリケーションが含まれるディレクトリです。
    - **`demo-library/.gitignore`**: デモアプリケーションのビルド成果物などをGit管理から除外します。
    - **`demo-library/index.html`**: デモアプリケーションのメインとなるWebページです。
    - **`demo-library/library-demo.ts`**: デモアプリケーションのフロントエンドロジックをTypeScriptで記述したファイルで、WASMライブラリをロードしてMIDI変換を実行します。
    - **`demo-library/package-lock.json`**: デモアプリケーションのNode.js依存関係の正確なバージョンを記録します。
    - **`demo-library/package.json`**: デモアプリケーションのNode.jsプロジェクトのマニフェストファイルです。
    - **`demo-library/style.css`**: デモアプリケーションの見た目を定義するスタイルシートです。
    - **`demo-library/tsconfig.json`**: TypeScriptコンパイラの設定ファイルです。
    - **`demo-library/vite.config.ts`**: デモアプリケーションのビルドツールViteの設定ファイルです。
- **`googled947dc864c270e07.html`**: Googleサイト認証用のファイルです。
- **`package-lock.json`**: プロジェクトルートにあるNode.js依存関係の正確なバージョンを記録するファイルです。
- **`package.json`**: プロジェクトルートにあるNode.jsプロジェクトのマニフェストファイルです。
- **`src/`**: Rustのソースコードが格納されているディレクトリです。
    - **`src/error.rs`**: プロジェクト固有のエラー型を定義します。
    - **`src/lib.rs`**: ライブラリクレートのエントリポイントであり、このプロジェクトの主要な変換ロジックやAPIを提供します。
    - **`src/main.rs`**: コマンドラインツールとしてアプリケーションを実行する際のエントリポイントです。
    - **`src/midi/`**: Standard MIDI File (SMF) の解析に関連するモジュールが含まれます。
        - **`src/midi/events.rs`**: MIDIイベントのデータ構造を定義します。
        - **`src/midi/mod.rs`**: `midi`モジュールのエントリポイントです。
        - **`src/midi/parser.rs`**: SMFを読み込み、内部的なMIDIイベント構造に変換するロジックを含みます。
        - **`src/midi/utils.rs`**: MIDIデータ処理に役立つユーティリティ関数を提供します。
    - **`src/wasm.rs`**: WebAssembly (WASM) 向けにRustの関数を公開するためのバインディングを定義します。
    - **`src/ym2151/`**: YM2151 FM音源チップのレジスタログ変換に関連するモジュールが含まれます。
        - **`src/ym2151/channel_allocation.rs`**: MIDIチャンネルをYM2151の8つのオペレータチャンネルに割り当てる戦略とロジックを実装します。
        - **`src/ym2151/converter.rs`**: MIDIイベントをYM2151レジスタ書き込みログに変換する主要なロジックを担います。
        - **`src/ym2151/converter_tests.rs`**: `converter.rs`モジュールのテストコードです。
        - **`src/ym2151/event_processor.rs`**: YM2151のイベントを処理し、レジスタ書き込みに変換します。
        - **`src/ym2151/events.rs`**: YM2151のレジスタ書き込みイベントのデータ構造を定義します。
        - **`src/ym2151/init.rs`**: YM2151チップの初期化に関連するレジスタ設定ロジックを含みます。
        - **`src/ym2151/mod.rs`**: `ym2151`モジュールのエントリポイントです。
        - **`src/ym2151/note_table.rs`**: MIDIノート番号とYM2151の周波数設定値間のマッピングを管理します。
        - **`src/ym2151/tempo_map.rs`**: MIDIファイル内のテンポ変化を追跡し、正確な時間計算を提供します。
        - **`src/ym2151/tone.rs`**: YM2151音色のデータ構造と、外部JSONファイルからの音色ロードロジックを扱います。
- **`tests/`**: 統合テストおよびテストに使用されるデータが格納されています。
    - **`tests/create_test_midi.py`**: テスト用のMIDIファイルをプログラム的に生成するためのPythonスクリプトです。
    - **`tests/integration_tests.rs`**: プロジェクト全体の機能を検証する統合テストコードです。
    - **`tests/test_data/`**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
- **`tones/`**: MIDIプログラムチェンジに対応したカスタムYM2151音色を定義するJSONファイルが格納されています。
    - **`tones/000.json`**: プログラム番号000番に割り当てられたカスタムYM2151音色データです。
    - **`tones/README.md`**: `tones`ディレクトリ内のカスタム音色ファイルフォーマットに関する説明です。

## 関数詳細説明
- **`catch` (demo-library/library-demo.ts)**:
    - 役割: 非同期処理中に発生したエラーを捕捉し、適切に処理します。
    - 引数: エラーオブジェクト (通常は `(error: any) => void` 形式のコールバック関数)。
    - 戻り値: Promiseチェーンを継続するためのPromiseオブジェクト。
- **`initWasm` (demo-library/library-demo.ts)**:
    - 役割: WebAssemblyモジュールを初期化し、Rustの変換ロジックをブラウザ環境で利用可能にします。
    - 引数: なし (または、WASMファイルのパスなど、初期化に必要な設定)。
    - 戻り値: Promise<void> (非同期処理のため)。
- **`displayResult` (demo-library/library-demo.ts)**:
    - 役割: MIDIファイル変換が成功した際に、その結果（中間イベントやYM2151ログ）をWebページ上に表示します。
    - 引数: 変換された結果データ。
    - 戻り値: なし。
- **`showError` (demo-library/library-demo.ts)**:
    - 役割: MIDIファイル変換プロセス中に発生したエラーメッセージをWebページのユーザーインターフェースに表示します。
    - 引数: 表示するエラーメッセージ文字列。
    - 戻り値: なし。
- **`setupFileInput` (demo-library/library-demo.ts)**:
    - 役割: Webページ上のファイル入力要素（MIDIファイルをアップロードする部分）を初期設定し、ファイルが選択されたときのイベントハンドラを登録します。
    - 引数: なし。
    - 戻り値: なし。
- **`if` (demo-library/library-demo.ts)**:
    - 役割: プログラミング言語における条件分岐の構文であり、特定の条件が真の場合にのみコードブロックを実行します。これは関数ではなく、TypeScriptの基本的な制御フローです。
    - 引数: 評価される条件式。
    - 戻り値: なし (コードの実行パスを決定する)。

## 関数呼び出し階層ツリー
```
- catch (demo-library/library-demo.ts)
  - initWasm (demo-library/library-demo.ts)
    - displayResult ()
      - showError ()
      - setupFileInput ()
- if (demo-library/library-demo.ts)

---
Generated at: 2026-02-09 07:09:49 JST

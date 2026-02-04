Last updated: 2026-02-05

# Project Overview

## プロジェクト概要
- Standard MIDIファイル（SMF）をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製ツールおよびライブラリです。
- MIDIチャンネルの和音数に基づくYM2151チャンネル割り当てやドラムチャンネル優先処理などの高度なボイス管理機能を持ちます。
- WebAssembly対応によるブラウザ実行、カスタム音色ファイルによるプログラムチェンジ対応、そしてライブラリAPI提供により多様な用途で利用可能です。

## 技術スタック
- フロントエンド:
    - **WebAssembly (WASM)**: RustコードをWebブラウザで実行可能にする技術です。
    - **TypeScript**: Webデモ（`src/main.ts`）のロジックを記述するために使用されるJavaScriptのスーパーセットです。
    - **HTML/CSS**: WebAssemblyデモのユーザーインターフェース（`index.html`, `src/style.css`）を構築するために使用されます。
    - **Vite**: WebAssemblyデモの高速な開発とビルドを可能にする、モダンなフロントエンド開発サーバーおよびビルドツールです。
- 音楽・オーディオ:
    - **Standard MIDI Files (SMF)**: プロジェクトの入力となる、音楽データの標準フォーマットです。
    - **YM2151**: 出力ターゲットとなるヤマハ製のFM音源チップで、そのレジスタ制御ログを生成します。
- 開発ツール:
    - **Rust**: プロジェクトの主要なプログラミング言語であり、型安全性と高パフォーマンスを提供します。
    - **Cargo**: Rustの公式なビルドシステムとパッケージマネージャです。
    - **git**: ソースコードのバージョン管理に使用されます。
    - **wasm-pack**: RustプロジェクトをWebAssemblyにビルドし、npmパッケージとして公開するためのツールです。
- テスト:
    - **cargo test**: Rustの標準テストフレームワークで、ユニットテストと統合テストの実行に使用されます。
    - **cargo tarpaulin**: Rustプロジェクトのコードカバレッジを測定し、レポートを生成するツールです。
- ビルドツール:
    - **Cargo**: Rustのソースコードをコンパイルし、実行可能ファイルやライブラリを生成します。
    - **wasm-pack**: RustのコードをWebAssemblyモジュールに変換する際に使用されます。
    - **Vite**: Webアプリケーションのビルドプロセスを管理します。
- 言語機能:
    - **Rustの型システム**: コンパイル時に多くのバグを捕捉し、堅牢で安全なコードを作成するのに役立ちます。
- 自動化・CI/CD:
    - 特に明示的なCI/CDパイプラインは記述されていませんが、以下のツールは自動化された品質チェックに利用できます。
- 開発標準:
    - **cargo fmt**: Rustコードの自動フォーマットを行い、コードスタイルの一貫性を保ちます。
    - **cargo clippy**: Rustコードの一般的な間違いや非効率性を検出するLintツールです。
    - **cargo audit**: プロジェクトの依存関係に既知のセキュリティ脆弱性がないかを確認します。

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
- **.gitignore**: Gitが追跡しないファイルやディレクトリを指定する設定ファイルです。
- **Cargo.lock**: プロジェクトの依存関係の正確なバージョンを記録し、再現可能なビルドを保証するファイルです。
- **Cargo.toml**: Rustプロジェクトの設定ファイルで、プロジェクト名、バージョン、依存関係などが定義されています。
- **DEMO_README.md**: デモの使用方法に関する詳細を記述したMarkdownドキュメントです。
- **LICENSE**: プロジェクトのライセンス情報が記述されています。
- **MML_INTEGRATION.md**: MML (Music Macro Language) との統合に関する詳細を記述したドキュメントです。
- **README.ja.md**: プロジェクトの概要と使い方を日本語で説明するメインのドキュメントです。
- **README.md**: プロジェクトの概要と使い方を英語で説明するメインのドキュメントです。
- **WASM_USAGE.md**: WebAssembly版の使用方法に関する詳細を記述したMarkdownドキュメントです。
- **_config.yml**: GitHub Pagesなどのサイト設定に使用される設定ファイルです。
- **generated-docs/**: `cargo doc`などで生成されたAPIドキュメントが格納されるディレクトリです。
- **googled947dc864c270e07.html**: Googleサイト認証のために使用されるHTMLファイルです。
- **index.html**: WebAssemblyデモのメインページとなるHTMLファイルで、ブラウザから変換ツールを試せるインターフェースを提供します。
- **issue-notes/**: 開発中の課題やメモを記録したMarkdownファイルが格納されているディレクトリです。
    - **21.md** ～ **58.md**: 個別の課題やメモを記述したファイルです。
- **package.json**: Node.jsプロジェクトの設定ファイルで、フロントエンドの依存関係やスクリプトが定義されています。
- **src/**: RustとTypeScriptのソースコードが格納されている主要なディレクトリです。
    - **error.rs**: プロジェクト全体で発生する可能性のあるカスタムエラー型が定義されています。
    - **lib.rs**: このプロジェクトがライブラリとして提供される際のエントリポイントとなるRustソースファイルです。
    - **main.rs**: コマンドラインツールとして実行される際のエントリポイントとなるRustソースファイルです。
    - **main.ts**: WebAssemblyデモのフロントエンドロジックを実装するTypeScriptファイルです。ファイル選択、変換実行、結果表示などのUI操作を扱います。
    - **style.css**: WebAssemblyデモのUIデザインを定義するCSSファイルです。
    - **wasm.rs**: RustコードをWebAssemblyに公開するためのブリッジとなるRustソースファイルです。
    - **midi/**: MIDIファイルのパースとイベント処理に関連するRustモジュールです。
        - **events.rs**: Standard MIDI File (SMF) から抽出される各種MIDIイベントのデータ構造を定義します。
        - **mod.rs**: `midi`モジュール内のサブモジュール（`events`, `parser`, `utils`）を宣言します。
        - **parser.rs**: MIDIファイルのバイナリデータを読み込み、構造化されたMIDIイベントに解析するロジックを実装します。
        - **utils.rs**: MIDI処理に役立つヘルパー関数やユーティリティが含まれています。
    - **ym2151/**: YM2151レジスタログへの変換ロジックに関連するRustモジュールです。
        - **channel_allocation.rs**: MIDIチャンネルからYM2151の8つのボイスチャンネルへの割り当て戦略（和音数ベース、ドラム優先など）を管理します。
        - **converter.rs**: MIDIイベントをYM2151のレジスタ書き込みログに変換する主要なロジックを実装します。
        - **converter_tests.rs**: `converter.rs`モジュール内の機能に対するユニットテストが含まれています。
        - **event_processor.rs**: 変換プロセスにおいて、YM2151イベントを順序立てて処理するロジックを提供します。
        - **events.rs**: YM2151レジスタ書き込みイベントのデータ構造を定義します。
        - **init.rs**: YM2151チップの初期化レジスタ設定に関するロジックが含まれています。
        - **mod.rs**: `ym2151`モジュール内のサブモジュールを宣言します。
        - **note_table.rs**: MIDIノート番号とYM2151が必要とする周波数やレジスタ値のマッピングを提供します。
        - **tempo_map.rs**: MIDIファイルのテンポチェンジイベントを管理し、正確なタイミング計算を可能にするテンポマップを構築します。
        - **tone.rs**: YM2151の音色（プログラムチェンジによって切り替わる音色データ）のロードと管理を行います。
- **tests/**: プロジェクトの統合テストやテストデータが格納されているディレクトリです。
    - **create_test_midi.py**: テスト目的で特定のMIDIファイルを生成するためのPythonスクリプトです。
    - **integration_tests.rs**: プロジェクト全体の主要な機能に対する統合テストケースを定義します。
    - **test_data/**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
        - **multi_channel.mid**: 複数のMIDIチャンネルが使用されているテスト用MIDIファイルです。
        - **multi_track.mid**: 複数のトラックが使用されているテスト用MIDIファイルです。
        - **program_change.mid**: プログラムチェンジイベントを含むテスト用MIDIファイルです。
        - **simple_melody.mid**: 単純なメロディを含むテスト用MIDIファイルです。
        - **tempo_change.mid**: テンポチェンジイベントを含むテスト用MIDIファイルです。
- **tones/**: カスタムYM2151音色を定義するJSONファイルが格納されているディレクトリです。
    - **000.json**: MIDIプログラム番号0に対応するデフォルトのYM2151音色データです。
    - **README.md**: `tones`ディレクトリ内のJSONファイルのフォーマットや使用方法を説明するドキュメントです。
- **tsconfig.json**: TypeScriptコンパイラの設定ファイルです。
- **vite.config.ts**: Viteビルドツール用の設定ファイルで、フロントエンドのビルドオプションを定義します。

## 関数詳細説明
- **initWasm** (src/main.ts): WebAssemblyモジュールを初期化し、ブラウザ上でRustコードを実行可能にする準備を行います。
    - 役割: WASMモジュールのロードと初期化。
    - 引数: なし
    - 戻り値: Promise<void>
- **checkMMLWasm** (src/main.ts): MML-WASMモジュールのロード状態や準備状況をチェックします。
    - 役割: MML関連のWebAssembly機能が利用可能か確認する。
    - 引数: なし
    - 戻り値: Promise<void>
- **displayResult** (src/main.ts): 変換処理の結果（YM2151ログJSONや中間イベントJSON）をWebページの指定された領域に表示します。
    - 役割: UIに結果を表示する。
    - 引数: `result`: 表示する文字列。
    - 戻り値: `void`
- **showError** (src/main.ts): エラーメッセージをWebページのユーザーインターフェースに表示し、ユーザーに問題が発生したことを通知します。
    - 役割: UIにエラーメッセージを表示する。
    - 引数: `message`: 表示するエラーメッセージの文字列。
    - 戻り値: `void`
- **setupFileInput** (src/main.ts): MIDIファイル入力フィールドのイベントリスナーを設定し、ユーザーがファイルをアップロードした際の処理を定義します。
    - 役割: ファイル入力要素にイベントリスナーを登録する。
    - 引数: なし
    - 戻り値: `void`
- **convertMML** (src/main.ts): MML形式の入力をYM2151レジスタログに変換する処理をトリガーします。
    - 役割: MML文字列をYM2151ログに変換し、結果をUIに表示する。
    - 引数: なし
    - 戻り値: Promise<void>
- **loadMMLExample** (src/main.ts): 事前定義されたMMLのサンプルコードをロードし、Webページの入力エリアに表示します。
    - 役割: サンプルMMLをロードしてUIに設定する。
    - 引数: `exampleId`: ロードするMMLサンプルの識別子。
    - 戻り値: `void`
- **switchTab** (src/main.ts): WebUIの異なるタブ（例: MIDI変換タブ、MML変換タブ）を切り替えるロジックを管理します。
    - 役割: UIのタブ表示を切り替える。
    - 引数: `tabId`: 切り替えるタブの識別子。
    - 戻り値: `void`
- **setupEventListeners** (src/main.ts): アプリケーション全体で必要なUIイベントリスナー（ボタンクリック、ファイル変更など）をまとめて設定します。
    - 役割: ページ上の各種UI要素にイベントリスナーを登録する。
    - 引数: なし
    - 戻り値: `void`

## 関数呼び出し階層ツリー
```
- catch (src/main.ts)
  - initWasm (src/main.ts)
    - checkMMLWasm ()
      - displayResult ()
      - showError ()
      - setupFileInput ()
      - convertMML ()
      - loadMMLExample ()
      - switchTab ()
      - setupEventListeners ()
- if (src/main.ts)

---
Generated at: 2026-02-05 07:10:06 JST

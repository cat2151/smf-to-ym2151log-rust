Last updated: 2026-02-07

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) をYM2151 FM音源チップのレジスタ書き込みログ（JSON形式）に変換するRust製ツール兼ライブラリです。
- MIDIファイルの解析、YM2151チャンネルへの割り当て、およびカスタム音色対応を含む2パス処理アーキテクチャを採用しています。
- ネイティブアプリケーションに加え、WebAssemblyとしてWebブラウザでの利用も可能であり、柔軟な音楽データ変換ソリューションを提供します。

## 技術スタック
- フロントエンド: 
    - **WebAssembly (WASM)**: RustコードをWebブラウザで実行可能にするためのバイナリ形式。
    - **TypeScript**: JavaScriptに静的型付けを追加した言語で、Webフロントエンドの主要な開発言語として使用されています。
    - **Vite**: 高速な開発サーバーとバンドル機能を提供するWebフロントエンドビルドツール。
    - **HTML**: Webページの構造を定義するためのマークアップ言語。
    - **CSS**: Webページのスタイル（見た目）を定義するためのスタイルシート言語。
- 音楽・オーディオ: 
    - **Standard MIDI Files (SMF)**: 電子楽器の演奏情報を記録するための標準ファイル形式（Format 0およびFormat 1をサポート）。
    - **YM2151 FM音源チップ**: ヤマハ製のFM音源チップで、レジスタ操作を通じて音を生成します。
    - **General MIDI**: MIDI機器間の互換性を保証するための標準仕様。ドラムチャンネル（MIDIチャンネル9）もこれに含まれます。
- 開発ツール: 
    - **Rust**: 高い安全性とパフォーマンスを特徴とするシステムプログラミング言語。
    - **Cargo**: Rustのビルドシステムとパッケージマネージャー。
    - **Git**: 分散型バージョン管理システム。
- テスト: 
    - **cargo test**: Rustプロジェクトのユニットテストと統合テストを実行するコマンド。
    - **cargo tarpaulin**: Rustコードのテストカバレッジを測定・レポートするツール。
- ビルドツール: 
    - **cargo build**: Rustプロジェクトをコンパイルするコマンド。
    - **wasm-pack**: RustをWebAssemblyにコンパイルし、JavaScriptから利用可能なパッケージを生成するツール。
    - **Vite**: (フロントエンド開発ツールと兼ねて) TypeScript/JavaScriptのバンドル、開発サーバー起動に使用。
- 言語機能: 
    - **Rust 1.70.0 以上**: プロジェクトのビルドに必要なRust言語のバージョン。
- 自動化・CI/CD: 
    - **wasm-pack build --target web --features wasm**: WebAssembly向けビルドコマンドで、継続的インテグレーション/デプロイメント (CI/CD) パイプラインの一部として利用されます。
- 開発標準: 
    - **cargo fmt --check**: Rustコードのフォーマットがスタイルガイドに準拠しているかを確認するコマンド。
    - **cargo clippy -- -D warnings**: Rustコードの潜在的なエラーやスタイル違反を検出するLinterツール。
    - **cargo audit**: プロジェクトの依存関係に既知の脆弱性がないかを確認するセキュリティ監査ツール。

## ファイル階層ツリー
```
📄 .gitignore
📄 Cargo.lock
📄 Cargo.toml
📖 DEMO_README.md
📖 DEMO_SEPARATION.md
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
  📊 package-lock.json
  📊 package.json
  🎨 style.css
  📊 tsconfig.json
  📘 vite.config.ts
📁 demo-mml/
  📄 .gitignore
  📖 README.md
  🌐 index.html
  📘 mml-demo.ts
  📊 package-lock.json
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
  📖 61.md
  📖 63.md
  📖 65.md
  📖 66-resolution.md
  📖 66.md
📊 package-lock.json
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
📜 verify-demos.js
📘 vite.config.ts
```

## ファイル詳細説明
- **.gitignore**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定する設定ファイルです。
- **Cargo.lock**: Rustプロジェクトの正確な依存関係のバージョンを記録するファイルです。
- **Cargo.toml**: Rustプロジェクトのメタデータ、依存関係、ビルド設定を定義するマニフェストファイルです。
- **DEMO_README.md**: デモアプリケーションに関する追加情報を提供するMarkdownファイルです。
- **DEMO_SEPARATION.md**: デモアプリケーションの分離設計に関する説明書です。
- **LICENSE**: プロジェクトのライセンス情報が記述されています。
- **MML_INTEGRATION.md**: MML（Music Macro Language）との統合に関するドキュメントです。
- **README.ja.md**: プロジェクトの概要を日本語で説明するメインのドキュメントです。
- **README.md**: プロジェクトの概要を英語で説明するメインのドキュメントです。
- **WASM_USAGE.md**: WebAssembly版の利用方法に関する詳細な説明書です。
- **_config.yml**: GitHub Pagesのサイト設定ファイルです。
- **demo-library/**: WebAssembly版のライブラリ利用デモに関連するファイル群です。
    - **.gitignore**: `demo-library`ディレクトリ内のGit無視設定ファイルです。
    - **README.md**: `demo-library`のReadmeファイルです。
    - **index.html**: ライブラリデモのメインとなるHTMLファイルで、WebAssemblyモジュールをロードしUIを提供します。
    - **library-demo.ts**: WebAssemblyモジュールと連携してMIDIファイルを処理し、結果を表示するTypeScriptコードです。
    - **package-lock.json**: Node.jsプロジェクトの依存関係の正確なバージョンを記録します。
    - **package.json**: Node.jsプロジェクトのメタデータと依存関係を定義するファイルです。
    - **style.css**: ライブラリデモのスタイルシートです。
    - **tsconfig.json**: TypeScriptコンパイラの設定ファイルです。
    - **vite.config.ts**: Viteビルドツールの設定ファイルです。
- **demo-mml/**: WebAssembly版のMML変換デモに関連するファイル群です。
    - **.gitignore**: `demo-mml`ディレクトリ内のGit無視設定ファイルです。
    - **README.md**: `demo-mml`のReadmeファイルです。
    - **index.html**: MMLデモのメインとなるHTMLファイルで、MML入力や結果表示のUIを提供します。
    - **mml-demo.ts**: WebAssemblyモジュールと連携してMMLをYM2151ログに変換するTypeScriptコードです。
    - **package-lock.json**: Node.jsプロジェクトの依存関係の正確なバージョンを記録します。
    - **package.json**: Node.jsプロジェクトのメタデータと依存関係を定義するファイルです。
    - **style.css**: MMLデモのスタイルシートです。
    - **tsconfig.json**: TypeScriptコンパイラの設定ファイルです。
    - **vite.config.ts**: Viteビルドツールの設定ファイルです。
- **generated-docs/**: プロジェクトから自動生成されたドキュメントが格納されるディレクトリです。
- **googled947dc864c270e07.html**: Google Search Consoleのサイト所有権確認用ファイルです。
- **index.html**: GitHub Pagesのトップページとして機能するHTMLファイルで、各種デモへのリンクを提供します。
- **issue-notes/**: プロジェクト開発中に記録された課題や検討事項に関するMarkdown形式のメモファイル群です。
- **package-lock.json**: ルートディレクトリのNode.jsプロジェクトの依存関係の正確なバージョンを記録します。
- **package.json**: ルートディレクトリのNode.jsプロジェクトのメタデータと依存関係を定義するファイルです。
- **src/**: RustのソースコードとTypeScriptのメインデモコードが格納されるディレクトリです。
    - **error.rs**: プロジェクト全体で使用されるカスタムエラー型を定義します。
    - **lib.rs**: このプロジェクトのライブラリクレートのエントリポイントで、公開APIを提供します。
    - **main.rs**: コマンドラインツールとして実行される場合のエントリポイントです。
    - **main.ts**: WebAssembly版のメインデモアプリケーションのTypeScriptコードです。
    - **midi/**: MIDIファイルの解析に関連するRustモジュールです。
        - **events.rs**: MIDIイベントを表す構造体と関連ロジックを定義します。
        - **mod.rs**: `midi`モジュールのルートファイルで、他のサブモジュールを公開します。
        - **parser.rs**: Standard MIDI Files (SMF) を解析し、中間イベントに変換する主要なロジックを実装します。
        - **utils.rs**: MIDI関連のユーティリティ関数やヘルパーを定義します。
    - **style.css**: メインデモのスタイルシートです。
    - **wasm.rs**: RustコードをWebAssembly (WASM) にエクスポートするためのバインディングロジックを定義します。
    - **ym2151/**: YM2151 FM音源チップへの変換に関連するRustモジュールです。
        - **channel_allocation.rs**: MIDIチャンネルをYM2151の限られたチャンネルに割り当てる戦略（和音数ベース、ドラム優先など）を実装します。
        - **converter.rs**: 中間MIDIイベントをYM2151レジスタ書き込みログに変換する主要なロジックを実装します。
        - **converter_tests.rs**: `converter.rs`モジュールのテストコードです。
        - **event_processor.rs**: YM2151のイベント処理ロジックをカプセル化します。
        - **events.rs**: YM2151のレジスタ操作や関連イベントを表す構造体を定義します。
        - **init.rs**: YM2151チップの初期化シーケンスやデフォルト設定を扱います。
        - **mod.rs**: `ym2151`モジュールのルートファイルで、他のサブモジュールを公開します。
        - **note_table.rs**: YM2151の発音に必要な音階や周波数関連のテーブルデータ、計算ロジックを定義します。
        - **tempo_map.rs**: MIDIファイル内のテンポ変更イベントを管理し、タイムスタンプをティックから時間へ変換するロジックを提供します。
        - **tone.rs**: YM2151の音色（トーン）データ構造と、外部JSONファイルからのカスタム音色ロードロジックを実装します。
- **tests/**: 統合テストに関連するファイル群です。
    - **create_test_midi.py**: 統合テストで使用するMIDIファイルをプログラムで生成するためのPythonスクリプトです。
    - **integration_tests.rs**: プロジェクト全体の主要な機能フローを検証する統合テストコードです。
    - **test_data/**: 統合テストで使用されるサンプルMIDIファイルが格納されています。
- **tones/**: カスタムYM2151音色定義JSONファイルが格納されるディレクトリです。
    - **000.json**: プログラムチェンジ0番に対応するYM2151音色定義ファイルです。
    - **README.md**: `tones`ディレクトリ内の音色ファイルの作成方法やフォーマットに関する説明書です。
- **tsconfig.json**: ルートディレクトリのTypeScriptコンパイラ設定ファイルです。
- **verify-demos.js**: デモページのリンクの動作を確認するためのJavaScriptスクリプトです。
- **vite.config.ts**: ルートディレクトリのViteビルドツールの設定ファイルです。

## 関数詳細説明
- **initWasm (demo-library/library-demo.ts, demo-mml/mml-demo.ts, src/main.ts)**:
    - 役割: WebAssemblyモジュールを非同期で初期化し、ロードします。
    - 引数: なし
    - 戻り値: なし (Promiseを返す可能性あり)
    - 機能: 必要なWebAssemblyバイナリを読み込み、JavaScript側で利用可能な状態にします。
- **displayResult (demo-library/library-demo.ts, src/main.ts)**:
    - 役割: 変換処理の結果（YM2151レジスタログJSONなど）をWebページ上に表示します。
    - 引数: `result`: 表示する結果データ（文字列またはJSONオブジェクト）
    - 戻り値: なし
    - 機能: HTML要素を操作して、ユーザーに変換の成否や出力内容を視覚的に伝えます。
- **showError (demo-library/library-demo.ts, demo-mml/mml-demo.ts, src/main.ts)**:
    - 役割: エラーメッセージをWebページ上に表示します。
    - 引数: `error`: 表示するエラーメッセージ（文字列）
    - 戻り値: なし
    - 機能: ユーザーがエラーを認識し、問題解決に役立つ情報を提供します。
- **setupFileInput (demo-library/library-demo.ts, src/main.ts)**:
    - 役割: ファイル入力要素にイベントリスナーを設定し、ユーザーがMIDIファイルをアップロードした際の処理を定義します。
    - 引数: なし
    - 戻り値: なし
    - 機能: MIDIファイルの読み込み、WebAssemblyへの渡し、結果の表示フローを管理します。
- **checkMMLWasm (demo-mml/mml-demo.ts)**:
    - 役割: MML変換用のWebAssemblyモジュールが利用可能かどうかをチェックします。
    - 引数: なし
    - 戻り値: なし
    - 機能: MMLデモのUI要素を、WASMのロード状態に応じて有効/無効化します。
- **convertMML (demo-mml/mml-demo.ts)**:
    - 役割: MML形式のテキスト入力値をYM2151レジスタログに変換します。
    - 引数: なし
    - 戻り値: なし
    - 機能: ユーザーが入力したMML文字列をWebAssemblyモジュールに渡し、変換結果を取得して表示します。
- **loadMMLExample (demo-mml/mml-demo.ts)**:
    - 役割: MMLのサンプルコードをテキストエリアにロードします。
    - 引数: `exampleId`: ロードするサンプルのID（文字列）
    - 戻り値: なし
    - 機能: ユーザーがMMLの記法を試すための手助けをします。
- **setupEventListeners (src/main.ts)**:
    - 役割: Webページ上の様々なUI要素（ファイル入力、ボタンなど）に対するイベントリスナーを一括で設定します。
    - 引数: なし
    - 戻り値: なし
    - 機能: ユーザーインタラクションに応じた処理を定義し、アプリケーションの動作を制御します。
- **verifyPage (verify-demos.js)**:
    - 役割: 特定のURLのページにアクセスし、そのページの動作やコンテンツが期待通りであるかを検証します。
    - 引数: `url`: 検証対象のURL（文字列）
    - 戻り値: 検証の成否を示すPromise
    - 機能: デモページの自動テストや健全性チェックに使用されます。
- **main (verify-demos.js)**:
    - 役割: `verify-demos.js`スクリプトのメイン実行ロジックです。複数のデモページの検証をオーケストレートします。
    - 引数: なし
    - 戻り値: なし (Promiseを返す可能性あり)
    - 機能: `verifyPage`関数を呼び出して、定義されたデモURL群を順次検証します。

## 関数呼び出し階層ツリー
```
- if (demo-library/library-demo.ts)
  - initWasm (demo-library/library-demo.ts)
    - displayResult ()
      - showError ()
      - setupFileInput ()
      - setupEventListeners ()
    - checkMMLWasm ()
      - convertMML ()
      - loadMMLExample ()
  - catch (demo-library/library-demo.ts)
    - verifyPage (verify-demos.js)
      - main ()
- for (verify-demos.js)

---
Generated at: 2026-02-07 07:08:20 JST

Last updated: 2026-02-04

# Project Overview

## プロジェクト概要
- Standard MIDI Files (SMF) を、ヤマハYM2151 FM音源チップ用のレジスタ書き込みログ（JSON形式）に変換するツールです。
- Rustで実装されており、高いパフォーマンスと型安全性を実現し、Webブラウザ (WebAssembly) でも利用できます。
- MIDIのプログラムチェンジに対応し、カスタム音色を外部JSONファイルから読み込んで適用することが可能です。

## 技術スタック
- フロントエンド: WebAssembly (WASM) 対応によりWebブラウザ上での実行を可能にします。クライアント側でのHTML、CSS、JavaScriptによるデモ提供も行われています。
- 音楽・オーディオ: Standard MIDI Files (SMF) の解析と、ヤマハYM2151 FM音源チップのレジスタ操作に関する知識が基盤となっています。
- 開発ツール: Git (バージョン管理)、Cargo (Rustのビルドシステム/パッケージマネージャ)、wasm-pack (WebAssemblyパッケージングツール)。
- テスト: `cargo test` (ユニットテスト、統合テスト)、`cargo tarpaulin` (テストカバレッジレポート生成)。
- ビルドツール: Cargo (Rustプロジェクトのビルドと依存関係管理)。
- 言語機能: Rust言語 (型安全性、メモリ安全性、高パフォーマンス)。
- 自動化・CI/CD: `cargo fmt --check` (コードフォーマットチェック)、`cargo clippy -- -D warnings` (コード品質リンティング)、`cargo audit` (セキュリティ脆弱性チェック) など、継続的インテグレーションで利用されるツール群。
- 開発標準: `cargo fmt` (自動コードフォーマッタ)、`cargo clippy` (コード品質向上リンター)。

## ファイル階層ツリー
```
.
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.ja.md
├── README.md
├── WASM_USAGE.md
├── _config.yml
├── googled947dc864c270e07.html
├── index.html
├── src/
│   ├── error.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── midi/
│   │   ├── events.rs
│   │   ├── mod.rs
│   │   ├── parser.rs
│   │   └── utils.rs
│   ├── wasm.rs
│   └── ym2151/
│       ├── channel_allocation.rs
│       ├── converter.rs
│       ├── converter_tests.rs
│       ├── event_processor.rs
│       ├── events.rs
│       ├── init.rs
│       ├── mod.rs
│       ├── note_table.rs
│       ├── tempo_map.rs
│       └── tone.rs
├── tests/
│   ├── create_test_midi.py
│   ├── integration_tests.rs
│   └── test_data/
│       ├── multi_channel.mid
│       ├── multi_track.mid
│       ├── program_change.mid
│       ├── simple_melody.mid
│       └── tempo_change.mid
└── tones/
    ├── 000.json
    └── README.md
```

## ファイル詳細説明
- **.gitignore**: Gitがバージョン管理の対象外とするファイルやディレクトリを指定します。
- **Cargo.lock**: プロジェクトの正確な依存関係のバージョンを記録し、再現性のあるビルドを保証します。
- **Cargo.toml**: Rustプロジェクトの設定ファイル。依存関係、ビルド設定、メタデータなどが記述されています。
- **LICENSE**: プロジェクトのライセンス情報（著作権や利用条件）を定義します。
- **README.ja.md**: プロジェクトの日本語版概要、使い方、開発情報などをまとめたドキュメントです。
- **README.md**: プロジェクトの英語版概要、使い方、開発情報などをまとめたドキュメントです。
- **WASM_USAGE.md**: WebAssembly版の利用方法に関する詳細な説明ドキュメントです。
- **_config.yml**: GitHub Pagesなどの静的サイトジェネレータで使用される設定ファイルです。
- **googled947dc864c270e07.html**: Googleサイト認証のために使用されるファイルです。
- **index.html**: WebAssembly版のデモンストレーションページを提供するHTMLファイルです。
- **src/**: プロジェクトのRustソースコードが格納されているディレクトリです。
    - **error.rs**: プロジェクト全体で利用されるカスタムエラー型とエラーハンドリングロジックを定義します。
    - **lib.rs**: ライブラリとしての公開APIエントリーポイントです。他のRustプロジェクトからこのライブラリを利用する際に参照されます。
    - **main.rs**: コマンドラインアプリケーションのエントリーポイントです。CLIツールとして実行される際のメインロジックを含みます。
    - **midi/**: MIDIファイルの解析とイベント処理に関連するモジュールを格納するディレクトリです。
        - **events.rs**: Standard MIDI File内の様々なMIDIイベントのデータ構造を定義します。
        - **mod.rs**: `midi`モジュールのルートファイルで、配下のサブモジュールを公開します。
        - **parser.rs**: MIDIファイルを読み込み、その内容を内部表現のMIDIイベントにパースするロジックを実装します。
        - **utils.rs**: MIDIデータ処理に関するユーティリティ関数やヘルパーを格納します。
    - **wasm.rs**: WebAssembly (WASM) 向けのバインディングとAPIを提供し、JavaScriptからRustの機能を呼び出せるようにします。
    - **ym2151/**: YM2151 FM音源チップへの変換と関連ロジックを格納するディレクトリです。
        - **channel_allocation.rs**: MIDIチャンネルからYM2151の限られた8つのチャンネルへの割り当て戦略を実装します。
        - **converter.rs**: MIDIイベントをYM2151のレジスタ書き込みログ（JSON形式）に変換する主要なロジックを含みます。
        - **converter_tests.rs**: `ym2151/converter.rs`のユニットテストコードです。
        - **event_processor.rs**: YM2151レジスタイベントを時間順に処理し、出力ログを生成するロジックを実装します。
        - **events.rs**: YM2151レジスタ書き込みログとして出力されるイベントのデータ構造を定義します。
        - **init.rs**: YM2151チップの初期化レジスタ設定に関するロジックを定義します。
        - **mod.rs**: `ym2151`モジュールのルートファイルで、配下のサブモジュールを公開します。
        - **note_table.rs**: MIDIノート番号とYM2151の音高レジスタ値（F-number/Block）のマッピングテーブルを管理します。
        - **tempo_map.rs**: MIDIファイルのテンポ情報（テンポチェンジイベントなど）を管理し、タイムスタンプを変換するロジックを提供します。
        - **tone.rs**: プログラムチェンジイベントに対応するYM2151の音色データ（レジスタ設定）の読み込みと管理を扱います。
- **tests/**: 統合テストのコードとテストデータが格納されているディレクトリです。
    - **create_test_midi.py**: テスト目的でダミーのMIDIファイルを生成するためのPythonスクリプトです。
    - **integration_tests.rs**: プロジェクト全体の主要な機能が正しく連携しているかを検証する統合テストコードです。
    - **test_data/**: 統合テストで使用されるサンプルMIDIファイル群が格納されています。
- **tones/**: カスタムYM2151音色（プログラムチェンジ用）のJSONファイルが格納されているディレクトリです。
    - **000.json**: MIDIプログラム0番に割り当てられるYM2151のカスタム音色定義ファイル（例）です。
    - **README.md**: `tones`ディレクトリ内のJSONファイル形式に関する説明ドキュメントです。

## 関数詳細説明
プロジェクト情報に具体的な関数の詳細な説明が提供されていないため、個々の関数については言及を控えます。コードベースを直接参照することで詳細な機能を確認できます。

## 関数呼び出し階層ツリー
```
関数呼び出し階層を分析できませんでした

---
Generated at: 2026-02-04 07:12:52 JST

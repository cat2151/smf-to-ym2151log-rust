# smf-to-ym2151log-rust GitHub Copilot 指示書

## プロジェクト概要

Rustで実装された、Standard MIDI File（SMF）をYM2151 FMシンセサイザーチップのレジスタ書き込みログ（JSON形式）に変換するツールです

**主な目的**: MIDIファイル（.mid）→ YM2151レジスタログ（JSON）への変換

## アーキテクチャ

### 2パス処理システム

本プロジェクトは2パス構成を採用しています：

1. **パスA（MIDIパーサー）**: MIDIファイル → 中間イベントJSON（デバッグ用）
   - SMF Format 0 および Format 1 を解析
   - 正規化されたMIDIイベントを `<ファイル名>_events.json` として出力
   
2. **パスB（YM2151コンバーター）**: 中間イベント → YM2151レジスタログJSON（最終出力）
   - MIDIイベントをYM2151レジスタ書き込みに変換
   - ym2151-zig-cc互換の `<ファイル名>_ym2151.json` を出力

### 主要モジュール

- `src/midi/` - MIDIファイルの解析とイベント処理
  - `parser.rs` - SMF解析ロジック
  - `events.rs` - MIDIイベント型定義
  - `utils.rs` - ユーティリティ関数（テンポ変換、ノートマッピング）
  
- `src/ym2151/` - YM2151変換ロジック
  - `converter.rs` - メイン変換ロジック
  - `events.rs` - YM2151イベント型定義
  - `init.rs` - YM2151初期化シーケンス
  - `note_table.rs` - MIDIノートからYM2151 KC/KF への変換

- `src/error.rs` - thiserrorを使用したエラー型定義
- `src/lib.rs` - ライブラリルート
- `src/main.rs` - CLIエントリーポイント

## ビルドとテスト

### ビルド
```bash
# デバッグビルド
cargo build
```

### テスト
```bash
# 全テスト実行
cargo test

# 特定のテストを実行
cargo test <テスト名>
```

**テスト構成**:
- ユニットテスト: コードと同じファイル内の `#[cfg(test)]` モジュール
- 統合テスト: `tests/integration_*.rs`（例: `tests/integration_midi.rs`, `tests/integration_conversion.rs`）
- テストデータ: `tests/test_data/`

### コード品質
```bash
# フォーマットチェック（コミット前に必須）
cargo fmt --check

# Lintチェック（コミット前に必須）
cargo clippy -- -D warnings

# セキュリティ監査
cargo audit
```

## 依存関係

### cat2151リポジトリのバージョン固定方針

cat2151のリポジトリからclone、`npm install`、`cargo install` する際は**バージョンを固定しないこと**。具体的には：
- `git clone`（特定のコミット/タグをチェックアウトしない — 常にデフォルトブランチのHEADを使用）
- `npm install github:cat2151/...`（特定のコミットやバージョンに固定しない）
- `cargo install --git https://github.com/cat2151/...`（rev/tagを指定しない）

**理由**: cat2151のリポジトリには毎日重要なバグ修正が入るため、常に最新版を取り込む必要があります。バージョンを固定すると重要な修正が取り込まれなくなります。

### 本番依存関係
- `midly` (0.5) - SMF解析ライブラリ
- `serde` + `serde_json` - JSONシリアライズ/デシリアライズ
- `anyhow` - アプリケーションレベルのエラーハンドリング
- `thiserror` - カスタムエラー型定義

## コーディング規約

### 言語
- **Rust Edition 2021**
- 最低Rustバージョン: 1.70.0

### スタイル
- 標準Rustフォーマット（`cargo fmt`）に従うこと
- `clippy` によるLint（CI では警告ゼロが必須）
- 可読性が向上する場合は型推論よりも明示的な型指定を優先すること
- 説明的な変数名を使用すること
- **コメント**: コードコメントやドキュメントは国際的な協力を支援するために英語を推奨。ドメイン固有の用語やバイリンガルドキュメントでは日本語も可

### エラーハンドリング
- バイナリのアプリケーションレベルエラーには `anyhow::Result` を使用
- ライブラリのカスタムエラー型定義には `thiserror` を使用
- `?` 演算子でエラーを伝播させること
- 本番コードでは unwrap/expect を避けること（テストでは可）

### テストガイドライン
- 純粋関数やアルゴリズムにはユニットテストを書くこと
- エンドツーエンドのワークフローには統合テストを書くこと
- 説明的なテスト名を使用すること（例: `test_parse_simple_melody`）
- 成功ケースとエラーケースの両方をテストすること
- テストデータファイルは小さくフォーカスされた内容に保つこと

### ドキュメント
- 公開APIにはdocコメント（`///`）でドキュメントを記載すること
- 必要に応じてdocコメントにサンプルを含めること
- コード変更に合わせてREADME.mdとIMPLEMENTATION.mdを同期させること

## JSON出力フォーマット

### イベントJSON (_events.json)
中間デバッグフォーマット:
```json
{
  "ticks_per_beat": 480,
  "tempo_bpm": 120.0,
  "events": [
    {
      "type": "note_on",
      "ticks": 0,
      "channel": 0,
      "note": 60,
      "velocity": 100
    }
  ]
}
```

### YM2151ログJSON (_ym2151.json)
最終出力フォーマット（ym2151-zig-cc互換が必須）:
```json
{
  "event_count": 50,
  "events": [
    {
      "time": 0,
      "addr": "0x08",
      "data": "0x00"
    }
  ]
}
```
- `time`: イベント発生時刻（秒単位の `f64`）
- `addr`: YM2151レジスタアドレス（16進数文字列）
- `data`: 書き込むデータ（16進数文字列）

## 重要な参考資料

- [YM2151データシート](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf) - 公式チップ仕様（注: HTTPリンク、HTTPSバージョンなし）

## 一般的な作業

### 新しいMIDIイベントサポートの追加
1. `src/midi/events.rs` にイベント型を追加
2. `src/midi/parser.rs` のパーサーを更新
3. `src/ym2151/converter.rs` に変換ロジックを追加
4. 必要に応じて `tests/integration_*.rs` のいずれか、または新規の `tests/integration_...rs` ファイルに統合テストを追加

### YM2151レジスタロジックの変更
1. YM2151データシートでレジスタ仕様を確認
2. `src/ym2151/converter.rs` の変換ロジックを更新
3. 必要に応じて `src/ym2151/note_table.rs` のノートテーブルを更新
4. レジスタ値を検証するテストを追加

### CLIオプションの追加
1. `src/main.rs` の引数解析を更新
2. README.mdの使用方法セクションを更新
3. 新しいオプションの統合テストを追加

## CI/CD

本プロジェクトはGitHub Actions（`.github/workflows/ci.yml`）を使用：
- プッシュ後にそれを検知して実行
- build、test、clippy、fmtチェックを実施

### デモのデプロイ

本プロジェクトは `.github/workflows/deploy-pages.yml` を通じてGitHub Pagesに単一のデモをデプロイ：
- **ライブラリデモ** (`/`) - MIDIファイル変換によるライブラリ使用のデモ

**デモの検証**

デモやデプロイに変更を加えた場合：
1. デプロイ後にデモが動作することを確認
2. ブラウザコンソールのJavaScriptエラーを確認
3. デモ機能をテスト - ファイルアップロード、変換、UIインタラクション
4. デプロイ手順を検証 - ビルドステップ、ファイルコピー、デプロイワークフローが正しいことを確認

# ソース行数
- 単一責任の原則に従ってソース分割すること。特に500行を超えたときはソース分割の優先度を高めること

# TypeScript demo
- demoを開発するとき、formatterとlinterを適用すること
  - `demo-library/` の TypeScript formatter と linter には Biome を使用すること
    - formatter の適用: `cd demo-library && npm run format`
    - linter の適用: `cd demo-library && npm run lint`
- TypeScript demo に使うライブラリ
  - demo用のMML to SMF、SMF to JSON は、cat2151のライブラリを利用せよ。cat2151のライブラリは毎日重要なバグ修正があり、それを取り込むのがマストであるため、バージョン指定はせず最新mainを毎回取り込むべし

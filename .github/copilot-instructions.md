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

## コーディング規約

### スタイル
- 標準Rustフォーマット（`cargo fmt`）に従うこと
- `clippy` によるLint（CI では警告ゼロが必須）

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

## CI

本プロジェクトはGitHub Actions（`.github/workflows/ci.yml`）を使用：
- プッシュ後にそれを検知して実行
- build、test、clippy、fmtチェックを実施

### デモのデプロイ

デプロイは2段階のワークフローで構成されており、404防止のため順序が重要です：

**ワークフローA: `deploy-wasm.yml`（WASMデプロイ）**
- mainブランチへのプッシュで自動実行
- RustコードをWASMにビルド（`wasm-pack build --target web --features wasm`）
- `_site/` 配下のうち、`pkg/` を含む最小セットをGitHub Pagesにデプロイ（この段階ではデモHTMLは未配置）
- デプロイ後、ワークフローBが自動的にトリガーされる

**ワークフローB: `deploy-demo.yml`（デモデプロイ）**
- ワークフローAの完了後に自動実行（または手動実行可能）
- `demo-library/` をビルドしてデモを生成（このときの `npm install` の `postinstall` で GitHub Pages 上の WASM `pkg/` を参照）
- 追加で GitHub Pages から WASM の `pkg/` を `curl` でダウンロード（リトライ付き、最大5回）し、デプロイ物に含める
- デモ全体を GitHub Pages に再デプロイ（WASM の `pkg/` も含む）

**404防止の注意点**
- ワークフローAは `pkg/` のみをデプロイするため、この段階ではデモHTMLページが存在しない（一時的に404となる）
- ワークフローBが完了することで、デモページを含む完全なサイトが揃う
- ワークフローBはWASMファイルのダウンロードにリトライを使用（Pagesの伝播遅延に対応）
- ワークフローBを手動実行する場合は、必ずワークフローAが成功した後に実行すること

**デモの検証**

デモやデプロイに変更を加えた場合：
1. デプロイ後にデモが動作することを確認
2. ブラウザコンソールのJavaScriptエラーを確認
3. デモ機能をテスト - ファイルアップロード、変換、UIインタラクション
4. デプロイ手順を検証 - ビルドステップ、ファイルコピー、デプロイワークフローが正しいことを確認

**注意**: `deploy-pages.yml` は廃止/非推奨で通常は使用しない（`workflow_dispatch` による手動起動のみ可能で、参考用に残存）。実際のデプロイは `deploy-wasm.yml` → `deploy-demo.yml` の順で行われる。

# ソース行数
- 単一責任の原則に従ってソース分割すること。特に500行を超えたときはソース分割の優先度を高めること

# TypeScript demo
- demoを開発するとき、formatterとlinterを適用すること
  - `demo-library/` の TypeScript formatter と linter には Biome を使用すること
    - formatter の適用: `cd demo-library && npm run format`
    - linter の適用: `cd demo-library && npm run lint`
- TypeScript demo に使うライブラリ
  - demo用のMML to SMF、SMF to JSON は、cat2151のライブラリを利用せよ。cat2151のライブラリは毎日重要なバグ修正があり、それを取り込むのがマストであるため、バージョン指定はせず最新mainを毎回取り込むべし

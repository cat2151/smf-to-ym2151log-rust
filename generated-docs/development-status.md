Last updated: 2026-02-15

# Development Status

## 現在のIssues
- `deploy-demo`ワークフローが複数回失敗しており、安定化が最優先課題です([Issue #124](../issue-notes/124.md), [Issue #120](../issue-notes/120.md))。
- 音色JSONフォーマットの改善、線形補間音色変化の実装、デモの自動再生機能追加など、音色表現とデモ体験の向上が進行中です([Issue #123](../issue-notes/123.md), [Issue #115](../issue-notes/115.md), [Issue #114](../issue-notes/114.md))。
- `copilot-instructions.md`の日本語化とデプロイ構造の明確化、TypeScriptデモへのBiome導入も進められています([Issue #122](../issue-notes/122.md), [Issue #117](../issue-notes/117.md))。

## 次の一手候補
1. deploy-demoワークフローの失敗原因調査と修正 [Issue #124](../issue-notes/124.md), [Issue #120](../issue-notes/120.md)
   - 最初の小さな一歩: 失敗したワークフローランのログを詳細に分析し、エラーが発生している具体的なステップとエラーメッセージを特定する。
   - Agent実行プロンプ:
     ```
     対象ファイル: .github/workflows/deploy-pages.yml, demo-library/vite.config.ts, demo-library/package.json

     実行内容: GitHub Actionsのワークフローラン https://github.com/cat2151/smf-to-ym2151log-rust/actions/runs/22018832539 と https://github.com/cat2151/smf-to-ym2151log-rust/actions/runs/22017670347 のログを詳細に分析し、`deploy-demo`ワークフローが失敗している具体的な原因を特定してください。特にエラーメッセージ、失敗したステップ、および関連するコードブロックに注目してください。

     確認事項: ログ内で参照されているスクリプトファイルや設定ファイル（例: `demo-library/vite.config.ts`, `demo-library/package.json` など）が存在し、それらに異常がないか。GitHub Pagesのデプロイ設定に最近変更がなかったか。

     期待する出力: 失敗の原因（例: 依存関係のインストール失敗、ビルドエラー、デプロイ権限の問題など）と、それを解決するための具体的な修正案をmarkdown形式で出力してください。
     ```

2. 音色JSONフォーマット変更とProgramChangeの自己記述性向上 [Issue #123](../issue-notes/123.md)
   - 最初の小さな一歩: 現在の`tones/000.json`と`src/ym2151/converter.rs`の音色JSONパースロジックを分析し、新しいフォーマット（ProgramChangeを項目名にするなど）を定義する。
   - Agent実行プロンプ:
     ```
     対象ファイル: tones/000.json, src/ym2151/converter.rs, demo-library/tone-json-demo.ts, demo-library/tone-json.html

     実行内容: [Issue #123](../issue-notes/123.md) に基づき、音色JSONフォーマットを自己記述性を高める形（`ProgramChange`をJSON項目名にする）に変更するための設計案をmarkdown形式で作成してください。具体的には、新しいJSONスキーマの定義、および`src/ym2151/converter.rs`における既存のパースロジックへの影響と変更点の概要を含めてください。また、既存のデモファイル(`demo-library/tone-json-demo.ts`, `demo-library/tone-json.html`)への影響も考察してください。

     確認事項: 既存の`tones/`ディレクトリ内のJSONファイルとの互換性、および`src/ym2151/converter.rs`の他の音色関連ロジックへの副作用がないことを確認してください。

     期待する出力: 新しい音色JSONフォーマットの定義（JSONスキーマ例）、`src/ym2151/converter.rs`の変更点概要、および`demo-library`内の関連ファイルに対する修正方針をmarkdown形式で出力してください。
     ```

3. `copilot-instructions.md` の日本語化と最新情報反映、デプロイ構造の明示 [Issue #122](../issue-notes/122.md)
   - 最初の小さな一歩: `copilot-instructions.md`の現在の内容を分析し、日本語化が必要な箇所、プロジェクトの最新状況（特に最近のRustコード分割やCI/CDの改善）を特定し、デプロイ関連の404エラー防止策として追加すべき情報（例: GitHub Pagesのパス構造）を洗い出す。
   - Agent実行プロンプ:
     ```
     対象ファイル: README.md, .github/copilot-instructions.md, issue-notes/122.md, .github/workflows/deploy-pages.yml

     実行内容: `copilot-instructions.md`を日本語化し、現在のプロジェクトの最新状況（特にRustコードの`src/ym2151/converter.rs`が`src/ym2151/converter/*.rs`に分割された点や、CI/CDに関する変更、デモのデプロイ状況）を反映するための改訂案をmarkdown形式で作成してください。また、GitHub Pagesでの404エラーを防ぐためのデプロイ構造に関する情報（例: `generated-docs/` の扱いやベースパス）も追加してください。

     確認事項: 既存の`README.md`との重複がないか、GitHub Copilotが理解しやすい表現になっているか、および提供されたファイル一覧でデプロイに関連するファイル（例: `_config.yml`, `demo-library/index.html`など）がどのようにGitHub Pagesに配置されるかを確認してください。

     期待する出力: 日本語化された`copilot-instructions.md`のドラフト、および変更点のハイライトをmarkdown形式で出力してください。

---
Generated at: 2026-02-15 07:08:15 JST

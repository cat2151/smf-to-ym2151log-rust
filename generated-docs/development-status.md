Last updated: 2026-03-10

# Development Status

## 現在のIssues
- [Issue #177](../issue-notes/177.md)と[Issue #83](../issue-notes/83.md)は、添付JSONと音色データの整理に関するドッグフーディングと課題解決を求めています。
- [Issue #33](../issue-notes/33.md)は、ym2151-tone-editorの出力するJSONを優先して読み込む仕様追加を検討しています。
- [Issue #22](../issue-notes/22.md)は、デフォルト音色（000.json～127.json）の手作業での配置と作成を課題としています。

## 次の一手候補
1. [Issue #33](../issue-notes/33.md) ym2151-tone-editorの出力するJSONを優先する仕様の検討と実装計画
   - 最初の小さな一歩: 現在の音色データ読み込みロジックがどのファイルでどのように処理されているかを特定し、`ym2151-tone-editor`からのJSONを優先的に読み込む際に変更が必要となる可能性のある箇所を洗い出す。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/ym2151/tone.rs`, `src/ym2151/converter.rs`, `tones/README.md`

     実行内容: `src/ym2151/tone.rs` を中心に、現在の音色データ（`tones/` ディレクトリ内のJSONファイル）の読み込みロジックを分析してください。特に、JSONファイルをどのように探し、パースし、アプリケーション内で利用しているかを詳細に把握し、Markdown形式でその処理フローを記述してください。

     確認事項: 既存の音色読み込み処理が依存する他のモジュールや設定ファイル（例: `tones/README.md` に記載されている情報など）があるかを確認してください。

     期待する出力: 既存の音色読み込み処理のフロー図（テキストベースで可）と、`ym2151-tone-editor` から出力されるJSONを優先的に読み込む際に変更が必要となる可能性のある箇所について、具体的な候補をMarkdown形式で提案してください。
     ```

2. [Issue #83](../issue-notes/83.md) 音色データの扱いについて整理（デフォルト音色データ未整備の現状把握）
   - 最初の小さな一歩: デフォルト音色データがどのように扱われることを想定しているか、現在の`tones/`ディレクトリの構造と既存の`tones/000.json`の内容、および関連するコード（`src/ym2151/tone.rs`など）を確認し、現状を把握する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tones/`, `tones/000.json`, `tones/README.md`, `src/ym2151/tone.rs`

     実行内容: `tones/` ディレクトリの現在の内容（特に`000.json`の構造）と、`src/ym2151/tone.rs` における音色データのロードおよび利用方法を分析してください。デフォルト音色データ0～127が未整備という課題に対して、現在のコードがどのように対応しているか、または対応できていないかを明確にしてください。

     確認事項: `src/ym2151/tone.rs` が音色データをどのように解決しているか（パス、ファイル名規則など）を確認し、`tones/README.md` に記載されている情報との整合性を確認してください。

     期待する出力: `tones/` ディレクトリの現状（存在しないファイルや仮のファイル）、`tones/000.json` の内容の簡単な説明、および `src/ym2151/tone.rs` が音色データをロードする際の具体的な挙動について、Markdown形式でレポートしてください。また、デフォルト音色データ整備の初期ステップとして、どのような情報が必要であるかを提案してください。
     ```

3. [Issue #177](../issue-notes/177.md) 添付JSONまわりのドッグフーディングに必要な環境とフローの確認
   - 最初の小さな一歩: `demo-library/tone-json-attachment.ts`と`demo-library/tone-json-demo.ts`を中心に、添付JSONがどのように生成され、デモで利用されているかを理解する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo-library/tone-json-attachment.ts`, `demo-library/tone-json-demo.ts`, `src/ym2151/tone.rs`, `src/ym2151/converter.rs`

     実行内容: `demo-library/tone-json-attachment.ts` と `demo-library/tone-json-demo.ts` がどのように連携し、添付JSONの生成と利用を行っているかを分析してください。また、それが `src/ym2151/tone.rs` や `src/ym2151/converter.rs` での音色変換にどのように影響しているかを調査し、Markdown形式でその現状と潜在的な課題を記述してください。

     確認事項: `demo-library` 側のTypeScriptコードが、WASMモジュール（`smf-to-ym2151log-rust`）のどの機能と連携しているか、特に添付JSONの形式や受け渡し方法に注目して確認してください。

     期待する出力: 添付JSONの生成・利用フローの概要、現在のデモにおける添付JSONの役割、および [Issue #177](../issue-notes/177.md) のドッグフーディングを進める上で考慮すべき技術的ポイントや潜在的な改善点をMarkdown形式で提案してください。
     ```

---
Generated at: 2026-03-10 07:11:38 JST

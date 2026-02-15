Last updated: 2026-02-16

# Development Status

## 現在のIssues
- 現在、YM2151レジスタビジュアライザの改善 ([Issue #128](../issue-notes/128.md)) やディレイビブラート/LFOの品質問題 ([Issue #127](../issue-notes/127.md)) など、demo機能の改善と品質向上が進行中です。
- 添付JSONのフォーマット変更 ([Issue #123](../issue-notes/123.md)) と、これに伴うポップノイズdemoでのJSON反映問題 ([Issue #126](../issue-notes/126.md)) が主要なタスクとして挙げられます。
- CIの自動Issue起票機能の実装 ([Issue #125](../issue-notes/125.md)) と `copilot-instructions.md` の更新 ([Issue #122](../issue-notes/122.md)) も優先的に取り組まれています。

## 次の一手候補
1. ディレイビブラート/LFOの品質問題分析とレジスタビジュアライザ改善 ([Issue #127](../issue-notes/127.md), [Issue #128](../issue-notes/128.md))
   - 最初の小さな一歩: [Issue #127](../issue-notes/127.md) の問題特定のため、現状のディレイビブラートとLFOが生成するログを詳細に分析し、クリックノイズの原因を特定する。
   - Agent実行プロンプ:
     ```
     対象ファイル: demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, src/ym2151/converter/pitch_effects.rs, src/ym2151/converter/register_effects.rs

     実行内容:
     1. `demo-library/delay-vibrato-demo.ts` と `demo-library/portamento-soft-lfo-demo.ts` のLFOおよびディレイビブラートがYM2151レジスタに書き込むデータ生成ロジックを分析する。
     2. クリックノイズが発生しているとされる現象について、レジスタ書き込みイベントのタイミングと値の変化をトレースし、不連続な変化点や不適切な値がないか調査する。
     3. `src/ym2151/converter/pitch_effects.rs` および `src/ym2151/converter/register_effects.rs` 内で、ディレイビブラートとLFOに関連するレジスタ値の計算方法を確認し、期待される滑らかな変化が実現されているか検証する。
     4. 分析結果に基づき、クリックノイズの原因として考えられる箇所を特定し、改善の方向性について提案をmarkdown形式で出力してください。

     確認事項:
     - `demo-library/delay-vibrato-demo.ts` と `demo-library/portamento-soft-lfo-demo.ts` の依存関係を確認し、関連するJavaScript/TypeScriptファイルも分析対象に含めること。
     - Rust側のレジスタ値計算ロジックが、期待されるYM2151の挙動と一致しているか、データシート等の資料と照らし合わせて確認すること。

     期待する出力:
     - クリックノイズの原因と考えられるレジスタ書き込みのパターンや、コード内の具体的な箇所を特定したmarkdown形式のレポート。
     - 可能であれば、問題解決に向けた具体的なコード変更の提案を含める。
     ```

2. 添付JSONフォーマットの変更と既存demoへの反映 ([Issue #123](../issue-notes/123.md), [Issue #126](../issue-notes/126.md))
   - 最初の小さな一歩: [Issue #123](../issue-notes/123.md) のJSONフォーマット変更案に基づき、具体的なJSONスキーマ（例: `tones/000.json`）の変更点を定義する。
   - Agent実行プロンプ:
     ```
     対象ファイル: issue-notes/123.md, tones/000.json, demo-library/tone-json-demo.ts, src/ym2151/tone.rs

     実行内容:
     1. `issue-notes/123.md` に記載されている「ProgramChangeはJSON項目名にProgramChangeという名前をつける」という要件を具体化し、新しい音色JSONの構造を定義する。
     2. 現在の `tones/000.json` (または他の既存の `tones/*.json` ファイル) の内容を新しいフォーマットに変換する具体的な例を生成する。
     3. 変換後のJSONフォーマットを基に、`demo-library/tone-json-demo.ts` がこの新しいフォーマットを正しく読み込み、処理できるようにするために必要な変更点を分析する。特に、[Issue #126](../issue-notes/126.md) の「ポップノイズdemoで、添付JSONを変更しても、最終log JSONに反映されていない」問題が、このフォーマット変更によってどう影響を受けるか、または解決されるかを検討する。
     4. Rust側の `src/ym2151/tone.rs` において、ProgramChangeに関するJSONデシリアライズロジックを新しいフォーマットに対応させるための変更点を洗い出す。

     確認事項:
     - 新しいJSONフォーマットが自己記述性を高め、既存の概念（ディレイビブラート、ポルタメント、LFO）との整合性を保っているか確認すること。
     - `demo-library` 側のTypeScriptコードとRust側の `src/ym2151/tone.rs` の両方で、フォーマット変更が正確に反映されるように考慮すること。
     - [Issue #126](../issue-notes/126.md) の問題が、単なるdemoのJSONロードの問題なのか、それともJSON構造の解釈の問題なのかを分析すること。

     期待する出力:
     - [Issue #123](../issue-notes/123.md) に基づく新しいJSONフォーマットの具体的な定義と、`tones/000.json` の変更例をmarkdown形式で出力。
     - `demo-library/tone-json-demo.ts` および `src/ym2151/tone.rs` における変更点の概要と、[Issue #126](../issue-notes/126.md) への影響を記述したmarkdown形式のレポート。
     ```

3. CIエラー時の自動Issue起票の実装と開発ガイドラインの最新化 ([Issue #125](../issue-notes/125.md), [Issue #122](../issue-notes/122.md))
   - 最初の小さな一歩: [Issue #125](../issue-notes/125.md) に向けて、既存のCIワークフロー (`.github/workflows/ci.yml`) に、エラー発生時にIssueを自動起票するためのジョブを追加する計画を立てる。
   - Agent実行プロンプ:
     ```
     対象ファイル: .github/workflows/ci.yml, .github/workflows/call-issue-note.yml, .github/actions-tmp/issue-notes/3.md, .github/copilot-instructions.md

     実行内容:
     1. `.github/workflows/ci.yml` が `clippy` や `fmt` でエラーになった際に、[Issue #125](../issue-notes/125.md) の要件に従いIssueを自動起票する仕組みを導入するための変更点を分析する。
     2. 既存のIssue自動起票ワークフロー (`.github/workflows/call-issue-note.yml` や `.github/actions-tmp/issue-notes/3.md` に記載されている `issue-note` 関連の共通ワークフロー) を参考に、CIエラーの詳細をIssue本文に含める方法を検討する。
     3. `copilot-instructions.md` ([Issue #122](../issue-notes/122.md)) を日本語化し、現在のプロジェクトの構成、デプロイプロセス、特にGitHub Pagesでの404エラー防止のための情報（例: `_config.yml` や `baseurl` の設定、demo-libraryのルート配置など）を含めて最新の状態に更新する。
     4. 更新された `copilot-instructions.md` が、プロジェクトの新しい開発者やCopilot利用者にとって有用であるかを確認する。

     確認事項:
     - CIエラー時の自動Issue起票が、重複Issueの作成を防ぎ、必要な情報（エラーメッセージ、コミットハッシュ、ワークフロー実行URLなど）を確実に含めるように設計されているか確認すること。
     - `copilot-instructions.md` の日本語化が正確であり、プロジェクトの現状を反映しているか、特にデプロイに関する説明が明確で分かりやすいか確認すること。

     期待する出力:
     - `.github/workflows/ci.yml` にCIエラー時のIssue自動起票ジョブを追加するための具体的なYAML変更案をmarkdown形式で出力。
     - `copilot-instructions.md` の日本語化および最新状況を反映した更新内容（Markdown形式）。

---
Generated at: 2026-02-16 07:08:51 JST

Last updated: 2026-03-07

# Development Status

## 現在のIssues
- CIジョブ失敗時に自動でIssueを起票する機能（[Issue #137](../issue-notes/137.md), [Issue #125](../issue-notes/125.md)）と`copilot-instructions.md`の最新化（[Issue #122](../issue-notes/122.md)）が進められています。
- YM2151レジスタビジュアライザの改善（[Issue #128](../issue-notes/128.md)）と、ディレイビブラート/LFOのクリックノイズ問題調査およびWAVエクスポートによる分析（[Issue #127](../issue-notes/127.md)）が課題です。
- 添付JSONフォーマットの変更による音色情報の自己記述性向上（[Issue #123](../issue-notes/123.md)）や、線形補間音色変化機能の実装（[Issue #115](../issue-notes/115.md)）、さらに音色データの整備（[Issue #83](../issue-notes/83.md), [Issue #33](../issue-notes/33.md), [Issue #22](../issue-notes/22.md)）に関するタスクが残っています。

## 次の一手候補
1. CIジョブ失敗時の自動Issue起票機能の最終確認とデプロイ [Issue #137](../issue-notes/137.md), [Issue #125](../issue-notes/125.md)
   - 最初の小さな一歩: `ci.yml`の現在の実装が[Issue #137](../issue-notes/137.md)で述べられている変更を完全に含んでおり、意図通りにCI失敗時にIssueが起票されることを確認するためのテスト計画を立てる。
   - Agent実行プロンプ:
     ```
     対象ファイル: .github/workflows/ci.yml, issue-notes/137.md, issue-notes/125.md

     実行内容: `.github/workflows/ci.yml`の現状を分析し、[Issue #137](../issue-notes/137.md)の変更内容（`issues: write`権限追加と`Create issue on failure`ステップ）が完全に反映されているか確認してください。また、この実装が[Issue #125](../issue-notes/125.md)の要件（clippy等のCI失敗時自動起票）を満たしているか評価し、今後のテスト計画（擬似的なCI失敗を発生させる方法など）を提案するmarkdown形式のドキュメントを作成してください。

     確認事項: 既存の`.github/workflows`ディレクトリ内の他のワークフロー（特に`issue-note.yml`など、Issue作成に関わるもの）との競合や既存の権限設定の確認。GitHub Actionsのイベントトリガーとパーミッションの制約。

     期待する出力: `ci.yml`の現状と[Issue #137](../issue-notes/137.md)の変更の整合性評価、[Issue #125](../issue-notes/125.md)の解決状況、および自動Issue起票機能をテストするための具体的な計画を記載したmarkdownドキュメント。
     ```

2. 添付JSONフォーマットの変更によるProgramChangeの自己記述性向上 [Issue #123](../issue-notes/123.md)
   - 最初の小さな一歩: 現在の添付JSONの構造（`tones/000.json`など）と、`src/ym2151/tone.rs`における音色関連のデータ構造を分析し、提案されているフォーマット変更のRustコードへの影響を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/000.json, src/ym2151/tone.rs, src/ym2151/converter.rs, demo-library/tone-json-demo.ts, demo-library/tone-json-attachment.ts, issue-notes/123.md

     実行内容: `issue-notes/123.md`で提案されている添付JSONフォーマット変更（ProgramChangeの自己記述性向上、LFO/Portamento/DelayVibratoのProgramChangeとのセット扱い）について、現在の`src/ym2151/tone.rs`のデータ構造と`src/ym2151/converter.rs`の変換ロジック、および`demo-library`内のJSON関連ファイル（`tone-json-demo.ts`, `tone-json-attachment.ts`）に与える影響を分析してください。変更が必要なRustの構造体、enum、メソッド、TypeScriptのインターフェース、型の箇所を具体的に特定し、それらの変更が他の部分にどのように波及するかをmarkdown形式で出力してください。

     確認事項: 既存のYM2151レジスタ変換ロジックへの影響、JSONパースロジックの変更点、既存の音色データとの後方互換性（一時的な共存の可能性）、およびTypeScript側のデモコードへの影響。

     期待する出力: 提案されたJSONフォーマット変更に対応するために必要なRustとTypeScriptのコード変更点の詳細なリスト、およびその変更による影響範囲を説明したmarkdownドキュメント。
     ```

3. ディレイビブラート/LFOクリックノイズ問題の調査とWAVエクスポート実装 [Issue #127](../issue-notes/127.md)
   - 最初の小さな一歩: 既存の`demo-library/delay-vibrato-demo.ts`を分析し、現在のデモでクリックノイズが発生する再現条件を特定するとともに、`demo-library`に簡単なWAVエクスポート機能を追加するための調査を行う。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/delay-vibrato-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/library-demo.ts, demo-library/vite.config.ts, issue-notes/127.md

     実行内容: `issue-notes/127.md`に記載されているディレイビブラートとLFOの低品質（クリックノイズ）問題について、現在の`delay-vibrato-demo.ts`や`portamento-soft-lfo-demo.ts`の実装を分析し、問題の原因となりうる箇所を推測してください。また、問題分析を支援するためのWAVエクスポート機能を`demo-library`に追加するための実現可能性と、そのための最小限の実装方針（Web Audio APIの`AudioBuffer`を操作して`WAV`形式でダウンロードさせる方法など）をmarkdown形式で提案してください。

     確認事項: `demo-library`の既存のビルドシステム（Vite）と`Web Audio API`との互換性、WAVエクスポートライブラリ（または自作）の選定、パフォーマンスへの影響、および既存デモへの組み込みの容易さ。

     期待する出力: クリックノイズの原因に関する仮説、WAVエクスポート機能の設計方針、実装に必要な主要なAPIやコード例、およびデモへの統合方法を記述したmarkdownドキュメント。

---
Generated at: 2026-03-07 07:10:54 JST

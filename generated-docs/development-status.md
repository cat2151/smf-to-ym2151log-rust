Last updated: 2026-03-04

# Development Status

## 現在のIssues
- コード品質向上として、`tests/integration_tests.rs` をはじめとする4つのファイルが500行を超過しており、リファクタリングが推奨されています ([Issue #134](../issue-notes/134.md))。
- `copilot-instructions.md` の日本語化、最新状況への更新、デプロイ構造の明示が課題として挙げられています ([Issue #133](../issue-notes/133.md), [Issue #122](../issue-notes/122.md))。
- デモ関連では、MMLコマンド「l」「t」の不動作 ([Issue #131](../issue-notes/131.md))、YM2151ビジュアライザの機能改善 ([Issue #128](../issue-notes/128.md))、ディレイビブラート/LFOの音質問題 ([Issue #127](../issue-notes/127.md))、添付JSONが反映されない問題 ([Issue #126](../issue-notes/126.md))、添付JSONフォーマットの変更 ([Issue #123](../issue-notes/123.md)) など、多岐にわたる改善が進行中です。

## 次の一手候補
1. `copilot-instructions.md` の日本語化と最新状況反映 ([Issue #133](../issue-notes/133.md), [Issue #122](../issue-notes/122.md))
   - 最初の小さな一歩: `copilot-instructions.md` の「Project Overview」セクションを日本語に翻訳し、現在のプロジェクトの目的、アーキテクチャ、主要モジュールを正確に反映するように更新する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `.github/copilot-instructions.md`

     実行内容: 対象ファイルを読み込み、「Project Overview」セクションの内容を日本語に翻訳し、最新のプロジェクト概要と一致するように更新してください。特に、「Main Purpose」「Architecture」「Key Modules」の記述を、現在の実装状況に合わせて修正してください。

     確認事項: 翻訳の正確性、プロジェクトの現在の状態との整合性、および[Issue #122](../issue-notes/122.md) で言及されているdeploy構造の明示が必要かどうかの検討。

     期待する出力: 更新された`.github/copilot-instructions.md` のファイル内容。
     ```

2. ディレイビブラートデモにおけるMMLの「l」「t」コマンド不動作の調査 ([Issue #131](../issue-notes/131.md))
   - 最初の小さな一歩: `demo-library/delay-vibrato-demo.ts` を開き、MML入力からSMFへの変換がどのように行われているか、特にMMLコマンド (`l`, `t`) の処理に関連する箇所を特定して分析する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `demo-library/delay-vibrato-demo.ts`, `demo-library/mml-support.ts`, `demo-library/tone-json-mml.ts`

     実行内容: `demo-library/delay-vibrato-demo.ts` のMML入力処理と、`mml-support.ts` および `demo-library/tone-json-mml.ts` で提供されるMMLランタイムの利用状況を分析してください。特に、MMLの「l」（音長）と「t」（テンポ）コマンドがどのように解析され、MIDIデータに変換されているか、または変換されていないかを調査し、その処理フローをmarkdown形式で説明してください。

     確認事項: [Issue #132](../issue-notes/132.md) での `mml-support.ts` の修正が、本Issueにどのような影響を与えているかを確認すること。MMLパーサー (`mmlabc-to-smf-rust`) 側の仕様や制約も考慮してください。

     期待する出力: `delay-vibrato-demo.ts` におけるMMLコマンド処理の分析結果をMarkdown形式で出力し、なぜ「l」「t」コマンドが動作しないのかの仮説を含めてください。
     ```

3. `tests/integration_tests.rs` の大規模テストファイルのリファクタリングに着手 ([Issue #134](../issue-notes/134.md))
   - 最初の小さな一歩: `tests/integration_tests.rs` 内で、類似のテストをグループ化する関数やモジュールを特定し、将来の分割に向けてテストの役割を明確化するコメントを追加する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `tests/integration_tests.rs`

     実行内容: `tests/integration_tests.rs` ファイルを分析し、以下の観点から改善点を特定してください:
     1) 論理的に関連性の高いテストグループ（例: MIDIパース関連、YM2151変換関連、エンドツーエンドフローなど）をリストアップしてください。
     2) 各グループについて、既存のテスト関数がそのグループ内でどのような役割を持っているかを簡潔に説明し、テストの重複や冗長性を指摘してください。
     3) 将来的なファイル分割やモジュール化を考慮した、テストグループごとのリファクタリングの方向性を提案するコメントを、コード内に直接追加する計画を立ててください。

     確認事項: テストの意図や依存関係を正確に理解すること。既存のテストが壊れないよう、分析は慎重に行うこと。

     期待する出力: `tests/integration_tests.rs` の分析結果をMarkdown形式で出力し、リファクタリングの計画と、具体的なコードへのコメント追加内容を説明してください。

---
Generated at: 2026-03-04 07:11:26 JST

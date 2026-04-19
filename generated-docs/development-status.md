Last updated: 2026-04-20

# Development Status

## 現在のIssues
- 添付JSONの利用状況把握と、デフォルト音色データ（tones/000.json〜127.json）の整理・作成が主要な課題です ([Issue #177](../issue-notes/177.md), [Issue #83](../issue-notes/83.md))。
- `ym2151-tone-editor` を利用した音色ファイルの作成と配置が進められており、音色データの初期整備が求められています ([Issue #22](../issue-notes/22.md))。
- これらを通じて、音色データの管理と利用方法を明確にし、開発の基盤を強化することが目標です。

## 次の一手候補
1. デフォルト音色データ `tones/000.json` から `tones/127.json` までの初期構造の作成 ([Issue #22](../issue-notes/22.md), [Issue #83](../issue-notes/83.md))
   - 最初の小さな一歩: `ym2151-tone-editor` の使用方法を調査し、`tones/000.json` と同じシンプルな構造を持つダミーの `tones/001.json` を作成する手順を確立する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tones/000.json`, `tones/README.md`

     実行内容: `ym2151-tone-editor` の使用ドキュメントやREADMEを調査し、シンプルな構造の `tones/000.json` を参考にして、`tones/001.json` を生成するための具体的な手順を記述してください。また、`tones/README.md` にデフォルト音色作成のガイドラインを追加することを検討してください。

     確認事項: `ym2151-tone-editor` がこのリポジトリとは別プロジェクトであることを考慮し、外部ツールとの連携方法について実現可能な範囲で手順を記述してください。生成されるJSONが既存のRustパーサーで問題なく処理できるか、基本的な互換性を意識してください。

     期待する出力: `ym2151-tone-editor` を用いた `tones/001.json` の作成手順をmarkdown形式で詳細に説明し、`tones/README.md` に追記する形で「デフォルト音色作成ガイドライン」の草案を含めてください。
     ```

2. 添付JSONの構造とRustの型定義の整合性検証と改善 ([Issue #177](../issue-notes/177.md))
   - 最初の小さな一歩: `src/options/attachments.rs` に定義されている `Attachment` 構造体と、それが期待するJSON形式との間のマッピングを詳細に分析し、特に最近の変更で影響がありそうな部分を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/options/attachments.rs`, `src/options/mod.rs`, `src/ym2151/converter_tests/attachments.rs`

     実行内容: `src/options/attachments.rs` 内の `Attachment` 構造体が、実際にパースされるJSONデータと完全に一致しているかを検証してください。特に、`serde` 属性や`#[derive(Debug, Clone, PartialEq, Default, Deserialize)]`などの利用がJSONスキーマに与える影響を分析し、不整合があれば報告してください。

     確認事項: 最近の変更 (`src/options/attachments.rs` の修正) が、既存のJSONデータパースに予期せぬ影響を与えていないか確認してください。テストケース (`src/ym2151/converter_tests/attachments.rs`) が現在の `Attachment` 定義と整合しているかを確認してください。

     期待する出力: `Attachment` 構造体の定義と対応するJSONスキーマの間の整合性に関する詳細な分析結果をmarkdown形式で記述し、もし不整合や改善点が見つかった場合は具体的な修正案を提示してください。
     ```

3. デモ環境における音色データ読み込み機能の強化 ([Issue #83](../issue-notes/83.md))
   - 最初の小さな一歩: `demo-library/tone-json-demo.ts` を分析し、`tones/000.json` などの音色ファイルを読み込む既存のロジックを理解する。もし存在しない場合、そのための最小限の読み込み機能を追加することを検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo-library/tone-json-demo.ts`, `demo-library/tone-json.html`, `tones/000.json`

     実行内容: `demo-library/tone-json-demo.ts` が `tones/000.json` のような音色ファイルをどのように読み込み、WebAudioAPI経由で利用しているかを分析してください。現在の実装で、複数の音色ファイルを動的に切り替えたり、ランダムな音色を適用したりする機能を追加する場合の課題点と、その実現可能性について検討してください。

     確認事項: `demo-library` の既存のファイル構造やスクリプトが、新しい音色読み込み機能の追加にどのように影響するかを確認してください。特に、TypeScriptの型定義や依存関係に注意してください。

     期待する出力: `demo-library/tone-json-demo.ts` における音色ファイル読み込みの現状を説明し、[Issue #83](../issue-notes/83.md)で述べられている「ブラウザ、ランダム音色で、音符はMMLでその場で入力して鳴らす、和音も可」を実現するためのロードマップをmarkdown形式で提示してください。最初のステップとして、動的に音色をロードする簡単な機能追加のコードスニペットを含めてください。
     ```

---
Generated at: 2026-04-20 07:14:15 JST

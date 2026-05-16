Last updated: 2026-05-17

# Development Status

## 現在のIssues
- [Issue #177](../issue-notes/177.md) では、添付JSON機能の実際の使用感を検証するドッグフーディングが計画されています。
- [Issue #83](../issue-notes/83.md) は、デフォルト音色データ（0-127）の不足を背景に、プロジェクトにおける音色データの扱いを整理することを目的としています。
- [Issue #22](../issue-notes/22.md) では、`ym2151-tone-editor` を利用して `tones/000.json` から `127.json` までの音色データを具体的に配置することが求められています。

## 次の一手候補
1. [Issue #177](../issue-notes/177.md) 添付JSONのRustコードでの適用方法を分析する
   - 最初の小さな一歩: `src/options/attachments.rs` で定義される `Attachment` 構造体がAPIを通じてロードされ、`src/ym2151/converter.rs` でどのようにYM2151イベントに変換・適用されるかのデータフローと主要ロジックを分析する。特に、`src/ym2151/converter_tests/attachments.rs` にあるテストケースの使用例から理解を深める。
   - Agent実行プロンプ:
     ```
     対象ファイル: src/options/attachments.rs, src/ym2151/converter.rs, src/ym2151/converter_tests/attachments.rs, src/api.rs

     実行内容: `src/options/attachments.rs` で定義される `Attachment` 構造体が、APIを通じてどのようにロードされ (`src/api.rs`)、`src/ym2151/converter.rs` 内でYM2151イベントに変換・適用されるかのデータフローと主要なロジックを分析してください。特に、`src/ym2151/converter_tests/attachments.rs` にあるテストケースがどのように `Attachment` を利用して変換結果に影響を与えているかを詳細に記述してください。

     確認事項: `Attachment` のパースロジック、YM2151レジスタへのマッピング、およびテストケースで検証されている `Attachment` の振る舞いの正確性を確認してください。

     期待する出力: `Attachment` のデータフローと主要ロジック、およびテストコードでの利用例をMarkdown形式で詳細に説明してください。
     ```

2. [Issue #83](../issue-notes/83.md) プロジェクト内の音色データの現状と利用方法を調査する
   - 最初の小さな一歩: 現在のプロジェクトにおける音色データ（`tones/` ディレクトリ内のJSONファイルや、Rustコード内のデフォルト音色設定）がどのように定義され、ロードされ、YM2151変換処理に利用されているかを分析する。特に、デフォルト音色（0-127）がどこまで実装されているか、未実装の場合のフォールバック動作を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/000.json, tones/README.md, src/ym2151/tone.rs, src/options/attachments.rs, src/ym2151/converter.rs, src/api.rs

     実行内容: プロジェクト内の音色データ（`tones/` ディレクトリ内のJSONファイルおよびRustコード内の関連する構造体やロジック）の定義、ロード、およびYM2151変換処理における利用方法を分析してください。特に、`tones/000.json` の構造と `src/ym2151/tone.rs` の関連性、そしてデフォルト音色（0-127）の扱い（どこまで実装され、未実装の場合のフォールバック動作）について詳しく説明してください。

     確認事項: 音色データのJSONスキーマとRustの構造体の整合性、および音色データが変換処理に渡されるパスと利用箇所を確認してください。

     期待する出力: 音色データの現状（定義、ロード、利用、デフォルト音色の実装状況）に関する詳細なレポートをMarkdown形式で生成してください。
     ```

3. [Issue #22](../issue-notes/22.md) デフォルト音色データ `tones/000.json` の詳細レビューと改善点特定
   - 最初の小さな一歩: `tones/000.json` が現在のシステムでどのように解釈され、実際のYM2151レジスタ値に変換されるかを、Rustコード (`src/ym2151/converter.rs` など) と照らし合わせて詳細にレビューする。この000番の音色が「デフォルト」として適切かどうかを簡易的に評価し、改善の余地を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/000.json, src/ym2151/converter.rs, src/ym2151/converter/register_fields.rs, src/options/attachments.rs

     実行内容: `tones/000.json` の内容を、`src/ym2151/converter.rs` と `src/ym2151/converter/register_fields.rs` のロジックに基づいて詳細にレビューしてください。具体的には、`000.json` の各 `addr` と `data` がYM2151のどのレジスタフィールドに対応し、どのように解釈されるかを説明し、現在の `000.json` がデフォルト音色として機能的に適切であるか、あるいは改善の余地があるかを評価してください。

     確認事項: `000.json` のJSON構造が `src/options/attachments.rs` でパース可能であること、およびレジスタ値がYM2151の仕様に沿って解釈されることを確認してください。

     期待する出力: `tones/000.json` の詳細なレビュー結果と、デフォルト音色としての評価（改善点を含む）をMarkdown形式で出力してください。

---
Generated at: 2026-05-17 07:21:04 JST

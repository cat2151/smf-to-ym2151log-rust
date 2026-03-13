Last updated: 2026-03-14

# Development Status

## 現在のIssues
- [Issue #180](../issue-notes/180.md): ポップノイズデモのデフォルト値が適切でないため、挙動確認と調整が必要です。
- [Issue #177](../issue-notes/177.md): 添付JSON機能のドッグフーディング（実践的な利用検証）が計画されており、その動作確認と改善が求められています。
- [Issue #83](../issue-notes/83.md): デフォルト音色データの不足と、ブラウザでのMML演奏とランダム音色の実現方法について整理が必要です。

## 次の一手候補
1. ポップノイズデモのデフォルト値修正と検証 ([Issue #180](../issue-notes/180.md))
   - 最初の小さな一歩: `demo-library/pop-noise-demo.ts` と `demo-library/pop-noise-detector.ts` を確認し、デモが意図した通りに動作し、ポップノイズ検出が適切に行われるようにデフォルト値を調整する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/pop-noise-demo.ts, demo-library/pop-noise-detector.ts, issue-notes/180.md

     実行内容: `issue-notes/180.md` に記載されているポップノイズデモのデフォルト値に関する問題点を解決するため、`demo-library/pop-noise-demo.ts` 内の初期設定値（例: `detector.detectionThreshold` など）が適切か分析してください。また、`demo-library/pop-noise-detector.ts` の検出ロジックと連携し、デモが直感的に理解できるよう、推奨されるデフォルト値の変更案を提示してください。

     確認事項: 最近のコミット (`b3bfd75`, `58c1ac4`, `3125ef0`) でのポップノイズ検出ロジックの変更が、デフォルト値の選定に与える影響を確認してください。

     期待する出力: `demo-library/pop-noise-demo.ts` の具体的なコード変更案（新しいデフォルト値の設定）をmarkdown形式で出力してください。
     ```

2. 添付JSON機能の動作検証と改善点の特定 ([Issue #177](../issue-notes/177.md))
   - 最初の小さな一歩: 添付JSONが意図通りにトーンに適用されるか、`demo-library/tone-json-demo.ts` や `demo-library/tone-json-attachment.ts` を使って基本的なケースと最近修正された `Tone.registers` を含むケースで手動で検証し、問題があれば特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/tone-json-attachment.ts, src/ym2151/converter.rs, src/ym2151/converter/register_effects.rs, src/ym2151/converter/register_fields.rs, issue-notes/177.md

     実行内容: `issue-notes/177.md` に基づき、添付JSONまわりのドッグフーディング（実践的な検証）計画を策定し、具体的なテストシナリオをリストアップしてください。特に、最近 `Tone.registers` の正規化修正 (`52618ef`) が行われた点を考慮し、`demo-library/tone-json-attachment.ts` が `src/ym2151/converter` 内のレジスタ変換ロジックと適切に連携しているか検証するためのシナリオを含めてください。

     確認事項: 既存の `src/ym2151/converter_tests/attachments_program_effects.rs` や `src/ym2151/converter_tests/attachments_change_to_next_tone.rs` といったテストが、現在の `demo-library` の実装と整合性が取れているか確認してください。

     期待する出力: 添付JSON機能の検証計画と、実行すべき具体的なテストケース（簡単なMMLやJSONデータ例を含む）をmarkdown形式で出力してください。また、検証で特に注意すべき点や、予期される課題も記述してください。
     ```

3. デフォルト音色データの管理とロードマップ策定 ([Issue #83](../issue-notes/83.md))
   - 最初の小さな一歩: `issue-notes/83.md`, `issue-notes/22.md`, `issue-notes/33.md` の内容を再確認し、現在の `tones/` ディレクトリのファイル一覧 (`tones/000.json`, `tones/README.md`) と照らし合わせて、最も優先度の高いデフォルト音色データ（例: GM互換の000.json）の作成に着手するか、そのための準備を行う。
   - Agent実行プロンプト:
     ```
     対象ファイル: issue-notes/83.md, issue-notes/22.md, issue-notes/33.md, tones/000.json, tones/README.md, src/ym2151/tone.rs

     実行内容: 複数のIssueで言及されている音色データの不足 ([Issue #83](../issue-notes/83.md), [Issue #22](../issue-notes/22.md)) と、`ym2151-tone-editor` との連携による読み込み優先順位の仕様 ([Issue #33](../issue-notes/33.md)) について、現状を整理し、今後の開発ロードマップを提案してください。具体的には、
     1. 現在不足しているデフォルト音色データ（0-127）のうち、最低限必要となる音色（例: piano, guitar, drumsなど）をリストアップし、その作成優先順位を付けてください。
     2. `ym2151-tone-editor` との連携 ([Issue #33](../issue-notes/33.md)) を考慮した場合の `tones/` ディレクトリの管理戦略について提案してください（例: シンボリックリンクの利用、生成データの配置場所など）。
     3. `tones/000.json` のような既存のファイルを参考に、`ym2151-tone-editor` で作成されたjsonファイルをプロジェクトに組み込む具体的な手順案を記述してください。

     確認事項: 音色データの構造が `src/ym2151/tone.rs` で定義されている構造と一致しているか、また `tones/000.json` が有効なJSON形式であることを確認してください。

     期待する出力: 音色データ管理に関するロードマップ、デフォルト音色データの作成優先順位リスト、`ym2151-tone-editor` との連携手順案をmarkdown形式で出力してください。
     ```

---
Generated at: 2026-03-14 07:11:58 JST

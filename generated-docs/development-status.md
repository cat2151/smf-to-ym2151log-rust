Last updated: 2026-03-21

# Development Status

## 現在のIssues
- [Issue #226](../issue-notes/226.md) は、`demo-library/pop-noise-demo.ts` が500行を超過しており、リファクタリングが推奨されています。
- [Issue #224](../issue-notes/224.md) は、既存のポップノイズ対策デモを参考に、すべてのデモページにランダム音色ボタンを実装するタスクです。
- [Issue #177](../issue-notes/177.md) は、添付JSON機能の実際の利用を通じて潜在的な改善点やバグを発見するドッグフーディング作業です。

## 次の一手候補
1. [Issue #226](../issue-notes/226.md) `demo-library/pop-noise-demo.ts` のリファクタリング
   - 最初の小さな一歩: `demo-library/pop-noise-demo.ts` のコードを読み込み、特にイベントリスナーのセットアップ関数（`setupAttachmentEditor`, `setupMmlInput` など）が肥大化していないか、共通化できる部分がないかを特定します。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo-library/pop-noise-demo.ts`

     実行内容: `demo-library/pop-noise-demo.ts` を分析し、特に初期化とイベントリスナー設定に関連する関数群（`setupAttachmentEditor`, `setupMmlInput`, `setupMidiInput`, `setupPlayButton`, `setupWavExportButton`, `setupRandomToneButton` など）に関して、コードの凝集度を高め、行数を削減できるようなリファクタリングの改善点をMarkdown形式で提案してください。具体的な改善案として、共通の初期化ユーティリティ関数や、関連する設定をグループ化するクラス構造の導入などを検討してください。

     確認事項: リファクタリングによって既存の機能が損なわれないこと、および他のデモページへの予期せぬ影響がないことを確認してください。

     期待する出力: リファクタリングの具体的な提案、改善後の擬似コード、およびリファクタリングのメリット（例: 可読性向上、行数削減）を含むMarkdown形式のレポート。
     ```

2. [Issue #224](../issue-notes/224.md) ランダム音色ボタンのロジックを他デモページで利用できるように共通化
   - 最初の小さな一歩: `demo-library/pop-noise-demo.ts` に実装されているランダム音色生成 (`applyRandomToneToAttachment`) とボタン設定 (`setupRandomToneButton`) のロジックを抽出するための計画を立て、これらの機能を `demo-library/shared-demo-utils.ts` のような新しいユーティリティファイルに移動させることを検討します。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo-library/pop-noise-demo.ts`, `demo-library/shared-demo.ts`, `demo-library/delay-vibrato-demo.ts`

     実行内容: `demo-library/pop-noise-demo.ts` からランダム音色生成 (`applyRandomToneToAttachment`) とボタン設定 (`setupRandomToneButton`) のロジックを抽出し、これらを共通化して `demo-library/shared-demo-utils.ts` のような新しいファイルに移動させるための具体的な計画を提案してください。また、その共通化した関数を `demo-library/delay-vibrato-demo.ts` (既存のデモの一つ) から呼び出す方法についても記述してください。

     確認事項: `pop-noise-demo.ts` の既存機能が変更後も維持されること、`ym2151-tone-editor` WASMの読み込みが適切に管理されること、および他のデモページへの影響を最小限に抑えることを確認してください。

     期待する出力: 新規ファイル `shared-demo-utils.ts` のコード案、`pop-noise-demo.ts` からの変更点、`delay-vibrato-demo.ts` からの利用例を含むMarkdown形式の計画書。
     ```

3. [Issue #177](../issue-notes/177.md) 添付JSONまわりのドッグフーディングを支援するシナリオ生成
   - 最初の小さな一歩: `pop-noise-demo.ts` の `DEFAULT_ATTACHMENT` や `tones/000.json` の内容を分析し、より複雑なパターンやエッジケースとなり得る添付JSONの構造について検討します。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo-library/pop-noise-demo.ts`, `tones/000.json`

     実行内容: `pop-noise-demo.ts` の `DEFAULT_ATTACHMENT` および `tones/000.json` の構造を分析し、添付JSONが影響を与える可能性のあるYM2151レジスタ（例: 音色設定、LFO、エンベロープ、プログラム変更など）に関する情報を提供してください。その上で、添付JSON機能のドッグフーディングに役立つ、より複雑な添付JSONのシナリオ例を3つ以上提案してください。各シナリオは、どのような機能や挙動を確認できるかを簡潔に説明してください。

     確認事項: 提案するシナリオが、既存のコードベースの機能と大きく乖離しないこと。ハルシネーションを避けるため、具体的なレジスタ値やプログラム変更の記述は簡潔に留めてください。

     期待する出力: 添付JSONが関連するYM2151レジスタに関する概要、および3つ以上の具体的なシナリオ例（各シナリオには簡単な説明を含む）を含むMarkdown形式のレポート。
     ```

---
Generated at: 2026-03-21 07:10:46 JST

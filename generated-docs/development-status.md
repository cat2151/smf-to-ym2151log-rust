Last updated: 2026-03-09

# Development Status

## 現在のIssues
- MML入力時に添付音色JSONのコンパクトnibble形式が意図せず上書きされ、ユーザーが混乱する問題（[Issue #171](../issue-notes/171.md)）が発生しています。
- 添付音色JSONのコンパクトnibble形式の項目名を`CompactTone`から`registers`へ変更し、`web-ym2151`との互換性を高める必要があります（[Issue #172](../issue-notes/172.md)）。
- `demo-library/log-visualizer.ts`と`src/ym2151/converter_tests/effects.rs`の2つのファイルが500行を超えており、コード品質向上のためのリファクタリングが推奨されています（[Issue #174](../issue-notes/174.md)）。

## 次の一手候補
1. [Issue #171](../issue-notes/171.md): MML入力による添付音色JSONの意図しない上書きを修正
   - 最初の小さな一歩: `demo-library/tone-json-demo.ts`の`readAttachmentBytes`関数が、MML入力時に`toneJsonField.value`を正規化する際に、アクティブなプリセットまたは手動でMML入力された内容を上書きしないようにロジックを分析・特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/tone-json-demo.ts

     実行内容: `readAttachmentBytes`関数を分析し、MML入力による`toneJsonField.value`の変更時、またはプリセットが選択されている場合に、既存の`toneJsonField.value`が意図せず上書きされないように修正してください。特に、`attachmentPresetSelect.value`の状態と`normalized.trim() !== original.trim()`の条件が、MML入力のシナリオで適切に機能しているか確認し、必要に応じて修正案を提案してください。

     確認事項: 変更が、ファイルからのMIDI読み込み時やプリセット選択時の挙動に悪影響を与えないことを確認してください。

     期待する出力: 修正された`readAttachmentBytes`関数のコード、およびその変更理由をmarkdown形式で出力してください。
     ```

2. [Issue #172](../issue-notes/172.md): 添付音色JSONのコンパクトnibble形式項目名を`registers`へ変更
   - 最初の小さな一歩: `demo-library/tone-json-attachment.ts` 内で定義されている`ATTACHMENT_PRESETS`の構造と、`normalizeAttachmentText`関数における`"CompactTone"`フィールドの取り扱いを分析し、`"registers"`への変更が影響する範囲を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/tone-json-attachment.ts

     実行内容: `demo-library/tone-json-attachment.ts`を分析し、`ATTACHMENT_PRESETS`内および`normalizeAttachmentText`関数において、コンパクトnibble形式の音色データで`"CompactTone"`フィールドが使用されている箇所を特定してください。`"CompactTone"`を`"registers"`に変更した場合の影響（`web-ym2151`との互換性向上も考慮）と、必要なコード修正案をmarkdown形式で出力してください。

     確認事項: 変更が、既存の添付音色JSONの解析や、`tone-json-demo.ts`でのプリセット読み込みに影響を与えないことを確認してください。

     期待する出力: `"CompactTone"`フィールドの変更に関する分析結果、影響範囲、および具体的な修正案をmarkdown形式で出力してください。
     ```

3. [Issue #174](../issue-notes/174.md): `demo-library/log-visualizer.ts`のリファクタリング
   - 最初の小さな一歩: `demo-library/log-visualizer.ts`内の`renderFromJson`関数を分析し、その巨大なロジックをより小さく、独立した責任を持つ関数に分割する候補を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/log-visualizer.ts

     実行内容: `demo-library/log-visualizer.ts`の`renderFromJson`関数を分析し、リファクタリングのための分割案を検討してください。特に、イベントのフィルタリング、DOM要素の生成、チャンネルレーンの描画、LFOレーンの描画、ノートバーの描画など、独立した機能を持つ部分を抽出し、それぞれを個別の関数として定義する具体的な提案をmarkdown形式で出力してください。

     確認事項: リファクタリングによって、既存の可視化機能やパフォーマンスに悪影響を与えないことを確認してください。

     期待する出力: `renderFromJson`関数のリファクタリング案、分割される各関数の役割と引数、および関数の呼び出し関係を示す概要をmarkdown形式で出力してください。

---
Generated at: 2026-03-09 07:08:29 JST

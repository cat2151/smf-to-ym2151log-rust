Last updated: 2026-03-11

# Development Status

## 現在のIssues
- [Issue #189](../issue-notes/189.md): ディレイビブラートのピッチビジュアライザーが連続的な変化を表現できておらず、表示が飛び飛びに見える問題を解決します。
- [Issue #188](../issue-notes/188.md): ソフトLFOに`key on sync`オプションを追加し、LFOがKey Onに連動せずフレーズ間で継続できるように改善します。
- [Issue #187](../issue-notes/187.md): ソフトLFOのレジスタビジュアライザーで、演奏データのないチャンネルを「ch1～ch7 : 演奏データなし」のようにコンパクトに表示するよう修正します。

## 次の一手候補
1. [Issue #189](../issue-notes/189.md): demo ディレイビブラート pitch ビジュアライザーの表示改善
   - 最初の小さな一歩: `demo-library/delay-vibrato-demo.ts`と`demo-library/log-visualizer-lfo.ts`内のピッチ描画ロジックを分析し、現在の固定Y幅描画を特定する。特に、`LogVisualizer`クラスやその`draw`メソッド、またはピッチデータを処理する部分に注目する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/delay-vibrato-demo.ts, demo-library/log-visualizer-lfo.ts, demo-library/log-visualizer.ts

     実行内容: ディレイビブラートのピッチビジュアライザーの描画ロジックを分析してください。特に、ピッチの変化が固定Y幅で描画されている箇所、および1pxごとのピッチの最大値と最小値を算出してY軸の可変幅描画を実現するために修正が必要な箇所を特定してください。

     確認事項: `LogVisualizer`クラスや関連する描画ヘルパー関数がどのようにピッチデータを消費し、Canvasに描画しているかを確認してください。既存の描画関数に与えられるデータの形式も確認し、変更の影響範囲を把握してください。

     期待する出力: ピッチデータの描画を担当する関数名とファイルパス、およびY軸可変幅描画に必要な変更の具体的なコード差分（擬似コードまたはTypeScriptコード）をMarkdown形式で出力してください。
     ```

2. [Issue #188](../issue-notes/188.md): demo ソフトLFOにkey on syncオプションを追加
   - 最初の小さな一歩: `src/ym2151/converter/register_effects.rs`内の`RegisterLfoDefinition`構造体に`key_on_sync`フィールド（`bool`型）を追加し、`append_register_lfo_for_segment`関数内で`start_time`の計算にこのフィールドを反映させるためのロジックを検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ym2151/converter/register_effects.rs, src/ym2151/mod.rs, src/ym2151/tone.rs

     実行内容: `src/ym2151/mod.rs`の`RegisterLfoDefinition`に`key_on_sync: bool`フィールドを追加し、この設定に基づいてソフトLFOの`start_time`を調整するロジックを`src/ym2151/converter/register_effects.rs`の`append_register_lfo_for_segment`関数に実装してください。`key_on_sync`が`false`の場合、LFOはノートの開始ではなく、曲の開始 (`0.0`秒) から継続して発動するように変更します。また、`src/ym2151/tone.rs`で`RegisterLfoDefinition`のJSONシリアライズ/デシリアライズに`key_on_sync`フィールドを追加してください。

     確認事項: `RegisterLfoDefinition`がJSONからどのようにロードされるか、および`append_register_lfo_for_segment`関数が`NoteSegment`の`start_time`をどのように利用しているかを確認してください。`start_time`を調整することで他のLFO関連計算 (`elapsed`, `phase`, `attack_ratio`) にどのような影響があるかも考慮してください。

     期待する出力: `RegisterLfoDefinition`構造体の変更案と、`append_register_lfo_for_segment`関数の修正案、および`src/ym2151/tone.rs`におけるJSON処理の変更案をRustコードブロックで出力してください。
     ```

3. [Issue #187](../issue-notes/187.md): demo ソフトLFOのレジスタビジュアライザー表示をコンパクト化
   - 最初の小さな一歩: `demo-library/portamento-soft-lfo-demo.ts`および`demo-library/log-visualizer-lfo.ts`内のレジスタビジュアライザーの描画ロジックを分析し、各チャンネルのデータを取得する部分と、HTML要素を生成する部分を特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/portamento-soft-lfo-demo.ts, demo-library/log-visualizer-lfo.ts, demo-library/log-visualizer.ts

     実行内容: ソフトLFOのレジスタビジュアライザーにおいて、演奏データが存在しない（key onがない）チャンネル (`ch1`～`ch7`) の表示をコンパクトにするための修正案を生成してください。具体的には、これらのチャンネルに対して「chX : 演奏データなし」のような一行表示に切り替えるロジックを提案してください。

     確認事項: `LogVisualizer`クラスやLFO関連のデモファイルがどのようにチャンネルごとのレジスタデータを処理し、HTMLに出力しているかを確認してください。チャンネルがアクティブかどうかを判断するための情報（例: `NoteSegment`の有無）が利用可能かどうかも確認してください。

     期待する出力: `demo-library/log-visualizer-lfo.ts`内の描画ロジック、または関連するHTML生成ロジックの修正案をTypeScript/HTMLコードブロックで出力してください。また、空のチャンネルを判定するためのロジックの概要も記述してください。

---
Generated at: 2026-03-11 07:10:24 JST

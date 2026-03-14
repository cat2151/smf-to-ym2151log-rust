Last updated: 2026-03-15

# Development Status

## 現在のIssues
- ポップノイズ関連の機能改善として、[Issue #213](../issue-notes/213.md)でランダム音色ボタンの追加、[Issue #212](../issue-notes/212.md)で波形ビュアの「前のノート」ボタンの修正が課題となっています。
- 音色データの扱いについては、[Issue #208](../issue-notes/208.md)で隣接音色補間デモのデフォルト音色をランダム生成すること、[Issue #33](../issue-notes/33.md)でYM2151 Tone Editorの出力優先度、[Issue #22](../issue-notes/22.md)でデフォルト音色データの配置が未解決です。
- その他、[Issue #177](../issue-notes/177.md)で添付JSONのドッグフーディング、[Issue #83](../issue-notes/83.md)で音色データ全体の整理が求められています。

## 次の一手候補
1. [Issue #212](../issue-notes/212.md) ポップノイズの波形ビュアの「前のノート」ボタンが動作しない
   - 最初の小さな一歩: `demo-library/pop-noise-demo.ts` 内で「前のノート」ボタンのクリックイベントハンドラを特定し、関連する波形データ更新ロジックにデバッグログを追加して原因を調査する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/pop-noise-demo.ts, demo-library/waveform-viewer.ts

     実行内容: `demo-library/pop-noise-demo.ts` において、「前のノート」ボタンのクリックイベントハンドラを特定し、関連する `waveform-viewer.ts` の `showPreviousNote` または類似の関数呼び出し箇所を分析してください。特に、ノートインデックスの更新ロジックと波形データの取得・描画フローに焦点を当て、問題箇所を特定するためのデバッグログ（`console.log`）をコードに追加してください。

     確認事項: 「前のノート」ボタンが `pop-noise-demo.html` でどのように定義され、`pop-noise-demo.ts` でどのようにイベントリスナーがアタッチされているかを確認してください。また、`waveform-viewer.ts` が期待通りにノートデータを扱っているか、そのインターフェースと利用方法を把握してください。

     期待する出力: `demo-library/pop-noise-demo.ts` と `demo-library/waveform-viewer.ts` の修正案をMarkdown形式で提示してください。修正案には、デバッグログの追加箇所と、修正が必要となると思われるロジックの具体的な変更内容を含めてください。
     ```

2. [Issue #213](../issue-notes/213.md) ポップノイズdemoに、ランダム音色ボタンをつけて、いろいろな音色での検証をしやすくする
   - 最初の小さな一歩: `demo-library/pop-noise-demo.html` に「ランダム音色」ボタンを追加し、`demo-library/pop-noise-demo.ts` にそのボタンのクリックイベントハンドラを仮実装する。このハンドラ内で、現在の音色設定を一時的にランダムな値に置き換える処理（ダミーで良い）を追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/pop-noise-demo.html, demo-library/pop-noise-demo.ts, demo-library/shared-demo.ts

     実行内容:
     1. `demo-library/pop-noise-demo.html` に「ランダム音色」ボタンを追加してください。
     2. `demo-library/pop-noise-demo.ts` に、追加したボタンに対応するクリックイベントリスナーを実装してください。
     3. クリックイベントハンドラ内で、`shared-demo.ts` に存在する可能性のあるランダム音色生成機能、またはそれに類する既存の音色設定関数を調査し、それを利用して現在の音色をランダムなものに更新する処理を追加してください。もし直接的な機能が見つからなければ、仮のランダムな音色データを生成し、デモに適用するロジックを記述してください。

     確認事項: `demo-library` 内で既にランダムな音色を生成するためのヘルパー関数やパターンが存在しないか、`shared-demo.ts` や他のデモファイル (`tone-json-demo.ts` など) を確認してください。また、既存の音色変更処理 (`updateTone` のようなもの) がどのように実装されているかを理解してください。

     期待する出力: `demo-library/pop-noise-demo.html` と `demo-library/pop-noise-demo.ts` の変更内容をMarkdown形式で提示してください。HTMLにはボタンの追加箇所、TSファイルにはイベントハンドラとランダム音色適用ロジックのコードを含めてください。
     ```

3. [Issue #208](../issue-notes/208.md) 隣接音色線形補間デモのデフォルトの音色2つは、web-ym2151のランダム音色関数を利用して生成する
   - 最初の小さな一歩: `demo-library/tone-interpolation-demo.ts` を開き、デフォルトの音色データを設定している箇所を特定する。ここに、ランダム音色を生成する関数を呼び出すためのプレースホルダーを追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/tone-interpolation-demo.ts, demo-library/tone-json-attachment.ts, src/ym2151/tone.rs

     実行内容: `demo-library/tone-interpolation-demo.ts` 内で、デモ開始時に使用される2つのデフォルト音色を定義している箇所を特定してください。これらの音色が`web-ym2151`のランダム音色関数（または`tone-json-attachment.ts`等、利用可能な既存のランダム音色生成ロジック）によって生成されるように、既存の音色定義を置き換える変更を検討してください。具体的には、音色生成ロジックの呼び出しと、生成された音色データをデモに適用する部分の実装案を記述してください。

     確認事項: `demo-library` 内でランダム音色を生成する既存のヘルパー関数や、`tone.rs` が定義する音色構造体との互換性を確認してください。デモが音色データをどのように受け取り、適用しているかを理解してください。

     期待する出力: `demo-library/tone-interpolation-demo.ts` の修正案をMarkdown形式で提示してください。変更内容には、デフォルト音色定義箇所と、ランダム音色生成関数呼び出し、および生成された音色をデモに組み込むコードを含めてください。
     ```

---
Generated at: 2026-03-15 07:09:10 JST

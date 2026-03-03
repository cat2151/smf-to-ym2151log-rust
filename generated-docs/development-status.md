Last updated: 2026-03-02

# Development Status

## 現在のIssues
- [Issue #130](../issue-notes/130.md) は、`demo-library`にTypeScriptのフォーマッター/リンターとしてBiomeを導入する作業を進めています。
- [Issue #128](../issue-notes/128.md) は、YM2151レジスタのビジュアライザを改善し、keyon/offとKC値の高低を可視化する機能強化を目指しています。
- [Issue #127](../issue-notes/127.md) は、ディレイビブラートとLFOの低品質なクリックノイズ問題の解決と、その分析のためにWAVエクスポート機能をデモに実装することを目標としています。

## 次の一手候補
1. [Issue #130](../issue-notes/130.md) `demo-library` にBiomeをTypeScriptのFormatter/Linterとして導入する
   - 最初の小さな一歩: `demo-library/package.json` に `@biomejs/biome` を開発依存として追加し、`lint`および`format`スクリプトを定義します。
   - Agent実行プロンプ:
     ```
     対象ファイル: demo-library/package.json

     実行内容: `demo-library/package.json` に、BiomeをTypeScriptのFormatter/Linterとして導入するための設定を追加してください。具体的には、`@biomejs/biome@^2.4.4` をdevDependencyに追加し、`"lint": "biome lint ./"` と `"format": "biome format ./ --write"` のスクリプトを `scripts` セクションに追加してください。

     確認事項: 既存の `scripts` エントリと重複がないか、また、`@biomejs/biome` のバージョンが指定の範囲内であるかを確認してください。

     期待する出力: 更新された `demo-library/package.json` ファイル。
     ```

2. [Issue #128](../issue-notes/128.md) YM2151レジスタのビジュアライザを改善し、KCの高低を可視化する
   - 最初の小さな一歩: 現在のYM2151レジスタビジュアライザの実装ファイルを特定し、KC値の取得方法とビジュアライザへのデータ連携方法について分析します。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/tone-json-demo.ts, src/ym2151/converter/pitch_effects.rs, src/ym2151/note_table.rs

     実行内容: YM2151レジスタビジュアライザ (`demo-library/tone-json-demo.ts` など) がどのようにYM2151のKC (Key Code) 値を受け取り、表示しているかを分析してください。特に、KC値の生成元 (`src/ym2151/note_table.rs` や `src/ym2151/converter/pitch_effects.rs` など) からデモ側へのデータフローと、現在のデモにおけるKCの最大値と最小値を把握する方法について焦点を当ててください。

     確認事項: YM2151のKCレジスタの仕様、現在のデモのデータ構造と描画ロジック、およびRust側でのKC値計算ロジックとの整合性を確認してください。

     期待する出力: KC値の取得、計算、およびビジュアライザへのデータ連携に関する詳細な分析結果をmarkdown形式で出力してください。
     ```

3. [Issue #127](../issue-notes/127.md) ディレイビブラート/LFOの品質改善のため、WAVエクスポート機能をデモに実装する
   - 最初の小さな一歩: `demo-library`にWAVエクスポート機能を追加するために、JavaScript環境でのWAVファイル生成・ダウンロード手法（Web Audio APIの`MediaRecorder`利用など）について調査します。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/delay-vibrato-demo.ts, demo-library/pop-noise-demo.ts, demo-library/portamento-soft-lfo-demo.ts, demo-library/tone-json-demo.ts

     実行内容: ブラウザで実行されるデモ (`demo-library/` 内の各種デモファイル) に、生成された音声をWAVファイルとしてエクスポートする機能を追加するための技術選定と実装方針を調査してください。特に、Web Audio APIの`MediaRecorder`や`AudioWorklet`の利用可能性、またはRust WASM側で直接オーディオデータを処理しWAVバイト列を返す方法を検討し、既存のデモコードへの組み込み方法を提案してください。

     確認事項: 各デモの現在のオーディオ生成フロー、ブラウザの互換性、WAVファイルフォーマットの要件、および既存のRust WASMとの連携方法。

     期待する出力: WAVエクスポート機能の実装に関する技術選定、API利用の提案、および概略の実装手順をmarkdown形式で出力してください。
     ```

---
Generated at: 2026-03-02 07:08:32 JST

Last updated: 2026-02-08

# Development Status

## 現在のIssues
- 波形レンダリングと描画機能が追加されたデモ [Issue #74](../issue-notes/74.md) の動作テストが進行中です。
- `demo-library`の動作確認 [Issue #65](../issue-notes/65.md) が必要とされています。
- `ym2151-tone-editor`の出力するGM000 variations format jsonを優先的に読み込む仕様追加 [Issue #33](../issue-notes/33.md) と、それに関連するトーンデータ作成 [Issue #22](../issue-notes/22.md) が残っています。

## 次の一手候補
1. 波形レンダリングと描画が追加されたMMLデモのテストと検証 [Issue #74](../issue-notes/74.md)
   - 最初の小さな一歩: 最新の`demo-mml`プロジェクトをビルドし、ブラウザでMMLを再生して、波形がUIに正しく描画されているか、およびコンソールにエラーがないかを確認する。
   - Agent実行プロンプ:
     ```
     対象ファイル: demo-mml/index.html, demo-mml/mml-demo.ts, src/ym2151-audio-utils.ts, src/ui-utils.ts

     実行内容: `demo-mml`プロジェクトが正しく動作し、MML再生中に波形データが生成され、UI上に描画されていることを確認するためのテスト計画を立案してください。特に、`src/ym2151-audio-utils.ts`で生成される波形データと`demo-mml/mml-demo.ts`での描画ロジックの連携に注目し、視覚的なフィードバックとコンソール出力の両面から確認すべきポイントを洗い出してください。

     確認事項: `demo-mml`ディレクトリ内で`npm install`が成功し、`npm run dev`で開発サーバーが起動するか。ブラウザの開発者コンソールにエラーや警告がないか。

     期待する出力: `demo-mml`のMML再生と波形描画機能の動作確認手順と、確認すべき主要なポイント（ブラウザのコンソールログ、UIでの波形表示の正確性など）をmarkdown形式で出力してください。
     ```

2. `demo-library`の動作確認 [Issue #65](../issue-notes/65.md)
   - 最初の小さな一歩: `demo-library`ディレクトリに移動し、`npm install`と`npm run dev`を実行して開発サーバーを起動する。ブラウザでアクセスし、デモページが表示されることを確認する。
   - Agent実行プロンプ:
     ```
     対象ファイル: demo-library/package.json, demo-library/index.html, demo-library/library-demo.ts, demo-library/vite.config.ts

     実行内容: `demo-library`プロジェクトが正しくセットアップされ、ブラウザで動作することを確認する手順を記述してください。具体的には、プロジェクトの依存関係のインストール、ビルド、開発サーバーの起動、基本的な機能（Midiファイルの読み込み、再生、音源の切り替えなど）が意図通りに機能しているかどうかの確認方法を含めてください。

     確認事項: `demo-library`ディレクトリ内で`npm install`が成功し、`npm run dev`で開発サーバーが起動するか。ブラウザでデモが正しく表示され、操作可能であるか。

     期待する出力: `demo-library`のセットアップから基本的な機能テストまでの手順をmarkdown形式で出力してください。
     ```

3. YM2151トーンデータ読み込みの仕様検討とデータ作成の効率化 [Issue #33](../issue-notes/33.md), [Issue #22](../issue-notes/22.md)
   - 最初の小さな一歩: 現在のトーンデータ読み込みロジック (`src/ym2151/tone.rs`) をレビューし、`tones/000.json`のような既存のJSONファイルの構造と、`ym2151-tone-editor`が出力すると想定されるGM000 variations format jsonの仕様について調査する。
   - Agent実行プロンプ:
     ```
     対象ファイル: src/ym2151/tone.rs, tones/000.json, issue-notes/33.md, issue-notes/22.md

     実行内容: [Issue #33](../issue-notes/33.md) で提案されている「`ym2151-tone-editor`の出力するGM000 variations format jsonがある場合、従来の`tones/`より優先して読み込む」という仕様について、既存のトーンデータ読み込みロジック (`src/ym2151/tone.rs`など) を分析し、その仕様を実装するために必要な変更点を洗い出してください。加えて、[Issue #22](../issue-notes/22.md) の「`tones/000.json`～`127.json`を実際に配置する」という手作業タスクを、`ym2151-tone-editor`との連携を含め、自動化または効率化するための具体的なアプローチを提案してください。

     確認事項: `src/ym2151/tone.rs`が現在どのようにトーンデータをロードし、どのように複数のトーンデータを管理しているか。`tones/`ディレクトリの既存の`json`ファイルのフォーマット。

     期待する出力: 新しいトーンデータ読み込み仕様の実装方針（優先順位付けのロジック、ファイルパス解決、エラーハンドリングなど）と、`ym2151-tone-editor`からのデータ取り込みを効率化するための具体的な提案（スクリプト、ビルドステップの追加など）をmarkdown形式で出力してください。

---
Generated at: 2026-02-08 07:09:12 JST

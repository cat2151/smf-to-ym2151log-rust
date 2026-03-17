Last updated: 2026-03-18

# Development Status

## 現在のIssues
- [Issue #177](../issue-notes/177.md) では、添付JSON関連機能のドッグフーディングが必要とされており、実際の使用を通じて改善点を見つけるフェーズです。
- [Issue #83](../issue-notes/83.md) は音色データの扱いを整理する課題で、特にデフォルト音色（0～127）の不足が和音SMFの再生品質に影響を与えています。
- [Issue #22](../issue-notes/22.md) は `ym2151-tone-editor` を活用し、不足しているデフォルト音色JSONファイル（`tones/000.json`～`127.json`）の作成と配置を目指しています。

## 次の一手候補
1. [Issue #22](../issue-notes/22.md) - デフォルト音色JSON (`tones/001.json`) の作成とリポジトリへの追加計画
   - 最初の小さな一歩: 既存の `tones/000.json` の構造を分析し、`tones/001.json` を作成するための基本的なひな形JSONの要素を理解する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tones/000.json`

     実行内容: `tones/000.json` の内容を分析し、YM2151の音色データがどのように表現されているかをmarkdown形式で記述してください。特に、`events` 配列内の各オブジェクトの `time`, `addr`, `data` フィールドの意味について説明してください。

     確認事項: YM2151のレジスタマップに関する既存情報（もしあれば）との整合性。`addr` の値がYM2151のレジスタアドレスに対応しているか、`data` がそのレジスタに書き込む値であることを確認してください。

     期待する出力: `tones/000.json` の解析結果と、`tones/001.json` を作成するためのテンプレートJSONの構造（キーと値の例を含む）をmarkdown形式で出力してください。
     ```

2. [Issue #83](../issue-notes/83.md) - `src/ym2151/tone.rs` における音色データロードロジックの解析
   - 最初の小さな一歩: `src/ym2151/tone.rs` のコードを読み込み、音色データがどのようにファイルシステムから読み込まれ、JSONとしてパースされているかを特定する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/ym2151/tone.rs`

     実行内容: `src/ym2151/tone.rs` における音色データ（tone JSON）のロードロジックを分析し、ファイルパスの解決方法、JSONのパース、およびロードされたデータが内部でどのように表現されているかについてmarkdown形式で出力してください。特に、`directories` クレートの使用箇所と、そのパス解決が `tones/` ディレクトリとどのように関連しているかに焦点を当ててください。

     確認事項: 最近のコミット (`8dfaef5`, `81cd2bc`, `fe902cb`) で変更されたファイルパス解決ロジックとの整合性、および `Tone` 構造体の定義。

     期待する出力: 音色データロードのフロー図またはステップバイステップの説明、参照されているファイルパスのパターン、および `Tone` 構造体の主要なフィールドとその役割をmarkdown形式で生成してください。
     ```

3. [Issue #177](../issue-notes/177.md) - 添付JSONのフロントエンドでの利用例の調査 (`demo-library/tone-json-attachment.ts`)
   - 最初の小さな一歩: `demo-library/tone-json-attachment.ts` を詳細に読み、添付JSONがどのようなデータ構造を持ち、どのタイミングで、どのようにAudioContextなどのWeb Audio APIと連携して適用されているかを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `demo-library/tone-json-attachment.ts`

     実行内容: `demo-library/tone-json-attachment.ts` を分析し、添付JSONがフロントエンドのデモ環境でどのように利用され、音源生成やエフェクトにどのように影響を与えているかをmarkdown形式で説明してください。特に、JSONデータの構造、それが適用されるイベントまたはライフサイクル、およびWeb Audio APIのパラメーターへのマッピングを記述してください。

     確認事項: 関連するHTMLファイル (`demo-library/tone-json.html` など) との連携、および `shared-demo.ts` や `library-demo.ts` との相互作用。

     期待する出力: 添付JSONの利用シナリオ、主要なデータ構造（TypeScriptの型定義があればそれも含む）、およびその適用プロセスを示す主要なコードスニペットを含むmarkdown形式のドキュメントを生成してください。
     ```

---
Generated at: 2026-03-18 07:14:35 JST

Last updated: 2026-02-05

# Development Status

## 現在のIssues
- [Issue #58](../issue-notes/58.md): `web-ym2151`ライブラリとしての利用に向け、最低限のデモを別途用意しデプロイ対象にすることを目指しています。
- [Issue #57](../issue-notes/57.md): デモ画面の左下にGitHubリポジトリへのリンクを「GitHub」という文言で追加するタスクが進行中です。
- [Issue #33](../issue-notes/33.md): `ym2151-tone-editor`が出力するGM000 variations format JSONを、従来の`tones/`よりも優先して読み込む新仕様の追加が検討されています。

## 次の一手候補
1. ライブラリとしてweb-ym2151から使えるところまで持っていくため、ライブラリとして利用した場合の最低限のdemo、を別建てで用意し、それもdeploy対象にする [Issue #58](../issue-notes/58.md)
   - 最初の小さな一歩: `src/main.ts`をコピーして`src/library-demo.ts`を作成し、`web-ym2151`がWASMとしてロードされ、シンプルなMIDIイベントを処理するだけの最小限のロジックを実装する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/main.ts`, `src/wasm.rs`, `vite.config.ts`, `package.json`

     実行内容:
     1. `src/main.ts`をコピーして`src/library-demo.ts`を作成し、`web-ym2151`ライブラリのWASMをロードし、`smf-to-ym2151log-rust`の機能を最小限で利用するシンプルなWebページを生成するためのエントリーポイントとする。
     2. `vite.config.ts`を修正し、`library-demo.ts`を新しいエントリーポイントとして含む、別のビルドターゲット（例: `dist-library-demo`）を追加する。
     3. `package.json`の`scripts`セクションに、新しいビルドターゲットを生成するためのコマンドを追加する。
     4. `src/wasm.rs`の内容に変更は加えず、ライブラリとして利用するためのエクスポートが適切に行われているか確認する。

     確認事項:
     - 既存の`src/main.ts`をベースにした現在のデモのビルドプロセスが壊れないことを確認。
     - `web-ym2151`クレートがWASMとして正しくビルド・エクスポートされていることを確認。
     - Viteの設定で複数のエントリーポイントが競合しないように注意。

     期待する出力:
     - `src/library-demo.ts`の新しいファイル内容。
     - `vite.config.ts`の変更された内容。
     - `package.json`の`scripts`セクションに追加されたコマンド。
     - 新しいデモのビルドと実行手順を記したmarkdown形式の簡単なドキュメント。
     ```

2. demoの左下に小さくGitHubという文言で、リポジトリへのリンクをつける [Issue #57](../issue-notes/57.md)
   - 最初の小さな一歩: `src/style.css`にフッターのスタイルを追加し、`index.html`にGitHubリポジトリへのリンクを持つフッター要素を配置する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `index.html`, `src/style.css`

     実行内容:
     1. `index.html`のbody要素の最下部に、`<footer>`タグでGitHubリポジトリ（`https://github.com/cat2151/smf-to-ym2151log-rust`）へのリンクを含む要素を記述する。
     2. このフッター要素が画面の左下に固定で表示され、小さめのフォントサイズになるように`src/style.css`にCSSスタイルを追加する。
     3. `GitHub`というテキストが表示されるようにする。

     確認事項:
     - 既存のレイアウトやスタイルに影響を与えないことを確認。
     - リンクが正しく機能し、新しいタブで開かれることを確認（`target="_blank"`）。
     - モバイルビューでの表示崩れがないか軽くチェック。

     期待する出力:
     - `index.html`に追加されたHTML要素。
     - `src/style.css`に追加されたCSSルール。
     - 変更後のデモ画面のスクリーンショット（または変更されたHTML/CSSがどのように表示されるかの説明）。
     ```

3. 仕様追加。ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む。仮仕様。tone editorのdirをsymlinkで検証想定。 [Issue #33](../issue-notes/33.md)
   - 最初の小さな一歩: `src/ym2151/tone.rs`または関連するロードロジック内で、既存の`tones/`ディレクトリよりも優先して読み込むべき外部ディレクトリパスを引数で受け取るための、関数シグネチャの変更と仮のロードロジックの追加を検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/ym2151/tone.rs`, `src/lib.rs` (wasmのbindingがある場合), `src/main.rs` (CLI利用がある場合)

     実行内容:
     1. `src/ym2151/tone.rs`内のトーンをロードする関数（例: `load_tone`や`ToneManager::new`のようなもの）を特定し、GM000 variations JSONを含む外部ディレクトリパスをオプションで受け取れるように関数シグネチャを修正する。
     2. この新しいパスが提供された場合、従来の`tones/`ディレクトリよりも優先してそのパスからトーンファイルを検索・ロードするロジックをスケルトンとして追加する。
     3. 外部ディレクトリパスが指定されない場合は、既存の`tones/`ディレクトリからロードする従来の振る舞いを維持する。

     確認事項:
     - 既存のトーンロードロジックが壊れないことを確認。
     - Rustの`std::fs`モジュールを使用してディレクトリを読み込む際のパス解決のロバスト性を考慮。
     - WASMバインディングやCLIインターフェースに変更が必要か確認。

     期待する出力:
     - `src/ym2151/tone.rs`の修正された関数シグネチャと新しいロードロジックの概要（具体的な実装ではなく、どの部分を変更するかを示す）。
     - `src/lib.rs`や`src/main.rs`でこの変更がどのように利用されるかの例（関数呼び出しの仮の変更）。
     - symlinkでの検証方法に関する短い説明。

---
Generated at: 2026-02-05 07:09:58 JST

Last updated: 2025-12-04

# Development Status

## 現在のIssues
- [Issue #33](../issue-notes/33.md) は、`ym2151-tone-editor`が出力するGM000 variations format JSONを従来のトーンデータより優先して読み込む新仕様の追加検討を進めています。
- [Issue #22](../issue-notes/22.md) は、`ym2151-tone-editor`を利用して`tones/000.json`から`127.json`までの仮データを手動で作成・配置する作業を指しています。
- これらのIssueは、外部ツールで作成された新しいトーンデータフォーマットの取り込みと、その優先的なロードロジックの実装に焦点を当てています。

## 次の一手候補
1. [Issue #33](../issue-notes/33.md) のための既存トーン読み込みロジック調査
   - 最初の小さな一歩: `src/ym2151/mod.rs`や`src/ym2151/tone.rs`など、既存の`tones/`ディレクトリ以下のJSONファイルを読み込んでいる箇所を特定し、そのデータ構造と処理フローを分析する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ym2151/mod.rs, src/ym2151/tone.rs, src/ym2151/init.rs

     実行内容: これらのファイル内で、`tones/`ディレクトリ以下のJSONファイルを読み込み、YM2151のトーン設定に変換しているロジックを特定し、そのデータ構造と処理フローをmarkdown形式で説明してください。

     確認事項: 既存のトーンデータがどのようにロードされ、アプリケーション内部のどのデータ構造にマッピングされているかを確認してください。特に、`tones/000.json`のようなファイルパスがどのように解決されているかに注目してください。

     期待する出力: `tones/`からのトーンデータ読み込みに関する処理フローと、関連するRustのデータ構造（`struct`など）について、コードスニペットを含めてmarkdown形式で詳細に説明してください。
     ```

2. [Issue #33](../issue-notes/33.md) のための「GM000 variations format json」の構造仮定とパース検討
   - 最初の小さな一歩: `ym2151-tone-editor`が出力するとされるGM000 variations format JSONの仮の構造を定義し、それをRustでパースするためのデータ構造（`serde`を利用）を検討する。既存の`tones/000.json`との違いに注目する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/000.json, src/ym2151/events.rs, src/ym2151/tone.rs

     実行内容: [Issue #33](../issue-notes/33.md) で言及されている「GM000 variations format json」が、既存の`tones/000.json`をベースにどのような拡張または変更が加えられるかを仮定し、その仮定されるJSON構造を記述してください。その後、その仮定されたJSON構造をRustでパースするために`serde`クレートを利用した`struct`定義を提案してください。

     確認事項: 既存の`tones/000.json`の構造を理解し、新しいフォーマットがそれにどのように関連するか、または異なるかを明確にしてください。特に、イベントの追加やデータフィールドの変更点に注目してください。

     期待する出力: 新しいGM000 variations format JSONの仮定される構造をJSON形式で提示し、そのJSONを`serde`でデシリアライズするためのRustの`struct`定義（必要な`#[derive(...)]`属性を含む）をmarkdown形式で出力してください。
     ```

3. [Issue #33](../issue-notes/33.md) のためのsymlink検証環境の検討
   - 最初の小さな一歩: Rustプロジェクトで外部ディレクトリをsymlinkで参照した場合のファイルパス解決について、標準ライブラリ（`std::path::Path`, `std::fs`など）の挙動を調査し、開発環境での設定方法を検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/lib.rs, src/main.rs, Cargo.toml

     実行内容: [Issue #33](../issue-notes/33.md) で提案されている「tone editorのdirをsymlinkで検証想定」を実現するために、Rustアプリケーションがsymlinkされたディレクトリ内のファイルをどのように読み込むべきかを調査し、ファイルパス解決における注意点や推奨されるアプローチを説明してください。具体的には、開発時に`tones/`ディレクトリを`ym2151-tone-editor`の出力ディレクトリへのsymlinkに置き換えることを想定した際の、Rustコード側の変更点やテスト戦略についても言及してください。

     確認事項: Rustの`std::fs`や`std::path`モジュールがsymlinkをどのように扱うか、特に`canonicalize`や`read_link`などの関数がこのシナリオでどのように機能するかを考慮してください。また、`Cargo.toml`で追加の依存関係が必要ないかも確認してください。

     期待する出力: symlinkを用いたファイルパス解決に関するRustの挙動の解説、symlink環境でのファイル読み込みロジックの実装方針、およびテスト戦略に関する考察をmarkdown形式で出力してください。

---
Generated at: 2025-12-04 07:08:28 JST

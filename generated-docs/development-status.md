Last updated: 2026-02-04

# Development Status

## 現在のIssues
-   [Issue #33](../issue-notes/33.md): `ym2151-tone-editor` が出力するGM000 variations format JSONを、従来の`tones/`ディレクトリよりも優先して読み込む新仕様の追加が検討されています。
-   [Issue #22](../issue-notes/22.md): `ym2151-tone-editor` を用いて、`tones/000.json`から`127.json`までのトーン定義ファイルを実際に作成し、配置する作業が課題となっています。
-   [Issue #51](../issue-notes/51.md): WASMデモのGitHub Pagesへのデプロイが最近行われたため、デモの動作確認と関連するドキュメント（例: `WASM_USAGE.md`）の更新が求められています。

## 次の一手候補
1.  [Issue #33](../issue-notes/33.md) `ym2151-tone-editor` 出力JSON優先読み込み機能の検討と実装準備
    -   最初の小さな一歩: `src/ym2151/tone.rs` や `src/ym2151/init.rs` での現在のトーンデータ読み込みロジックを分析し、外部パスからのトーンファイル読み込みを可能にするための変更点を特定します。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `src/ym2151/tone.rs`, `src/ym2151/init.rs`

        実行内容: `src/ym2151/tone.rs` で定義されているトーン構造体と、`src/ym2151/init.rs` でトーンデータがどのように読み込まれているかを分析してください。特に、既存の `tones/` ディレクトリからの読み込み処理を特定し、ユーザーが指定した外部ディレクトリ（symlinkで指定される可能性のある `ym2151-tone-editor` 出力ディレクトリ）からトーンデータを優先的に読み込むための高レベルな設計変更案を検討してください。

        確認事項: 既存のトーン読み込みロジックにおけるファイルパス解決の仕組みと、`Tone` 構造体へのデータのパース方法との依存関係を確認してください。

        期待する出力: 外部パスからのトーンデータ優先読み込みを実装するための設計案をMarkdown形式で出力してください。これには、関連する関数シグネチャの変更案、ファイルシステム操作の抽象化、およびエラーハンドリングに関する考察を含めてください。
        ```

2.  [Issue #22](../issue-notes/22.md) `ym2151-tone-editor` を用いたトーンデータ作成プロセスを検討
    -   最初の小さな一歩: 現在の `tones/000.json` の構造を分析し、`ym2151-tone-editor` のGM000 variations format JSON出力との互換性や、本プロジェクトで利用するために必要な変換の有無を調査します。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `tones/000.json`, `src/ym2151/tone.rs`

        実行内容: `tones/000.json` のJSON構造を詳細に分析し、`src/ym2151/tone.rs` で定義されている `Tone` 構造体と `ym2151-tone-editor` のGM000 variations format JSON（仮定される構造）との間に互換性があるかを評価してください。互換性がない場合、どのようなデータ変換が必要か、あるいは `Tone` 構造体をどのように修正すべきかを検討してください。

        確認事項: `tones/000.json` の各フィールドが `Tone` 構造体のどのメンバーに対応しているか、また、想定されるGM000 variations format JSONが持つであろう追加または異なるフィールドについて確認してください。

        期待する出力: 現在の `tones/000.json` と `Tone` 構造体、および `ym2151-tone-editor` のGM000 variations format JSON（仮定）の比較分析結果をMarkdown形式で出力してください。これには、データのマッピング、必要な構造体変更案、およびデータ変換ロジックに関する初期の考察を含めてください。
        ```

3.  [Issue #51](../issue-notes/51.md) WASMデモのデプロイ後の動作確認とGitHub Pagesドキュメントの更新
    -   最初の小さな一歩: デプロイされたGitHub Pages上のWASMデモ (`index.html`) にアクセスし、関連ドキュメント (`WASM_USAGE.md`, `README.md`) の内容が現在のデモの状態と合致しているかを確認します。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `WASM_USAGE.md`, `README.md`, `index.html`, `.github/workflows/deploy-pages.yml`

        実行内容: `WASM_USAGE.md` および `README.md` に記載されているWASMデモの利用手順、アクセスURL、および機能説明が、現在のGitHub Pages (`index.html`) にデプロイされているデモの実際の動作と完全に一致しているかを確認してください。特に、デモへのリンクが機能しているか、説明されている機能が正しく動作するか、そしてデモの開始方法が明確かを評価してください。

        確認事項: `.github/workflows/deploy-pages.yml` が生成・デプロイするファイルの配置と、`index.html` からの参照パスが正しいことを確認してください。また、デモの利用にあたりユーザーが直面する可能性のある既知の制限事項や前提条件（例: 対応ブラウザ、ファイルサイズ制限）がドキュメントに明記されているかを確認してください。

        期待する出力: WASMデモのGitHub Pages (`index.html`) とドキュメント (`WASM_USAGE.md`, `README.md`) の間の整合性に関する詳細なレポートをMarkdown形式で出力してください。これには、ドキュメントの更新が必要な具体的な箇所（例: URLの修正、手順の追加、機能説明の明確化）、および提案される変更内容を含めてください。

---
Generated at: 2026-02-04 07:12:56 JST

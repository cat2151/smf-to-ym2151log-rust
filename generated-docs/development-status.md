Last updated: 2026-02-13

# Development Status

## 現在のIssues
- デフォルト音色データ ([Issue #83](../issue-notes/83.md), [Issue #22](../issue-notes/22.md)) が未整備で、和音SMFがsine waveでしか鳴らせない現状がある。
- `ym2151-tone-editor`で作成されたGM000 variations formatのJSONを、既存の`tones/`ディレクトリより優先して読み込む仮仕様 ([Issue #33](../issue-notes/33.md)) の導入が検討されている。
- ブラウザでのランダム音色・MML演奏デモは別リポジトリ(`web-ym2151`)での実施 ([Issue #83](../issue-notes/83.md)) が計画されている。

## 次の一手候補
1.  デフォルト音色データの簡易作成と配置（[Issue #22](../issue-notes/22.md), [Issue #83](../issue-notes/83.md)）
    -   最初の小さな一歩: `tones/000.json`を参考に、いくつかの基本的なGM音色（例: Piano, Organ, Guitarなど）のダミーJSONファイルを`tones/`ディレクトリに作成し、既存の`tones/000.json`と同様の構造で配置する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `tones/000.json`、および新しく作成する`tones/`ディレクトリ内のJSONファイル群、`tones/README.md`

        実行内容: `tones/000.json`の内容をベースに、以下のGM音色番号に対応するダミーJSONファイルを作成してください。内容は`000.json`と同一で構いません。
        - 001.json (Acoustic Grand Piano)
        - 002.json (Bright Acoustic Piano)
        - 005.json (Electric Piano 1)
        - 025.json (Acoustic Guitar (nylon))
        また、`tones/README.md`に、これらのダミーファイルが手動生成されたものであり、将来的には`ym2151-tone-editor`で生成された実際の音色データに置き換えられる予定である旨を追記してください。

        確認事項: 生成するJSONファイル名がGM音色番号と一致していること。`tones/README.md`の記述が明確であること。

        期待する出力: 新しく作成されるJSONファイル群（`tones/001.json`, `tones/002.json`, `tones/005.json`, `tones/025.json`）のMarkdownコードブロックと、`tones/README.md`の更新内容を示すMarkdown変更提案。
        ```

2.  `ym2151-tone-editor`出力JSONの優先読み込みロジックの仮実装（[Issue #33](../issue-notes/33.md)）
    -   最初の小さな一歩: 音色読み込みの中心となるファイル（例: `src/ym2151/tone.rs`や`src/lib.rs`）を特定し、現在の`tones/`からの読み込みパスに加えて、仮の優先パス（例: `editor_tones/`）からの読み込みを試みるスケルトンコードを追加する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `src/lib.rs`, `src/ym2151/tone.rs`, `src/ym2151/converter.rs` (音色読み込みロジックに関連する可能性のあるファイル)

        実行内容: `src/ym2151/tone.rs`または`src/lib.rs`内で、音色JSONファイルを読み込む現在のロジックを分析し、以下の仕様を満たすように仮の優先読み込みパスを追加してください。
        1. 任意の音色番号（例: 000）について、まず`editor_tones/000.json`というパスからのファイル読み込みを試みる。
        2. もし`editor_tones/000.json`が存在しなかった場合、または読み込みに失敗した場合に限り、既存の`tones/000.json`からの読み込みにフォールバックする。
        3. `editor_tones/`パスは、最初はハードコードで構いません。

        確認事項: 既存の音色読み込み処理（`tones/`からの読み込み）が、新しいロジックによって意図せず破壊されないこと。ファイルI/Oエラーハンドリングが適切に行われること。

        期待する出力: 提案されるRustコードスニペットと、関連ファイルの変更差分をMarkdown形式で記述。
        ```

3.  音色データ読み込みの統合テストケースの追加（[Issue #83](../issue-notes/83.md)関連）
    -   最初の小さな一歩: `tests/integration_tests.rs`に、異なる音色データファイル（例えば、デフォルトの`tones/000.json`と、`editor_tones/000.json`という別の内容のファイルがある状況をシミュレートする）を読み込み、期待される音色がロードされているかを確認する簡単なテストケースを追加する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `tests/integration_tests.rs`, `src/ym2151/tone.rs` (テストに必要な公開APIの確認のため)

        実行内容: `tests/integration_tests.rs`に、音色読み込みロジック（特にIssue #33で提案されている優先読み込み）の動作を検証する新しい統合テストケースを追加してください。
        このテストは、以下のシナリオをカバーするものとします。
        1. ダミーの`editor_tones/000.json`ファイルを一時的に作成し、それが`tones/000.json`よりも優先して読み込まれることを確認する。
        2. `editor_tones/000.json`が存在しない場合に、`tones/000.json`が正しく読み込まれることを確認する。
        テストに必要なダミーファイルは、テストケース内で作成・削除してください。

        確認事項: テストが既存のテストスイートに影響を与えないこと。テストデータとして使用するJSONファイルの内容は簡潔で、テストの意図を明確に表すものであること。

        期待する出力: `tests/integration_tests.rs`に追加されるRustテストコードのMarkdownブロック。
        ```

---
Generated at: 2026-02-13 07:12:22 JST

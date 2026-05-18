Last updated: 2026-05-19

# Development Status

## 現在のIssues
- [Issue #177](../issue-notes/177.md) は、添付JSON機能が期待通りに動作するかを手動で確認する「ドッグフーディング」の実施を求めています。
- [Issue #83](../issue-notes/83.md) では、プロジェクト内の音色データの扱いを整理し、デフォルト音色の不足と和音SMFの対応が課題として挙げられており、一部デモは別リポジトリで行う方針です。
- [Issue #22](../issue-notes/22.md) は、[Issue #83](../issue-notes/83.md) の具体策として、`ym2151-tone-editor` を用いてデフォルト音色JSON（`tones/000.json`から`127.json`）を作成・配置する人力タスクです。

## 次の一手候補
1. [Issue #22](../issue-notes/22.md) への対応支援として、既存の `tones/000.json` の構造を分析し、他のデフォルト音色JSONを作成する際の雛形となる情報を提供する。
   - 最初の小さな一歩: `tones/000.json` の主要なフィールド（`type`, `events`内の`time`, `addr`, `data`）とその意味について、他のJSON作成の参考となる説明をMarkdownで記述する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `tones/000.json`

     実行内容: `tones/000.json` を分析し、主要なキー (`type`, `events`内の`time`, `addr`, `data`) の役割と想定される値の範囲について説明を生成してください。特に `events` 配列内の各オブジェクトがYM2151レジスタ設定イベントを表すことを明記してください。

     確認事項: `tones/000.json` 以外のJSONファイルは存在しないため、既存の構造のみに焦点を当ててください。

     期待する出力: `tones/000.json` の構造を説明するMarkdown形式のドキュメント。他の音色JSONを作成する際の参考となるよう、各フィールドの意味を明確にしてください。
     ```

2. [Issue #83](../issue-notes/83.md) の音色データ整理を支援するため、音色データ（tones/*.json）がRustコード内でどのようにロード・利用されているかの全体像を把握する。
   - 最初の小さな一歩: `src/options/attachments.rs` と `src/ym2151/tone.rs` を中心に、JSONファイルからYM2151のTone構造体へのパースおよびデータ利用の流れを追跡し、概要を説明する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `src/options/attachments.rs`, `src/ym2151/tone.rs`, `src/api.rs`

     実行内容: これらのファイルを参照し、`tones/*.json` ファイルがアプリケーションにロードされ、YM2151のToneデータとして内部的に扱われるまでの主要な処理フローを分析してください。特に、JSONのパース、Tone構造体への変換、そしてそのデータがYM2151のレジスタ設定にどのように利用されるかについて記述してください。

     確認事項: Toneデータがシステム全体でどのように管理され、Midiイベント処理中に適用されるかのハイレベルな視点に焦点を当ててください。詳細なレジスタ操作ロジックは深く追求しないものとします。

     期待する出力: `tones/*.json` のロードからYM2151レジスタ設定への利用までのフローを説明するMarkdown形式のドキュメント。
     ```

3. [Issue #177](../issue-notes/177.md) の添付JSONドッグフーディングに備え、既存の添付JSON関連テストケース (`src/ym2151/converter_tests/attachments.rs` など) を分析し、どのようなケースがカバーされているかをまとめる。
   - 最初の小さな一歩: `src/ym2151/converter_tests/attachments.rs` 内のテスト関数をリストアップし、それぞれのテストがどのような添付JSONの挙動を検証しているかを簡潔に説明する。
   - Agent実行プロンプ:
     ```
     対象ファイル: `src/ym2151/converter_tests/attachments.rs`, `src/ym2151/converter_tests/attachments_change_to_next_tone/guards.rs`, `src/ym2151/converter_tests/attachments_change_to_next_tone/interpolation.rs`, `src/ym2151/converter_tests/attachments_change_to_next_tone/keep_fields.rs`, `src/ym2151/converter_tests/attachments_change_to_next_tone/mod.rs`, `src/ym2151/converter_tests/attachments_program_effects.rs`

     実行内容: これらのテストファイルを分析し、添付JSONが音色変換ロジックにどのように影響するかを検証している主要なテストケースを洗い出してください。具体的には、音色変更時の挙動、ガード条件、補間などがどのようにテストされているかを記述してください。

     確認事項: テストコードの具体的な実装詳細よりも、各テストケースが「何を検証しているか」という観点に集中してください。

     期待する出力: 添付JSON関連のテストケースの概要をまとめたMarkdown形式のドキュメント。これにより、未テストのシナリオやドッグフーディングで確認すべき重点領域が明らかになります。

---
Generated at: 2026-05-19 07:26:34 JST

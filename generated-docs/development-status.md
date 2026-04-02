Last updated: 2026-04-03

# Development Status

## 現在のIssues
- [Issue #177](../issue-notes/177.md) は、添付JSON機能全般の実際の利用と検証（ドッグフーディング）を通じて、その振る舞いを深く理解し、改善点を見つけることに重点を置いています。
- [Issue #83](../issue-notes/83.md) は、デフォルト音色データ（0-127）の未整備状態を解消し、音色データの管理と利用方法を整理することを目的としています。
- [Issue #22](../issue-notes/22.md) は、`ym2151-tone-editor` を活用し、`tones/` ディレクトリに具体的なデフォルト音色JSONファイル（000.json～127.json）を配置する作業を進めます。

## 次の一手候補
1. デフォルト音色JSONファイルの雛形生成 [Issue #22](../issue-notes/22.md)
   - 最初の小さな一歩: 既存の `tones/000.json` の構造を分析し、他のデフォルト音色（001-127）の空のJSONファイルの雛形を生成する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tones/000.json`, `src/options/attachments.rs`

     実行内容: `tones/000.json` のJSONスキーマ（主要なキーと値の型）を分析し、その構造に基づいた `tones/001.json`, `tones/002.json`, `tones/003.json` の3つの空のJSONファイルを生成してください。`type` と `events` フィールドを持つ基本的な構造を維持してください。

     確認事項: 生成されるJSONファイルが `src/options/attachments.rs` でパース可能であるか、および基本的な構造が `tones/000.json` と整合しているかを確認してください。

     期待する出力: 生成された `tones/001.json`, `tones/002.json`, `tones/003.json` の内容をMarkdownのコードブロックで出力してください。
     ```

2. 音色データ読み込み・適用フローの分析 [Issue #83](../issue-notes/83.md)
   - 最初の小さな一歩: `tones/` ディレクトリの音色JSONファイルが、アプリケーションのどのコードパスで読み込まれ、YM2151のレジスタ設定として適用されるかを分析する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/api.rs`, `src/options/attachments.rs`, `src/ym2151/converter.rs`, `src/ym2151/tone.rs`, `src/ym2151/events.rs`, `src/ym2151/init.rs`

     実行内容: `tones/` ディレクトリに配置されたJSON形式の音色データが、アプリケーション起動時またはMIDIイベント処理時にどのように読み込まれ、`src/ym2151/converter.rs` でYM2151のレジスタ設定として適用されるか、その主要なコードパスとデータフローを分析し、Markdown形式で説明してください。

     確認事項: `src/options/attachments.rs` の `Attachment` 構造体がJSONパースにどのように関与しているか、`Tone` 構造体と `ToneEvent` がどのように変換されるかを特に注目してください。

     期待する出力: 音色データ読み込みからレジスタ適用までのデータフローを簡潔に説明したMarkdownドキュメント。
     ```

3. `ChangeToNextTone` 保持フィールド機能の検証準備 [Issue #177](../issue-notes/177.md)
   - 最初の小さな一歩: 最近追加された `ChangeToNextTone` 機能の「保持フィールドのJSON指定」について、簡単な検証シナリオと必要な添付JSON設定例を考案する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/options/attachments.rs`, `src/ym2151/converter_tests/attachments_change_to_next_tone/keep_fields.rs`, `src/ym2151/converter/register_effects/tone_interpolation.rs`

     実行内容: コミット `d155352` で追加された `ChangeToNextTone` 機能における「保持フィールドのJSON指定」の動作を理解するため、関連するコードと既存のテストケースを分析してください。この機能の具体的な利用方法を示す簡単な検証シナリオと、それに必要な添付JSONの設定例をMarkdown形式で提案してください。

     確認事項: 提案するシナリオは、実際にコードで表現可能であり、かつ、`ChangeToNextTone` の保持フィールド機能の効果が明確に確認できるものであることを確認してください。

     期待する出力: `ChangeToNextTone` の保持フィールド設定に関する機能説明、検証用の添付JSON設定例、および期待される動作の簡潔な説明を記述したMarkdownドキュメント。
     ```

---
Generated at: 2026-04-03 07:15:22 JST

Last updated: 2025-11-16

# Development Status

## 現在のIssues
- [Issue #22](../issue-notes/22.md) では、YM2151音源の豊かな表現を実現するため、`ym2151-tone-editor` を用いて `tones/000.json` から `127.json` までのトーン定義ファイルを作成するタスクが主要課題です。
- 最近の進捗として、MIDIチャンネル9のYM2151チャンネル0への優先マッピング、ポリフォニーに基づいたチャンネル割り当て、プログラムチェンジのサポートなど、MIDI変換ロジックの改善が進みました。
- これらの変換ロジックとトーン定義ファイルが連携することで、最終的なYM2151ログの品質が向上するため、音色データの充実が次の重要なステップとなります。

## 次の一手候補
1. [Issue #22](../issue-notes/22.md) の `tones/000.json` の内容をレビューし、基本的な音色として機能するか確認する
   - 最初の小さな一歩: `tones/000.json` の内容を読み解き、YM2151の音源設定として意図通りか、または基本的な音が出せる状態かを確認する。
   - Agent実行プロンプ:
     ```
     対象ファイル: tones/000.json と src/ym2151/tone.rs

     実行内容: tones/000.json のYM2151レジスタ設定 (`addr`, `data`) が、`src/ym2151/tone.rs` で定義されているYM2151の音源設定ロジックとどのように関連しているかを分析し、設定されている値が一般的なPCM音源やFM音源の基本的な音色（例: sine wave, square wave）を生成するのに適しているか評価してください。可能であれば、各レジスタ設定の簡単な説明を追記してください。

     確認事項: `ym2151-tone-editor` がどのようなJSONフォーマットを生成するか、またYM2151のレジスタマップに関する既存の知識と照らし合わせながら分析してください。

     期待する出力: `tones/000.json` の各イベントの `addr` と `data` について、YM2151のレジスタマップに照らした簡単な説明と、それが生成する音色について考察したmarkdown形式のレビュー結果。
     ```

2. [Issue #22](../issue-notes/22.md) に向けて、`ym2151-tone-editor` の利用手順を調査・文書化する
   - 最初の小さな一歩: `ym2151-tone-editor` の最新版の入手先、ビルド方法、基本的な使い方（特にJSON出力機能）に関する情報を収集する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/README.md (または新規ファイルとして docs/tone-editor-guide.md)

     実行内容: `ym2151-tone-editor` のGitHubリポジトリ（または関連する情報源）をインターネット検索で特定し、その導入手順（ビルド方法、依存関係など）と、`tones/*.json` 形式のファイルを生成するための基本的な操作手順を調査してmarkdown形式でまとめてください。

     確認事項: `ym2151-tone-editor` が提供するJSON出力が、現在のプロジェクトの `tones/*.json` ファイル構造と互換性があるかを確認してください。もし情報がなければ、互換性確認が今後の課題である旨を記述してください。

     期待する出力: `ym2151-tone-editor` の導入・利用ガイドをmarkdown形式で出力してください。
     ```

3. 最近実装されたMIDI変換ロジックの統合テストを拡充する
   - 最初の小さな一歩: `tests/integration_tests.rs` と `src/ym2151/converter.rs` を分析し、最近のコミットで追加された機能が既存のテストでカバーされているか、または新たなテストケースが必要かを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tests/integration_tests.rs, src/ym2151/converter.rs, src/midi/parser.rs, tests/create_test_midi.py, tests/test_data/program_change.mid

     実行内容: 最近のコミット (`polyphony-based allocation`, `MIDI channel 9 priority mapping`, `Program Change support`) で導入された機能が、`tests/integration_tests.rs` の既存のテストケースで適切に検証されているかを確認してください。特に、`tests/test_data/program_change.mid` が変換ロジックにどのように影響するかを分析し、必要であればこれらの新機能を網羅するような追加のテストケースの概要を提案してください。

     確認事項: 既存のテストデータ (`tests/test_data/`) や `create_test_midi.py` を活用して、効率的にテストケースを設計できるか検討してください。

     期待する出力: 現在のテストの網羅性に関する分析結果と、新しいMIDI変換機能を検証するために推奨される追加のテストケース（ファイル名、期待されるシナリオの概要）をmarkdown形式で出力してください。

---
Generated at: 2025-11-16 07:06:18 JST

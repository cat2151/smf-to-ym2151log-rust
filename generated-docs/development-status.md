Last updated: 2025-11-22

# Development Status

## 現在のIssues
- [Issue #22](../issue-notes/22.md) は、YM2151の音色データ `tones/000.json` から `tones/127.json` を作成するという手作業タスクです。
- これらの音色データは `ym2151-tone-editor` を利用して個別に作成し、`tones/` ディレクトリに配置する必要があります。
- 現在、`tones/000.json` のみが存在し、残りの127個の音色ファイルはまだ作成されていません。

## 次の一手候補
1. [Issue #22](../issue-notes/22.md) 基本的な音色データをいくつか作成し、コンバータでのロードを検証
   - 最初の小さな一歩: `ym2151-tone-editor` の出力形式に合わせて `tones/001.json` にシンプルなピアノの音色を仮で作成し、`src/ym2151/converter.rs` がこのファイルをロードできるか確認するための簡単なテストを追加する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tones/001.json`, `src/ym2151/converter.rs`, `tests/integration_tests.rs`

     実行内容: `tones/000.json`を参考に、`tones/001.json`という名前で簡単なピアノの音色設定をJSON形式で作成してください。その後、`src/ym2151/converter.rs`が`tones/`ディレクトリ内のファイルを相対パスで正しくロードできるかを確認するため、`tests/integration_tests.rs`に`tones/001.json`をロードするテストケースを追加してください。

     確認事項: `src/ym2151/converter.rs`が`tones/`ディレクトリ内のファイルを相対パスで正しく参照できるか、およびJSONファイルのパース処理に問題がないかを確認してください。

     期待する出力: `tones/001.json`のファイル内容と、`src/ym2151/converter.rs`がそのファイルをロードできることを示す、またはその機能追加を示すRustコードの変更点。テストケースを追加した場合は、テスト結果の確認を可能にするコード変更。
     ```

2. [Issue #22](../issue-notes/22.md) 音色データロード/管理の拡張性検討
   - 最初の小さな一歩: `src/ym2151/tone.rs` に、`tones/`ディレクトリから全ての `.json` ファイル（`000.json`〜`127.json`）を読み込み、Program Change番号と関連付けるための関数や構造体のスケルトンを定義する。
   - Agent実行プロンプト:
     ```
     対象ファイル: `src/ym2151/tone.rs`, `src/ym2151/mod.rs`

     実行内容: `src/ym2151/tone.rs`に、`tones/`ディレクトリから`000.json`から`127.json`の形式のファイルを一括で読み込み、内部的にマップ構造（`HashMap<u8, Tone>`）として保持する機能の設計案を記述し、その関数のプロトタイプをRustコードで実装してください。`src/ym2151/mod.rs`からこの機能を呼び出すことを想定した変更も検討してください。

     確認事項: ファイルシステムからの読み込みエラーハンドリング、存在しないProgram Change番号の扱いの検討。現在の`Tone`構造体が音色管理に適しているか。

     期待する出力: 音色データの一括ロードを行うRust関数のスケルトンコードと、この機能に関する設計メモをMarkdown形式で出力してください。
     ```

3. [Issue #22](../issue-notes/22.md) 既存MIDIファイルと新規音色データの変換テスト
   - 最初の小さな一歩: `tests/integration_tests.rs` 内の既存のテストを変更または新規追加し、`tones/000.json` を使用して `tests/test_data/simple_melody.mid` を変換し、出力されるYM2151ログの基本構造が正しいことをアサートする。
   - Agent実行プロンプト:
     ```
     対象ファイル: `tests/integration_tests.rs`, `src/ym2151/converter.rs`, `tests/test_data/simple_melody.mid`, `tones/000.json`

     実行内容: `tests/integration_tests.rs`を開き、`tests/test_data/simple_melody.mid`を`src/ym2151/converter.rs`で変換するテストケースを追加または修正してください。このテストケースでは、`tones/000.json`を音色データとして利用し、変換後のYM2151イベントのリストが空でないこと、または特定のYM2151レジスタ設定が含まれていることを確認するアサートを追加してください。

     確認事項: テストが`tones/000.json`を正しく参照できるか。変換ロジックが期待通りのYM2151イベントを生成するか。

     期待する出力: `tests/integration_tests.rs`に追加された、MIDIファイルと音色データを統合した変換テストケースのRustコード。
     ```

---
Generated at: 2025-11-22 07:07:25 JST

Last updated: 2026-02-10

# Development Status

## 現在のIssues
- デフォルト音色データ([Issue #83](../issue-notes/83.md))が未整備で和音SMFがsine wave再生に限られるため、ym2151-tone-editorからのGM000 variations format JSONを優先的に読み込む仕様([Issue #33](../issue-notes/33.md))の追加が求められています。
- これには、ym2151-tone-editorを使用して`tones/000.json`～`127.json`を作成・配置し、音色データの種類を増やす作業([Issue #22](../issue-notes/22.md))が先行します。
- 最終的に、別リポジトリ(web-ym2151)での音色デモとMML演奏デモの実施が対策案として挙げられています。

## 次の一手候補
1.  ym2151-tone-editorの出力形式JSON（GM000 variations format）を既存`tones/`より優先して読み込む機能の実装 ([Issue #33](../issue-notes/33.md))
    -   最初の小さな一歩: `src/ym2151/init.rs`などの既存音色読み込み処理を分析し、外部パスからの優先ロードロジック追加の設計案を作成する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `src/ym2151/init.rs`, `src/ym2151/tone.rs`, `src/ym2151/mod.rs`

        実行内容: `src/ym2151`内の音色データ読み込みロジックを分析し、外部パスから指定されたJSONファイルを既存の`tones/`ディレクトリよりも優先してロードするための設計案をmarkdown形式で出力してください。特に、既存の`tones/`ディレクトリのパス管理と、新しい優先パスの追加方法について詳細に記述してください。

        確認事項: 既存の音色データロード処理との競合がないか、ファイルI/Oエラーハンドリングが適切に考慮されているか、パフォーマンスへの影響がないかを確認してください。

        期待する出力: 優先読み込み機能を実現するためのAPI（例: `load_tone_from_path(path: &str)`）の提案と、それを既存のシステムに統合するための変更点の概要をmarkdown形式で提供してください。
        ```

2.  ym2151-tone-editorを利用したデフォルト音色データ(GM000-127)の生成と仮配置 ([Issue #22](../issue-notes/22.md))
    -   最初の小さな一歩: `tones/000.json`の構造を参考に、`tones/001.json`と`tones/002.json`のダミー音色JSONファイルを2つ生成し、`tones/`ディレクトリに仮配置する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `tones/`ディレクトリ

        実行内容: `tones/000.json`の構造を分析し、これを参考に`tones/001.json`と`tones/002.json`の2つのダミー音色JSONファイルを作成してください。これらのファイルは、`ym2151-tone-editor`で生成される形式を模倣し、異なる音色設定を示すように適当な値を設定してください。

        確認事項: 生成されるJSONファイルが`tones/000.json`と同様の構造と`events`配列を持っていることを確認してください。また、既存のファイルに上書きしないように注意してください。

        期待する出力: `tones/001.json`と`tones/002.json`のファイル内容をmarkdownコードブロックで出力し、それらを`tones/`ディレクトリに配置することを提案してください。
        ```

3.  既存音色データを用いた和音再生の検証と、sine wave以外の音色適用ロジックの改善 ([Issue #83](../issue-notes/83.md))
    -   最初の小さな一歩: `src/ym2151/channel_allocation.rs` や `src/ym2151/event_processor.rs` を中心に、和音再生時の音色割り当てロジックを分析し、`tones/000.json`がどのように適用されているか（またはされていないか）を特定する。
    -   Agent実行プロンプ:
        ```
        対象ファイル: `src/ym2151/channel_allocation.rs`, `src/ym2151/event_processor.rs`, `src/ym2151/converter.rs`, `src/midi/parser.rs`

        実行内容: これらのファイル群において、MIDIメッセージからYM2151のレジスタ設定への変換、特に音色（program change）と和音（multiple notes on a single channel or across channels）の処理ロジックを分析し、`tones/000.json`のようなカスタム音色データがどのように利用されているか、またはなぜsine waveに限定されているのかを調査してください。現在の挙動と期待される挙動のギャップを特定し、改善点があれば提案してください。

        確認事項: MIDI `Program Change`イベントがYM2151の音色設定にどのようにマッピングされているか、`tones/`ディレクトリのJSONデータが実際にロードされ、レジスタに適用されているかのコードパスを確認してください。

        期待する出力: 既存の音色データロード/適用フローの現状分析結果と、和音再生時にsine wave以外の音色を正しく適用するための具体的な改善提案（コード変更の概要または擬似コード）をmarkdown形式で出力してください。
        ```

---
Generated at: 2026-02-10 07:16:22 JST

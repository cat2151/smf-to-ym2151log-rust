Last updated: 2026-04-02

# Development Status

## 現在のIssues
- [Issue #234](../issue-notes/234.md) は、隣接音色線形補間デモにおいてランダム音色のレジスタ設定を維持しつつ、添付JSONでMUL維持などの個別編集を可能にするための機能改善を求めています。
- [Issue #177](../issue-notes/177.md)、[Issue #83](../issue-notes/83.md)、[Issue #22](../issue-notes/22.md) は、音色データの扱いと整理が課題であり、添付JSONのドッグフーディング、データ整理、`ym2151-tone-editor`を用いたデフォルト音色（`tones/000.json`～`127.json`）の作成と配置が進行中です。
- 全体として、音色データの柔軟な管理とデモでの活用に関する機能改善とデータ整備が主要な開発テーマとなっています。

## 次の一手候補
1. 隣接音色線形補間デモのMUL維持on/offなどを添付JSONで設定可能にする [Issue #234](../issue-notes/234.md)
   - 最初の小さな一歩: `src/ym2151/converter/register_effects/tone_interpolation.rs` 内で、ランダム音色生成時にMULの設定が添付JSONで上書き可能か、またはMUL維持の設定を導入できるかを調査する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ym2151/converter/register_effects/tone_interpolation.rs, src/ym2151/tone.rs

     実行内容: `tone_interpolation.rs` の音色生成ロジックにおいて、添付JSON (Tone構造体) からMUL維持on/offなどの設定を読み込み、ランダム音色生成時のMUL値に適用するための変更点を分析し、その実装プランをmarkdown形式で出力してください。具体的には、Tone構造体へのフィールド追加、パースロジックの変更、および補間処理での利用方法を記述します。

     確認事項: 既存の音色補間ロジックや、Tone構造体を利用する他の箇所への影響、およびデシリアライズ処理の互換性を確認してください。

     期待する出力: MUL維持on/off設定をTone JSONに追加し、`tone_interpolation.rs` で利用するための実装プランをmarkdown形式で記述してください。
     ```

2. `tones/000.json`～`127.json` のいくつかを作成し、プロジェクトに配置する [Issue #22](../issue-notes/22.md)
   - 最初の小さな一歩: `ym2151-tone-editor` を利用して、任意のデフォルト音色（例: 001.json）を一つ作成し、`tones/` ディレクトリに配置する。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/001.json (新規作成)

     実行内容: `ym2151-tone-editor` を使って、基本的なピアノのような音色を表現するJSONファイル `tones/001.json` を作成してください。この際、提供されている `tones/000.json` を参考に、必須となるレジスタ設定を含めてください。

     確認事項: 生成されたJSONがYM2151のレジスタ設定として有効であり、既存の`tones/000.json`のフォーマットと整合性が取れていることを確認してください。

     期待する出力: 新規作成した `tones/001.json` の内容をmarkdown形式のコードブロックで出力し、その音色の特徴（例: ピアノ、ベースなど）を簡潔に説明してください。
     ```

3. 添付JSONと音色データの扱いに関するドキュメントと利用整理 [Issue #177](../issue-notes/177.md), [Issue #83](../issue-notes/83.md)
   - 最初の小さな一歩: `src/ym2151/tone.rs` の `Tone` 構造体と、それに対応する `tones/000.json` の内容を比較し、現在サポートされているフィールドと、将来的に拡張したいフィールド（特に [Issue #234](../issue-notes/234.md) で言及されているMUL維持など）を洗い出す。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ym2151/tone.rs, tones/000.json

     実行内容: `src/ym2151/tone.rs` で定義されている `Tone` 構造体のフィールドと、実際の `tones/000.json` で使われているJSONキーを比較し、現状の対応状況を分析してください。また、`issue-notes/234.md` で言及されているMUL維持on/offのような拡張性について、`Tone` 構造体への追加を検討する際の考慮事項をmarkdown形式で記述してください。

     確認事項: `Tone` 構造体の既存フィールドがJSONと正しくマッピングされているか、および新たなフィールドを追加する場合のパース互換性やデフォルト値の扱いを検討してください。

     期待する出力: `Tone` 構造体と `tones/000.json` の現状のマッピング表と、今後の拡張（特に #234 の内容）に向けた `Tone` 構造体変更の検討事項をmarkdown形式で出力してください。
     ```

---
Generated at: 2026-04-02 07:16:55 JST

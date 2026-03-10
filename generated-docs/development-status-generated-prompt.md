Last updated: 2026-03-11

# 開発状況生成プロンプト（開発者向け）

## 生成するもの：
- 現在openされているissuesを3行で要約する
- 次の一手の候補を3つlistする
- 次の一手の候補3つそれぞれについて、極力小さく分解して、その最初の小さな一歩を書く

## 生成しないもの：
- 「今日のissue目標」などuserに提案するもの
  - ハルシネーションの温床なので生成しない
- ハルシネーションしそうなものは生成しない（例、無価値なtaskや新issueを勝手に妄想してそれをuserに提案する等）
- プロジェクト構造情報（来訪者向け情報のため、別ファイルで管理）

## 「Agent実行プロンプト」生成ガイドライン：
「Agent実行プロンプト」作成時は以下の要素を必ず含めてください：

### 必須要素
1. **対象ファイル**: 分析/編集する具体的なファイルパス
2. **実行内容**: 具体的な分析や変更内容（「分析してください」ではなく「XXXファイルのYYY機能を分析し、ZZZの観点でmarkdown形式で出力してください」）
3. **確認事項**: 変更前に確認すべき依存関係や制約
4. **期待する出力**: markdown形式での結果や、具体的なファイル変更

### Agent実行プロンプト例

**良い例（上記「必須要素」4項目を含む具体的なプロンプト形式）**:
```
対象ファイル: `.github/workflows/translate-readme.yml`と`.github/workflows/call-translate-readme.yml`

実行内容: 対象ファイルについて、外部プロジェクトから利用する際に必要な設定項目を洗い出し、以下の観点から分析してください：
1) 必須入力パラメータ（target-branch等）
2) 必須シークレット（GEMINI_API_KEY）
3) ファイル配置の前提条件（README.ja.mdの存在）
4) 外部プロジェクトでの利用時に必要な追加設定

確認事項: 作業前に既存のworkflowファイルとの依存関係、および他のREADME関連ファイルとの整合性を確認してください。

期待する出力: 外部プロジェクトがこの`call-translate-readme.yml`を導入する際の手順書をmarkdown形式で生成してください。具体的には：必須パラメータの設定方法、シークレットの登録手順、前提条件の確認項目を含めてください。
```

**避けるべき例**:
- callgraphについて調べてください
- ワークフローを分析してください
- issue-noteの処理フローを確認してください

## 出力フォーマット：
以下のMarkdown形式で出力してください：

```markdown
# Development Status

## 現在のIssues
[以下の形式で3行でオープン中のissuesを要約。issue番号を必ず書く]
- [1行目の説明]
- [2行目の説明]
- [3行目の説明]

## 次の一手候補
1. [候補1のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

2. [候補2のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```

3. [候補3のタイトル。issue番号を必ず書く]
   - 最初の小さな一歩: [具体的で実行可能な最初のアクション]
   - Agent実行プロンプト:
     ```
     対象ファイル: [分析/編集する具体的なファイルパス]

     実行内容: [具体的な分析や変更内容を記述]

     確認事項: [変更前に確認すべき依存関係や制約]

     期待する出力: [markdown形式での結果や、具体的なファイル変更の説明]
     ```
```


# 開発状況情報
- 以下の開発状況情報を参考にしてください。
- Issue番号を記載する際は、必ず [Issue #番号](../issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。

## プロジェクトのファイル一覧
- .github/actions-tmp/.github/workflows/call-callgraph.yml
- .github/actions-tmp/.github/workflows/call-check-large-files.yml
- .github/actions-tmp/.github/workflows/call-daily-project-summary.yml
- .github/actions-tmp/.github/workflows/call-issue-note.yml
- .github/actions-tmp/.github/workflows/call-rust-windows-check.yml
- .github/actions-tmp/.github/workflows/call-translate-readme.yml
- .github/actions-tmp/.github/workflows/callgraph.yml
- .github/actions-tmp/.github/workflows/check-large-files.yml
- .github/actions-tmp/.github/workflows/check-recent-human-commit.yml
- .github/actions-tmp/.github/workflows/daily-project-summary.yml
- .github/actions-tmp/.github/workflows/issue-note.yml
- .github/actions-tmp/.github/workflows/rust-windows-check.yml
- .github/actions-tmp/.github/workflows/translate-readme.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/callgraph.ql
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/codeql-pack.lock.yml
- .github/actions-tmp/.github_automation/callgraph/codeql-queries/qlpack.yml
- .github/actions-tmp/.github_automation/callgraph/config/example.json
- .github/actions-tmp/.github_automation/callgraph/docs/callgraph.md
- .github/actions-tmp/.github_automation/callgraph/presets/callgraph.js
- .github/actions-tmp/.github_automation/callgraph/presets/style.css
- .github/actions-tmp/.github_automation/callgraph/scripts/analyze-codeql.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/callgraph-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-codeql-exists.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/check-node-version.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/common-utils.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/copy-commit-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/extract-sarif-info.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/find-process-results.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generate-html-graph.cjs
- .github/actions-tmp/.github_automation/callgraph/scripts/generateHTML.cjs
- .github/actions-tmp/.github_automation/check-large-files/README.md
- .github/actions-tmp/.github_automation/check-large-files/check-large-files.toml.default
- .github/actions-tmp/.github_automation/check-large-files/scripts/check_large_files.py
- .github/actions-tmp/.github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
- .github/actions-tmp/.github_automation/project_summary/docs/daily-summary-setup.md
- .github/actions-tmp/.github_automation/project_summary/prompts/development-status-prompt.md
- .github/actions-tmp/.github_automation/project_summary/prompts/project-overview-prompt.md
- .github/actions-tmp/.github_automation/project_summary/scripts/ProjectSummaryCoordinator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/GitUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/development/IssueTracker.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/generate-project-summary.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/CodeAnalyzer.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectAnalysisOrchestrator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataCollector.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectDataFormatter.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/overview/ProjectOverviewGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/BaseGenerator.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/FileSystemUtils.cjs
- .github/actions-tmp/.github_automation/project_summary/scripts/shared/ProjectFileUtils.cjs
- .github/actions-tmp/.github_automation/translate/docs/TRANSLATION_SETUP.md
- .github/actions-tmp/.github_automation/translate/scripts/translate-readme.cjs
- .github/actions-tmp/.gitignore
- .github/actions-tmp/.vscode/settings.json
- .github/actions-tmp/LICENSE
- .github/actions-tmp/README.ja.md
- .github/actions-tmp/README.md
- .github/actions-tmp/_config.yml
- .github/actions-tmp/generated-docs/callgraph.html
- .github/actions-tmp/generated-docs/callgraph.js
- .github/actions-tmp/generated-docs/development-status-generated-prompt.md
- .github/actions-tmp/generated-docs/development-status.md
- .github/actions-tmp/generated-docs/project-overview-generated-prompt.md
- .github/actions-tmp/generated-docs/project-overview.md
- .github/actions-tmp/generated-docs/style.css
- .github/actions-tmp/googled947dc864c270e07.html
- .github/actions-tmp/issue-notes/10.md
- .github/actions-tmp/issue-notes/11.md
- .github/actions-tmp/issue-notes/12.md
- .github/actions-tmp/issue-notes/13.md
- .github/actions-tmp/issue-notes/14.md
- .github/actions-tmp/issue-notes/15.md
- .github/actions-tmp/issue-notes/16.md
- .github/actions-tmp/issue-notes/17.md
- .github/actions-tmp/issue-notes/18.md
- .github/actions-tmp/issue-notes/19.md
- .github/actions-tmp/issue-notes/2.md
- .github/actions-tmp/issue-notes/20.md
- .github/actions-tmp/issue-notes/21.md
- .github/actions-tmp/issue-notes/22.md
- .github/actions-tmp/issue-notes/23.md
- .github/actions-tmp/issue-notes/24.md
- .github/actions-tmp/issue-notes/25.md
- .github/actions-tmp/issue-notes/26.md
- .github/actions-tmp/issue-notes/27.md
- .github/actions-tmp/issue-notes/28.md
- .github/actions-tmp/issue-notes/29.md
- .github/actions-tmp/issue-notes/3.md
- .github/actions-tmp/issue-notes/30.md
- .github/actions-tmp/issue-notes/35.md
- .github/actions-tmp/issue-notes/38.md
- .github/actions-tmp/issue-notes/4.md
- .github/actions-tmp/issue-notes/40.md
- .github/actions-tmp/issue-notes/44.md
- .github/actions-tmp/issue-notes/52.md
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
- .github/scripts/create_ci_issue.py
- .github/workflows/call-check-large-files.yml
- .github/workflows/call-daily-project-summary.yml
- .github/workflows/call-issue-note.yml
- .github/workflows/call-translate-readme.yml
- .github/workflows/ci.yml
- .github/workflows/deploy-demo.yml
- .github/workflows/deploy-pages.yml
- .github/workflows/deploy-wasm.yml
- .gitignore
- Cargo.lock
- Cargo.toml
- LICENSE
- README.ja.md
- README.md
- WASM_USAGE.md
- _config.yml
- demo-library/.gitignore
- demo-library/biome.json
- demo-library/delay-vibrato-demo.ts
- demo-library/delay-vibrato.html
- demo-library/envelope-generator.ts
- demo-library/globals.d.ts
- demo-library/index.html
- demo-library/library-demo.ts
- demo-library/log-visualizer-lfo.ts
- demo-library/log-visualizer-note-segments.ts
- demo-library/log-visualizer.ts
- demo-library/mml-support.ts
- demo-library/package-lock.json
- demo-library/package.json
- demo-library/pop-noise-demo.ts
- demo-library/pop-noise.html
- demo-library/portamento-soft-lfo-demo.ts
- demo-library/portamento-soft-lfo.html
- demo-library/shared-demo.ts
- demo-library/style.css
- demo-library/tone-interpolation-demo.ts
- demo-library/tone-interpolation.html
- demo-library/tone-json-attachment.ts
- demo-library/tone-json-demo.ts
- demo-library/tone-json-mml.ts
- demo-library/tone-json.html
- demo-library/tsconfig.json
- demo-library/vite.config.ts
- demo-library/waveform-canvas.ts
- demo-library/waveform-simulator.ts
- demo-library/waveform-viewer.ts
- demo-library/ym2151-utils.ts
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
- issue-notes/105.md
- issue-notes/111.md
- issue-notes/112.md
- issue-notes/115.md
- issue-notes/123.md
- issue-notes/157.md
- issue-notes/166.md
- issue-notes/177.md
- issue-notes/178.md
- issue-notes/180.md
- issue-notes/181.md
- issue-notes/183.md
- issue-notes/184.md
- issue-notes/185.md
- issue-notes/186.md
- issue-notes/187.md
- issue-notes/188.md
- issue-notes/189.md
- issue-notes/22.md
- issue-notes/33.md
- issue-notes/45.md
- issue-notes/47.md
- issue-notes/66-resolution.md
- issue-notes/70.md
- issue-notes/83.md
- issue-notes/90.md
- issue-notes/91.md
- issue-notes/93.md
- package-lock.json
- package.json
- src/error.rs
- src/lib.rs
- src/main.rs
- src/midi/events.rs
- src/midi/mod.rs
- src/midi/parser.rs
- src/midi/utils.rs
- src/midi/utils_tests.rs
- src/wasm.rs
- src/ym2151/channel_allocation.rs
- src/ym2151/converter/pitch_effects.rs
- src/ym2151/converter/register_effects.rs
- src/ym2151/converter/waveform.rs
- src/ym2151/converter.rs
- src/ym2151/converter_tests/attachments.rs
- src/ym2151/converter_tests/basic.rs
- src/ym2151/converter_tests/channels.rs
- src/ym2151/converter_tests/drums.rs
- src/ym2151/converter_tests/effects.rs
- src/ym2151/converter_tests/lfo.rs
- src/ym2151/converter_tests/portamento.rs
- src/ym2151/converter_tests/programs.rs
- src/ym2151/converter_tests.rs
- src/ym2151/event_processor.rs
- src/ym2151/event_processor_tests.rs
- src/ym2151/events.rs
- src/ym2151/init.rs
- src/ym2151/mod.rs
- src/ym2151/note_table.rs
- src/ym2151/tempo_map.rs
- src/ym2151/tone.rs
- tests/create_test_midi.py
- tests/integration_conversion.rs
- tests/integration_midi.rs
- tests/integration_multichannel.rs
- tests/integration_program_change.rs
- tests/integration_wasm.rs
- tests/test_data/multi_channel.mid
- tests/test_data/multi_track.mid
- tests/test_data/program_change.mid
- tests/test_data/simple_melody.mid
- tests/test_data/tempo_change.mid
- tones/000.json
- tones/README.md

## 現在のオープンIssues
## [Issue #189](../issue-notes/189.md): demo ディレイビブラート pitch ビジュアライザーの表示が、実態と違って飛び飛びに見えてしまう
[issue-notes/189.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/189.md)

...
ラベル: 
--- issue-notes/189.md の内容 ---

```markdown
# issue demo ディレイビブラート pitch ビジュアライザーの表示が、実態と違って飛び飛びに見えてしまう #189
[issues #189](https://github.com/cat2151/smf-to-ym2151log-rust/issues/189)

- 今は、x 1pxごとに y 10pxくらいのものをy幅を固定で描画しているため、
    - x ごとに20px のpitch跳躍があると、例え実際にはもっと連続的なデータであっても、
    - pxとpxの間が空白で描画されてしまうため、
    - まるで離散的かのように描画されてしまう
    - userが混乱する
- 対策は、y幅を可変にすること。pitchがそのx pxにおいて上下で取った値のぶん、y幅を伸ばすこと
    - これによって、
    - 前回のyと今回のyとの間に、中間の連続的なpitchが存在するときに、
    - 前回と今回がつながって、まるで折れ線グラフのような見た目で描画ができる
    - また、ディレイビブラートがかかっていない普通のnoteの場合は、
        - 適切に、折れ線グラフでない、ピアノロール的に、描画ができる
- また、これにより、グラフの上下の中央を0、上をプラス、下をマイナスとしたとき、
    - これまではビブラートの上端と下端において、yがまちまちな値で描画されていたのが、
    - ちゃんとグラフの上端と下端を叩くよう描画できるようになる想定
- さらに現在問題として、ディレイビブラート開始時に、pxが途切れ途切れに描画されている
    - これは描画処理の誤りである、と判断する
    - 描画のx 1pxごとに描画をすべきである
        - イメージとしては、x 1px ごとにコールバックして、
            - そのx 1px の描画に必要な、y0, y1 を算出する
            - そのy0,y1は、1pxぶんの範囲内の、pitchのmaxとminである

```

## [Issue #188](../issue-notes/188.md): demo ソフトLFO に、key on sync : true/falseをつける
[issue-notes/188.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/188.md)

...
ラベル: 
--- issue-notes/188.md の内容 ---

```markdown
# issue demo ソフトLFO に、key on sync : true/falseをつける #188
[issues #188](https://github.com/cat2151/smf-to-ym2151log-rust/issues/188)

- 今はkey on sync trueの状態である
- userがほしいものは、falseにすると、フレーズをまたいでLFOが続くこと、である
- key on sync falseにすると、key onに連動せず、曲の先頭でtriggerして、ずっと発動が続く
    - なお、この場合、最初のkey onの直前の1つ、から前の書き込みはムダになるが、
        - ひとまずシンプル優先でそのままとする
        - 今後、post processでそこをまとめて最適化削除するようなものを別途検討すればよい想定

```

## [Issue #187](../issue-notes/187.md): demo ソフトLFO のレジスタビジュアライザーが、ch0のあと、ch1～ch7が空欄があるため、見づらい
[issue-notes/187.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/187.md)

...
ラベル: 
--- issue-notes/187.md の内容 ---

```markdown
# issue demo ソフトLFO のレジスタビジュアライザーが、ch0のあと、ch1～ch7が空欄があるため、見づらい #187
[issues #187](https://github.com/cat2151/smf-to-ym2151log-rust/issues/187)


- userがほしいもの：
    - ch0のすぐ下に、ch0のレジスタビジュアライザーを表示してほしい
    - ch1～ch7はkey onがない空欄であれば、コンパクトに表示してほしい
        - 例 「ch1～ch7 : 演奏データなし」の文言を1行表示するだけ

```

## [Issue #186](../issue-notes/186.md): 音色補間demoで、添付JSON入力で、Tone配下にregistersを書くとエラーになる
[issue-notes/186.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/186.md)

...
ラベル: 
--- issue-notes/186.md の内容 ---

```markdown
# issue 音色補間demoで、添付JSON入力で、Tone配下にregistersを書くとエラーになる #186
[issues #186](https://github.com/cat2151/smf-to-ym2151log-rust/issues/186)

- userがほしいもの
    - eventsとregistersに両対応する

```

## [Issue #185](../issue-notes/185.md): 音色補間demoで、添付JSON入力に KC KF が入っていると、それまで使われてしまう
[issue-notes/185.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/185.md)

...
ラベル: 
--- issue-notes/185.md の内容 ---

```markdown
# issue 音色補間demoで、添付JSON入力に KC KF が入っていると、それまで使われてしまう #185
[issues #185](https://github.com/cat2151/smf-to-ym2151log-rust/issues/185)


- userがほしいもの
    - KC KFは、音色補間するしないで影響がないようにしてほしい
    - より正確には、添付JSON入力でeventsを読み込んで、音色補間用registersを用意するときに、
        - KCとKFとkey onレジスターを対象外にしてほしい

```

## [Issue #184](../issue-notes/184.md): ポップノイズdemoで、波形ビュアがバグっている
[issue-notes/184.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/184.md)

...
ラベル: 
--- issue-notes/184.md の内容 ---

```markdown
# issue ポップノイズdemoで、波形ビュアがバグっている #184
[issues #184](https://github.com/cat2151/smf-to-ym2151log-rust/issues/184)


- 表示幅がバグっている
    - 1secの波形なのに、4000msと表示され、その幅いっぱいに描画されている。矛盾しており何が正解かわからない
- key on 緑の縦線 がバグっているか不明
    - 500msという数値は正確である
    - 表示幅が4000msが正確なら、それに対する緑の縦線の場所も正確ではあるかも
    - だが表示幅が4000msというのがバグっているので、
    - 緑の縦線がその影響で、実際の波形と違う位置に描画されている可能性が高そう
- 前のノート、次のノート、がバグっているか不明
    - 一応、緑の縦線のある場所にjumpしているので、OKな可能性はある
- オレンジ エンベロープ の意図が不明
    - もしYM2151のエンベロープのシミュレーションをしているならNG
        - バグの温床、複雑な計算式が必要で変更に弱い
    - 今回のポップノイズの可視化には効果がない
        - なぜならエンベロープの余地がない、key offリリースした直後にkey onをしている、
        - という状態でのポップノイズの可視化をしたいので
    - 対策、シンプル優先で、オレンジ エンベロープの機能を削除する

```

## [Issue #183](../issue-notes/183.md): ポップノイズdemoに、wavファイルexportをつける
[issue-notes/183.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/183.md)

...
ラベル: 
--- issue-notes/183.md の内容 ---

```markdown
# issue ポップノイズdemoに、wavファイルexportをつける #183
[issues #183](https://github.com/cat2151/smf-to-ym2151log-rust/issues/183)



```

## [Issue #182](../issue-notes/182.md): Fix PopNoiseEnvelope: move existing key-off to apply_time so envelope actually decays
`PopNoiseEnvelope` was a no-op when enabled for back-to-back notes: the previous note's key-off happened at the same time as the next key-on, so the envelope never had time to decay (even with faster-release register overrides applied).

## Changes

- **`register_effects.rs`** — `append_pop_noise_en...
ラベル: 
--- issue-notes/182.md の内容 ---

```markdown

```

## [Issue #181](../issue-notes/181.md): ポップノイズdemoで、PopNoiseEnvelope がenabledのときバグっている
[issue-notes/181.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/181.md)

...
ラベル: 
--- issue-notes/181.md の内容 ---

```markdown
# issue ポップノイズdemoで、PopNoiseEnvelope がenabledのときバグっている #181
[issues #181](https://github.com/cat2151/smf-to-ym2151log-rust/issues/181)


- 必須処理である、「key off eventを、post processor段で、timeを前にずらして、そのぶんevent位置も前にずらす」が漏れている
- 現状
    - 元データのkey off時刻の、offsetSeconds前の時刻
        - レジスタ変更
    - 元データのkey off時刻
        - レジスタ復帰
        - key off
    - ※これではまったく意味がない
- userがほしいもの
    - 次のkey on時刻の、offsetSeconds前の時刻
        - レジスタ変更
        - key off
    - 次のkey on時刻
        - レジスタ復帰
        - key on
    - ※また、元データのkey off時刻の直前に、userの指示していない小さいtimeのズレをハードコーディングで仕込むのも、NGである
        - ムダに処理と出力が複雑になり、バグ修正の邪魔となっていた
        - 「時間順を明示するために時刻をずらす」のは、この局面ではアンチパターンである
        - ムダなフォールバックと類似したアンチパターンであり、
            - 複雑なcodeが、バグのリスクを高めている

```

## [Issue #180](../issue-notes/180.md): ポップノイズdemoで、デフォルト値がおかしい
[issue-notes/180.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/180.md)

...
ラベル: 
--- issue-notes/180.md の内容 ---

```markdown
# issue ポップノイズdemoで、デフォルト値がおかしい #180
[issues #180](https://github.com/cat2151/smf-to-ym2151log-rust/issues/180)



```

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/2.md
```md
{% raw %}
# issue GitHub Actions「関数コールグラフhtmlビジュアライズ生成」を共通ワークフロー化する #2
[issues #2](https://github.com/cat2151/github-actions/issues/2)


# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-callgraph_enhanced.yml
まずplanしてください
```

# 結果
- indent
    - linter？がindentのエラーを出しているがyml内容は見た感じOK
    - テキストエディタとagentの相性問題と判断する
    - 別のテキストエディタでsaveしなおし、テキストエディタをreload
    - indentのエラーは解消した
- LLMレビュー
    - agent以外の複数のLLMにレビューさせる
    - prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューしてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 共通ワークフロー

# GitHub Actions Reusable Workflow for Call Graph Generation
name: Generate Call Graph

# TODO Windowsネイティブでのtestをしていた名残が残っているので、今後整理していく。今はWSL act でtestしており、Windowsネイティブ環境依存問題が解決した
#  ChatGPTにレビューさせるとそこそこ有用そうな提案が得られたので、今後それをやる予定
#  agentに自己チェックさせる手も、セカンドオピニオンとして選択肢に入れておく

on:
  workflow_call:

jobs:
  check-commits:
    runs-on: ubuntu-latest
    outputs:
      should-run: ${{ steps.check.outputs.should-run }}
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          fetch-depth: 50 # 過去のコミットを取得

      - name: Check for user commits in last 24 hours
        id: check
        run: |
          node .github/scripts/callgraph_enhanced/check-commits.cjs

  generate-callgraph:
    needs: check-commits
    if: needs.check-commits.outputs.should-run == 'true'
    runs-on: ubuntu-latest
    permissions:
      contents: write
      security-events: write
      actions: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Set Git identity
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

      - name: Remove old CodeQL packages cache
        run: rm -rf ~/.codeql/packages

      - name: Check Node.js version
        run: |
          node .github/scripts/callgraph_enhanced/check-node-version.cjs

      - name: Install CodeQL CLI
        run: |
          wget https://github.com/github/codeql-cli-binaries/releases/download/v2.22.1/codeql-linux64.zip
          unzip codeql-linux64.zip
          sudo mv codeql /opt/codeql
          echo "/opt/codeql" >> $GITHUB_PATH

      - name: Install CodeQL query packs
        run: |
          /opt/codeql/codeql pack install .github/codeql-queries

      - name: Check CodeQL exists
        run: |
          node .github/scripts/callgraph_enhanced/check-codeql-exists.cjs

      - name: Verify CodeQL Configuration
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs verify-config

      - name: Remove existing CodeQL DB (if any)
        run: |
          rm -rf codeql-db

      - name: Perform CodeQL Analysis
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs analyze

      - name: Check CodeQL Analysis Results
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs check-results

      - name: Debug CodeQL execution
        run: |
          node .github/scripts/callgraph_enhanced/analyze-codeql.cjs debug

      - name: Wait for CodeQL results
        run: |
          node -e "setTimeout(()=>{}, 10000)"

      - name: Find and process CodeQL results
        run: |
          node .github/scripts/callgraph_enhanced/find-process-results.cjs

      - name: Generate HTML graph
        run: |
          node .github/scripts/callgraph_enhanced/generate-html-graph.cjs

      - name: Copy files to generated-docs and commit results
        run: |
          node .github/scripts/callgraph_enhanced/copy-commit-results.cjs

--- 呼び出し元
# 呼び出し元ワークフロー: call-callgraph_enhanced.yml
name: Call Call Graph Enhanced

on:
  schedule:
    # 毎日午前5時(JST) = UTC 20:00前日
    - cron: '0 20 * * *'
  workflow_dispatch:

jobs:
  call-callgraph-enhanced:
    # uses: cat2151/github-actions/.github/workflows/callgraph_enhanced.yml
    uses: ./.github/workflows/callgraph_enhanced.yml # ローカルでのテスト用
```

# レビュー結果OKと判断する
- レビュー結果を人力でレビューした形になった

# test
- #4 同様にローカル WSL + act でtestする
- エラー。userのtest設計ミス。
  - scriptの挙動 : src/ がある前提
  - 今回の共通ワークフローのリポジトリ : src/ がない
  - 今回testで実現したいこと
    - 仮のソースでよいので、関数コールグラフを生成させる
  - 対策
    - src/ にダミーを配置する
- test green
  - ただしcommit pushはしてないので、html内容が0件NG、といったケースの検知はできない
  - もしそうなったら別issueとしよう

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする
- もしhtml内容が0件NG、などになったら、別issueとするつもり

{% endraw %}
```

### .github/actions-tmp/issue-notes/3.md
```md
{% raw %}
# issue GitHub Actions「issue note生成」を共通ワークフロー化する #3
[issues #3](https://github.com/cat2151/github-actions/issues/3)

- 前提
  - userは、translateのworkflowを共通ワークフロー化し、動くようにしたので、共通ワークフロー化の知識がある
  - よって、今回は、promptでplanさせ、そのplanをuserがレビューし、planの改善を指示できるので、ハルシネーションのリスクを下げることができる

- prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlはまだ共通ワークフロー化されず、
呼び出し元からコピーされたままの状態です。
このymlファイル名のまま、
中身を共通ワークフロー化してください。
呼び出し元ymlは別途書きますので気にしないでください。
まずplanしてください
```

- plan 生成結果
```
共通ワークフロー化のためのplanは以下の通りです。

1. 目的
現在の issue-note.yml を「共通ワークフロー」として再設計し、他リポジトリや他ワークフローから workflow_call で呼び出せるようにする。
2. 主な変更点
on: トリガーを workflow_call に変更（直接イベントで起動しない）。
必要な入力（inputs）やシークレット（secrets）を workflow_call セクションで定義。
github.event.issue などのイベント依存部分を、inputs経由で受け取る形に修正。
既存の処理（issueノート生成、コミット、issue body更新）はそのまま活かすが、パラメータ化する。
3. 具体的な修正内容
on: issues: → on: workflow_call: に変更。
inputs で必要な値（例: issue_title, issue_number, issue_body, issue_url など）を受け取る。
スクリプト内の ${{ github.event.issue.xxx }} を ${{ inputs.xxx }} に置換。
github-script ステップも同様にinputsを参照。
必要に応じて secrets も workflow_call で受け取れるようにする。
4. ファイル名・配置
ファイル名はそのまま issue-note.yml。
配置場所は workflows でOK（共通ワークフローとしてもこの場所で良い）。
5. 呼び出し元への影響
呼び出し元ワークフローで uses: <repo>/.github/workflows/issue-note.yml@main のように参照し、inputsを渡す形になる。
このplanで進めます。次に、実際の共通ワークフロー化のためのyml修正案を作成しますか？
```

- userがplanレビューし、OKを出した
- agentに生成結果ymlをレビューさせた、OKとなった
  - userはこのレビューは認知負荷が高いのでskipとする（単純な細かい間違い探し作業になるので、それはagentに任せた）
-  エラー。呼び出し元ymlのエラーがあり修正。共通ワークフローのエラー指摘があり、共通ワークフロー側の修正が必要となった
```
原因の分析：

呼び出し元（tonejs-mml-to-jsonのissue-note.yml）は、reusable workflow（cat2151/github-actions/.github/workflows/issue-note.yml@main）をusesで呼び出し、inputsをwithで渡しています。
共通ワークフロー（reusable workflow）は inputs を workflow_call で受け取り、各ステップで ${{ inputs.xxx }} で参照しています。
しかし、エラーが発生したのは actions/github-script@v7 の script 内です。
エラーの本質的な原因：

actions/github-script の script: で ${{ inputs.issue_number }} のような式を直接 JavaScript テンプレートリテラル内で使っていますが、これはサポートされていません。
script: の中は JavaScript であり、YAMLの ${{ ... }} は展開されません（または意図通りに展開されません）。
そのため、inputs 変数は script 内の JavaScript で未定義となり、ReferenceError になります。
正しい書き方：

actions/github-script の script: でワークフローの inputs を使いたい場合は、with: で input を environment 変数や script の引数として渡し、JavaScript で process.env から取得するか、github-script の input 機能を使う必要があります。
例（修正版）：

このように、inputs の値を env で渡し、script 内では process.env で参照してください。

まとめ：

actions/github-script の script 内で ${{ inputs.xxx }} を直接使うと ReferenceError になる
env: で値を渡し、process.env で参照するのが正しい
修正が必要です。修正版のワークフローが必要な場合はお知らせください。
```

- test green
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/4.md
```md
{% raw %}
# issue GitHub Actions「project概要生成」を共通ワークフロー化する #4
[issues #4](https://github.com/cat2151/github-actions/issues/4)

# prompt
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
このymlファイルを、以下の2つのファイルに分割してください。
1. 共通ワークフロー       cat2151/github-actions/.github/workflows/daily-project-summary.yml
2. 呼び出し元ワークフロー cat2151/github-actions/.github/workflows/call-daily-project-summary.yml
まずplanしてください
```

# 結果、あちこちハルシネーションのあるymlが生成された
- agentの挙動があからさまにハルシネーション
    - インデントが修正できない、「失敗した」という
    - 構文誤りを認識できない
- 人力で修正した

# このagentによるセルフレビューが信頼できないため、別のLLMによるセカンドオピニオンを試す
```
あなたはGitHub Actionsと共通ワークフローのスペシャリストです。
以下の2つのファイルをレビューしてください。最優先で、エラーが発生するかどうかだけレビューてください。エラー以外の改善事項のチェックをするかわりに、エラー発生有無チェックに最大限注力してください。

--- 呼び出し元

name: Call Daily Project Summary

on:
  schedule:
    # 日本時間 07:00 (UTC 22:00 前日)
    - cron: '0 22 * * *'
  workflow_dispatch:

jobs:
  call-daily-project-summary:
    uses: cat2151/github-actions/.github/workflows/daily-project-summary.yml
    secrets:
      GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}

--- 共通ワークフロー
name: Daily Project Summary
on:
  workflow_call:

jobs:
  generate-summary:
    runs-on: ubuntu-latest

    permissions:
      contents: write
      issues: read
      pull-requests: read

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0  # 履歴を取得するため

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: |
          # 一時的なディレクトリで依存関係をインストール
          mkdir -p /tmp/summary-deps
          cd /tmp/summary-deps
          npm init -y
          npm install @google/generative-ai @octokit/rest
          # generated-docsディレクトリを作成
          mkdir -p $GITHUB_WORKSPACE/generated-docs

      - name: Generate project summary
        env:
          GEMINI_API_KEY: ${{ secrets.GEMINI_API_KEY }}
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          GITHUB_REPOSITORY: ${{ github.repository }}
          NODE_PATH: /tmp/summary-deps/node_modules
        run: |
          node .github/scripts/generate-project-summary.cjs

      - name: Check for generated summaries
        id: check_summaries
        run: |
          if [ -f "generated-docs/project-overview.md" ] && [ -f "generated-docs/development-status.md" ]; then
            echo "summaries_generated=true" >> $GITHUB_OUTPUT
          else
            echo "summaries_generated=false" >> $GITHUB_OUTPUT
          fi

      - name: Commit and push summaries
        if: steps.check_summaries.outputs.summaries_generated == 'true'
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          # package.jsonの変更のみリセット（generated-docsは保持）
          git restore package.json 2>/dev/null || true
          # サマリーファイルのみを追加
          git add generated-docs/project-overview.md
          git add generated-docs/development-status.md
          git commit -m "Update project summaries (overview & development status)"
          git push

      - name: Summary generation result
        run: |
          if [ "${{ steps.check_summaries.outputs.summaries_generated }}" == "true" ]; then
            echo "✅ Project summaries updated successfully"
            echo "📊 Generated: project-overview.md & development-status.md"
          else
            echo "ℹ️ No summaries generated (likely no user commits in the last 24 hours)"
          fi
```

# 上記promptで、2つのLLMにレビューさせ、合格した

# 細部を、先行する2つのymlを参照に手直しした

# ローカルtestをしてからcommitできるとよい。方法を検討する
- ローカルtestのメリット
    - 素早く修正のサイクルをまわせる
    - ムダにgit historyを汚さない
        - これまでの事例：「実装したつもり」「エラー。修正したつもり」「エラー。修正したつもり」...（以降エラー多数）
- 方法
    - ※検討、WSL + act を環境構築済みである。test可能であると判断する
    - 呼び出し元のURLをコメントアウトし、相対パス記述にする
    - ※備考、テスト成功すると結果がcommit pushされる。それでよしとする
- 結果
    - OK
    - secretsを簡略化できるか試した、できなかった、現状のsecrets記述が今わかっている範囲でベストと判断する
    - OK

# test green

# commit用に、yml 呼び出し元 uses をlocal用から本番用に書き換える

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/7.md
```md
{% raw %}
# issue issue note生成できるかのtest用 #7
[issues #7](https://github.com/cat2151/github-actions/issues/7)

- 生成できた
- closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/8.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズ生成の対象ソースファイルを、呼び出し元ymlで指定できるようにする #8
[issues #8](https://github.com/cat2151/github-actions/issues/8)

# これまでの課題
- 以下が決め打ちになっていた
```
  const allowedFiles = [
    'src/main.js',
    'src/mml2json.js',
    'src/play.js'
  ];
```

# 対策
- 呼び出し元ymlで指定できるようにする

# agent
- agentにやらせることができれば楽なので、初手agentを試した
- 失敗
    - ハルシネーションしてscriptを大量破壊した
- 分析
    - 修正対象scriptはagentが生成したもの
    - 低品質な生成結果でありソースが巨大
    - ハルシネーションで破壊されやすいソース
    - AIの生成したソースは、必ずしもAIフレンドリーではない

# 人力リファクタリング
- 低品質コードを、最低限agentが扱えて、ハルシネーションによる大量破壊を防止できる内容、にする
- 手短にやる
    - そもそもビジュアライズは、agentに雑に指示してやらせたもので、
    - 今後別のビジュアライザを選ぶ可能性も高い
    - 今ここで手間をかけすぎてコンコルド効果（サンクコストバイアス）を増やすのは、project群をトータルで俯瞰して見たとき、損
- 対象
    - allowedFiles のあるソース
        - callgraph-utils.cjs
            - たかだか300行未満のソースである
            - この程度でハルシネーションされるのは予想外
            - やむなし、リファクタリングでソース分割を進める

# agentに修正させる
## prompt
```
allowedFilesを引数で受け取るようにしたいです。
ないならエラー。
最終的に呼び出し元すべてに波及して修正したいです。

呼び出し元をたどってエントリポイントも見つけて、
エントリポイントにおいては、
引数で受け取ったjsonファイル名 allowedFiles.js から
jsonファイル allowedFiles.jsonの内容をreadして
変数 allowedFilesに格納、
後続処理に引き渡す、としたいです。

まずplanしてください。
planにおいては、修正対象のソースファイル名と関数名を、呼び出し元を遡ってすべて特定し、listしてください。
```

# 修正が順調にできた
- コマンドライン引数から受け取る作りになっていなかったので、そこだけ指示して修正させた
- yml側は人力で修正した

# 他のリポジトリから呼び出した場合にバグらないよう修正する
- 気付いた
    - 共通ワークフローとして他のリポジトリから使った場合はバグるはず。
        - ymlから、共通ワークフロー側リポジトリのcheckoutが漏れているので。
- 他のyml同様に修正する
- あわせて全体にymlをリファクタリングし、修正しやすくし、今後のyml読み書きの学びにしやすくする

# local WSL + act : test green

# closeとする
- もし生成されたhtmlがNGの場合は、別issueとするつもり

{% endraw %}
```

### .github/actions-tmp/issue-notes/9.md
```md
{% raw %}
# issue 関数コールグラフhtmlビジュアライズが0件なので、原因を可視化する #9
[issues #9](https://github.com/cat2151/github-actions/issues/9)

# agentに修正させたり、人力で修正したりした
- agentがハルシネーションし、いろいろ根の深いバグにつながる、エラー隠蔽などを仕込んでいたため、検知が遅れた
- 詳しくはcommit logを参照のこと
- WSL + actの環境を少し変更、act起動時のコマンドライン引数を変更し、generated-docsをmountする（ほかはデフォルト挙動であるcpだけにする）ことで、デバッグ情報をコンテナ外に出力できるようにし、デバッグを効率化した

# test green

# closeとする

{% endraw %}
```

### issue-notes/180.md
```md
{% raw %}
# issue ポップノイズdemoで、デフォルト値がおかしい #180
[issues #180](https://github.com/cat2151/smf-to-ym2151log-rust/issues/180)



{% endraw %}
```

### issue-notes/181.md
```md
{% raw %}
# issue ポップノイズdemoで、PopNoiseEnvelope がenabledのときバグっている #181
[issues #181](https://github.com/cat2151/smf-to-ym2151log-rust/issues/181)


- 必須処理である、「key off eventを、post processor段で、timeを前にずらして、そのぶんevent位置も前にずらす」が漏れている
- 現状
    - 元データのkey off時刻の、offsetSeconds前の時刻
        - レジスタ変更
    - 元データのkey off時刻
        - レジスタ復帰
        - key off
    - ※これではまったく意味がない
- userがほしいもの
    - 次のkey on時刻の、offsetSeconds前の時刻
        - レジスタ変更
        - key off
    - 次のkey on時刻
        - レジスタ復帰
        - key on
    - ※また、元データのkey off時刻の直前に、userの指示していない小さいtimeのズレをハードコーディングで仕込むのも、NGである
        - ムダに処理と出力が複雑になり、バグ修正の邪魔となっていた
        - 「時間順を明示するために時刻をずらす」のは、この局面ではアンチパターンである
        - ムダなフォールバックと類似したアンチパターンであり、
            - 複雑なcodeが、バグのリスクを高めている

{% endraw %}
```

### issue-notes/183.md
```md
{% raw %}
# issue ポップノイズdemoに、wavファイルexportをつける #183
[issues #183](https://github.com/cat2151/smf-to-ym2151log-rust/issues/183)



{% endraw %}
```

### issue-notes/184.md
```md
{% raw %}
# issue ポップノイズdemoで、波形ビュアがバグっている #184
[issues #184](https://github.com/cat2151/smf-to-ym2151log-rust/issues/184)


- 表示幅がバグっている
    - 1secの波形なのに、4000msと表示され、その幅いっぱいに描画されている。矛盾しており何が正解かわからない
- key on 緑の縦線 がバグっているか不明
    - 500msという数値は正確である
    - 表示幅が4000msが正確なら、それに対する緑の縦線の場所も正確ではあるかも
    - だが表示幅が4000msというのがバグっているので、
    - 緑の縦線がその影響で、実際の波形と違う位置に描画されている可能性が高そう
- 前のノート、次のノート、がバグっているか不明
    - 一応、緑の縦線のある場所にjumpしているので、OKな可能性はある
- オレンジ エンベロープ の意図が不明
    - もしYM2151のエンベロープのシミュレーションをしているならNG
        - バグの温床、複雑な計算式が必要で変更に弱い
    - 今回のポップノイズの可視化には効果がない
        - なぜならエンベロープの余地がない、key offリリースした直後にkey onをしている、
        - という状態でのポップノイズの可視化をしたいので
    - 対策、シンプル優先で、オレンジ エンベロープの機能を削除する

{% endraw %}
```

### issue-notes/185.md
```md
{% raw %}
# issue 音色補間demoで、添付JSON入力に KC KF が入っていると、それまで使われてしまう #185
[issues #185](https://github.com/cat2151/smf-to-ym2151log-rust/issues/185)


- userがほしいもの
    - KC KFは、音色補間するしないで影響がないようにしてほしい
    - より正確には、添付JSON入力でeventsを読み込んで、音色補間用registersを用意するときに、
        - KCとKFとkey onレジスターを対象外にしてほしい

{% endraw %}
```

### issue-notes/186.md
```md
{% raw %}
# issue 音色補間demoで、添付JSON入力で、Tone配下にregistersを書くとエラーになる #186
[issues #186](https://github.com/cat2151/smf-to-ym2151log-rust/issues/186)

- userがほしいもの
    - eventsとregistersに両対応する

{% endraw %}
```

### issue-notes/187.md
```md
{% raw %}
# issue demo ソフトLFO のレジスタビジュアライザーが、ch0のあと、ch1～ch7が空欄があるため、見づらい #187
[issues #187](https://github.com/cat2151/smf-to-ym2151log-rust/issues/187)


- userがほしいもの：
    - ch0のすぐ下に、ch0のレジスタビジュアライザーを表示してほしい
    - ch1～ch7はkey onがない空欄であれば、コンパクトに表示してほしい
        - 例 「ch1～ch7 : 演奏データなし」の文言を1行表示するだけ

{% endraw %}
```

### issue-notes/188.md
```md
{% raw %}
# issue demo ソフトLFO に、key on sync : true/falseをつける #188
[issues #188](https://github.com/cat2151/smf-to-ym2151log-rust/issues/188)

- 今はkey on sync trueの状態である
- userがほしいものは、falseにすると、フレーズをまたいでLFOが続くこと、である
- key on sync falseにすると、key onに連動せず、曲の先頭でtriggerして、ずっと発動が続く
    - なお、この場合、最初のkey onの直前の1つ、から前の書き込みはムダになるが、
        - ひとまずシンプル優先でそのままとする
        - 今後、post processでそこをまとめて最適化削除するようなものを別途検討すればよい想定

{% endraw %}
```

### issue-notes/189.md
```md
{% raw %}
# issue demo ディレイビブラート pitch ビジュアライザーの表示が、実態と違って飛び飛びに見えてしまう #189
[issues #189](https://github.com/cat2151/smf-to-ym2151log-rust/issues/189)

- 今は、x 1pxごとに y 10pxくらいのものをy幅を固定で描画しているため、
    - x ごとに20px のpitch跳躍があると、例え実際にはもっと連続的なデータであっても、
    - pxとpxの間が空白で描画されてしまうため、
    - まるで離散的かのように描画されてしまう
    - userが混乱する
- 対策は、y幅を可変にすること。pitchがそのx pxにおいて上下で取った値のぶん、y幅を伸ばすこと
    - これによって、
    - 前回のyと今回のyとの間に、中間の連続的なpitchが存在するときに、
    - 前回と今回がつながって、まるで折れ線グラフのような見た目で描画ができる
    - また、ディレイビブラートがかかっていない普通のnoteの場合は、
        - 適切に、折れ線グラフでない、ピアノロール的に、描画ができる
- また、これにより、グラフの上下の中央を0、上をプラス、下をマイナスとしたとき、
    - これまではビブラートの上端と下端において、yがまちまちな値で描画されていたのが、
    - ちゃんとグラフの上端と下端を叩くよう描画できるようになる想定
- さらに現在問題として、ディレイビブラート開始時に、pxが途切れ途切れに描画されている
    - これは描画処理の誤りである、と判断する
    - 描画のx 1pxごとに描画をすべきである
        - イメージとしては、x 1px ごとにコールバックして、
            - そのx 1px の描画に必要な、y0, y1 を算出する
            - そのy0,y1は、1pxぶんの範囲内の、pitchのmaxとminである

{% endraw %}
```

### issue-notes/83.md
```md
{% raw %}
# issue 音色データの扱いについて整理する #83
[issues #83](https://github.com/cat2151/smf-to-ym2151log-rust/issues/83)

# これまでの課題
- デフォルト音色データ 0～127 がまだ揃っていない
- ガワだけでいいので用意したほうがいいが、まだ揃っていない
- このため、和音SMFも、sine wave音色で鳴らすことしかできていないはず
- これをやりたい：「ブラウザ、ランダム音色で、音符はMMLでその場で入力して鳴らす、和音も可」
- 制約：ブラウザ音色エディタは別リポジトリ（web-ym2151）である
- 対策案： 音色データreadを使ったdemo、上記のランダム音色かつ、音符MML演奏demo、は、web-ym2151側で実施とする

{% endraw %}
```

### src/ym2151/converter/register_effects.rs
```rs
{% raw %}
//! Register-side effects
//!
//! Provides software LFO, pop-noise envelope, and attack continuation fix implementations.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::ym2151::{NoteSegment, ToneDefinition, Ym2151Event};
use crate::{AttackContinuationFix, PopNoiseEnvelope, ProgramAttachment, RegisterLfoDefinition};

use super::waveform::lfo_waveform_value;

pub(super) const RESTORE_BEFORE_NOTE_EPSILON: f64 = 1e-6;
/// Small tolerance for time-loop termination conditions to absorb accumulated f64 rounding errors.
const TIME_LOOP_EPSILON: f64 = 1e-9;

pub(super) fn append_register_lfo_events(
    lfo_defs: &[RegisterLfoDefinition],
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut Vec<Ym2151Event>,
) {
    if lfo_defs.is_empty() || segments.is_empty() {
        return;
    }

    let mut ordered_segments = segments.to_vec();
    ordered_segments.sort_by(|a, b| {
        a.start_time
            .partial_cmp(&b.start_time)
            .unwrap_or(Ordering::Equal)
    });

    for segment in &ordered_segments {
        for def in lfo_defs {
            let Some(base_reg) = parse_hex_byte(&def.base_register) else {
                continue;
            };
            let resolved_addr = resolve_register_for_channel(base_reg, segment.ym2151_channel);
            let Some(base_value) = cache.latest_value(resolved_addr, segment.start_time) else {
                continue;
            };

            append_register_lfo_for_segment(def, segment, resolved_addr, base_value, events);
        }
    }
}

fn append_register_lfo_for_segment(
    def: &RegisterLfoDefinition,
    segment: &NoteSegment,
    resolved_addr: u8,
    base_value: u8,
    events: &mut Vec<Ym2151Event>,
) {
    if def.rate_hz <= 0.0 || def.depth.abs() < f64::EPSILON {
        return;
    }

    let start_time = segment.start_time + def.delay_seconds;
    let stop_time = segment.end_time;
    if stop_time <= start_time {
        return;
    }

    // Use enough samples per period so consecutive values differ by at most 1 integer step.
    // A triangle wave with amplitude `depth` has a max slope of 4*depth per period,
    // so we need at least 4*depth samples to avoid stepping by more than 1.
    let samples_per_period = (4.0 * def.depth.abs()).max(8.0).ceil();
    let time_step = (1.0 / def.rate_hz.max(f64::EPSILON)) / samples_per_period;
    if !time_step.is_finite() || time_step <= 0.0 {
        return;
    }

    let addr_str = format!("0x{:02X}", resolved_addr);
    let mut time = start_time;
    let mut last_value: Option<u8> = None;

    while time <= stop_time + f64::EPSILON {
        let elapsed = time - start_time;
        let attack_ratio = if def.attack_seconds <= 0.0 {
            1.0
        } else {
            (elapsed / def.attack_seconds).clamp(0.0, 1.0)
        };

        let phase = (elapsed * def.rate_hz) % 1.0;
        let waveform = lfo_waveform_value(def.waveform, phase);
        let offset = def.depth * attack_ratio * waveform;
        let value = ((base_value as f64) + offset).round().clamp(0.0, 255.0) as u8;

        if Some(value) != last_value {
            events.push(Ym2151Event {
                time,
                addr: addr_str.clone(),
                data: format!("0x{:02X}", value),
            });
            last_value = Some(value);
        }

        time += time_step;
    }
}

pub(super) fn append_pop_noise_envelope_events(
    config: &PopNoiseEnvelope,
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut Vec<Ym2151Event>,
) {
    if !config.enabled || config.registers.is_empty() || segments.is_empty() {
        return;
    }

    let mut ordered_segments = segments.to_vec();
    ordered_segments.sort_by(|a, b| {
        a.start_time
            .partial_cmp(&b.start_time)
            .unwrap_or(Ordering::Equal)
    });

    let offset = config.offset_seconds.max(0.0);

    for segment in ordered_segments {
        if segment.start_time <= offset || offset <= RESTORE_BEFORE_NOTE_EPSILON {
            continue;
        }
        let apply_time = segment.start_time - offset;
        let restore_time = (segment.start_time - RESTORE_BEFORE_NOTE_EPSILON).max(0.0);

        for reg in &config.registers {
            let Some(base_reg) = parse_hex_byte(&reg.base_register) else {
                continue;
            };
            let Some(override_value) = parse_hex_byte(&reg.value) else {
                continue;
            };
            let resolved_addr = resolve_register_for_channel(base_reg, segment.ym2151_channel);
            let Some(base_value) = cache.latest_value(resolved_addr, restore_time) else {
                continue;
            };
            if base_value == override_value {
                continue;
            }

            let addr_str = format!("0x{:02X}", resolved_addr);
            events.push(Ym2151Event {
                time: apply_time,
                addr: addr_str.clone(),
                data: format!("0x{:02X}", override_value),
            });
            events.push(Ym2151Event {
                time: restore_time,
                addr: addr_str,
                data: format!("0x{:02X}", base_value),
            });
        }
    }
}

pub(super) fn append_attack_continuation_fix_events(
    config: &AttackContinuationFix,
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut Vec<Ym2151Event>,
) {
    if !config.enabled || segments.is_empty() {
        return;
    }

    let override_release = config.release_rate;
    let offset = config.offset_seconds.max(0.0);

    let mut ordered_segments = segments.to_vec();
    ordered_segments.sort_by(|a, b| {
        a.start_time
            .partial_cmp(&b.start_time)
            .unwrap_or(Ordering::Equal)
    });

    for segment in ordered_segments {
        if segment.start_time <= offset || offset <= RESTORE_BEFORE_NOTE_EPSILON {
            continue;
        }
        let pre_time = segment.start_time - offset;
        let restore_time = (segment.start_time - RESTORE_BEFORE_NOTE_EPSILON).max(0.0);

        let mut release_registers = Vec::new();
        for op in 0..4 {
            let base_reg = 0xE0u8 + (op * 8);
            let resolved = resolve_register_for_channel(base_reg, segment.ym2151_channel);
            if let Some(base_value) = cache.latest_value(resolved, pre_time) {
                if base_value != override_release {
                    release_registers.push((resolved, base_value));
                }
            }
        }

        if release_registers.is_empty() {
            continue;
        }

        for (addr, _) in &release_registers {
            events.push(Ym2151Event {
                time: pre_time,
                addr: format!("0x{:02X}", *addr),
                data: format!("0x{:02X}", override_release),
            });
        }

        events.push(Ym2151Event {
            time: pre_time,
            addr: "0x08".to_string(),
            data: format!("0x{:02X}", segment.ym2151_channel),
        });

        for (addr, base_value) in &release_registers {
            events.push(Ym2151Event {
                time: restore_time,
                addr: format!("0x{:02X}", *addr),
                data: format!("0x{:02X}", *base_value),
            });
        }
    }
}

pub(super) struct RegisterStateCache {
    by_addr: HashMap<u8, Vec<(f64, u8)>>,
}

pub(super) fn build_register_state_cache(events: &[Ym2151Event]) -> RegisterStateCache {
    let mut by_addr: HashMap<u8, Vec<(f64, u8)>> = HashMap::new();

    for e in events {
        let Some(addr) = parse_hex_byte(&e.addr) else {
            continue;
        };
        let Some(value) = parse_hex_byte(&e.data) else {
            continue;
        };
        by_addr.entry(addr).or_default().push((e.time, value));
    }

    for values in by_addr.values_mut() {
        values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    }

    RegisterStateCache { by_addr }
}

impl RegisterStateCache {
    fn latest_value(&self, addr: u8, time: f64) -> Option<u8> {
        let entries = self.by_addr.get(&addr)?;
        let mut lo = 0;
        let mut hi = entries.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if entries[mid].0 <= time + f64::EPSILON {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == 0 {
            None
        } else {
            Some(entries[lo - 1].1)
        }
    }
}

/// Append looping linear interpolation events between adjacent program tones.
///
/// For each `ProgramAttachment` with `change_to_next_tone: true`, this generates register
/// write events that continuously morph from the current program's tone to the next
/// program's tone (program_change + 1) over `change_to_next_tone_time` seconds, then
/// back again, repeating for the duration of the song.
pub(super) fn append_change_to_next_tone_events(
    program_attachments: &[ProgramAttachment],
    tones: &HashMap<u8, ToneDefinition>,
    used_channels: &[u8],
    song_end_time: f64,
    events: &mut Vec<Ym2151Event>,
) {
    if song_end_time <= 0.0 || used_channels.is_empty() {
        return;
    }

    for attachment in program_attachments {
        if !attachment.change_to_next_tone {
            continue;
        }
        let period = attachment.change_to_next_tone_time;
        if period <= 0.0 {
            continue;
        }

        let Some(next_program) = attachment.program_change.checked_add(1) else {
            continue;
        };
        let Some(tone_from) = tones.get(&attachment.program_change) else {
            continue;
        };
        let Some(tone_to) = tones.get(&next_program) else {
            continue;
        };

        // Build a lookup from tone_to: base_addr -> value
        let mut to_values: HashMap<u8, u8> = HashMap::new();
        for ev in &tone_to.events {
            let Some(addr) = parse_hex_byte(&ev.addr) else {
                continue;
            };
            let Some(value) = parse_hex_byte(&ev.data) else {
                continue;
            };
            to_values.insert(addr, value);
        }

        // Collect registers that differ between the two tones
        let mut register_changes: Vec<(u8, u8, u8)> = Vec::new(); // (base_addr, from, to)
        for ev in &tone_from.events {
            let Some(base_addr) = parse_hex_byte(&ev.addr) else {
                continue;
            };
            let Some(value_from) = parse_hex_byte(&ev.data) else {
                continue;
            };
            let Some(&value_to) = to_values.get(&base_addr) else {
                continue;
            };
            if value_from != value_to {
                register_changes.push((base_addr, value_from, value_to));
            }
        }

        if register_changes.is_empty() {
            continue;
        }

        let cycle = 2.0 * period;

        for &ch in used_channels {
            for &(base_addr, value_from, value_to) in &register_changes {
                let resolved_addr = resolve_register_for_channel(base_addr, ch);
                let addr_str = format!("0x{:02X}", resolved_addr);
                let delta = (value_to as i32) - (value_from as i32);
                let steps = delta.unsigned_abs() as usize;

                // One event per integer step; keep time_step fine enough for smooth changes
                let time_step = period / steps.max(1) as f64;

                let mut last_value: Option<u8> = None;
                let mut time = 0.0_f64;

                while time <= song_end_time + TIME_LOOP_EPSILON {
                    let cycle_pos = time % cycle;
                    // Forward ramp [0, period]: t goes 0→1; backward [period, 2*period]: t goes 1→0
                    let t = if cycle_pos <= period {
                        cycle_pos / period
                    } else {
                        2.0 - cycle_pos / period
                    };

                    let value = ((value_from as f64) + (delta as f64) * t)
                        .round()
                        .clamp(0.0, 255.0) as u8;

                    if Some(value) != last_value {
                        events.push(Ym2151Event {
                            time,
                            addr: addr_str.clone(),
                            data: format!("0x{:02X}", value),
                        });
                        last_value = Some(value);
                    }

                    time += time_step;
                }
            }
        }
    }
}

fn resolve_register_for_channel(base_register: u8, channel: u8) -> u8 {
    match base_register {
        0x20..=0x27 => 0x20 + channel,
        0x28..=0x2F => 0x28 + channel,
        0x30..=0x37 => 0x30 + channel,
        0x38..=0x3F => 0x38 + channel,
        0x40..=0xFF => {
            let base = base_register & 0xE0;
            let slot = base_register & 0x1F;
            let operator = slot / 8;
            let new_slot = channel + (operator * 8);
            base + new_slot
        }
        _ => base_register,
    }
}

fn parse_hex_byte(value: &str) -> Option<u8> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u8::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u8>().ok()
    }
}

{% endraw %}
```

### src/ym2151/converter_tests/effects.rs
```rs
{% raw %}
//! Effects tests for YM2151 converter (delay vibrato, pop noise, attack)
use super::*;

#[test]
fn test_delay_vibrato_generates_additional_pitch_events() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 69, // A4 (440 Hz)
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 1920, // 2 seconds at 120 BPM
                channel: 0,
                note: 69,
            },
        ],
    };

    let options = ConversionOptions {
        delay_vibrato: true,
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // Vibrato should emit KC/KF writes after the 200ms delay
    let kc_events_after_delay: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.2)
        .collect();
    assert!(
        !kc_events_after_delay.is_empty(),
        "KC events should include vibrato modulation after delay"
    );

    // Some KF events should deviate from the base (0) once vibrato ramps in
    let non_zero_kf_after_delay: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x30" && e.time > 0.2 && e.data != "0x00")
        .collect();
    assert!(
        !non_zero_kf_after_delay.is_empty(),
        "KF events should include fractional pitch changes from vibrato"
    );
}

#[test]
fn test_pop_noise_envelope_adds_pre_note_overrides() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 240,
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 720,
                channel: 0,
                note: 64,
            },
        ],
    };

    let options = ConversionOptions {
        pop_noise_envelope: Some(PopNoiseEnvelope {
            enabled: true,
            offset_seconds: 0.001,
            registers: vec![RegisterOverride {
                base_register: "0xA0".to_string(),
                value: "0x02".to_string(),
            }],
        }),
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let pre_overrides: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0xA0" && e.data == "0x02" && e.time > 0.4 && e.time < 0.5)
        .collect();
    assert_eq!(
        pre_overrides.len(),
        1,
        "Second note should get one override"
    );

    let restores: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0xA0" && e.time >= 0.499 && e.time <= 0.5)
        .collect();
    assert!(
        restores.iter().any(|e| e.data == "0x05"),
        "Override should be restored to the base D1R value"
    );
}

#[test]
fn test_attack_continuation_fix_forces_release_before_note_on() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 240,
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 720,
                channel: 0,
                note: 64,
            },
        ],
    };

    let options = ConversionOptions {
        attack_continuation_fix: Some(AttackContinuationFix {
            enabled: true,
            offset_seconds: 0.001,
            release_rate: 0xF0,
        }),
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let target_release_addrs = ["0xE0", "0xE8", "0xF0", "0xF8"];
    let release_overrides: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            target_release_addrs.contains(&e.addr.as_str())
                && e.data == "0xF0"
                && e.time > 0.49
                && e.time < 0.5
        })
        .collect();
    assert_eq!(
        release_overrides.len(),
        4,
        "All four operators should receive a pre-note release override"
    );
    assert!(release_overrides.iter().all(|e| e.data == "0xF0"));

    let key_off = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x00" && e.time > 0.49 && e.time < 0.5)
        .expect("Pre-note key off should be generated");
    assert!(key_off.time < 0.5);

    let restore_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            target_release_addrs.contains(&e.addr.as_str())
                && e.data == "0xF7"
                && e.time >= 0.499
                && e.time <= 0.5
        })
        .collect();
    assert!(
        restore_events.iter().all(|e| e.data == "0xF7"),
        "Release rate should return to the base value before key on"
    );
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
227d7af Revise issue notes for pitch visualizer rendering
1d11a9d Add issue note for #189 [auto]
f75d13a Update issue notes for key on sync feature #188
e81902e Add issue note for #188 [auto]
7a0c47a Update issue notes for issue #187
fdcd727 Add issue note for #187 [auto]
aa980d1 Update issue notes for issue #186
8461d8e Add issue note for #186 [auto]
8031ad8 Update issue notes for sound interpolation demo
50d1fe4 Add issue note for #185 [auto]

### 変更されたファイル:
issue-notes/186.md
issue-notes/187.md
issue-notes/188.md
issue-notes/189.md


---
Generated at: 2026-03-11 07:10:02 JST

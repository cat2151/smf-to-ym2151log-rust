Last updated: 2026-03-12

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
- demo-library/wav-exporter.ts
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
## [Issue #193](../issue-notes/193.md): 大きなファイルの検出: 1個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `src/ym2151/converter_tests/attachments.rs` | 550 | +50 |

## テスト実施のお願い

- リファクタリング前後にテストを実行し、それぞれのテスト失敗件数を報告してください
- リファクタリング前後のどちらかでテストがredの場合、まず別issueでtest greenにしてからリファクタリングしてください

## 推奨事項
...
ラベル: refactoring, code-quality, automated
--- issue-notes/193.md の内容 ---

```markdown

```

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

## [Issue #180](../issue-notes/180.md): ポップノイズdemoで、デフォルト値がおかしい
[issue-notes/180.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/180.md)

...
ラベル: 
--- issue-notes/180.md の内容 ---

```markdown
# issue ポップノイズdemoで、デフォルト値がおかしい #180
[issues #180](https://github.com/cat2151/smf-to-ym2151log-rust/issues/180)



```

## [Issue #177](../issue-notes/177.md): （人力）添付JSONまわりのドッグフーディングをする
[issue-notes/177.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/177.md)

...
ラベル: 
--- issue-notes/177.md の内容 ---

```markdown
# issue （人力）添付JSONまわりのドッグフーディングをする #177
[issues #177](https://github.com/cat2151/smf-to-ym2151log-rust/issues/177)



```

## [Issue #83](../issue-notes/83.md): （人力）音色データの扱いについて整理する
[issue-notes/83.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/83.md)

...
ラベル: 
--- issue-notes/83.md の内容 ---

```markdown
# issue 音色データの扱いについて整理する #83
[issues #83](https://github.com/cat2151/smf-to-ym2151log-rust/issues/83)

# これまでの課題
- デフォルト音色データ 0～127 がまだ揃っていない
- ガワだけでいいので用意したほうがいいが、まだ揃っていない
- このため、和音SMFも、sine wave音色で鳴らすことしかできていないはず
- これをやりたい：「ブラウザ、ランダム音色で、音符はMMLでその場で入力して鳴らす、和音も可」
- 制約：ブラウザ音色エディタは別リポジトリ（web-ym2151）である
- 対策案： 音色データreadを使ったdemo、上記のランダム音色かつ、音符MML演奏demo、は、web-ym2151側で実施とする

```

## [Issue #33](../issue-notes/33.md): 仕様追加。ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む。仮仕様。tone editorのdirをsymlinkで検証想定。
[issue-notes/33.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/33.md)

...
ラベル: 
--- issue-notes/33.md の内容 ---

```markdown
# issue 仕様追加。ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む。仮仕様。tone editorのdirをsymlinkで検証想定。 #33
[issues #33](https://github.com/cat2151/smf-to-ym2151log-rust/issues/33)



```

## [Issue #22](../issue-notes/22.md): （手作業）仮で tones/000.json～127.json のうちいくつかを実際に配置する。ym2151-tone-editorを利用して作成できる
[issue-notes/22.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/22.md)

...
ラベル: 
--- issue-notes/22.md の内容 ---

```markdown
# issue tones/000.json～127.json を、ym2151-tone-editorを利用して作成する #22
[issues #22](https://github.com/cat2151/smf-to-ym2151log-rust/issues/22)



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

### .github/actions-tmp/issue-notes/22.md
```md
{% raw %}
# issue project-summary の development-status 生成時、Geminiに与えたprompt、もcommit push、を試す #22
[issues #22](https://github.com/cat2151/github-actions/issues/22)

# 何が困るの？
- 生成された development-status.md の妥当性がわかりづらいし、バグった場合の原因調査がしづらい

# 対策案
- Geminiに与えたpromptをfileにしてcommit pushしておくと、デバッグに役立つ可能性がある。

# 方法案
- Geminiに与えるprompt を生成時、それをfileにsaveし、commit push対象にする。
- ひとまずgenerated-docs/ に保存する。落ち着いたら移動先を検討する。
    - generated-docs/ 配下のまま、も有力な候補である。
        - なぜなら、cjsによってgenerateされたdocなので。

# 日次バッチでpromptを生成させ、agentに投げた
- レビューした
- 修正させた

# 結果
- エラー。pathのミス。呼び出し元側に保存したいのに、共通ワークフロー側に保存となってしまった。
- 対策、ymlで引数を指定するようにした。
- testする。

# 結果
- test green。呼び出し元側にcommitされていることを確認した。
- 20Kbytesである
    - Geminiにわたすサイズとしても許容範囲内と判断する
        - token数から概算して100Kbytes～1Mbytes程度を想定

# closeとする

{% endraw %}
```

### issue-notes/22.md
```md
{% raw %}
# issue tones/000.json～127.json を、ym2151-tone-editorを利用して作成する #22
[issues #22](https://github.com/cat2151/smf-to-ym2151log-rust/issues/22)



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

### issue-notes/177.md
```md
{% raw %}
# issue （人力）添付JSONまわりのドッグフーディングをする #177
[issues #177](https://github.com/cat2151/smf-to-ym2151log-rust/issues/177)



{% endraw %}
```

### issue-notes/180.md
```md
{% raw %}
# issue ポップノイズdemoで、デフォルト値がおかしい #180
[issues #180](https://github.com/cat2151/smf-to-ym2151log-rust/issues/180)



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

### issue-notes/33.md
```md
{% raw %}
# issue 仕様追加。ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む。仮仕様。tone editorのdirをsymlinkで検証想定。 #33
[issues #33](https://github.com/cat2151/smf-to-ym2151log-rust/issues/33)



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

### issue-notes/93.md
```md
{% raw %}
# issue ディレイビブラートdemo pageを、既存pageとは別に追加する #93
[issues #93](https://github.com/cat2151/smf-to-ym2151log-rust/issues/93)

- 用途は、ディレイビブラートの検証用
- smfをuploadして、それにディレイビブラートをかける
- ディレイビブラートJSONがtextareaで編集可能、デバウンスして出力JSONに反映
- 演奏（web-ym2151ライブラリを利用する）

{% endraw %}
```

### src/ym2151/converter_tests/attachments.rs
```rs
{% raw %}
//! Program attachment tests for YM2151 converter
use super::*;

#[test]
fn test_change_to_next_tone_skips_kc_kf_key_on_registers() {
    // Tone events that include KC (0x28), KF (0x30), and key-on (0x08) along with
    // a real tone register (TL 0x60).  The KC/KF/key-on differences must NOT produce
    // interpolation events; only TL should be interpolated.
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
                ticks: 4800,
                channel: 0,
                note: 60,
            },
        ],
    };

    let make_tone = |tl: &str, key_on: &str, kc: &str, kf: &str| ToneDefinition {
        events: vec![
            // Key-on register — must be ignored even when values differ between tones
            Ym2151Event {
                time: 0.0,
                addr: "0x08".to_string(),
                data: key_on.to_string(),
            },
            // KC register — must be ignored
            Ym2151Event {
                time: 0.0,
                addr: "0x28".to_string(),
                data: kc.to_string(),
            },
            // KF register — must be ignored
            Ym2151Event {
                time: 0.0,
                addr: "0x30".to_string(),
                data: kf.to_string(),
            },
            // TL register — must be interpolated
            Ym2151Event {
                time: 0.0,
                addr: "0x60".to_string(),
                data: tl.to_string(),
            },
        ],
        ..ToneDefinition::default()
    };

    // Use different key-on values (0x78 vs 0x38) so the 0x08 register would have been
    // interpolated before the fix, verifying that is_note_register(0x08) is exercised.
    let tone0 = make_tone("0x10", "0x78", "0x4E", "0x00"); // TL=0x10, key-on=0x78, KC=A4, KF=0
    let tone1 = make_tone("0x30", "0x38", "0x5E", "0x20"); // TL=0x30, key-on=0x38 (different!), KC different, KF different

    let options = ConversionOptions {
        program_attachments: vec![
            ProgramAttachment {
                program_change: 0,
                change_to_next_tone: true,
                change_to_next_tone_time: 5.0,
                ..ProgramAttachment::default()
            },
            ProgramAttachment {
                program_change: 1,
                ..ProgramAttachment::default()
            },
        ],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0);
            m.insert(1, tone1);
            m
        },
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // TL (0x60) differences must still be interpolated
    let tl_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();
    assert!(
        tl_events.len() > 2,
        "TL register must still be interpolated; got {} events",
        tl_events.len()
    );

    // KC (0x28) must NOT receive interpolation events — it is note-related
    let kc_extra: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.01)
        .collect();
    assert!(
        kc_extra.is_empty(),
        "KC register must NOT be interpolated; got {} unexpected KC events",
        kc_extra.len()
    );

    // 0x08 (key-on) must NOT receive interpolation events — values differ between
    // tone0 (0x78) and tone1 (0x38), so without the is_note_register guard the
    // interpolation loop would emit writes between those values.
    // Only the initial key-on write at t=0 and the key-off write are expected.
    let key_on_interpolated: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            if e.addr != "0x08" {
                return false;
            }
            // Any value strictly between 0x38 and 0x78 would be an interpolated write
            if let Some(hex) = e.data.strip_prefix("0x") {
                if let Ok(v) = u8::from_str_radix(hex, 16) {
                    return v > 0x38 && v < 0x78;
                }
            }
            false
        })
        .collect();
    assert!(
        key_on_interpolated.is_empty(),
        "Key-on register (0x08) must NOT be interpolated; got {} unexpected writes: {:?}",
        key_on_interpolated.len(),
        key_on_interpolated
    );

    // KF (0x30) must NOT receive interpolation events
    let kf_extra: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x30" && e.time > 0.01)
        .collect();
    assert!(
        kf_extra.is_empty(),
        "KF register must NOT be interpolated; got {} unexpected KF events",
        kf_extra.len()
    );
}

#[test]
fn test_change_to_next_tone_generates_interpolation_events() {
    // A 10-second song with program 0 and program 1 tones that differ in TL (0x60).
    // changeToNextTone should produce continuously changing register writes.
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
            // Last event at tick 9600 = 10 seconds at 120 BPM
            MidiEvent::NoteOff {
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    // Program 0 tone: TL operator 0 = 0x10; Program 1 tone: TL = 0x30 (delta = 32)
    let tone_program0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(), // TL op0, ch0
            data: "0x10".to_string(),
        }],
        ..ToneDefinition::default()
    };
    let tone_program1 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x30".to_string(),
        }],
        ..ToneDefinition::default()
    };

    let options = ConversionOptions {
        program_attachments: vec![
            ProgramAttachment {
                program_change: 0,
                change_to_next_tone: true,
                change_to_next_tone_time: 5.0,
                ..ProgramAttachment::default()
            },
            ProgramAttachment {
                program_change: 1,
                ..ProgramAttachment::default()
            },
        ],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone_program0);
            m.insert(1, tone_program1);
            m
        },
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // TL register 0x60 (channel 0) should have multiple writes as values interpolate
    let tl_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();

    assert!(
        tl_events.len() > 2,
        "changeToNextTone must produce multiple TL register writes; got {}",
        tl_events.len()
    );

    // Values should include the start (0x10) and progress toward the end (0x30)
    let has_start_value = tl_events.iter().any(|e| e.data == "0x10");
    let has_mid_value = tl_events
        .iter()
        .any(|e| e.data != "0x10" && e.data != "0x30");
    assert!(has_start_value, "First interpolation step must write 0x10");
    assert!(
        has_mid_value,
        "Intermediate interpolated values must be written"
    );
}

#[test]
fn test_change_to_next_tone_disabled_produces_no_extra_events() {
    // When change_to_next_tone is false, no interpolation events should be generated.
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
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    let tone0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x10".to_string(),
        }],
        ..ToneDefinition::default()
    };
    let tone1 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x30".to_string(),
        }],
        ..ToneDefinition::default()
    };

    let base_options = ConversionOptions {
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0.clone());
            m.insert(1, tone1.clone());
            m
        },
        ..ConversionOptions::default()
    };
    let interpolation_options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 0,
            change_to_next_tone: false, // explicitly disabled
            ..ProgramAttachment::default()
        }],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0);
            m.insert(1, tone1);
            m
        },
        ..ConversionOptions::default()
    };

    let result_base = convert_to_ym2151_log_with_options(&midi_data, &base_options).unwrap();
    let result_disabled =
        convert_to_ym2151_log_with_options(&midi_data, &interpolation_options).unwrap();

    assert_eq!(
        result_base.event_count, result_disabled.event_count,
        "Disabling changeToNextTone must not add extra events"
    );
}

#[test]
fn test_change_to_next_tone_requires_both_tones() {
    // If tone N+1 is not defined, no interpolation events should be generated.
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
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    let tone0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x10".to_string(),
        }],
        ..ToneDefinition::default()
    };

    // Only tone 0 defined; tone 1 is missing
    let options_missing_next = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 0,
            change_to_next_tone: true,
            change_to_next_tone_time: 5.0,
            ..ProgramAttachment::default()
        }],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0.clone());
            // No tone 1
            m
        },
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options_missing_next).unwrap();

    // Without tone 1, no interpolation events should be added.
    // The only TL writes should be from initialization (0x00) and tone0 application (0x10).
    // There should be no intermediate interpolated values between 0x10 and 0x30.
    let has_interpolated_values = result.events.iter().any(|e| {
        if e.addr != "0x60" {
            return false;
        }
        if let Some(hex) = e.data.strip_prefix("0x") {
            if let Ok(v) = u8::from_str_radix(hex, 16) {
                return v > 0x10 && v < 0x30;
            }
        }
        false
    });
    assert!(
        !has_interpolated_values,
        "Missing tone N+1 must not produce interpolation events with intermediate values"
    );
}

#[test]
fn test_program_attachment_delay_vibrato_applies_only_to_matching_program() {
    // Notes under program 0 should get vibrato; notes under program 1 should not.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Program 0 note (2 seconds long — long enough for vibrato to activate)
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 0,
            },
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 69,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 1920, // 2 seconds at 120 BPM
                channel: 0,
                note: 69,
            },
            // Program 1 note on a second channel
            MidiEvent::ProgramChange {
                ticks: 1920,
                channel: 1,
                program: 1,
            },
            MidiEvent::NoteOn {
                ticks: 1920,
                channel: 1,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 3840, // another 2 seconds
                channel: 1,
                note: 60,
            },
        ],
    };

    let options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 0,
            delay_vibrato: true,
            ..ProgramAttachment::default()
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // Program 0 note (note 69 / A4, channel 0 → YM KC register 0x28) should have
    // KC writes after the 200ms vibrato delay.
    let kc_ch0_after_delay: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.2)
        .collect();
    assert!(
        !kc_ch0_after_delay.is_empty(),
        "Program 0 note should receive vibrato KC modulation"
    );

    // Program 1 note (channel 1 → YM KC register 0x29) must NOT have any KC writes
    // after the note starts at 2 s — vibrato is not enabled for program 1.
    let kc_ch1_after_start: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x29" && e.time > 2.0 + 0.2)
        .collect();
    assert!(
        kc_ch1_after_start.is_empty(),
        "Program 1 note must not receive vibrato KC modulation"
    );
}

#[test]
fn test_program_attachment_no_effects_entry_produces_no_extra_events() {
    // A ProgramAttachment with all effect flags at their defaults (no delay_vibrato,
    // portamento, software_lfo, pop_noise_envelope, or attack_continuation_fix) must
    // not crash and must not generate any vibrato/LFO/etc events.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 5,
            },
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
        ],
    };

    // Attachment entry with no effects enabled (all flags remain at default)
    let options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 5,
            ..ProgramAttachment::default()
        }],
        ..ConversionOptions::default()
    };

    // Should succeed without panicking
    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();
    assert!(result.event_count > 0);

    // No extra KC events from vibrato should be present
    let vibrato_kc: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.2)
        .collect();
    assert!(
        vibrato_kc.is_empty(),
        "No-effects attachment must not produce vibrato events"
    );
}

#[test]
fn test_program_attachment_unmatched_program_produces_no_extra_events() {
    // An attachment for program 99 should do nothing when only program 0 is used.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 0,
            },
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
        ],
    };

    let options_with_attachment = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 99, // not used in the MIDI
            delay_vibrato: true,
            ..ProgramAttachment::default()
        }],
        ..ConversionOptions::default()
    };

    let result_with =
        convert_to_ym2151_log_with_options(&midi_data, &options_with_attachment).unwrap();
    let result_without = convert_to_ym2151_log(&midi_data).unwrap();

    // Both outputs should have the same event count — unmatched attachment is a no-op.
    assert_eq!(
        result_with.event_count, result_without.event_count,
        "Unmatched program attachment must not add extra events"
    );
}

{% endraw %}
```

### tones/000.json
```json
{% raw %}
{
  "type": "YM2151 tone",
  "events": [
    {
      "time": 0.0,
      "addr": "0x20",
      "data": "0xC7"
    },
    {
      "time": 0.0,
      "addr": "0x38",
      "data": "0x00"
    },
    {
      "time": 0.0,
      "addr": "0x40",
      "data": "0x01"
    },
    {
      "time": 0.0,
      "addr": "0x60",
      "data": "0x00"
    },
    {
      "time": 0.0,
      "addr": "0x80",
      "data": "0x1F"
    },
    {
      "time": 0.0,
      "addr": "0xA0",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xC0",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xE0",
      "data": "0xF7"
    },
    {
      "time": 0.0,
      "addr": "0x48",
      "data": "0x01"
    },
    {
      "time": 0.0,
      "addr": "0x68",
      "data": "0x7F"
    },
    {
      "time": 0.0,
      "addr": "0x88",
      "data": "0x1F"
    },
    {
      "time": 0.0,
      "addr": "0xA8",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xC8",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xE8",
      "data": "0xF7"
    },
    {
      "time": 0.0,
      "addr": "0x50",
      "data": "0x01"
    },
    {
      "time": 0.0,
      "addr": "0x70",
      "data": "0x7F"
    },
    {
      "time": 0.0,
      "addr": "0x90",
      "data": "0x1F"
    },
    {
      "time": 0.0,
      "addr": "0xB0",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xD0",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xF0",
      "data": "0xF7"
    },
    {
      "time": 0.0,
      "addr": "0x58",
      "data": "0x01"
    },
    {
      "time": 0.0,
      "addr": "0x78",
      "data": "0x7F"
    },
    {
      "time": 0.0,
      "addr": "0x98",
      "data": "0x1F"
    },
    {
      "time": 0.0,
      "addr": "0xB8",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xD8",
      "data": "0x05"
    },
    {
      "time": 0.0,
      "addr": "0xF8",
      "data": "0xF7"
    }
  ]
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
2b59989 Merge pull request #192 from cat2151/copilot/fix-waveform-viewer-bug
07a972c remove envelope simulation description from pop-noise waveform viewer section
1f9a02b fix: anchor waveform pixel origin to exact viewStart to prevent high-zoom shift
2a130c9 fix waveform viewer bugs: fix samplesPerPixel alignment, remove orange envelope overlay
74a4249 Initial plan
afd5270 Merge pull request #191 from cat2151/copilot/fix-kc-kf-issue-demo
e609c58 Fix test: use differing 0x08 values to properly exercise is_note_register guard
1a68255 Fix: Exclude KC, KF, and key-on registers from tone interpolation (#185)
8964ed5 Initial plan
5c608bb Merge pull request #190 from cat2151/copilot/add-wav-file-export

### 変更されたファイル:
demo-library/pop-noise-demo.ts
demo-library/pop-noise.html
demo-library/wav-exporter.ts
demo-library/waveform-canvas.ts
demo-library/waveform-simulator.ts
demo-library/waveform-viewer.ts
generated-docs/development-status-generated-prompt.md
generated-docs/development-status.md
generated-docs/project-overview-generated-prompt.md
generated-docs/project-overview.md
issue-notes/183.md
issue-notes/189.md
src/ym2151/converter/register_effects.rs
src/ym2151/converter_tests/attachments.rs
src/ym2151/converter_tests/effects.rs


---
Generated at: 2026-03-12 07:10:47 JST

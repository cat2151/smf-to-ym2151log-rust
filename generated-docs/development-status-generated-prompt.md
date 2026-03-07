Last updated: 2026-03-08

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
- demo-library/globals.d.ts
- demo-library/index.html
- demo-library/library-demo.ts
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
- demo-library/tone-json-attachment.ts
- demo-library/tone-json-demo.ts
- demo-library/tone-json-mml.ts
- demo-library/tone-json.html
- demo-library/tsconfig.json
- demo-library/vite.config.ts
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
- issue-notes/105.md
- issue-notes/111.md
- issue-notes/112.md
- issue-notes/115.md
- issue-notes/123.md
- issue-notes/126.md
- issue-notes/145.md
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
- src/ym2151/converter_tests/basic.rs
- src/ym2151/converter_tests/channels.rs
- src/ym2151/converter_tests/effects.rs
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
## [Issue #149](../issue-notes/149.md): 大きなファイルの検出: 1個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `src/ym2151/converter_tests/programs.rs` | 534 | +34 |

## テスト実施のお願い

- リファクタリング前後にテストを実行し、それぞれのテスト失敗件数を報告してください
- リファクタリング前後のどちらかでテストがredの場合、まず別issueでtest greenにしてからリファクタリングしてください

## 推奨事項

1....
ラベル: refactoring, code-quality, automated
--- issue-notes/149.md の内容 ---

```markdown

```

## [Issue #148](../issue-notes/148.md): fix: remove permissions block from ci.yml to fix CI auto-format 403 error
Fixes the CI `Auto Format` job's 403 error when pushing formatted code.

## Root Cause

`ci.yml` had an explicit workflow-level `permissions` block with `contents: read`, which completely overrides the repository's Settings → Actions → Workflow permissions (read and write). Once a `permissions` bloc...
ラベル: 
--- issue-notes/148.md の内容 ---

```markdown

```

## [Issue #147](../issue-notes/147.md): CI auto-format failed
## CI auto-format failed

**Workflow run:** https://github.com/cat2151/smf-to-ym2151log-rust/actions/runs/22800045695
**Branch:** main
**Commit:** 7cbd1d27b4b83f5bcebd95e88ccd198d5c9cdcf3
**Triggered by:** push

Please investigate the workflow logs for details....
ラベル: ci-format-failure
--- issue-notes/147.md の内容 ---

```markdown

```

## [Issue #127](../issue-notes/127.md): ディレイビブラートとLFOが低品質。ずっとクリックノイズが乗っている。wav exportをdemoに実装して分析せよ。ポルタメントは大丈夫のようだ

ラベル: 
--- issue-notes/127.md の内容 ---

```markdown

```

## [Issue #126](../issue-notes/126.md): ポップノイズdemoで、添付JSONを変更しても、最終log JSONに反映されていない
[issue-notes/126.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/126.md)

...
ラベル: 
--- issue-notes/126.md の内容 ---

```markdown
# issue ポップノイズdemoで、添付JSONを変更しても、最終log JSONに反映されていない #126
[issues #126](https://github.com/cat2151/smf-to-ym2151log-rust/issues/126)



```

## [Issue #115](../issue-notes/115.md): 添付JSONについて、隣接する（後続の番号への）線形補間音色変化のon/offを実装し、demo pageも新規追加する
[issue-notes/115.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/115.md)

...
ラベル: 
--- issue-notes/115.md の内容 ---

```markdown
# issue 添付JSONについて、隣接する（後続の番号への）線形補間音色変化のon/offを実装し、demo pageも新規追加する #115
[issues #115](https://github.com/cat2151/smf-to-ym2151log-rust/issues/115)

- 要は、ProgramChange0と1を定義し、さらにProgramChange0側に、「changeToNextTone: true」「changeToNextToneTime: 5」を指定すれば、
- 5秒で、ProgramChange1の音色に線形補間で変化する。
- つまりレジスタbase値を、5秒の間、連続して変更し続けて、レジスタ書き込みeventを発生させる。
- これにより、MULがProgramChange0において1で、ProgramChange1においてMUL6だった音色は、1秒ごとにMULの値が1ずつ変化して、音色変化をすることになる。
- よりスタンダードな用途としては、モジュレータTLがProgramChange0と1とで、20違う音色があれば、5秒間でモジュレータTLが変わって音色の倍音成分が滑らかに変化していくことになる。
- ProgramChange1の音色に変わったのち、5秒かけて、ProgramChange0の音色に戻る。これをループする。
- これは仮formatである
   -  UX検証後、検証データを元に、「隣接音色」という結合、を使わない、より使いやすいformatを検討する。
   - 例えばNRPN定義を添付JSONで行い、そこで定義したNRPNがSMFにあればこれをon/offや、動的なパラメータ変更、あるいはProgramChangeがある場所をみて自動的に時間を決定、などである。
   - 今やらないのは、データ不足、YAGNI、であるから。

   

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
### .github/actions-tmp/issue-notes/15.md
```md
{% raw %}
# issue project_summary scripts cjs を分解し、できるだけ1ファイル200行未満にし、agentによるメンテをしやすくする #15
[issues #15](https://github.com/cat2151/github-actions/issues/15)

# 状況
- agentに、最初の小さな一歩のAgent実行プロンプトを実行させた
- 結果、以下を得た：
    - project_summary_cjs_analysis.md
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
- 結果
    - 生成させたpromptをレビューした
    - promptを修正した
    - agentに投げた
    - 結果、GitUtils.cjsを切り出しできた
    - それをリファクタリングミスがないかチェックさせた
    - agentによるチェック結果は合格だった
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
- 結果
    - 生成させたpromptをレビューした
        - promptの対象ファイルから project_summary_cjs_analysis.md が漏れていることがわかったので修正した
    - promptを修正した
    - agentに投げた
    - 結果、FileSystemUtils.cjsを切り出しできた
    - それをリファクタリングミスがないかチェックさせた
    - agentによるチェック結果は合格だった
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
- 結果
    - 生成させたpromptをレビューした
    - 今回は低品質、NG、と判断した
    - 判断基準は、project_summary_cjs_analysis.md と乖離してしまっている点。今回はハルシネーションを含んだplanである、と判断した
    - 人力でpromptを書き、planさせ、plan結果をレビューし、agentに投げた
    - 結果、CodeAnalyzer.cjsとProjectAnalyzer.cjsを切り出しできた
- どうする？
    - 次の一手をagentに生成させてみる（翌日の日次バッチで自動生成させる）
    - 備考、課題、Geminiに生成させているdocumentは2つある。かなり位置づけが違うものである。
        - projectのソースファイル分析。
        - projectのissues分析。
        - この2つについて、class, cjs, yml まで分割をするかを、あとで検討する。
        - おそらく、class分割どまりとし、ソースファイル分析結果をissues分析の参考資料としてGeminiのcontextに与える改善をする、がよい、と想定しておく。
- 課題、エラーで落ちた。昨日は落ちてない。
    - 原因、昨日のagentのリファクタリング時に、ハルシネーションで、
        - codeが破壊されていた
        - run メソッドが削除されていた
        - 一つ前のrevisionにはrun メソッドがあった
        - ほかにもcode破壊があったのかは不明、調査省略、明日の日次バッチをtestと調査として利用するつもり
- どうする？
    - 単純に一つ前のrevisionからrun メソッドを復活させ、明日の日次バッチをtestと調査として利用する
- 再発防止策は？
    - ノーアイデア。昨日それなりにagentにチェックをさせたはずだが根本的な大きなミスが発生していた。
    - 構文チェックは通っていたが、問題を検知できなかった。
    - チェックが機能していない、あるいは機能として不足している。
    - 分析。変更量が大きかったぶんミスのリスクが増えていた。
    - 対策案。もっと小さく一歩ずつ変更させる。
    - 対策案。リファクタリング時、いきなりメソッド削除をさせない。
        - まず全cjsの全メソッドのlistをさせる。
        - のち、削除対象の重複メソッドのlistをさせる。
        - そして削除planをさせる。
        - のち、削除させる。
        - さらに削除後のメソッドlistをさせる。
        - そして削除しすぎていないかを削除前後のlist比較でチェックさせる。
        - これでrunまで削除してしまうのを防止できるかもしれない。
        - これは人力からみると、おかしな話である。人力なら1つずつ移動をするだけであり、ミスのしようがない。
        - LLMの典型的なハルシネーション問題の一つである、と認識する
- 結果は？
    - test green
    - run メソッドの人力復活は成功した
    - 日次バッチで生成した次の一手のpromptを投げた
    - リファクタリング成功した。ProjectSummaryGenerator を切り出した
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - 先に、2つのdocument生成を、1つずつ生成できるよう疎結合にリファクタリング、をしたほうがよさそう
    - agentにそれを投げた
    - 成功した、と判断する
    - 課題、`BaseSummaryGenerator.cjs` は、baseの機能と、`ProjectOverviewGenerator.cjs`専用の機能とが混ざっている。
        - baseに集約すべきは、`ProjectSummaryCoordinator.cjs`と`ProjectOverviewGenerator.cjs`とが必ずどちらも使う機能、である、と考える。
        - 対策、明日以降それをagentに投げる
    - `project_summary_cjs_analysis.md` は削除とする。役目が完了した、と判断する。リファクタリング前のソース構造の分析documentであり、今は存在しているとわかりづらくなる。シンプル優先のため削除とする。
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - `BaseSummaryGenerator.cjs` を切り出したのは成功した、と判断する
    - `BaseSummaryGenerator.cjs` を2分割するため、agentにplanさせた
    - レビューした
    - agentに2分割させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - `BaseSummaryGenerator.cjs` を2分割は成功した、と判断する
    - issue track機能構造をリファクタリングし、以下にする
        - development status generator : baseを継承する
        - issue tracker : 汎用関数群
    - agentに実施させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - DevelopmentStatusGeneratorとissue trackerのリファクタリングは成功した、と判断する
    - ProjectOverview生成機能のリファクタリングをする
    - agentに実施させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - ProjectOverview生成機能のリファクタリングは成功した、と判断する
    - 課題、overviewと、developmentStatusとが混在し、dirが読みづらい。
    - 対策、shared/、overview/、development/、の3つのdirに切り分ける
    - agentに分析、planさせ、レビューし、planさせ、実施させた
    - レビューした。OKと判断する
- どうする？
    - 次の一手をagentに生成させてみる（agentに投げるpromptを、翌日の日次バッチで自動生成させる）
- 結果
    - test green
    - shared/、overview/、development/、の3つのdirに切り分けるリファクタリングは成功した、と判断する
    - agentに、agentがメンテしやすいか？の観点からレビューさせた
    - 詳細は割愛
        - `> 最優先で取り組むべきは 設定管理の一元化 と エラーハンドリングの統一 です。これにより、Agentにとって予測可能で理解しやすいコードベースになります。`
        - それは別issueで、設定変更をマストでやるので、OKと判断する
- これでagentによるメンテは十分しやすくなった、と判断する
- closeとする

{% endraw %}
```

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

### .github/actions-tmp/issue-notes/26.md
```md
{% raw %}
# issue userによるcommitがなくなって24時間超経過しているのに、毎日ムダにproject summaryとcallgraphの自動生成が行われてしまっている #26
[issues #26](https://github.com/cat2151/github-actions/issues/26)

# どうする？
- logを確認する。24時間チェックがバグっている想定。
- もしlogから判別できない場合は、logを改善する。

# log確認結果
- botによるcommitなのに、user commitとして誤判別されている
```
Checking for user commits in the last 24 hours...
User commits found: true
Recent user commits:
7654bf7 Update callgraph.html [auto]
abd2f2d Update project summaries (overview & development status)
```

# ざっくり調査結果
- #27 が判明した

# どうする？
- [x] #27 を修正する。これで自動的に #26 も修正される想定。
    - 当該処理を修正する。
    - もしデータ不足なら、より詳細なlog生成を実装する。
- 別件として、このチェックはむしろworkflow ymlの先頭で行うのが適切と考える。なぜなら、以降のムダな処理をカットできるのでエコ。
    - [x] #28 を起票したので、そちらで実施する。

# close条件は？
- 前提
    - [x] 先行タスクである #27 と #28 が完了済みであること
- 誤爆がなくなること。
    - つまり、userによるcommitがなくなって24時間超経過後の日次バッチにて、
        - ムダなdevelopment status生成、等がないこと
        - jobのlogに「commitがないので処理しません」的なmessageが出ること
- どうする？
    - 日次バッチを本番を流して本番testする

# 結果
- github-actions logより：
    - 直近24hのcommitはbotによる1件のみであった
    - よって後続jobはskipとなった
    - ことを確認した
- close条件を満たした、と判断する
```
Run node .github_automation/check_recent_human_commit/scripts/check-recent-human-commit.cjs
BOT: Commit 5897f0c6df6bc2489f9ce3579b4f351754ee0551 | Author: github-actions[bot] <41898282+github-actions[bot]@users.noreply.github.com> | Message: Update project summaries (overview & development status) [auto]
has_recent_human_commit=false
```

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/27.md
```md
{% raw %}
# issue LLMが生成したコードに、commit時のemailとnameについて公式推奨と公式非推奨の2つがブレて混在している。さらに判定処理が片方だけ対応になっている #27
[issues #27](https://github.com/cat2151/github-actions/issues/27)

# 補足
- さらに、commit messageもブレている。auto があったりなかったりしている。
    - auto があるほうが適切である、と判断する。
- 公式推奨とは：
    - name, emailが推奨、commit messageにautoが入っている
```
        git config user.name "github-actions[bot]"
        git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
        git commit -m "Update callgraph.html [auto]"
```
- 公式非推奨とは：
    - name, emailが非推奨、commit messageにもautoが入っていない
```
        git config user.name github-actions
        git config user.email github-actions@github.com
        git commit -m "Add issue note for #${{ inputs.issue_number }}"
```

# どうする？
- 当該のworkflowとscriptで、github-actions@github.com 等をgrepし、公式推奨に統一する
    - 影響範囲
        - 24hチェック側もブレているので、しばらくは誤爆が続く
        - #28 を修正して24h待てば、誤爆は解消する見込みである
- 注意、24hチェックは変更しない。
    - なぜなら #28 で全面的に修正するため、変更しても手戻りになる。
- 過去commit messageはそのままとする

# close条件は？
- [x] name, email, commit comment のlogicが、公式推奨に統一されること
- [x] #28 が修正されること
- 以上を満たせば、test不要で、机上checkのみでcloseとする。
- ほかは #26 のtestで担保する。

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

### .github/workflows/ci.yml
```yml
{% raw %}
name: CI

on:
  push:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

permissions:
  contents: read
  issues: write

jobs:
  build:
    name: Build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}
      - name: Build
        run: cargo build --verbose
      - name: Build release
        run: cargo build --release --verbose
      - name: Create issue on failure
        if: failure()
        env:
          GH_TOKEN: ${{ github.token }}
        run: python .github/scripts/create_ci_issue.py --label "ci-build-failure" --title "CI build failed"

  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}
      - name: Run tests
        run: cargo test --verbose
      - name: Create issue on failure
        if: failure()
        env:
          GH_TOKEN: ${{ github.token }}
        run: python .github/scripts/create_ci_issue.py --label "ci-test-failure" --title "CI test failed"

  format:
    name: Auto Format
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - name: Run cargo fmt
        run: cargo fmt
      - name: Commit formatted files
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git diff --quiet || (git add -A && git commit -m "style: auto-format with cargo fmt")
      - name: Push changes
        run: git push
      - name: Create issue on failure
        if: failure()
        env:
          GH_TOKEN: ${{ github.token }}
        run: python .github/scripts/create_ci_issue.py --label "ci-format-failure" --title "CI auto-format failed"

  clippy:
    name: Clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}
      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Create issue on failure
        if: failure()
        env:
          GH_TOKEN: ${{ github.token }}
        run: python .github/scripts/create_ci_issue.py --label "ci-clippy-failure" --title "CI clippy failed"

{% endraw %}
```

### issue-notes/115.md
```md
{% raw %}
# issue 添付JSONについて、隣接する（後続の番号への）線形補間音色変化のon/offを実装し、demo pageも新規追加する #115
[issues #115](https://github.com/cat2151/smf-to-ym2151log-rust/issues/115)

- 要は、ProgramChange0と1を定義し、さらにProgramChange0側に、「changeToNextTone: true」「changeToNextToneTime: 5」を指定すれば、
- 5秒で、ProgramChange1の音色に線形補間で変化する。
- つまりレジスタbase値を、5秒の間、連続して変更し続けて、レジスタ書き込みeventを発生させる。
- これにより、MULがProgramChange0において1で、ProgramChange1においてMUL6だった音色は、1秒ごとにMULの値が1ずつ変化して、音色変化をすることになる。
- よりスタンダードな用途としては、モジュレータTLがProgramChange0と1とで、20違う音色があれば、5秒間でモジュレータTLが変わって音色の倍音成分が滑らかに変化していくことになる。
- ProgramChange1の音色に変わったのち、5秒かけて、ProgramChange0の音色に戻る。これをループする。
- これは仮formatである
   -  UX検証後、検証データを元に、「隣接音色」という結合、を使わない、より使いやすいformatを検討する。
   - 例えばNRPN定義を添付JSONで行い、そこで定義したNRPNがSMFにあればこれをon/offや、動的なパラメータ変更、あるいはProgramChangeがある場所をみて自動的に時間を決定、などである。
   - 今やらないのは、データ不足、YAGNI、であるから。

   

{% endraw %}
```

### issue-notes/126.md
```md
{% raw %}
# issue ポップノイズdemoで、添付JSONを変更しても、最終log JSONに反映されていない #126
[issues #126](https://github.com/cat2151/smf-to-ym2151log-rust/issues/126)



{% endraw %}
```

### issue-notes/33.md
```md
{% raw %}
# issue 仕様追加。ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む。仮仕様。tone editorのdirをsymlinkで検証想定。 #33
[issues #33](https://github.com/cat2151/smf-to-ym2151log-rust/issues/33)



{% endraw %}
```

### issue-notes/47.md
```md
{% raw %}
# issue ブラウザ対応: SMFバイナリをJSONに変換するWASMインターフェースの実装 #47
[issues #47](https://github.com/cat2151/smf-to-ym2151log-rust/issues/47)

## agentに投げるprompt案
- 現在CLIとして動作しているこのツールについて、変換機能を、Webブラウザ上（JavaScript）からも利用可能にしたいと考えています。  
- そのために wasm-bindgen を導入し、SMFのバイナリデータを受け取って解析結果をJSON文字列として返すWASM関数を実装してください。
- 必要に応じて、バイナリデータをSMFとしてparseする処理なども、ライブラリクレートとして分離するようリファクタリングしてください。
- それらを呼び出すWASM用のエントリポイントを追加してください。
- 完了条件
    - WASMビルドが成功し、pkgディレクトリに成果物が配置されること。
    - 既存のCLIツールとしての機能（cargo run）が破壊されず、そのまま動作すること。

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

### src/ym2151/converter_tests/programs.rs
```rs
{% raw %}
//! Program change and drum channel tests for YM2151 converter
use super::*;

#[test]
fn test_convert_program_change() {
    // Test that program change events trigger tone changes
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Program change at the start
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 42,
            },
            // Play a note
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

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Should have initialization + program change tone events + note events
    // 8 KEY OFF + 26 channel init + 26 program change tone + note on (3) + note off (1)
    // = 64 events
    assert_eq!(result.event_count, 64);

    // Verify there are tone setting events at time 0
    // Look for RL_FB_CONNECT register writes (0x20-0x27)
    let tone_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4 && e.time < 0.001)
        .collect();

    // Should have 2 writes: one from init, one from program change
    assert!(
        tone_events.len() >= 2,
        "Should have tone settings from both init and program change"
    );
}

#[test]
fn test_convert_program_change_unused_channel() {
    // Program change on a channel that has no notes should be ignored
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Program change on channel 5
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 5,
                program: 10,
            },
            // But only channel 0 plays a note
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

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Should only have events for channel 0
    // 8 KEY OFF + 26 channel 0 init + note on (3) + note off (1) = 38
    assert_eq!(result.event_count, 38);
}

#[test]
fn test_convert_program_change_with_attachment_tone() {
    // Program change should use tone definitions supplied via attachment JSON
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 99,
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

    let mut options = ConversionOptions::default();
    options.tones.insert(
        99,
        ToneDefinition {
            events: vec![Ym2151Event {
                time: 0.0,
                addr: "0x20".to_string(),
                data: "0xAB".to_string(),
            }],
        },
    );

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // 8 KEY OFF + 26 init + 1 attachment tone + note on (3) + note off (1) = 39
    assert_eq!(result.event_count, 39);

    let has_custom_tone = result.events.iter().any(|e| e.data == "0xAB");
    assert!(
        has_custom_tone,
        "Attachment tone definition should be applied for program 99"
    );
}

#[test]
fn test_convert_multiple_program_changes() {
    // Test multiple program changes on the same channel
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 10,
            },
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
            // Change to a different program
            MidiEvent::ProgramChange {
                ticks: 240,
                channel: 0,
                program: 20,
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

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // 8 KEY OFF + 26 init + 26 program 10 + note (3) + note off (1)
    // + 26 program 20 + note (3) + note off (1) = 94
    assert_eq!(result.event_count, 94);

    // Verify both program changes generated tone events
    // Check for RL_FB_CONNECT register writes at time 0
    let tone_events_time_0: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4 && e.time < 0.001)
        .collect();
    assert!(
        tone_events_time_0.len() >= 2,
        "Should have init + program 10 tone events"
    ); // init + program 10

    // Second program change should be at a different time
    let tone_events_later: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4 && e.time > 0.001)
        .collect();
    assert!(
        !tone_events_later.is_empty(),
        "Should have tone change at later time"
    );
}

#[test]
fn test_convert_drum_channel_note_on_channel_0() {
    // Test that MIDI channel 9 (drum) maps to YM2151 channel 0
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 9, // Drum channel
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 9,
                note: 60,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Find KC register write for channel 0 (0x28)
    let kc_events: Vec<&Ym2151Event> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time < 0.001)
        .collect();

    assert_eq!(
        kc_events.len(),
        1,
        "Drum channel should use YM2151 channel 0 (KC register 0x28)"
    );

    // Verify KEY ON uses channel 0
    let key_on = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time < 0.001)
        .expect("Should have KEY ON for channel 0");
    assert_eq!(key_on.data, "0x78"); // 0x78 = all operators on, channel 0
}

#[test]
fn test_convert_drum_and_regular_channels_together() {
    // Test with both drum channel and regular channels
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Drum channel (MIDI 9) at same tick
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 9,
                note: 36, // Bass drum
                velocity: 100,
            },
            // Regular channel (MIDI 0) at same tick
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            // Regular channel (MIDI 1) at same tick
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 1,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 9,
                note: 36,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 1,
                note: 64,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Verify drum channel uses YM2151 channel 0
    let drum_kc = result
        .events
        .iter()
        .find(|e| e.addr == "0x28" && e.time < 0.001)
        .expect("Drum should use YM2151 channel 0");
    assert!(drum_kc.data.starts_with("0x"));

    // Verify MIDI channel 0 uses YM2151 channel 1
    let ch0_kc = result
        .events
        .iter()
        .find(|e| e.addr == "0x29" && e.time < 0.001)
        .expect("MIDI ch 0 should use YM2151 channel 1");
    assert!(ch0_kc.data.starts_with("0x"));

    // Verify MIDI channel 1 uses YM2151 channel 2
    let ch1_kc = result
        .events
        .iter()
        .find(|e| e.addr == "0x2A" && e.time < 0.001)
        .expect("MIDI ch 1 should use YM2151 channel 2");
    assert!(ch1_kc.data.starts_with("0x"));

    // Verify KEY ON events at this time correspond to the expected channels,
    // without relying on their relative ordering in the event list.
    let key_on_events: Vec<&Ym2151Event> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time < 0.001 && e.data.starts_with("0x7"))
        .collect();

    // Should have 3 KEY ON events (drum + 2 regular channels)
    assert_eq!(key_on_events.len(), 3);

    // Collect the KEY ON data bytes and verify they include the expected channels.
    let key_on_data: Vec<&str> = key_on_events.iter().map(|e| e.data.as_str()).collect();

    assert!(
        key_on_data.contains(&"0x78"),
        "Expected a KEY ON for YM2151 channel 0 (drum)"
    );
    assert!(
        key_on_data.contains(&"0x79"),
        "Expected a KEY ON for YM2151 channel 1 (MIDI ch 0)"
    );
    assert!(
        key_on_data.contains(&"0x7A"),
        "Expected a KEY ON for YM2151 channel 2 (MIDI ch 1)"
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
fn test_program_attachment_tone_only_entry_skipped_without_panic() {
    // An attachment entry with only a Tone and no effects should be silently skipped
    // without applying any vibrato/portamento/LFO/etc events.
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

    // Attachment with a tone-only entry (no effects flags set)
    let options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 5,
            // All effect flags remain false / None (default)
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
        "Tone-only attachment must not produce vibrato events"
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
7cbd1d2 Merge pull request #146 from cat2151/copilot/fix-web-ym2151-load-issue
4aa90b1 fix: update web-ym2151 URL from sine_test.js to ym2151.js
0b623af Initial plan
286cae6 Add issue note for #145 [auto]
0d01189 Merge pull request #144 from cat2151/copilot/translate-copilot-instructions-to-japanese
455ba0c docs: fix deploy description accuracy per review feedback
e17b248 docs: update copilot-instructions.md with current module list and deploy structure
422b0af Initial plan
d1fe83f Merge pull request #142 from cat2151/copilot/change-json-format-for-programchange
990afd2 perf/test: pre-group segments by program, skip effect-free entries, add program_attachment tests

### 変更されたファイル:
.github/copilot-instructions.md
.github/scripts/create_ci_issue.py
.github/workflows/ci.yml
.gitignore
demo-library/delay-vibrato-demo.ts
demo-library/log-visualizer.ts
demo-library/pop-noise-demo.ts
demo-library/portamento-soft-lfo-demo.ts
demo-library/shared-demo.ts
demo-library/style.css
demo-library/tone-json-attachment.ts
issue-notes/122.md
issue-notes/125.md
issue-notes/128.md
issue-notes/131.md
issue-notes/133.md
issue-notes/145.md
src/lib.rs
src/ym2151/converter.rs
src/ym2151/converter_tests.rs
src/ym2151/converter_tests/channels.rs
src/ym2151/converter_tests/effects.rs
src/ym2151/converter_tests/programs.rs
src/ym2151/event_processor.rs
tests/integration_conversion.rs
tests/integration_midi.rs
tests/integration_multichannel.rs
tests/integration_program_change.rs


---
Generated at: 2026-03-08 07:07:51 JST

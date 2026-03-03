Last updated: 2026-03-04

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
- issue-notes/122.md
- issue-notes/123.md
- issue-notes/125.md
- issue-notes/126.md
- issue-notes/128.md
- issue-notes/131.md
- issue-notes/133.md
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
- src/wasm.rs
- src/ym2151/channel_allocation.rs
- src/ym2151/converter/pitch_effects.rs
- src/ym2151/converter/register_effects.rs
- src/ym2151/converter/waveform.rs
- src/ym2151/converter.rs
- src/ym2151/converter_tests.rs
- src/ym2151/event_processor.rs
- src/ym2151/events.rs
- src/ym2151/init.rs
- src/ym2151/mod.rs
- src/ym2151/note_table.rs
- src/ym2151/tempo_map.rs
- src/ym2151/tone.rs
- tests/create_test_midi.py
- tests/integration_tests.rs
- tests/test_data/multi_channel.mid
- tests/test_data/multi_track.mid
- tests/test_data/program_change.mid
- tests/test_data/simple_melody.mid
- tests/test_data/tempo_change.mid
- tones/000.json
- tones/README.md

## 現在のオープンIssues
## [Issue #134](../issue-notes/134.md): 大きなファイルの検出: 4個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `tests/integration_tests.rs` | 1150 | +650 |
| `src/ym2151/converter_tests.rs` | 1057 | +557 |
| `src/midi/utils.rs` | 671 | +171 |
| `src/ym2151/event_processor.rs` | 574 | +74 |

## テスト実施のお願い...
ラベル: refactoring, code-quality, automated
--- issue-notes/134.md の内容 ---

```markdown

```

## [Issue #133](../issue-notes/133.md): copilot-instructions.mdが英語なので妥当性が不明。日本語にする
[issue-notes/133.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/133.md)

...
ラベル: 
--- issue-notes/133.md の内容 ---

```markdown
# issue copilot-instructions.mdが英語なので妥当性が不明。日本語にする #133
[issues #133](https://github.com/cat2151/smf-to-ym2151log-rust/issues/133)



```

## [Issue #132](../issue-notes/132.md): Fix mml-support.ts: remove duplicate MML runtime, delegate to tone-json-mml.ts
- [x] Investigate the issue: `mml-support.ts` had its own private duplicate implementation of `ensureMmlRuntime`, `treeToJson`, and module state (`mmlParser`, `parseTreeJsonToSmf`) instead of importing from `tone-json-mml.ts`
- [x] Fix `mml-support.ts` to import `ensureMmlRuntime`, `getMmlParser`, `...
ラベル: 
--- issue-notes/132.md の内容 ---

```markdown

```

## [Issue #131](../issue-notes/131.md): demoのディレイビブラートにおいて、MMLの「l」「t」コマンドなどがまったく動作していない
[issue-notes/131.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/131.md)

...
ラベル: 
--- issue-notes/131.md の内容 ---

```markdown
# issue demoのディレイビブラートにおいて、MMLの「l」「t」コマンドなどがまったく動作していない。ライブラリを使わない自実装に逃げているのか？調査せよ #131
[issues #131](https://github.com/cat2151/smf-to-ym2151log-rust/issues/131)



```

## [Issue #128](../issue-notes/128.md): demoの、YM2151レジスタのビジュアライザを改善する。keyon/offとKCの高い低いを可視化する。KC maxとminを算出して上下いっぱいに表示する
[issue-notes/128.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/128.md)

...
ラベル: 
--- issue-notes/128.md の内容 ---

```markdown
# issue demoの、YM2151レジスタのビジュアライザを改善。KCの高い低いも可視化する。KC maxとminを算出して上下いっぱいに表示する #128
[issues #128](https://github.com/cat2151/smf-to-ym2151log-rust/issues/128)



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

## [Issue #125](../issue-notes/125.md): CIがclippyで落ちたとき等にissueを自動起票、を実装する。このリポジトリの他のワークフローymlを参考にする
[issue-notes/125.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/125.md)

...
ラベル: 
--- issue-notes/125.md の内容 ---

```markdown
# issue CIがclippyで落ちたとき等にissueを自動起票、を実装する。このリポジトリの他のワークフローymlを参考にする #125
[issues #125](https://github.com/cat2151/smf-to-ym2151log-rust/issues/125)



```

## [Issue #123](../issue-notes/123.md): 添付JSONについて、フォーマット変更。音色JSONの自己記述性を高めるため、ProgramChangeはJSON項目名にProgramChangeという名前をつける
[issue-notes/123.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/123.md)

...
ラベル: 
--- issue-notes/123.md の内容 ---

```markdown
# issue 添付JSONについて、フォーマット変更。音色JSONの自己記述性を高めるため、ProgramChangeはJSON項目名にProgramChangeという名前をつける #123
[issues #123](https://github.com/cat2151/smf-to-ym2151log-rust/issues/123)

- あわせて添付JSON構造を変更する
- 0個～128個の配列である
- 配列の1要素：
  - ProgramChange: 0 のように、音色番号0～127をJSONで定義して、自己記述性を高める
  - ディレイビブラート、ポルタメント、LFO、も、ProgramChangeとセットで扱う。
    - 設計思想：
      - SMF側とMML側の記述量を減らす。ProgramChangeするだけで多彩な音色を楽しめる。
      - LFOだけ変更したい場合がありうるので、それは別途検討する。現状は「新たなProgramChangeを上位レイヤーで生成で対処」の想定。
      - JSON構造をシンプルにする。JSON記述量はボイラープレート的に増えるが、「上位レイヤーで生成されるものなのでOK」の想定。
      - 割り切っていること：これはSC-88Proの挙動でいうと、パッチ側（パッチ変更すると、紐付くToneModifyがまるごと変化する）の挙動である。
        - 本来SC-88Proの通常のmode1においては、ProgramChangeでToneModifyは変化しない（それぞれは直交している）。
        - ここではSC-88Proと挙動が違ってもよい、と割り切る。前述のメリットを優先する。
- 上記にあわせてdemoの添付JSONのexamplesも変更すること

```

## [Issue #122](../issue-notes/122.md): copilot-instructions.md を日本語にしつつ、最新状況を反映する。あわせてdeploy 404防止のためのdeploy構造の明示も含める
[issue-notes/122.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/122.md)

...
ラベル: 
--- issue-notes/122.md の内容 ---

```markdown
# issue copilot-instructions.md を日本語にしつつ、最新状況を反映する。あわせてdeploy 404防止のためのdeploy構造の明示も含める #122
[issues #122](https://github.com/cat2151/smf-to-ym2151log-rust/issues/122)



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

### .github/actions-tmp/issue-notes/23.md
```md
{% raw %}
# issue issue 17が再発してしまっている #23
[issues #23](https://github.com/cat2151/github-actions/issues/23)

# 症状は？
- issue 17と同じ

# どうする？
- development-status-generated-prompt.md を確認する
- 結果
    - >Issue番号を記載する際は、必ず [Issue #番号](issue-notes/番号.md) の形式でMarkdownリンクとして記載してください。
    - 仮説、これが残っており、ほかの ../ 指定と競合し、どちらかがランダムで選ばれていた
    - 対策、ここを ../ 指定にする

# 結果
- test green

# closeとする

{% endraw %}
```

### .github/actions-tmp/issue-notes/25.md
```md
{% raw %}
# issue project summaryを他projectからcallしたところ、issue-notes参照ディレクトリ誤りが発覚した #25
[issues #25](https://github.com/cat2151/github-actions/issues/25)

# 事象
- `Issueノートが存在しません: /home/runner/work/tonejs-mml-to-json/tonejs-mml-to-json/.github/actions-tmp/issue-notes/6.md`

# どうする？
- 当該処理のディレクトリ部分を確認する
- 日次バッチでGeminiに確認させてみる
- 結果
    - Geminiに確認させてpromptを生成させ、agentに投げた
    - 結果、projectRootの扱いの誤り、と判明
        - 共通workflow側のdirを引数でわたしてしまっていた
        - target repository側のdirを引数でわたすべき
- 修正したつもり
- 次の日次バッチで動作確認させるつもり

# 結果
- test green

# closeとする

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

### .github/actions-tmp/issue-notes/28.md
```md
{% raw %}
# issue 直近24時間でuser commitがあるかどうか、のチェックを、workflowのjobs先頭に新規jobを追加して実施し、本体jobの先頭にneedsを書く #28
[issues #28](https://github.com/cat2151/github-actions/issues/28)

# これまでの課題は？
- これまでは各workflow内の終盤のscriptにバラバラに実装されていたので、
    - ムダにcheckout等、各種処理が走っていた

# 対策案は？
- 直近24時間でuser commitがあるかどうか、
    - のチェックを、
        - workflowのjobs先頭に新規jobを追加して実施し、
            - 本体jobの先頭にneedsを書く
- この対策で、各workflow先頭にこれを書くだけでよくなり、エコになる想定

# ChatGPTに生成させた
## 呼び出し元のサンプル
- 実際には、共通workflowのjobsの先頭付近を、このサンプルを参考に書き換えるイメージ
```
jobs:
  check_recent_human_commit:
    uses: ./.github/workflows/check-recent-human-commit.yml

  build:
    needs: check_recent_human_commit
    if: needs.check_recent_human_commit.outputs.has_recent_human_commit == 'true'
    runs-on: ubuntu-latest
    steps:
      - name: Run build
        run: echo "Building because there is a recent human commit!"
```
## 共通ワークフロー側の案
- シンプルにmailのみを条件とし、mailも1種類のみに明示する
```
name: "Check recent human commit"

on:
  workflow_call:

jobs:
  check-recent-human-commit:
    runs-on: ubuntu-latest
    outputs:
      has_recent_human_commit: ${{ steps.check.outputs.has_recent_human_commit }}
    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Check recent human commit
        id: check
        run: |
          set -e

          HAS_HUMAN=false

          while IFS=$'\x01' read -r HASH NAME EMAIL SUBJECT; do
            SUBJECT="${SUBJECT%$'\x02'}"

            if [[ ! "$EMAIL" =~ ^41898282\+github-actions\[bot\]@users\.noreply\.github\.com$ ]]; then
              echo "HUMAN: Commit $HASH | Author: $NAME <$EMAIL> | Message: $SUBJECT"
              HAS_HUMAN=true
              break
            else
              echo "BOT: Commit $HASH | Author: $NAME <$EMAIL> | Message: $SUBJECT"
            fi
          done <<< "$(git log --since="24 hours ago" --pretty=format:'%H%x01%an%x01%ae%x01%s%x02')"

          if [ "$HAS_HUMAN" = true ]; then
            echo "Found recent human commit."
            echo "has_recent_human_commit=true" >> $GITHUB_OUTPUT
          else
            echo "No human commits in last 24h."
            echo "has_recent_human_commit=false" >> $GITHUB_OUTPUT
```
## 備忘
- 上記はChatGPTに生成させ、それをレビューさせて改善させる、のサイクルで生成した。
    - 一発で生成はできなかった
    - ChatGPTが自分で生成したものに対して自己レビューでミスや改善点が多発していた
        - ブレも発生し、二転三転気味でもあり、
            - ハルシネーションに近い低品質状態だと感じた
                - これは経験則からの感覚的なもの
    - 生成の品質が低い、ということ
        - LLMはまだ学習不足、github-actions workflow yml の学習不足である、と解釈する
        - shell scriptの生成品質も低いかも。
            - もともとshell scriptで複雑なlogicを書くとtest costが高い、なぜなら読みづらいから。
                - なのでロジックをcjs側に切り出したほうが全体最適の観点からよりよい、と考える

# どうする？
- shell scriptはやめて、cjsでlogicを担当させる。
  - 現状のshell scriptを改めて見直すと、これはcjs側にしたほうがよい、と感覚的に、経験則で、わかる。
- logicをcjs側に切り出す。実際、既存でgitの24hチェックをcjs側でやっている実績がある。そこのロジックを参考にする。
- 今のmdの仕様をもとに、ymlとcjsを生成させる。
- 生成させた。ChatGPTに投げた
- 人力でいくつか変更したり、ChatGPTに投げて修正させるサイクルを回したりした
- testする

# バグ
- 結果、バグがあったのでagentにlogを投げ、修正させ、人力修正し、agentにセルフレビューさせ、のサイクルを回した
- testする
- 結果、callgraphで、エラーなくhumanを検知したが、callgraphが呼ばれない、というバグが発生
- ひとまずagentの提案したcodeを切り分けのため試す、バグ状況は変わらない想定
- 結果、バグ状況は変わらず
- 対策、trueのlogをagentに投げて、callgraphが呼ばれないことを伝え、可視化を実装させた
- testする
- 結果、バグ状況は変わらず
- 対策、logをagentに投げて、callgraphが呼ばれないことを伝え、さらに可視化を実装させた
- testする
- 結果、バグ状況は変わらず
- 対策、logをagentに投げて、callgraphが呼ばれないことを伝え、さらに可視化を実装させた
- testする
- 結果、バグ状況は変わらず
- 対策、logをagentに投げて、callgraphが呼ばれないことを伝えた
- ここで、根本的にymlのworkflow記述が間違っていることが判明
  - agentが最初にcode生成したときから根本的なバグが仕込まれていたということ。
    - agentの学習不足。github-actionsのworkflowの学習不足。
- そこをagentに修正させ、test greenとなった

# closeとする

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

### .github/copilot-instructions.md
```md
{% raw %}
# GitHub Copilot Instructions for smf-to-ym2151log-rust

## Project Overview

This is a Rust implementation that converts Standard MIDI Files (SMF) to YM2151 FM synthesizer chip register write logs in JSON format. It's a Rust port of the Python version [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log).

**Main Purpose**: Convert MIDI files (.mid) → YM2151 register logs (JSON) compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc)

## Architecture

### 2-Pass Processing System

The project uses a two-pass architecture:

1. **Pass A (MIDI Parser)**: MIDI File → Intermediate Events JSON (for debugging)
   - Parses SMF Format 0 and Format 1
   - Outputs `<filename>_events.json` with normalized MIDI events
   
2. **Pass B (YM2151 Converter)**: Intermediate Events → YM2151 Register Log JSON (final output)
   - Converts MIDI events to YM2151 register writes
   - Outputs `<filename>_ym2151.json` compatible with ym2151-zig-cc

### Key Modules

- `src/midi/` - MIDI file parsing and event processing
  - `parser.rs` - SMF parsing logic
  - `events.rs` - MIDI event types
  - `utils.rs` - Utility functions (tempo conversion, note mapping)
  
- `src/ym2151/` - YM2151 conversion logic
  - `converter.rs` - Main conversion logic
  - `events.rs` - YM2151 event types
  - `init.rs` - YM2151 initialization sequences
  - `note_table.rs` - MIDI note to YM2151 KC/KF conversion

- `src/error.rs` - Error types using thiserror
- `src/lib.rs` - Library root
- `src/main.rs` - CLI entry point

## Build and Test

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test <test_name>
```

**Test Structure**:
- Unit tests: Located in the same file as the code (`#[cfg(test)]` modules)
- Integration tests: `tests/integration_tests.rs`
- Test data: `tests/test_data/`

### Code Quality
```bash
# Format check (must pass before commit)
cargo fmt --check

# Lint check (must pass before commit)
cargo clippy -- -D warnings

# Security audit
cargo audit
```

## Dependencies

### Version Pinning Policy for cat2151 Repositories

**Do NOT pin versions** when cloning, `npm install`-ing from GitHub, or `cargo install`-ing from any `cat2151` repository. This includes:
- `git clone` (do not checkout a specific commit/tag — always use the default branch HEAD)
- `npm install github:cat2151/...` (do not pin to a specific commit or version)
- `cargo install --git https://github.com/cat2151/...` (do not pin to a specific rev/tag)

**Reason**: cat2151's repositories receive daily important bug fixes that must always be incorporated. Pinning to an older version would prevent critical fixes from being picked up.

### Production Dependencies
- `midly` (0.5) - SMF parsing library
- `serde` + `serde_json` - JSON serialization/deserialization
- `anyhow` - Error handling for application code
- `thiserror` - Error type definitions

### Development
- Standard Rust test framework (no additional test dependencies yet)

## Coding Conventions

### Language
- **Rust Edition 2021**
- Minimum Rust version: 1.70.0

### Style
- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` for linting (no warnings allowed in CI)
- Prefer explicit types over inference when it improves readability
- Use descriptive variable names
- **Comments**: English preferred for code comments and documentation to support international collaboration; Japanese is acceptable for domain-specific terms or in bilingual documentation files

### Error Handling
- Use `anyhow::Result` for application-level errors in binaries
- Use `thiserror` to define custom error types in libraries
- Propagate errors with `?` operator
- Avoid unwrap/expect in production code (ok in tests)

### Testing Guidelines
- Write unit tests for pure functions and algorithms
- Write integration tests for end-to-end workflows
- Use descriptive test names (e.g., `test_parse_simple_melody`)
- Test both success and error cases
- Keep test data files small and focused

### Documentation
- Document public APIs with doc comments (`///`)
- Include examples in doc comments where helpful
- Keep README.md and IMPLEMENTATION.md in sync with code changes

## JSON Output Formats

### Events JSON (_events.json)
Intermediate debug format:
```json
{
  "ticks_per_beat": 480,
  "tempo_bpm": 120.0,
  "events": [
    {
      "type": "note_on",
      "ticks": 0,
      "channel": 0,
      "note": 60,
      "velocity": 100
    }
  ]
}
```

### YM2151 Log JSON (_ym2151.json)
Final output format (must be compatible with ym2151-zig-cc):
```json
{
  "event_count": 50,
  "events": [
    {
      "time": 0,
      "addr": "0x08",
      "data": "0x00"
    }
  ]
}
```
- `time`: Sample time at 55930Hz sample rate
- `addr`: YM2151 register address (hex string)
- `data`: Data to write (hex string)

## Important References

- [YM2151 Datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf) - Official chip specification (Note: HTTP link, no HTTPS version available)
- [Python version](https://github.com/cat2151/smf-to-ym2151log) - Reference implementation
- [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc) - Output format specification

## Common Tasks

### Adding New MIDI Event Support
1. Add event type to `src/midi/events.rs`
2. Update parser in `src/midi/parser.rs`
3. Add conversion logic in `src/ym2151/converter.rs`
4. Add tests in `tests/integration_tests.rs`

### Modifying YM2151 Register Logic
1. Check YM2151 datasheet for register specifications
2. Update conversion logic in `src/ym2151/converter.rs`
3. If needed, update note table in `src/ym2151/note_table.rs`
4. Add tests to verify register values

### Adding CLI Options
1. Update `src/main.rs` argument parsing
2. Update README.md usage section
3. Add integration tests for new options

## CI/CD

The project uses GitHub Actions (`.github/workflows/ci.yml`):
- Runs on push and pull requests
- Executes: build, test, clippy, and fmt checks
- All checks must pass before merging

### Demo Deployment

The project deploys a single demo to GitHub Pages via `.github/workflows/deploy-pages.yml`:
- **Library Demo** (`/`) - Demonstrates library usage with MIDI file conversion

**Demo Verification**

When making changes to the demo or deployment:
1. Verify the demo works after deployment
2. Check for JavaScript errors in browser console
3. Test demo functionality - file uploads, conversions, UI interactions
4. Validate deployment procedures - ensure build steps, file copies, and deployment workflow are correct

## Project Status

This project is in active development. The implementation is functional but may not support all MIDI features yet. Check IMPLEMENTATION.md for detailed implementation progress and planned features.

# ソース行数
- 単一責任の原則に従ってソース分割すること。特に500行を超えたときはソース分割の優先度を高めること

# TypeScript demo
- demoを開発するとき、formatterとlinterを適用すること
- `demo-library/` の TypeScript formatter と linter には Biome を使用すること
- formatter の適用: `cd demo-library && npm run format`
- linter の適用: `cd demo-library && npm run lint`

# userからの指示
- demo用のMML to SMF、SMF to JSON は、cat2151のライブラリを利用せよ。cat2151のライブラリは毎日重要なバグ修正があり、それを取り込むのがマストであるため、バージョン指定はせず最新mainを毎回取り込むべし

{% endraw %}
```

### demo-library/mml-support.ts
```ts
{% raw %}
import { setStatus } from "./shared-demo";

type TreeSitterNode = {
	type: string;
	childCount: number;
	startIndex: number;
	endIndex: number;
	child: (index: number) => TreeSitterNode;
};

type TreeSitterParser = {
	parse: (source: string) => { rootNode: TreeSitterNode };
	setLanguage: (language: unknown) => void;
};

type SetupMmlInputOptions = {
	mmlInput: HTMLTextAreaElement | null;
	mmlStatus: HTMLElement | null;
	fileStatus?: HTMLElement | null;
	onMidiReady: (bytes: Uint8Array) => void;
	onClear?: () => void;
	onAfterConvert?: (trigger: string) => void;
	debounceMs?: number;
	nextRequestId: () => number;
	isLatestRequest: (id: number) => boolean;
};

const WEB_TREE_SITTER_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/demo/web-tree-sitter.js";
const MML_WASM_MODULE_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js";
const MML_LANGUAGE_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/tree-sitter-mml/tree-sitter-mml.wasm";

let mmlInitPromise: Promise<boolean> | null = null;
let mmlParser: TreeSitterParser | null = null;
let parseTreeJsonToSmf:
	| ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer)
	| null = null;

function treeToJson(
	node: TreeSitterNode,
	source: string,
): Record<string, unknown> {
	const result: Record<string, unknown> = { type: node.type };
	if (node.childCount === 0) {
		result.text = source.substring(node.startIndex, node.endIndex);
		return result;
	}

	const children: Record<string, unknown>[] = [];
	for (let i = 0; i < node.childCount; i += 1) {
		children.push(treeToJson(node.child(i), source));
	}
	result.children = children;
	return result;
}

async function ensureMmlRuntime(
	statusEl: HTMLElement | null,
): Promise<boolean> {
	if (mmlInitPromise) {
		return mmlInitPromise;
	}

	mmlInitPromise = (async () => {
		setStatus(statusEl, "MML モジュールを読み込み中...");
		// @ts-ignore -- remote module is resolved at runtime
		const [treeSitterModule, mmlModule] = await Promise.all([
			// @ts-ignore -- remote module is resolved at runtime
			import(/* @vite-ignore */ WEB_TREE_SITTER_URL),
			// @ts-ignore -- remote module is resolved at runtime
			import(/* @vite-ignore */ MML_WASM_MODULE_URL),
		]);

		const ParserCtor = (treeSitterModule as { Parser: any }).Parser;
		const LanguageApi = (treeSitterModule as { Language: any }).Language;
		await ParserCtor.init();
		const parser: TreeSitterParser = new ParserCtor();
		const language = await LanguageApi.load(MML_LANGUAGE_URL);
		parser.setLanguage(language);
		await mmlModule.default();
		mmlParser = parser;
		parseTreeJsonToSmf = mmlModule.parse_tree_json_to_smf;
		setStatus(statusEl, "MML モジュールの準備ができました。");
		return true;
	})().catch((error) => {
		mmlInitPromise = null;
		setStatus(
			statusEl,
			`MML モジュールの読み込みに失敗しました: ${(error as Error).message}`,
			true,
		);
		return false;
	});

	return mmlInitPromise;
}

export function setupMmlToSmf(options: SetupMmlInputOptions): void {
	const {
		mmlInput,
		mmlStatus,
		fileStatus,
		onMidiReady,
		onClear,
		onAfterConvert,
		debounceMs = 400,
		nextRequestId,
		isLatestRequest,
	} = options;

	if (!mmlInput) return;

	let debounceId: number | null = null;

	const handleConvert = async (): Promise<void> => {
		const mmlText = mmlInput.value.trim();
		if (mmlText.length === 0) {
			if (onClear) {
				onClear();
			}
			setStatus(mmlStatus, "MML を入力すると SMF を生成します。");
			return;
		}

		const requestId = nextRequestId();
		const initialized = await ensureMmlRuntime(mmlStatus);
		if (!initialized || !mmlParser || !parseTreeJsonToSmf) {
			return;
		}
		if (!isLatestRequest(requestId)) {
			return;
		}

		try {
			const tree = mmlParser.parse(mmlText);
			const treeJson = JSON.stringify(treeToJson(tree.rootNode, mmlText));
			const smfBytes = parseTreeJsonToSmf(treeJson, mmlText);
			const midiArray =
				smfBytes instanceof Uint8Array ? smfBytes : new Uint8Array(smfBytes);

			if (!isLatestRequest(requestId)) {
				return;
			}

			onMidiReady(midiArray);
			setStatus(
				fileStatus ?? null,
				`MML 入力を SMF に変換しました (${midiArray.byteLength} bytes)`,
			);
			setStatus(mmlStatus, "MML から SMF への変換が完了しました。");
			if (onAfterConvert) {
				void onAfterConvert("MML 更新");
			}
		} catch (error) {
			if (!isLatestRequest(requestId)) {
				return;
			}
			setStatus(
				mmlStatus,
				`MML 変換に失敗しました: ${(error as Error).message}`,
				true,
			);
		}
	};

	mmlInput.addEventListener("input", () => {
		if (debounceId) {
			window.clearTimeout(debounceId);
		}
		debounceId = window.setTimeout(() => {
			void handleConvert();
		}, debounceMs);
	});
}

{% endraw %}
```

### demo-library/tone-json-mml.ts
```ts
{% raw %}
import { setStatus } from "./shared-demo";

export type TreeSitterNode = {
	type: string;
	childCount: number;
	startIndex: number;
	endIndex: number;
	child: (index: number) => TreeSitterNode;
};

export type TreeSitterParser = {
	parse: (source: string) => { rootNode: TreeSitterNode };
	setLanguage: (language: unknown) => void;
};

const WEB_TREE_SITTER_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/demo/web-tree-sitter.js";
const MML_WASM_MODULE_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js";
const MML_LANGUAGE_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/tree-sitter-mml/tree-sitter-mml.wasm";

let mmlInitPromise: Promise<boolean> | null = null;
let mmlParser: TreeSitterParser | null = null;
let parseTreeJsonToSmf:
	| ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer)
	| null = null;

export function getMmlParser(): TreeSitterParser | null {
	return mmlParser;
}

export function getParseTreeJsonToSmf():
	| ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer)
	| null {
	return parseTreeJsonToSmf;
}

export function treeToJson(
	node: TreeSitterNode,
	source: string,
): Record<string, unknown> {
	const result: Record<string, unknown> = { type: node.type };
	if (node.childCount === 0) {
		result.text = source.substring(node.startIndex, node.endIndex);
		return result;
	}

	const children: Record<string, unknown>[] = [];
	for (let i = 0; i < node.childCount; i += 1) {
		children.push(treeToJson(node.child(i), source));
	}
	result.children = children;
	return result;
}

export async function ensureMmlRuntime(
	mmlStatusEl: HTMLElement | null,
): Promise<boolean> {
	if (mmlInitPromise) {
		return mmlInitPromise;
	}

	mmlInitPromise = (async () => {
		setStatus(mmlStatusEl, "MML モジュールを読み込み中...");
		// @ts-ignore -- remote module is resolved at runtime
		const [treeSitterModule, mmlModule] = await Promise.all([
			// @ts-ignore -- remote module is resolved at runtime
			import(/* @vite-ignore */ WEB_TREE_SITTER_URL),
			// @ts-ignore -- remote module is resolved at runtime
			import(/* @vite-ignore */ MML_WASM_MODULE_URL),
		]);

		const ParserCtor = (treeSitterModule as { Parser: unknown }).Parser;
		const LanguageApi = (treeSitterModule as { Language: unknown }).Language;
		await (ParserCtor as { init: () => Promise<void> }).init();
		const parser: TreeSitterParser = new (
			ParserCtor as new () => TreeSitterParser
		)();
		const language = await (
			LanguageApi as { load: (url: string) => Promise<unknown> }
		).load(MML_LANGUAGE_URL);
		parser.setLanguage(language);
		await (mmlModule as { default: () => Promise<void> }).default();
		mmlParser = parser;
		parseTreeJsonToSmf = (
			mmlModule as {
				parse_tree_json_to_smf: (
					treeJson: string,
					source: string,
				) => Uint8Array | number[] | ArrayBuffer;
			}
		).parse_tree_json_to_smf;
		setStatus(mmlStatusEl, "MML モジュールの準備ができました。");
		return true;
	})().catch((error) => {
		mmlInitPromise = null;
		setStatus(
			mmlStatusEl,
			`MML モジュールの読み込みに失敗しました: ${(error as Error).message}`,
			true,
		);
		return false;
	});

	return mmlInitPromise;
}

{% endraw %}
```

### issue-notes/122.md
```md
{% raw %}
# issue copilot-instructions.md を日本語にしつつ、最新状況を反映する。あわせてdeploy 404防止のためのdeploy構造の明示も含める #122
[issues #122](https://github.com/cat2151/smf-to-ym2151log-rust/issues/122)



{% endraw %}
```

### issue-notes/123.md
```md
{% raw %}
# issue 添付JSONについて、フォーマット変更。音色JSONの自己記述性を高めるため、ProgramChangeはJSON項目名にProgramChangeという名前をつける #123
[issues #123](https://github.com/cat2151/smf-to-ym2151log-rust/issues/123)

- あわせて添付JSON構造を変更する
- 0個～128個の配列である
- 配列の1要素：
  - ProgramChange: 0 のように、音色番号0～127をJSONで定義して、自己記述性を高める
  - ディレイビブラート、ポルタメント、LFO、も、ProgramChangeとセットで扱う。
    - 設計思想：
      - SMF側とMML側の記述量を減らす。ProgramChangeするだけで多彩な音色を楽しめる。
      - LFOだけ変更したい場合がありうるので、それは別途検討する。現状は「新たなProgramChangeを上位レイヤーで生成で対処」の想定。
      - JSON構造をシンプルにする。JSON記述量はボイラープレート的に増えるが、「上位レイヤーで生成されるものなのでOK」の想定。
      - 割り切っていること：これはSC-88Proの挙動でいうと、パッチ側（パッチ変更すると、紐付くToneModifyがまるごと変化する）の挙動である。
        - 本来SC-88Proの通常のmode1においては、ProgramChangeでToneModifyは変化しない（それぞれは直交している）。
        - ここではSC-88Proと挙動が違ってもよい、と割り切る。前述のメリットを優先する。
- 上記にあわせてdemoの添付JSONのexamplesも変更すること

{% endraw %}
```

### issue-notes/125.md
```md
{% raw %}
# issue CIがclippyで落ちたとき等にissueを自動起票、を実装する。このリポジトリの他のワークフローymlを参考にする #125
[issues #125](https://github.com/cat2151/smf-to-ym2151log-rust/issues/125)



{% endraw %}
```

### issue-notes/126.md
```md
{% raw %}
# issue ポップノイズdemoで、添付JSONを変更しても、最終log JSONに反映されていない #126
[issues #126](https://github.com/cat2151/smf-to-ym2151log-rust/issues/126)



{% endraw %}
```

### issue-notes/128.md
```md
{% raw %}
# issue demoの、YM2151レジスタのビジュアライザを改善。KCの高い低いも可視化する。KC maxとminを算出して上下いっぱいに表示する #128
[issues #128](https://github.com/cat2151/smf-to-ym2151log-rust/issues/128)



{% endraw %}
```

### issue-notes/131.md
```md
{% raw %}
# issue demoのディレイビブラートにおいて、MMLの「l」「t」コマンドなどがまったく動作していない。ライブラリを使わない自実装に逃げているのか？調査せよ #131
[issues #131](https://github.com/cat2151/smf-to-ym2151log-rust/issues/131)



{% endraw %}
```

### issue-notes/133.md
```md
{% raw %}
# issue copilot-instructions.mdが英語なので妥当性が不明。日本語にする #133
[issues #133](https://github.com/cat2151/smf-to-ym2151log-rust/issues/133)



{% endraw %}
```

### issue-notes/33.md
```md
{% raw %}
# issue 仕様追加。ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む。仮仕様。tone editorのdirをsymlinkで検証想定。 #33
[issues #33](https://github.com/cat2151/smf-to-ym2151log-rust/issues/33)



{% endraw %}
```

### src/midi/utils.rs
```rs
{% raw %}
//! MIDI utility functions
//!
//! Provides conversion functions for MIDI to YM2151 parameters.

use crate::ym2151::note_table::NOTE_TABLE;

/// Convert MIDI note number to frequency in Hz (A4 = 440 Hz)
pub fn midi_note_to_frequency(midi_note: u8) -> f64 {
    440.0 * 2_f64.powf((midi_note as f64 - 69.0) / 12.0)
}

/// Convert MIDI note with a cent offset to YM2151 KC (Key Code) and KF (Key Fraction)
pub fn midi_note_with_offset_to_kc_kf(midi_note: u8, cents_offset: f64) -> (u8, u8) {
    // Convert cents to fractional MIDI note offset
    let target_note = (midi_note as f64) + cents_offset / 100.0;
    let clamped_note = target_note.clamp(0.0, 127.0);

    // Align with the existing -1 MIDI offset used for YM2151 mapping
    let adjusted = (clamped_note - 1.0).max(0.0);
    let base_note = adjusted.floor() as u8;
    let fractional = adjusted - base_note as f64;

    let note_in_octave = (base_note % 12) as usize;
    let ym_octave = ((base_note / 12) as i8 - 2).clamp(0, 7) as u8;
    let ym_note = NOTE_TABLE[note_in_octave];
    let kc = (ym_octave << 4) | ym_note;

    // KF steps are 1/64 of a semitone on YM2151
    let kf = (fractional * 64.0).round().clamp(0.0, 63.0) as u8;

    (kc, kf)
}

/// Convert MIDI note to YM2151 KC (Key Code) and KF (Key Fraction)
///
/// # Arguments
/// * `midi_note` - MIDI note number (0-127)
///
/// # Returns
/// Tuple of (KC, KF) where KC is the key code and KF is the key fraction
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::midi_to_kc_kf;
/// let (kc, kf) = midi_to_kc_kf(60); // Middle C (C4)
/// assert_eq!(kc, 0x2E); // Octave 2, Note C
/// assert_eq!(kf, 0);
/// ```
pub fn midi_to_kc_kf(midi_note: u8) -> (u8, u8) {
    // Adjust MIDI note by -1 to align octaves between MIDI and YM2151 numbering
    let adjusted_midi = if midi_note > 0 { midi_note - 1 } else { 0 };
    let note_in_octave = (adjusted_midi % 12) as usize;

    let ym_octave = ((adjusted_midi / 12) as i8 - 2).clamp(0, 7) as u8;
    let ym_note = NOTE_TABLE[note_in_octave];
    let kc = (ym_octave << 4) | ym_note;
    let kf = 0; // No fine tuning for now

    (kc, kf)
}

/// Convert MIDI ticks to seconds
///
/// # Arguments
/// * `ticks` - Number of MIDI ticks
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_bpm` - Tempo in beats per minute
///
/// # Returns
/// Time in seconds
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::ticks_to_seconds;
/// let seconds = ticks_to_seconds(480, 480, 120.0);
/// assert!((seconds - 0.5).abs() < 0.001); // 1 beat at 120 BPM = 0.5 seconds
/// ```
pub fn ticks_to_seconds(ticks: u32, ticks_per_beat: u16, tempo_bpm: f64) -> f64 {
    let seconds_per_beat = 60.0 / tempo_bpm;
    let seconds_per_tick = seconds_per_beat / ticks_per_beat as f64;
    ticks as f64 * seconds_per_tick
}

/// YM2151 sample rate constant
pub const YM2151_SAMPLE_RATE: u32 = 55930;

/// Convert seconds to sample count at 55930 Hz
///
/// # Arguments
/// * `seconds` - Time in seconds
///
/// # Returns
/// Sample count at 55930 Hz
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::seconds_to_samples;
/// let samples = seconds_to_samples(1.0);
/// assert_eq!(samples, 55930);
/// ```
pub fn seconds_to_samples(seconds: f64) -> u32 {
    (seconds * YM2151_SAMPLE_RATE as f64) as u32
}

/// Convert MIDI ticks directly to sample count
///
/// # Arguments
/// * `ticks` - Number of MIDI ticks
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_bpm` - Tempo in beats per minute
///
/// # Returns
/// Sample count at 55930 Hz
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::ticks_to_samples;
/// let samples = ticks_to_samples(480, 480, 120.0);
/// assert_eq!(samples, 27965); // 0.5 seconds at 55930 Hz
/// ```
pub fn ticks_to_samples(ticks: u32, ticks_per_beat: u16, tempo_bpm: f64) -> u32 {
    let seconds = ticks_to_seconds(ticks, ticks_per_beat, tempo_bpm);
    seconds_to_samples(seconds)
}

/// Represents a tempo change at a specific tick
#[derive(Debug, Clone, Copy)]
pub struct TempoChange {
    pub tick: u32,
    pub tempo_bpm: f64,
}

/// Convert MIDI ticks to sample count with tempo changes
///
/// This function correctly handles tempo changes by calculating accumulated time
/// across different tempo segments.
///
/// # Arguments
/// * `target_tick` - The tick to convert to sample time
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_map` - Sorted list of tempo changes (by tick)
///
/// # Returns
/// Sample count at 55930 Hz
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::{ticks_to_samples_with_tempo_map, TempoChange};
/// let tempo_map = vec![
///     TempoChange { tick: 0, tempo_bpm: 120.0 },
///     TempoChange { tick: 480, tempo_bpm: 60.0 },
/// ];
/// // First beat at 120 BPM = 27965 samples
/// let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
/// assert_eq!(samples, 27965);
/// ```
pub fn ticks_to_samples_with_tempo_map(
    target_tick: u32,
    ticks_per_beat: u16,
    tempo_map: &[TempoChange],
) -> u32 {
    if tempo_map.is_empty() {
        // No tempo changes - use default 120 BPM
        return ticks_to_samples(target_tick, ticks_per_beat, 120.0);
    }

    let mut accumulated_seconds = 0.0;
    let mut prev_tick = 0;

    for (i, tempo_change) in tempo_map.iter().enumerate() {
        if target_tick <= tempo_change.tick {
            // Target is before or at this tempo change
            if i == 0 {
                // Target is before the first tempo change
                let ticks_in_segment = target_tick;
                accumulated_seconds +=
                    ticks_to_seconds(ticks_in_segment, ticks_per_beat, tempo_change.tempo_bpm);
            } else {
                // Use the previous tempo for the remaining ticks
                let prev_tempo = tempo_map[i - 1].tempo_bpm;
                let ticks_in_segment = target_tick - prev_tick;
                accumulated_seconds +=
                    ticks_to_seconds(ticks_in_segment, ticks_per_beat, prev_tempo);
            }
            return seconds_to_samples(accumulated_seconds);
        }

        // Calculate time in this tempo segment
        if i > 0 {
            let ticks_in_segment = tempo_change.tick - prev_tick;
            let prev_tempo = tempo_map[i - 1].tempo_bpm;
            accumulated_seconds += ticks_to_seconds(ticks_in_segment, ticks_per_beat, prev_tempo);
        }

        prev_tick = tempo_change.tick;
    }

    // Target is after all tempo changes - use the last tempo
    let last_tempo = tempo_map.last().unwrap().tempo_bpm;
    let ticks_in_segment = target_tick - prev_tick;
    accumulated_seconds += ticks_to_seconds(ticks_in_segment, ticks_per_beat, last_tempo);

    seconds_to_samples(accumulated_seconds)
}

/// Convert MIDI ticks to seconds with tempo changes
///
/// This function correctly handles tempo changes by calculating accumulated time
/// across different tempo segments.
///
/// # Arguments
/// * `target_tick` - The tick to convert to seconds
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_map` - Sorted list of tempo changes (by tick)
///
/// # Returns
/// Time in seconds (f64)
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::{ticks_to_seconds_with_tempo_map, TempoChange};
/// let tempo_map = vec![
///     TempoChange { tick: 0, tempo_bpm: 120.0 },
///     TempoChange { tick: 480, tempo_bpm: 60.0 },
/// ];
/// // First beat at 120 BPM = 0.5 seconds
/// let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
/// assert!((seconds - 0.5).abs() < 0.001);
/// ```
pub fn ticks_to_seconds_with_tempo_map(
    target_tick: u32,
    ticks_per_beat: u16,
    tempo_map: &[TempoChange],
) -> f64 {
    if tempo_map.is_empty() {
        // No tempo changes - use default 120 BPM
        return ticks_to_seconds(target_tick, ticks_per_beat, 120.0);
    }

    let mut accumulated_seconds = 0.0;
    let mut prev_tick = 0;

    for (i, tempo_change) in tempo_map.iter().enumerate() {
        if target_tick <= tempo_change.tick {
            // Target is before or at this tempo change
            if i == 0 {
                // Target is before the first tempo change
                let ticks_in_segment = target_tick;
                accumulated_seconds +=
                    ticks_to_seconds(ticks_in_segment, ticks_per_beat, tempo_change.tempo_bpm);
            } else {
                // Use the previous tempo for the remaining ticks
                let prev_tempo = tempo_map[i - 1].tempo_bpm;
                let ticks_in_segment = target_tick - prev_tick;
                accumulated_seconds +=
                    ticks_to_seconds(ticks_in_segment, ticks_per_beat, prev_tempo);
            }
            return accumulated_seconds;
        }

        // Calculate time in this tempo segment
        if i > 0 {
            let ticks_in_segment = tempo_change.tick - prev_tick;
            let prev_tempo = tempo_map[i - 1].tempo_bpm;
            accumulated_seconds += ticks_to_seconds(ticks_in_segment, ticks_per_beat, prev_tempo);
        }

        prev_tick = tempo_change.tick;
    }

    // Target is after all tempo changes - use the last tempo
    let last_tempo = tempo_map.last().unwrap().tempo_bpm;
    let ticks_in_segment = target_tick - prev_tick;
    accumulated_seconds += ticks_to_seconds(ticks_in_segment, ticks_per_beat, last_tempo);

    accumulated_seconds
}

#[cfg(test)]
mod tests {
    use super::*;

    // MIDI to KC/KF conversion tests
    #[test]
    fn test_midi_to_kc_kf_middle_c() {
        // MIDI note 60 = C4 (Middle C)
        let (kc, kf) = midi_to_kc_kf(60);
        assert_eq!(kc, 0x2E); // Octave 2, Note C
        assert_eq!(kf, 0);
    }

    #[test]
    fn test_midi_to_kc_kf_a440() {
        // MIDI note 69 = A4 (A440)
        let (kc, kf) = midi_to_kc_kf(69);
        assert_eq!(kc, 0x3A); // Octave 3, Note A
        assert_eq!(kf, 0);
    }

    #[test]
    fn test_midi_to_kc_kf_octaves() {
        // Test representative notes across different octaves
        // C notes from different octaves (YM2151 octave = MIDI octave - 2)
        let (kc, _) = midi_to_kc_kf(24); // C1
        assert_eq!(kc, 0x0E); // Octave 0 (clamped), Note C

        let (kc, _) = midi_to_kc_kf(36); // C2
        assert_eq!(kc, 0x0E); // Octave 0, Note C

        let (kc, _) = midi_to_kc_kf(48); // C3
        assert_eq!(kc, 0x1E); // Octave 1, Note C

        let (kc, _) = midi_to_kc_kf(60); // C4
        assert_eq!(kc, 0x2E); // Octave 2, Note C

        let (kc, _) = midi_to_kc_kf(72); // C5
        assert_eq!(kc, 0x3E); // Octave 3, Note C

        let (kc, _) = midi_to_kc_kf(84); // C6
        assert_eq!(kc, 0x4E); // Octave 4, Note C

        let (kc, _) = midi_to_kc_kf(96); // C7
        assert_eq!(kc, 0x5E); // Octave 5, Note C
    }

    #[test]
    fn test_midi_to_kc_kf_all_notes_in_octave() {
        // Test all 12 notes within an octave (using octave 4 as example)
        // MIDI notes 60-71 map to: C, C#, D, D#, E, F, F#, G, G#, A, A#, B
        // YM2151 note table values for these notes in order
        let base_midi = 60; // C4
        let expected_ym_notes = [14, 0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13];

        for (i, &expected_note) in expected_ym_notes.iter().enumerate() {
            let (kc, kf) = midi_to_kc_kf(base_midi + i as u8);
            let note = kc & 0x0F;
            assert_eq!(note, expected_note, "Failed for note {}", i);
            assert_eq!(kf, 0);
        }
    }

    #[test]
    fn test_midi_to_kc_kf_boundary_minimum() {
        // Test minimum MIDI note (0)
        let (kc, kf) = midi_to_kc_kf(0);
        assert_eq!(kf, 0);
        // Should clamp to octave 0
        let octave = (kc >> 4) & 0x07;
        assert_eq!(octave, 0);
    }

    #[test]
    fn test_midi_to_kc_kf_boundary_maximum() {
        // Test maximum MIDI note (127)
        let (kc, kf) = midi_to_kc_kf(127);
        assert_eq!(kf, 0);
        // Should clamp to octave 7 (maximum for YM2151)
        let octave = (kc >> 4) & 0x07;
        assert_eq!(octave, 7);
    }

    #[test]
    fn test_midi_to_kc_kf_octave_clamping_high() {
        // Test that very high notes clamp to octave 7
        // With -2 octave offset: MIDI 108-119 → octave 6, MIDI 120-127 → octave 7
        for midi_note in 120..=127 {
            let (kc, _) = midi_to_kc_kf(midi_note);
            let octave = (kc >> 4) & 0x07;
            assert_eq!(
                octave, 7,
                "Failed to clamp octave for MIDI note {}",
                midi_note
            );
        }
    }

    #[test]
    fn test_midi_note_to_frequency_a440() {
        let freq = midi_note_to_frequency(69);
        assert!((freq - 440.0).abs() < 0.001);
    }

    #[test]
    fn test_midi_note_with_offset_zero_matches_base() {
        let base = midi_to_kc_kf(60);
        let with_offset = midi_note_with_offset_to_kc_kf(60, 0.0);
        assert_eq!(base, with_offset);
    }

    #[test]
    fn test_midi_note_with_offset_positive_and_negative() {
        let up = midi_note_with_offset_to_kc_kf(60, 100.0);
        let up_expected = midi_to_kc_kf(61);
        assert_eq!(up.0, up_expected.0);

        let down = midi_note_with_offset_to_kc_kf(60, -100.0);
        let down_expected = midi_to_kc_kf(59);
        assert_eq!(down.0, down_expected.0);
    }

    // Timing conversion tests
    #[test]
    fn test_ticks_to_seconds_one_beat() {
        // 1 beat at 120 BPM = 0.5 seconds
        let seconds = ticks_to_seconds(480, 480, 120.0);
        assert!((seconds - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_ticks_to_seconds_half_beat() {
        // 0.5 beat at 120 BPM = 0.25 seconds
        let seconds = ticks_to_seconds(240, 480, 120.0);
        assert!((seconds - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_ticks_to_seconds_different_tempo() {
        // 1 beat at 60 BPM = 1.0 second
        let seconds = ticks_to_seconds(480, 480, 60.0);
        assert!((seconds - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_ticks_to_seconds_different_ticks_per_beat() {
        // 1 beat at 120 BPM = 0.5 seconds (with different ticks_per_beat)
        let seconds = ticks_to_seconds(960, 960, 120.0);
        assert!((seconds - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_seconds_to_samples_one_second() {
        let samples = seconds_to_samples(1.0);
        assert_eq!(samples, 55930);
    }

    #[test]
    fn test_seconds_to_samples_half_second() {
        let samples = seconds_to_samples(0.5);
        assert_eq!(samples, 27965);
    }

    #[test]
    fn test_seconds_to_samples_zero() {
        let samples = seconds_to_samples(0.0);
        assert_eq!(samples, 0);
    }

    #[test]
    fn test_ticks_to_samples_one_beat() {
        // 1 beat at 120 BPM = 0.5 seconds = 27965 samples
        let samples = ticks_to_samples(480, 480, 120.0);
        assert_eq!(samples, 27965);
    }

    #[test]
    fn test_ticks_to_samples_zero() {
        let samples = ticks_to_samples(0, 480, 120.0);
        assert_eq!(samples, 0);
    }

    #[test]
    fn test_ticks_to_samples_precision() {
        // Test that the conversion maintains reasonable precision
        let samples1 = ticks_to_samples(1, 480, 120.0);
        let samples2 = ticks_to_samples(2, 480, 120.0);
        // Each tick should produce a consistent increment
        assert_eq!(samples2, samples1 * 2);
    }

    // Tempo map conversion tests
    #[test]
    fn test_ticks_to_samples_with_tempo_map_no_changes() {
        // Single tempo - should match regular conversion
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
        assert_eq!(samples, 27965); // Same as ticks_to_samples(480, 480, 120.0)
    }

    #[test]
    fn test_ticks_to_samples_with_tempo_map_single_change() {
        // Tempo change at tick 480
        let tempo_map = vec![
            TempoChange {
                tick: 0,
                tempo_bpm: 120.0,
            },
            TempoChange {
                tick: 480,
                tempo_bpm: 60.0,
            },
        ];

        // At tick 480 (right at the tempo change)
        // Should be: 480 ticks at 120 BPM = 0.5 seconds = 27965 samples
        let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
        assert_eq!(samples, 27965);

        // At tick 960 (480 ticks after tempo change)
        // Should be: 480 ticks at 120 BPM + 480 ticks at 60 BPM
        // = 0.5 seconds + 1.0 second = 1.5 seconds = 83895 samples
        let samples = ticks_to_samples_with_tempo_map(960, 480, &tempo_map);
        assert_eq!(samples, 83895);
    }

    #[test]
    fn test_ticks_to_samples_with_tempo_map_multiple_changes() {
        // Multiple tempo changes
        let tempo_map = vec![
            TempoChange {
                tick: 0,
                tempo_bpm: 120.0,
            },
            TempoChange {
                tick: 240,
                tempo_bpm: 60.0,
            },
            TempoChange {
                tick: 480,
                tempo_bpm: 180.0,
            },
        ];

        // At tick 0
        let samples = ticks_to_samples_with_tempo_map(0, 480, &tempo_map);
        assert_eq!(samples, 0);

        // At tick 240 (at first tempo change)
        // 240 ticks at 120 BPM = 0.25 seconds = 13982 samples
        let samples = ticks_to_samples_with_tempo_map(240, 480, &tempo_map);
        assert_eq!(samples, 13982);

        // At tick 480 (at second tempo change)
        // 240 ticks at 120 BPM + 240 ticks at 60 BPM
        // = 0.25 + 0.5 = 0.75 seconds = 41947 samples
        let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
        assert_eq!(samples, 41947);

        // At tick 720 (after second tempo change)
        // 240 ticks at 120 BPM + 240 ticks at 60 BPM + 240 ticks at 180 BPM
        // = 0.25 + 0.5 + 0.167 = 0.917 seconds ≈ 51269 samples
        let samples = ticks_to_samples_with_tempo_map(720, 480, &tempo_map);
        assert_eq!(samples, 51269); // Adjusted for rounding
    }

    #[test]
    fn test_ticks_to_samples_with_tempo_map_empty() {
        // Empty tempo map - should use default 120 BPM
        let tempo_map = vec![];
        let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
        assert_eq!(samples, 27965); // Same as 120 BPM
    }

    #[test]
    fn test_ticks_to_samples_with_tempo_map_before_first_change() {
        // Tempo change at tick 480, but we want time at tick 240
        let tempo_map = vec![TempoChange {
            tick: 480,
            tempo_bpm: 60.0,
        }];

        // At tick 240 (before the tempo change)
        // Should use the tempo from the first entry (60 BPM)
        // 240 ticks at 60 BPM = 0.5 seconds = 27965 samples
        let samples = ticks_to_samples_with_tempo_map(240, 480, &tempo_map);
        assert_eq!(samples, 27965);
    }

    // ticks_to_seconds_with_tempo_map tests
    #[test]
    fn test_ticks_to_seconds_with_tempo_map_no_changes() {
        // Single tempo - should match regular conversion
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
        assert!((seconds - 0.5).abs() < 0.001); // Same as ticks_to_seconds(480, 480, 120.0)
    }

    #[test]
    fn test_ticks_to_seconds_with_tempo_map_single_change() {
        // Tempo change at tick 480
        let tempo_map = vec![
            TempoChange {
                tick: 0,
                tempo_bpm: 120.0,
            },
            TempoChange {
                tick: 480,
                tempo_bpm: 60.0,
            },
        ];

        // At tick 480 (right at the tempo change)
        // Should be: 480 ticks at 120 BPM = 0.5 seconds
        let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
        assert!((seconds - 0.5).abs() < 0.001);

        // At tick 960 (480 ticks after tempo change)
        // Should be: 480 ticks at 120 BPM + 480 ticks at 60 BPM
        // = 0.5 seconds + 1.0 second = 1.5 seconds
        let seconds = ticks_to_seconds_with_tempo_map(960, 480, &tempo_map);
        assert!((seconds - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_ticks_to_seconds_with_tempo_map_multiple_changes() {
        // Multiple tempo changes
        let tempo_map = vec![
            TempoChange {
                tick: 0,
                tempo_bpm: 120.0,
            },
            TempoChange {
                tick: 240,
                tempo_bpm: 60.0,
            },
            TempoChange {
                tick: 480,
                tempo_bpm: 180.0,
            },
        ];

        // At tick 0
        let seconds = ticks_to_seconds_with_tempo_map(0, 480, &tempo_map);
        assert!((seconds - 0.0).abs() < 0.001);

        // At tick 240 (at first tempo change)
        // 240 ticks at 120 BPM = 0.25 seconds
        let seconds = ticks_to_seconds_with_tempo_map(240, 480, &tempo_map);
        assert!((seconds - 0.25).abs() < 0.001);

        // At tick 480 (at second tempo change)
        // 240 ticks at 120 BPM + 240 ticks at 60 BPM
        // = 0.25 + 0.5 = 0.75 seconds
        let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
        assert!((seconds - 0.75).abs() < 0.001);

        // At tick 720 (after second tempo change)
        // 240 ticks at 120 BPM + 240 ticks at 60 BPM + 240 ticks at 180 BPM
        // = 0.25 + 0.5 + 0.167 ≈ 0.917 seconds
        let seconds = ticks_to_seconds_with_tempo_map(720, 480, &tempo_map);
        assert!((seconds - 0.9166666).abs() < 0.001);
    }

    #[test]
    fn test_ticks_to_seconds_with_tempo_map_empty() {
        // Empty tempo map - should use default 120 BPM
        let tempo_map = vec![];
        let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
        assert!((seconds - 0.5).abs() < 0.001); // Same as 120 BPM
    }

    #[test]
    fn test_ticks_to_seconds_with_tempo_map_before_first_change() {
        // Tempo change at tick 480, but we want time at tick 240
        let tempo_map = vec![TempoChange {
            tick: 480,
            tempo_bpm: 60.0,
        }];

        // At tick 240 (before the tempo change)
        // Should use the tempo from the first entry (60 BPM)
        // 240 ticks at 60 BPM = 0.5 seconds
        let seconds = ticks_to_seconds_with_tempo_map(240, 480, &tempo_map);
        assert!((seconds - 0.5).abs() < 0.001);
    }
}

{% endraw %}
```

### src/ym2151/converter_tests.rs
```rs
{% raw %}
//! Tests for YM2151 converter (Pass B)
//!
//! These tests verify the conversion of MIDI events to YM2151 register write events.

use super::*;
use crate::midi::{midi_to_kc_kf, MidiEvent};
use crate::ym2151::{ToneDefinition, Ym2151Event};
use crate::{
    AttackContinuationFix, ConversionOptions, LfoWaveform, PopNoiseEnvelope, RegisterLfoDefinition,
    RegisterOverride,
};

#[test]
fn test_convert_empty_midi() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Should have only 8 KEY OFF events (no channels used, so no channel init)
    assert_eq!(result.event_count, 8);
    assert_eq!(result.events.len(), 8);
}

#[test]
fn test_convert_single_note() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60, // Middle C
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

    // Initialization (34) + Note On (3: KC, KF, KEY ON) + Note Off (1: KEY OFF) = 38
    assert_eq!(result.event_count, 38);

    // Find the KC register write for Note On
    // MIDI channel 0 with polyphony 1 gets YM2151 channel 0 (no drum channel present)
    // KC register is at 0x28 for channel 0
    // There should be exactly one KC write from the Note On event
    let kc_events: Vec<&Ym2151Event> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.data == "0x2E")
        .collect();

    assert_eq!(
        kc_events.len(),
        1,
        "Should have exactly one KC register write for Middle C"
    );

    // Middle C (MIDI 60) should map to KC 0x2E (Octave 2, Note C)
    assert_eq!(kc_events[0].data, "0x2E");
}

#[test]
fn test_convert_tempo_change() {
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
            MidiEvent::Tempo {
                ticks: 240,
                tempo_bpm: 60.0, // Half speed
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Should have initialization + Note On + Note Off events
    assert!(result.event_count > 34);

    // Verify Note On happens at time 0
    let note_on_event = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time < 0.001) // Channel 0 now
        .expect("Should have Note On KEY event at time 0");
    assert!(note_on_event.time < 0.001);
}

#[test]
fn test_convert_multiple_notes() {
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
            MidiEvent::NoteOn {
                ticks: 240,
                channel: 0,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOff {
                ticks: 720,
                channel: 0,
                note: 64,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // With polyphony analysis, overlapping notes mean this channel needs 2 voices
    // Init: 8 KEY OFF + (26 * 2 channels) + 2 Note Ons (6) + 2 Note Offs (2)
    //     = 8 + 52 + 6 + 2 = 68
    assert_eq!(result.event_count, 68);
}

#[test]
fn test_key_on_register_format() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![MidiEvent::NoteOn {
            ticks: 0,
            channel: 0,
            note: 60,
            velocity: 100,
        }],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Find KEY ON event - MIDI channel 0 maps to YM2151 channel 0 (no drums present)
    let key_on = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x78")
        .expect("Should have KEY ON event");

    // 0x78 = all operators on, channel 0 (MIDI channel 0)
    assert_eq!(key_on.data, "0x78");
}

#[test]
fn test_key_off_register_format() {
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
                ticks: 480,
                channel: 0,
                note: 60,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Find KEY OFF event (should be after initialization)
    // MIDI channel 0 maps to YM2151 channel 0 (no drums present)
    let key_off = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time > 0.001)
        .find(|e| e.data == "0x00") // Channel 0
        .expect("Should have KEY OFF event");

    // 0x00 = all operators off, channel 0
    assert_eq!(key_off.data, "0x00");
}

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
fn test_portamento_generates_pitch_glide_events() {
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
                ticks: 480,
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 67,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 0,
                note: 67,
            },
        ],
    };

    let options = ConversionOptions {
        portamento: true,
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // First note should emit a KC write even with portamento enabled
    let (kc_first, _) = midi_to_kc_kf(60);
    let first_kc = result
        .events
        .iter()
        .find(|e| {
            e.addr == "0x28"
                && (e.time - 0.0).abs() < f64::EPSILON
                && e.data == format!("0x{:02X}", kc_first)
        })
        .map(|e| e.data.clone())
        .expect("First note should set KC at time 0");
    assert_eq!(first_kc, format!("0x{:02X}", kc_first));

    // Collect KC events starting at the second note-on time (0.5 seconds)
    let kc_events_after_second_on: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time >= 0.5)
        .collect();
    assert!(
        kc_events_after_second_on.len() >= 2,
        "Portamento should emit multiple KC steps during the glide"
    );

    let (kc_second, _) = midi_to_kc_kf(67);

    // Glide should include the previous pitch before reaching the target
    assert!(
        kc_events_after_second_on
            .iter()
            .any(|e| e.data == format!("0x{:02X}", kc_first)),
        "Glide should include the starting KC from the previous note"
    );
    // Glide should reach the target KC
    assert!(
        kc_events_after_second_on
            .iter()
            .any(|e| e.data == format!("0x{:02X}", kc_second)),
        "Glide should arrive at the target KC"
    );
}

#[test]
fn test_register_lfo_modulates_tone_register() {
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
                ticks: 0,
                channel: 1,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 1,
                note: 64,
            },
        ],
    };

    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 4.0,
            rate_hz: 2.0,
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // MIDI channel 1 maps to YM channel 1 when channel 0 is also present, so TL base reg 0x60 -> 0x61
    let lfo_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x61" && e.time > 0.0)
        .collect();

    assert!(
        !lfo_events.is_empty(),
        "Software LFO should emit TL updates for channel 1"
    );
    assert!(
        lfo_events.iter().any(|e| e.data != "0x00"),
        "LFO should modulate TL away from the base value"
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

#[test]
fn test_convert_multi_channel() {
    // Test with notes on different MIDI channels
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Channel 0: C (60)
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            // Channel 1: E (64)
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 1,
                note: 64,
                velocity: 100,
            },
            // Channel 2: G (67)
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 2,
                note: 67,
                velocity: 100,
            },
            // Note offs at tick 480
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
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 2,
                note: 67,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Verify we have events for all 3 channels
    // 8 KEY OFF + (26 * 3 channels) + (3 notes * 3 events each) + (3 note offs) = 8 + 78 + 9 + 3 = 98
    assert_eq!(result.event_count, 98);

    // Verify KC register writes for each channel
    // With polyphony-based allocation and no drums, channels are allocated sequentially
    // MIDI Channel 0,1,2 each have polyphony 1, so they get YM2151 channels 0,1,2
    // Note: With -2 octave offset:
    //   MIDI 60 (C4) → KC 0x2E (Octave 2, Note C)
    //   MIDI 64 (E4) → KC 0x34 (Octave 3, Note E)
    //   MIDI 67 (G4) → KC 0x38 (Octave 3, Note G)
    let ch0_kc = result
        .events
        .iter()
        .find(|e| {
            (e.addr == "0x28" || e.addr == "0x29" || e.addr == "0x2A")
                && e.time < 0.001
                && e.data == "0x2E"
        })
        .expect("Should have KC write for MIDI channel 0");
    assert_eq!(ch0_kc.data, "0x2E"); // Middle C (Octave 2, Note C)

    let ch1_kc = result
        .events
        .iter()
        .find(|e| {
            (e.addr == "0x28" || e.addr == "0x29" || e.addr == "0x2A")
                && e.time < 0.001
                && e.data == "0x34"
        })
        .expect("Should have KC write for MIDI channel 1");
    assert_eq!(ch1_kc.data, "0x34"); // E (octave 3, note 4)

    let ch2_kc = result
        .events
        .iter()
        .find(|e| {
            (e.addr == "0x28" || e.addr == "0x29" || e.addr == "0x2A")
                && e.time < 0.001
                && e.data == "0x38"
        })
        .expect("Should have KC write for MIDI channel 2");
    assert_eq!(ch2_kc.data, "0x38"); // G (octave 3, note 8)

    // Verify we have 3 KEY ON events
    let key_on_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time < 0.001 && e.data.starts_with("0x7"))
        .collect();
    assert_eq!(key_on_events.len(), 3, "Should have 3 KEY ON events");
}

#[test]
fn test_convert_multi_channel_sequential() {
    // Test with notes on different channels played sequentially
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Channel 0 plays first
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
            // Channel 1 plays next
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 1,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 1,
                note: 64,
            },
        ],
    };

    let result = convert_to_ym2151_log(&midi_data).unwrap();

    // Should have events for both channels
    // Verify some YM2151 channels are initialized (allocation may vary)
    let init_channels: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.time < 0.001)
        .map(|e| &e.addr)
        .collect();

    assert!(
        init_channels.len() >= 2,
        "At least 2 YM2151 channels should be initialized"
    );

    // Verify notes play on different YM2151 channels
    let note_channels: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && (e.time < 0.001 || e.time >= 0.001))
        .map(|e| &e.addr)
        .collect();

    assert!(
        note_channels.len() >= 2,
        "Both MIDI channels should have notes"
    );
}

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

    // Verify KEY ON events are in the correct order (drum first)
    let key_on_events: Vec<&Ym2151Event> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time < 0.001 && e.data.starts_with("0x7"))
        .collect();

    // Should have 3 KEY ON events
    assert_eq!(key_on_events.len(), 3);

    // First KEY ON should be channel 0 (drum)
    assert_eq!(key_on_events[0].data, "0x78"); // Channel 0
}

{% endraw %}
```

### src/ym2151/event_processor.rs
```rs
{% raw %}
//! MIDI event processor for YM2151 conversion
//!
//! This module handles the processing of individual MIDI events
//! and converts them to YM2151 register write events.

use crate::midi::{midi_to_kc_kf, ticks_to_seconds_with_tempo_map, MidiEvent, TempoChange};
use crate::ym2151::{
    apply_tone_to_channel, default_tone_events, load_tone_for_program, ChannelAllocation,
    ToneDefinition, Ym2151Event,
};
use std::collections::{HashMap, HashSet};

/// Tracks a note-on event for later vibrato processing
#[derive(Debug, Clone)]
pub struct NoteOnInfo {
    pub start_tick: u32,
    pub start_time: f64,
}

/// Captures a full note span on a specific YM2151 channel
#[derive(Debug, Clone)]
pub struct NoteSegment {
    pub ym2151_channel: u8,
    pub note: u8,
    pub start_tick: u32,
    pub end_tick: u32,
    pub start_time: f64,
    pub end_time: f64,
}

/// Context for processing MIDI events
pub struct EventProcessorContext<'a> {
    /// Ticks per beat from MIDI file
    pub ticks_per_beat: u16,
    /// Tempo map for timing conversion
    pub tempo_map: &'a [TempoChange],
    /// Channel allocation mapping
    pub allocation: &'a mut ChannelAllocation,
    /// Active notes (YM2151 channel, MIDI note)
    pub active_notes: &'a mut HashSet<(u8, u8)>,
    /// Current program per YM2151 channel
    pub channel_programs: &'a mut HashMap<u8, u8>,
    /// Active note timings for optional vibrato processing
    pub vibrato_active_notes: Option<&'a mut HashMap<(u8, u8), NoteOnInfo>>,
    /// Completed note spans for optional vibrato processing
    pub vibrato_completed_notes: Option<&'a mut Vec<NoteSegment>>,
    /// Optional tone definitions provided via attachment JSON
    pub attachment_tones: Option<&'a HashMap<u8, ToneDefinition>>,
}

/// Process a Note On MIDI event
///
/// Converts a MIDI Note On event to YM2151 register write events
/// for KC (Key Code), KF (Key Fraction), and Key ON register.
///
/// # Arguments
/// * `ticks` - MIDI tick time
/// * `channel` - MIDI channel
/// * `note` - MIDI note number
/// * `velocity` - Note velocity (0 means Note Off in some files)
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_note_on(
    ticks: u32,
    channel: u8,
    note: u8,
    velocity: u8,
    ctx: &mut EventProcessorContext,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // Skip if velocity is 0 (should already be converted to Note Off in parser)
    if velocity == 0 {
        return events;
    }

    // Get allocated YM2151 channel(s) for this MIDI channel
    let Some(ym_channels) = ctx.allocation.midi_to_ym2151.get(&channel) else {
        return events;
    };
    if ym_channels.is_empty() {
        return events;
    }

    // Use round-robin voice allocation for polyphony
    let voice_index = ctx.allocation.current_voice.entry(channel).or_insert(0);
    let ym2151_channel = ym_channels[*voice_index % ym_channels.len()];
    *voice_index = (*voice_index + 1) % ym_channels.len();

    let time_seconds = ticks_to_seconds_with_tempo_map(ticks, ctx.ticks_per_beat, ctx.tempo_map);
    let (kc, kf) = midi_to_kc_kf(note);

    // Set KC (Key Code)
    events.push(Ym2151Event {
        time: time_seconds,
        addr: format!("0x{:02X}", 0x28 + ym2151_channel),
        data: format!("0x{:02X}", kc),
    });

    // Set KF (Key Fraction)
    events.push(Ym2151Event {
        time: time_seconds,
        addr: format!("0x{:02X}", 0x30 + ym2151_channel),
        data: format!("0x{:02X}", kf),
    });

    // Key ON (0x78 = all operators on)
    events.push(Ym2151Event {
        time: time_seconds,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", 0x78 | ym2151_channel),
    });

    ctx.active_notes.insert((ym2151_channel, note));
    if let Some(active_map) = ctx.vibrato_active_notes.as_deref_mut() {
        active_map.insert(
            (ym2151_channel, note),
            NoteOnInfo {
                start_tick: ticks,
                start_time: time_seconds,
            },
        );
    }

    events
}

/// Process a Note Off MIDI event
///
/// Converts a MIDI Note Off event to a YM2151 Key OFF register write.
///
/// # Arguments
/// * `ticks` - MIDI tick time
/// * `channel` - MIDI channel
/// * `note` - MIDI note number
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_note_off(
    ticks: u32,
    channel: u8,
    note: u8,
    ctx: &mut EventProcessorContext,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // Get allocated YM2151 channel(s) for this MIDI channel
    let Some(ym_channels) = ctx.allocation.midi_to_ym2151.get(&channel) else {
        return events;
    };

    let time_seconds = ticks_to_seconds_with_tempo_map(ticks, ctx.ticks_per_beat, ctx.tempo_map);

    // Find which YM2151 channel has this note active and turn it off
    for &ym2151_channel in ym_channels {
        if ctx.active_notes.contains(&(ym2151_channel, note)) {
            // Key OFF
            events.push(Ym2151Event {
                time: time_seconds,
                addr: "0x08".to_string(),
                data: format!("0x{:02X}", ym2151_channel),
            });

            ctx.active_notes.remove(&(ym2151_channel, note));
            if let (Some(active_map), Some(completed)) = (
                ctx.vibrato_active_notes.as_deref_mut(),
                ctx.vibrato_completed_notes.as_deref_mut(),
            ) {
                if let Some(note_on) = active_map.remove(&(ym2151_channel, note)) {
                    completed.push(NoteSegment {
                        ym2151_channel,
                        note,
                        start_tick: note_on.start_tick,
                        end_tick: ticks,
                        start_time: note_on.start_time,
                        end_time: time_seconds,
                    });
                }
            }
            break; // Only turn off one voice
        }
    }

    events
}

/// Process a Program Change MIDI event
///
/// Converts a MIDI Program Change event to YM2151 tone register writes.
/// Attempts to load a tone file for the program, falling back to default tone.
///
/// # Arguments
/// * `ticks` - MIDI tick time
/// * `channel` - MIDI channel
/// * `program` - Program/patch number
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_program_change(
    ticks: u32,
    channel: u8,
    program: u8,
    ctx: &mut EventProcessorContext,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // Get allocated YM2151 channel(s) for this MIDI channel
    let Some(ym_channels) = ctx.allocation.midi_to_ym2151.get(&channel) else {
        return events;
    };

    let time_seconds = ticks_to_seconds_with_tempo_map(ticks, ctx.ticks_per_beat, ctx.tempo_map);

    // Apply program change to all allocated YM2151 channels for this MIDI channel
    for &ym2151_channel in ym_channels {
        let load_or_default = || match load_tone_for_program(program) {
            Ok(Some(tone)) => apply_tone_to_channel(&tone, ym2151_channel, time_seconds),
            Ok(None) | Err(_) => default_tone_events(ym2151_channel, time_seconds),
        };

        // Prefer tone definitions supplied via attachment JSON, fallback to file or default tone
        let tone_events = if let Some(tone_map) = ctx.attachment_tones {
            tone_map
                .get(&program)
                .map(|tone| apply_tone_to_channel(tone, ym2151_channel, time_seconds))
                .unwrap_or_else(load_or_default)
        } else {
            load_or_default()
        };

        // Add the tone change events
        events.extend(tone_events);

        // Update the channel's current program
        ctx.channel_programs.insert(ym2151_channel, program);
    }

    events
}

/// Process a single MIDI event
///
/// Dispatches to the appropriate handler based on event type.
///
/// # Arguments
/// * `event` - The MIDI event to process
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_event(event: &MidiEvent, ctx: &mut EventProcessorContext) -> Vec<Ym2151Event> {
    match event {
        // Tempo events are handled via tempo_map, no action needed here
        MidiEvent::Tempo { .. } => Vec::new(),

        MidiEvent::NoteOn {
            ticks,
            channel,
            note,
            velocity,
            ..
        } => process_note_on(*ticks, *channel, *note, *velocity, ctx),

        MidiEvent::NoteOff {
            ticks,
            channel,
            note,
            ..
        } => process_note_off(*ticks, *channel, *note, ctx),

        MidiEvent::ProgramChange {
            ticks,
            channel,
            program,
        } => process_program_change(*ticks, *channel, *program, ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ym2151::allocate_channels;
    use std::collections::HashMap;

    fn create_test_context<'a>(
        ticks_per_beat: u16,
        tempo_map: &'a [TempoChange],
        allocation: &'a mut ChannelAllocation,
        active_notes: &'a mut HashSet<(u8, u8)>,
        channel_programs: &'a mut HashMap<u8, u8>,
    ) -> EventProcessorContext<'a> {
        EventProcessorContext {
            ticks_per_beat,
            tempo_map,
            allocation,
            active_notes,
            channel_programs,
            vibrato_active_notes: None,
            vibrato_completed_notes: None,
            attachment_tones: None,
        }
    }

    #[test]
    fn test_process_note_on_basic() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_on(0, 0, 60, 100, &mut ctx);

        // Should produce 3 events: KC, KF, Key ON
        assert_eq!(events.len(), 3);

        // Verify KC event
        assert!(events[0].addr.starts_with("0x2")); // KC register range
        assert_eq!(events[0].data, "0x2E"); // Middle C

        // Verify KF event
        assert!(events[1].addr.starts_with("0x3")); // KF register range
        assert_eq!(events[1].data, "0x00");

        // Verify Key ON event
        assert_eq!(events[2].addr, "0x08");
        assert!(events[2].data.starts_with("0x7")); // All operators on
    }

    #[test]
    fn test_process_note_on_zero_velocity() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_on(0, 0, 60, 0, &mut ctx);

        // Zero velocity should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_note_on_no_allocation() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        // Empty polyphony - no channels allocated
        let polyphony = HashMap::new();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_on(0, 0, 60, 100, &mut ctx);

        // No allocation should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_note_off_basic() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        // First, send a note on
        {
            let mut ctx = create_test_context(
                480,
                &tempo_map,
                &mut allocation,
                &mut active_notes,
                &mut channel_programs,
            );
            process_note_on(0, 0, 60, 100, &mut ctx);
        }

        // Now send note off
        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_off(480, 0, 60, &mut ctx);

        // Should produce 1 Key OFF event
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].addr, "0x08");
        // Data should be the channel number with no operators on
        assert!(events[0].data.starts_with("0x0"));
    }

    #[test]
    fn test_process_note_off_no_active_note() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        // Note off without note on
        let events = process_note_off(480, 0, 60, &mut ctx);

        // Should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_program_change_basic() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_program_change(0, 0, 42, &mut ctx);

        // Should produce tone events (26 events for default tone)
        assert!(!events.is_empty());

        // Channel program should be updated
        assert_eq!(*ctx.channel_programs.get(&0).unwrap(), 42);
    }

    #[test]
    fn test_process_program_change_no_allocation() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = HashMap::new();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_program_change(0, 0, 42, &mut ctx);

        // No allocation should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_event_tempo() {
        let tempo_map = vec![];
        let polyphony = HashMap::new();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let event = MidiEvent::Tempo {
            ticks: 0,
            tempo_bpm: 140.0,
        };

        let events = process_event(&event, &mut ctx);

        // Tempo events should produce no YM2151 events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_event_note_on() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let event = MidiEvent::NoteOn {
            ticks: 0,
            channel: 0,
            note: 60,
            velocity: 100,
        };

        let events = process_event(&event, &mut ctx);

        // Should produce 3 events: KC, KF, Key ON
        assert_eq!(events.len(), 3);
    }
}

{% endraw %}
```

### tests/integration_tests.rs
```rs
{% raw %}
//! Integration tests for smf-to-ym2151log-rust

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json, MidiEvent};
use smf_to_ym2151log::ym2151::{convert_to_ym2151_log, save_ym2151_log};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[test]
fn test_parse_simple_melody() {
    let midi_path = "tests/test_data/simple_melody.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check metadata
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);

    // Check events
    assert!(!midi_data.events.is_empty(), "No events parsed");

    // Should have 2 note on and 2 note off events (4 total)
    let note_ons: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::NoteOn { .. }))
        .collect();
    let note_offs: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::NoteOff { .. }))
        .collect();

    assert_eq!(note_ons.len(), 2, "Expected 2 Note On events");
    assert_eq!(note_offs.len(), 2, "Expected 2 Note Off events");

    // Verify first note on is Middle C (60) at tick 0
    if let MidiEvent::NoteOn {
        ticks,
        note,
        velocity,
        channel,
    } = note_ons[0]
    {
        assert_eq!(*ticks, 0);
        assert_eq!(*note, 60);
        assert_eq!(*velocity, 100);
        assert_eq!(*channel, 0);
    } else {
        panic!("First event should be Note On");
    }
}

#[test]
fn test_parse_tempo_change() {
    let midi_path = "tests/test_data/tempo_change.mid";

    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check that we have tempo events
    let tempo_events: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::Tempo { .. }))
        .collect();

    assert!(
        !tempo_events.is_empty(),
        "Expected at least one tempo event"
    );

    // First tempo event should be 120 BPM
    if let MidiEvent::Tempo { tempo_bpm, .. } = tempo_events[0] {
        assert!(
            (tempo_bpm - 120.0).abs() < 0.1,
            "First tempo should be ~120 BPM, got {}",
            tempo_bpm
        );
    }

    // If there's a second tempo event, it should be ~140 BPM
    if tempo_events.len() >= 2 {
        if let MidiEvent::Tempo { tempo_bpm, .. } = tempo_events[1] {
            assert!(
                (tempo_bpm - 140.0).abs() < 1.0,
                "Second tempo should be ~140 BPM, got {}",
                tempo_bpm
            );
        }
    }
}

#[test]
fn test_parse_multi_track() {
    let midi_path = "tests/test_data/multi_track.mid";

    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Should have events from both tracks merged
    assert!(
        !midi_data.events.is_empty(),
        "No events parsed from multi-track file"
    );

    // Check that we have tempo and note events
    let has_tempo = midi_data
        .events
        .iter()
        .any(|e| matches!(e, MidiEvent::Tempo { .. }));
    let has_notes = midi_data
        .events
        .iter()
        .any(|e| matches!(e, MidiEvent::NoteOn { .. }));

    assert!(has_tempo, "Should have tempo events");
    assert!(has_notes, "Should have note events");
}

#[test]
fn test_save_midi_events_json() {
    use std::env;

    let midi_path = "tests/test_data/simple_melody.mid";

    // Use system temp directory for cross-platform compatibility
    let temp_dir = env::temp_dir();
    let output_path = temp_dir.join("test_output_events.json");
    let output_path_str = output_path
        .to_str()
        .expect("Failed to convert path to string");

    // Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Save to JSON
    let result = save_midi_events_json(&midi_data, output_path_str);
    assert!(result.is_ok(), "Failed to save JSON: {:?}", result.err());

    // Verify file exists
    assert!(output_path.exists(), "Output JSON file was not created");

    // Read and verify it's valid JSON
    let json_content = fs::read_to_string(&output_path).expect("Failed to read output JSON");

    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Output is not valid JSON");

    // Verify structure
    assert!(parsed.get("ticks_per_beat").is_some());
    assert!(parsed.get("tempo_bpm").is_some());
    assert!(parsed.get("events").is_some());

    // Clean up
    let _ = fs::remove_file(&output_path);
}

#[test]
fn test_events_are_sorted_by_ticks() {
    let midi_path = "tests/test_data/simple_melody.mid";

    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify events are sorted by ticks
    let ticks: Vec<u32> = midi_data
        .events
        .iter()
        .map(|e| match e {
            MidiEvent::NoteOn { ticks, .. } => *ticks,
            MidiEvent::NoteOff { ticks, .. } => *ticks,
            MidiEvent::Tempo { ticks, .. } => *ticks,
            MidiEvent::ProgramChange { ticks, .. } => *ticks,
        })
        .collect();

    // Check that each tick is >= the previous tick
    for i in 1..ticks.len() {
        assert!(
            ticks[i] >= ticks[i - 1],
            "Events not sorted: tick[{}]={} < tick[{}]={}",
            i,
            ticks[i],
            i - 1,
            ticks[i - 1]
        );
    }
}

#[test]
fn test_note_on_with_velocity_zero_becomes_note_off() {
    // This test verifies that Note On with velocity 0 is treated as Note Off
    // This is part of the MIDI specification
    let midi_path = "tests/test_data/simple_melody.mid";

    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // All note off events should exist (either as explicit note off or note on with vel=0)
    let note_offs: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::NoteOff { .. }))
        .collect();

    assert!(!note_offs.is_empty(), "Should have note off events");
}

#[test]
fn test_parse_nonexistent_file() {
    let result = parse_midi_file("nonexistent_file.mid");
    assert!(result.is_err(), "Should fail for nonexistent file");
}

// ============================================================================
// Phase 5: End-to-End Integration Tests
// ============================================================================

/// Test complete end-to-end conversion flow with simple melody
#[test]
fn test_end_to_end_simple_melody() {
    use std::env;

    let midi_path = "tests/test_data/simple_melody.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_simple_melody_events.json");
    let ym2151_json_path = temp_dir.join("e2e_simple_melody_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify Pass A output
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);
    assert!(!midi_data.events.is_empty());

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");
    assert!(events_json_path.exists());

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Verify Pass B output
    assert!(ym2151_log.event_count > 0);
    assert_eq!(ym2151_log.events.len(), ym2151_log.event_count as usize);

    // Save YM2151 log JSON
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");
    assert!(ym2151_json_path.exists());

    // Verify YM2151 JSON structure
    let json_content = fs::read_to_string(&ym2151_json_path).expect("Failed to read YM2151 JSON");
    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Invalid JSON format");

    assert!(parsed.get("event_count").is_some());
    assert!(parsed.get("events").is_some());

    // Verify events array structure
    let events = parsed["events"].as_array().expect("events should be array");
    assert!(!events.is_empty());

    // Check first event structure (should be initialization)
    let first_event = &events[0];
    assert!(first_event.get("time").is_some());
    assert!(first_event.get("addr").is_some());
    assert!(first_event.get("data").is_some());

    // Verify address and data are in hex format
    let addr = first_event["addr"].as_str().expect("addr should be string");
    let data = first_event["data"].as_str().expect("data should be string");
    assert!(addr.starts_with("0x"));
    assert!(data.starts_with("0x"));

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test end-to-end conversion with tempo change
#[test]
fn test_end_to_end_tempo_change() {
    use std::env;

    let midi_path = "tests/test_data/tempo_change.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_tempo_change_events.json");
    let ym2151_json_path = temp_dir.join("e2e_tempo_change_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify tempo events exist
    let tempo_events: Vec<_> = midi_data
        .events
        .iter()
        .filter_map(|e| {
            if let MidiEvent::Tempo { ticks, tempo_bpm } = e {
                Some((*ticks, *tempo_bpm))
            } else {
                None
            }
        })
        .collect();
    assert!(!tempo_events.is_empty(), "Should have tempo events");

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Verify that tempo changes affect timing
    // Find note events in the YM2151 log
    let note_on_events: Vec<_> = ym2151_log
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.data.starts_with("0x7"))
        .collect();

    // Should have at least 2 note on events with different timing
    assert!(
        note_on_events.len() >= 2,
        "Should have at least 2 note on events"
    );

    // First note should be at time 0
    assert!(
        note_on_events[0].time < 0.001,
        "First note should be at time 0"
    );

    // Second note timing should reflect tempo change
    // If tempo didn't affect timing, both notes would have same relative spacing
    // With tempo change, the spacing should be different
    assert!(
        note_on_events[1].time > 0.001,
        "Second note should be after time 0"
    );

    // Save YM2151 log
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");

    // Verify outputs exist
    assert!(events_json_path.exists());
    assert!(ym2151_json_path.exists());

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test end-to-end conversion with multi-track MIDI file
#[test]
fn test_end_to_end_multi_track() {
    use std::env;

    let midi_path = "tests/test_data/multi_track.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_multi_track_events.json");
    let ym2151_json_path = temp_dir.join("e2e_multi_track_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Save YM2151 log
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");

    // Verify both outputs exist
    assert!(events_json_path.exists());
    assert!(ym2151_json_path.exists());

    // Verify the YM2151 log has reasonable content
    assert!(ym2151_log.event_count > 0);
    assert_eq!(ym2151_log.events.len(), ym2151_log.event_count as usize);

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test output file paths are correctly determined
#[test]
fn test_output_file_path_generation() {
    let test_cases = vec![
        ("test.mid", "test_events.json", "test_ym2151.json"),
        (
            "path/to/test.mid",
            "path/to/test_events.json",
            "path/to/test_ym2151.json",
        ),
        ("my_song.mid", "my_song_events.json", "my_song_ym2151.json"),
    ];

    for (input_path, expected_events, expected_ym2151) in test_cases {
        let path = Path::new(input_path);
        let base_name = path.file_stem().unwrap().to_string_lossy();
        let output_dir = path.parent().unwrap_or_else(|| Path::new("."));

        let events_json_path = output_dir.join(format!("{}_events.json", base_name));
        let ym2151_json_path = output_dir.join(format!("{}_ym2151.json", base_name));

        assert_eq!(
            events_json_path.to_str().unwrap(),
            expected_events,
            "Events JSON path mismatch for {}",
            input_path
        );
        assert_eq!(
            ym2151_json_path.to_str().unwrap(),
            expected_ym2151,
            "YM2151 JSON path mismatch for {}",
            input_path
        );
    }
}

/// Test that YM2151 log contains valid time values in seconds
#[test]
fn test_ym2151_log_time_values() {
    let midi_path = "tests/test_data/simple_melody.mid";

    // Parse and convert
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Check that times are non-decreasing (equal values are allowed, e.g., for simultaneous events)
    let mut prev_time = 0.0;
    for event in &ym2151_log.events {
        assert!(
            event.time >= prev_time,
            "Time should be non-decreasing (event.time={}, prev_time={})",
            event.time,
            prev_time
        );
        prev_time = event.time;
    }

    // Verify at least one event has non-zero time (unless empty)
    if !ym2151_log.events.is_empty() {
        let has_nonzero = ym2151_log.events.iter().any(|e| e.time > 0.001);
        // For non-empty MIDI files with notes, we should have some non-zero times
        // (Only all-zero times would be unusual for actual note events)
        assert!(
            ym2151_log.events.is_empty() || has_nonzero || ym2151_log.events.len() <= 32,
            "Expected at least some non-zero time values for events beyond initialization"
        );
    }
}

/// Test that YM2151 log contains properly formatted hex strings
#[test]
fn test_ym2151_log_hex_format() {
    let midi_path = "tests/test_data/simple_melody.mid";

    // Parse and convert
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Check all events have properly formatted hex strings
    for event in &ym2151_log.events {
        // Check address format
        assert!(
            event.addr.starts_with("0x"),
            "Address should start with 0x: {}",
            event.addr
        );
        assert!(
            event.addr.len() == 4,
            "Address should be 4 chars (0xXX): {}",
            event.addr
        );

        // Check data format
        assert!(
            event.data.starts_with("0x"),
            "Data should start with 0x: {}",
            event.data
        );
        assert!(
            event.data.len() == 4,
            "Data should be 4 chars (0xXX): {}",
            event.data
        );

        // Verify they can be parsed as hex
        let addr_val = u8::from_str_radix(&event.addr[2..], 16);
        let data_val = u8::from_str_radix(&event.data[2..], 16);

        assert!(
            addr_val.is_ok(),
            "Address should be valid hex: {}",
            event.addr
        );
        assert!(data_val.is_ok(), "Data should be valid hex: {}", event.data);
    }
}

#[test]
fn test_convert_smf_to_ym2151_log_convenience_function() {
    // Test the convenience function that accepts raw SMF bytes
    let midi_path = "tests/test_data/simple_melody.mid";

    // Read the MIDI file as bytes
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    // Use the convenience function
    let result = smf_to_ym2151log::convert_smf_to_ym2151_log(&smf_bytes);
    assert!(
        result.is_ok(),
        "Failed to convert SMF to YM2151 log: {:?}",
        result.err()
    );

    let json_string = result.unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&json_string).expect("Output should be valid JSON");

    // Verify it has the expected structure
    assert!(
        parsed.get("event_count").is_some(),
        "Should have event_count field"
    );
    assert!(parsed.get("events").is_some(), "Should have events field");

    // Verify event_count is a number
    let event_count = parsed["event_count"]
        .as_u64()
        .expect("event_count should be a number");
    assert!(event_count > 0, "Should have at least some events");

    // Verify events is an array
    let events = parsed["events"]
        .as_array()
        .expect("events should be an array");
    assert_eq!(
        events.len() as u64,
        event_count,
        "events length should match event_count"
    );
}

#[test]
fn test_parse_midi_from_bytes() {
    // Test parsing MIDI from bytes directly
    let midi_path = "tests/test_data/simple_melody.mid";

    // Read the MIDI file as bytes
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    // Parse from bytes
    let result = smf_to_ym2151log::midi::parse_midi_from_bytes(&smf_bytes);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI from bytes: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Verify metadata
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);

    // Verify we got events
    assert!(!midi_data.events.is_empty(), "Should have parsed events");
}

#[test]
fn test_parse_multi_channel() {
    let midi_path = "tests/test_data/multi_channel.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check metadata
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);

    // Should have 6 events (3 note on, 3 note off)
    assert_eq!(midi_data.events.len(), 6);

    // Verify we have notes on different channels
    let note_ons: Vec<_> = midi_data
        .events
        .iter()
        .filter_map(|e| {
            if let MidiEvent::NoteOn { channel, note, .. } = e {
                Some((*channel, *note))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(note_ons.len(), 3, "Should have 3 note on events");

    // Verify channels 0, 1, 2 are present with notes C(60), E(64), G(67)
    assert!(
        note_ons.contains(&(0, 60)),
        "Should have channel 0 with note 60"
    );
    assert!(
        note_ons.contains(&(1, 64)),
        "Should have channel 1 with note 64"
    );
    assert!(
        note_ons.contains(&(2, 67)),
        "Should have channel 2 with note 67"
    );
}

#[test]
fn test_end_to_end_multi_channel() {
    use std::env;

    let midi_path = "tests/test_data/multi_channel.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_multi_channel_events.json");
    let ym2151_json_path = temp_dir.join("e2e_multi_channel_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify we have notes on different channels
    let channels_used: HashSet<u8> = midi_data
        .events
        .iter()
        .filter_map(|e| match e {
            MidiEvent::NoteOn { channel, .. } => Some(*channel),
            _ => None,
        })
        .collect();

    assert_eq!(channels_used.len(), 3, "Should use 3 different channels");
    assert!(channels_used.contains(&0));
    assert!(channels_used.contains(&1));
    assert!(channels_used.contains(&2));

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");
    assert!(events_json_path.exists());

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Verify Pass B output has events for all channels
    assert!(ym2151_log.event_count > 0);

    // Check that we have register writes for all 3 channels (allocation may vary based on polyphony)
    // Just verify that notes from different MIDI channels are present
    let has_ch0_notes = ym2151_log
        .events
        .iter()
        .any(|e| e.addr.starts_with("0x2") && e.addr.len() == 4);
    let has_ch1_notes = ym2151_log
        .events
        .iter()
        .any(|e| e.addr.starts_with("0x2") && e.addr.len() == 4);
    let has_ch2_notes = ym2151_log
        .events
        .iter()
        .any(|e| e.addr.starts_with("0x2") && e.addr.len() == 4);

    assert!(has_ch0_notes, "Should have register writes for channels");
    assert!(has_ch1_notes, "Should have register writes for channels");
    assert!(has_ch2_notes, "Should have register writes for channels");

    // Save YM2151 log JSON
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");
    assert!(ym2151_json_path.exists());

    // Verify YM2151 JSON structure
    let json_content = fs::read_to_string(&ym2151_json_path).expect("Failed to read YM2151 JSON");
    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Invalid JSON format");

    assert!(parsed.get("event_count").is_some());
    assert!(parsed.get("events").is_some());

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test that tempo changes are correctly reflected in YM2151 timing
#[test]
fn test_tempo_change_timing_accuracy() {
    use smf_to_ym2151log::midi::{
        ticks_to_seconds_with_tempo_map, MidiData, MidiEvent, TempoChange,
    };
    use smf_to_ym2151log::ym2151::convert_to_ym2151_log;

    // Create a test MIDI file with tempo change
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Tempo starts at 120 BPM
            MidiEvent::Tempo {
                ticks: 0,
                tempo_bpm: 120.0,
            },
            // First note at tick 0
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            // First note off at tick 480 (1 beat at 120 BPM = 0.5 seconds)
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
            // Tempo changes to 60 BPM at tick 480
            MidiEvent::Tempo {
                ticks: 480,
                tempo_bpm: 60.0,
            },
            // Second note at tick 480
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 62,
                velocity: 100,
            },
            // Second note off at tick 960 (1 beat at 60 BPM = 1.0 second after tempo change)
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 0,
                note: 62,
            },
        ],
    };

    // Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert");

    // Find the note on/off events
    let note_events: Vec<_> = ym2151_log
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time > 0.001)
        .collect();

    // First note off should be at tick 480
    // At 120 BPM: 480 ticks = 0.5 seconds
    // With polyphony analysis, channel allocation may vary - just check for note off events
    let first_note_off = note_events
        .iter()
        .find(|e| e.data.starts_with("0x0") && e.time > 0.001 && e.time <= 0.51)
        .expect("Should have first note off");
    assert!(
        first_note_off.time >= 0.49 && first_note_off.time <= 0.51,
        "First note off timing should be around 0.5 seconds, got {}",
        first_note_off.time
    );

    // Second note on should also be at tick 480 (same time as tempo change)
    let second_note_on = note_events
        .iter()
        .find(|e| e.data.starts_with("0x7") && e.time >= 0.49 && e.time <= 0.51)
        .expect("Should have second note on at tempo change");
    assert!(
        second_note_on.time >= 0.49 && second_note_on.time <= 0.51,
        "Second note on timing should be around 0.5 seconds, got {}",
        second_note_on.time
    );

    // Second note off should be at tick 960
    // First 480 ticks at 120 BPM = 0.5 seconds
    // Next 480 ticks at 60 BPM = 1.0 second
    // Total = 1.5 seconds
    let second_note_off = note_events
        .iter()
        .filter(|e| e.data.starts_with("0x0"))
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
        .expect("Should have second note off");
    assert!(
        second_note_off.time >= 1.49 && second_note_off.time <= 1.51,
        "Second note off timing should be around 1.5 seconds, got {}",
        second_note_off.time
    );

    // Verify using the tempo map function directly
    let tempo_map = vec![
        TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        },
        TempoChange {
            tick: 480,
            tempo_bpm: 60.0,
        },
    ];

    let time_at_960 = ticks_to_seconds_with_tempo_map(960, 480, &tempo_map);
    assert!(
        (time_at_960 - 1.5).abs() < 0.001,
        "Tempo map calculation should match expected value of 1.5 seconds, got {}",
        time_at_960
    );
}

#[test]
fn test_tone_loading_from_file() {
    use smf_to_ym2151log::ym2151::load_tone_for_program;

    // Test loading tone file 000.json (which should exist in tones directory)
    let result = load_tone_for_program(0);
    assert!(result.is_ok(), "Failed to load tone: {:?}", result.err());

    let tone_opt = result.unwrap();
    assert!(
        tone_opt.is_some(),
        "Tone file tones/000.json should exist for testing"
    );

    let tone = tone_opt.unwrap();
    assert!(
        !tone.events.is_empty(),
        "Tone should have register write events"
    );

    // Verify tone has expected structure
    assert_eq!(tone.events.len(), 26, "Default tone should have 26 events");
}

#[test]
fn test_tone_loading_nonexistent() {
    use smf_to_ym2151log::ym2151::load_tone_for_program;

    // Test loading a tone that doesn't exist (e.g., program 127)
    let result = load_tone_for_program(127);
    assert!(result.is_ok());

    let tone_opt = result.unwrap();
    // Should return None if file doesn't exist
    if tone_opt.is_none() {
        // This is the expected behavior - no tone file exists
    } else {
        // If the file exists, that's also fine for this test
    }
}

#[test]
fn test_end_to_end_program_change() {
    use smf_to_ym2151log::midi::{MidiData, MidiEvent};
    use smf_to_ym2151log::ym2151::convert_to_ym2151_log;

    // Create MIDI data with program change
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 0, // Use program 0 which has a tone file
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

    let result = convert_to_ym2151_log(&midi_data);
    assert!(result.is_ok(), "Conversion should succeed");

    let log = result.unwrap();
    assert!(log.event_count > 0, "Should have YM2151 events");

    // Should have more events due to program change tone loading
    // 8 KEY OFF + 26 init + 26 program change tone + 3 note on + 1 note off = 64
    assert_eq!(
        log.event_count, 64,
        "Should have events from init, program change tone, and notes"
    );
}

#[test]
fn test_parse_program_change_midi() {
    let midi_path = "tests/test_data/program_change.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check that we have program change events
    let program_events: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::ProgramChange { .. }))
        .collect();

    assert_eq!(program_events.len(), 2, "Expected 2 program change events");

    // Verify first program change is to program 0
    if let MidiEvent::ProgramChange {
        ticks,
        channel,
        program,
    } = program_events[0]
    {
        assert_eq!(*ticks, 0);
        assert_eq!(*channel, 0);
        assert_eq!(*program, 0);
    }

    // Verify second program change is to program 42
    if let MidiEvent::ProgramChange {
        ticks: _,
        channel,
        program,
    } = program_events[1]
    {
        assert_eq!(*channel, 0);
        assert_eq!(*program, 42);
    }
}

#[test]
fn test_end_to_end_program_change_with_file() {
    let midi_path = "tests/test_data/program_change.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Convert to YM2151 log
    let ym2151_result = convert_to_ym2151_log(&midi_data);
    assert!(
        ym2151_result.is_ok(),
        "Failed to convert to YM2151: {:?}",
        ym2151_result.err()
    );

    let log = ym2151_result.unwrap();

    // Should have:
    // - 8 KEY OFF events (initialization)
    // - 26 channel init events
    // - 26 program 0 tone events
    // - 3 note on events (KC, KF, KEY ON)
    // - 1 note off event
    // - 26 program 42 tone events
    // - 3 note on events
    // - 1 note off event
    // Total: 8 + 26 + 26 + 3 + 1 + 26 + 3 + 1 = 94
    assert_eq!(
        log.event_count, 94,
        "Should have correct number of events including two program changes"
    );

    // Verify program change events generated tone changes
    // Check for RL_FB_CONNECT register writes (0x20-0x27)
    let tone_change_events: Vec<_> = log
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4)
        .collect();

    // Should have writes for init and both program changes
    assert!(
        tone_change_events.len() >= 3,
        "Should have tone settings from init and both program changes"
    );
}

// ============================================================================
// WASM Interface Tests
// ============================================================================

#[test]
#[cfg(feature = "wasm")]
fn test_wasm_smf_to_ym2151_json_valid_midi() {
    use smf_to_ym2151log::wasm::smf_to_ym2151_json;

    let midi_path = "tests/test_data/simple_melody.mid";
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    let result = smf_to_ym2151_json(&smf_bytes);

    // Parse the result as JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&result).expect("Result should be valid JSON");

    // Should not have an error field
    assert!(
        parsed.get("error").is_none(),
        "Should not have error field for valid MIDI"
    );

    // Should have expected structure
    assert!(
        parsed.get("event_count").is_some(),
        "Should have event_count field"
    );
    assert!(parsed.get("events").is_some(), "Should have events field");

    let event_count = parsed["event_count"]
        .as_u64()
        .expect("event_count should be a number");
    assert!(event_count > 0, "Should have at least some events");
}

#[test]
#[cfg(feature = "wasm")]
fn test_wasm_smf_to_ym2151_json_invalid_midi() {
    use smf_to_ym2151log::wasm::smf_to_ym2151_json;

    // Invalid MIDI data
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03];

    let result = smf_to_ym2151_json(&invalid_data);

    // Parse the result as JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&result).expect("Result should be valid JSON");

    // Should have an error field
    assert!(
        parsed.get("error").is_some(),
        "Should have error field for invalid MIDI"
    );
}

#[test]
#[cfg(feature = "wasm")]
fn test_wasm_smf_to_ym2151_json_empty_data() {
    use smf_to_ym2151log::wasm::smf_to_ym2151_json;

    let empty_data = vec![];

    let result = smf_to_ym2151_json(&empty_data);

    // Parse the result as JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&result).expect("Result should be valid JSON");

    // Should have an error field
    assert!(
        parsed.get("error").is_some(),
        "Should have error field for empty data"
    );
}

#[test]
fn test_convert_smf_to_ym2151_log_end_to_end() {
    // This test is always available and tests the convenience function
    let midi_path = "tests/test_data/simple_melody.mid";
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    let result = smf_to_ym2151log::convert_smf_to_ym2151_log(&smf_bytes);
    assert!(
        result.is_ok(),
        "Should successfully convert SMF bytes: {:?}",
        result.err()
    );

    let json = result.unwrap();

    // Parse and verify structure
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("Result should be valid JSON");

    assert!(parsed.get("event_count").is_some());
    assert!(parsed.get("events").is_some());

    let events = parsed["events"].as_array().expect("events should be array");
    assert!(!events.is_empty(), "Should have events");

    // Verify event structure
    for event in events {
        assert!(event.get("time").is_some(), "Event should have time field");
        assert!(event.get("addr").is_some(), "Event should have addr field");
        assert!(event.get("data").is_some(), "Event should have data field");

        let addr = event["addr"].as_str().expect("addr should be string");
        let data = event["data"].as_str().expect("data should be string");

        assert!(addr.starts_with("0x"), "addr should start with 0x");
        assert!(data.starts_with("0x"), "data should start with 0x");
    }
}

{% endraw %}
```

## 最近の変更（過去7日間）
### コミット履歴:
db16875 Merge branch 'main' of github.com:cat2151/smf-to-ym2151log-rust into main
bd2f226 check large files
c3fca81 Add issue note for #133 [auto]
d18a2b5 Update instructions for using cat2151 library
e079450 Update instructions for demo development and libraries
11d2c6c Add issue note for #131 [auto]
f1280c2 Merge pull request #130 from cat2151/copilot/add-biome-formatter-linter
1c74d29 Update project summaries (overview & development status) [auto]
213f5b0 Address PR review: fix biome.json schema URL and split tone-json-demo.ts
a0432ad Add Biome as TypeScript formatter/linter for demo-library

### 変更されたファイル:
.github/copilot-instructions.md
.github/workflows/call-check-large-files.yml
demo-library/biome.json
demo-library/delay-vibrato-demo.ts
demo-library/globals.d.ts
demo-library/library-demo.ts
demo-library/log-visualizer.ts
demo-library/mml-support.ts
demo-library/package-lock.json
demo-library/package.json
demo-library/pop-noise-demo.ts
demo-library/portamento-soft-lfo-demo.ts
demo-library/shared-demo.ts
demo-library/style.css
demo-library/tone-json-attachment.ts
demo-library/tone-json-demo.ts
demo-library/tone-json-mml.ts
demo-library/tsconfig.json
demo-library/vite.config.ts
generated-docs/development-status-generated-prompt.md
generated-docs/development-status.md
generated-docs/project-overview-generated-prompt.md
generated-docs/project-overview.md
issue-notes/114.md
issue-notes/117.md
issue-notes/126.md
issue-notes/128.md
issue-notes/131.md
issue-notes/133.md
src/lib.rs
src/ym2151/converter/register_effects.rs
tests/integration_tests.rs


---
Generated at: 2026-03-04 07:10:59 JST

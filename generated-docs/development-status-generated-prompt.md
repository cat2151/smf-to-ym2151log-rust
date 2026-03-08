Last updated: 2026-03-09

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
- issue-notes/171.md
- issue-notes/172.md
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
## [Issue #174](../issue-notes/174.md): 大きなファイルの検出: 2個のファイルが500行を超えています
以下のファイルが500行を超えています。リファクタリングを検討してください。

## 検出されたファイル

| ファイル | 行数 | 超過行数 |
|---------|------|----------|
| `demo-library/log-visualizer.ts` | 583 | +83 |
| `src/ym2151/converter_tests/effects.rs` | 524 | +24 |

## テスト実施のお願い

- リファクタリング前後にテストを実行し、それぞれのテスト失敗件数を報告してください
- リファクタリング前後のどちらかでテストがredの場合、まず別...
ラベル: refactoring, code-quality, automated
--- issue-notes/174.md の内容 ---

```markdown

```

## [Issue #173](../issue-notes/173.md): Fix compact nibble preset being overwritten by normalized ym2151log form on MML input
- [x] Fix `readAttachmentBytes` in `tone-json-demo.ts` to not overwrite textarea or reset preset dropdown when a preset is active
- [x] Fix `isPresetActive` to use explicit `!= null` check so type is `boolean` (not nullable union)
- [x] Biome format/lint pass
- [x] Code review and CodeQL passed

<!-...
ラベル: 
--- issue-notes/173.md の内容 ---

```markdown

```

## [Issue #172](../issue-notes/172.md): 添付音色JSONについて、コンパクトnibble形式の項目名は、"CompactTone"をやめて"registers"にする（web-ym2151に寄せる）
[issue-notes/172.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/172.md)

...
ラベル: 
--- issue-notes/172.md の内容 ---

```markdown
# issue 添付音色JSONについて、コンパクトnibble形式の項目名は、CompactToneをやめてregistersにする（web-ym2151に寄せる） #172
[issues #172](https://github.com/cat2151/smf-to-ym2151log-rust/issues/172)



```

## [Issue #171](../issue-notes/171.md): 添付音色JSONについて、プルダウンでコンパクトnibble形式を選んだあと、MMLでcを入力すると、nibbleが消えてym2151log形式になってしまいuserが混乱する
[issue-notes/171.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/171.md)

...
ラベル: 
--- issue-notes/171.md の内容 ---

```markdown
# issue 添付音色JSONについて、プルダウンでコンパクトnibble形式を選んだあと、MMLでcを入力すると、nibbleが消えてym2151log形式になってしまいuserが混乱する #171
[issues #171](https://github.com/cat2151/smf-to-ym2151log-rust/issues/171)



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

### demo-library/log-visualizer.ts
```ts
{% raw %}
import {
	type YmLogEvent,
	NOTE_CODE_TO_SEMITONE,
	PIXELS_PER_SECOND,
	parseHexByte,
} from "./ym2151-utils";

type LaneElements = {
	root: HTMLElement;
	track: HTMLElement;
};

type NoteOnState = { time: number; kc: number; kf: number };

type NoteSegment = {
	startTime: number;
	endTime: number;
	kc: number;
	kf: number;
	ch: number;
};

/** Config for a single LFO-modulated register (base register in hex, e.g. "0x60"). */
export type LfoRegisterConfig = {
	baseRegister: string;
	label?: string;
};

export type LogVisualizer = {
	renderFromJson: (jsonText: string | null | undefined) => void;
	clear: () => void;
	/** Provide LFO register config so the visualizer can draw waveform lanes. */
	setLfoRegisters: (registers: LfoRegisterConfig[]) => void;
};

const DEFAULT_CHANNELS = 8;
const MIN_TRACK_WIDTH = 640;
const MAX_TRACK_WIDTH = 6400;
const EVENT_WIDTH = 4;
const KC_REGISTER_BASE = 0x28;
const KF_REGISTER_BASE = 0x30;
const TRACK_HEIGHT = 80;
const NOTE_BAR_HEIGHT = 8;
const MIN_NOTE_WIDTH = 2;
const NOTE_WIDTH_GAP = 1;
const UNCLOSED_NOTE_EXTENSION_S = 0.1;
// Minimum segment duration to keep DOM node count manageable.
// Segments shorter than one pixel at the current scale are coalesced into their successor.
const MIN_SEGMENT_SECONDS = MIN_NOTE_WIDTH / PIXELS_PER_SECOND;

/**
 * Mirrors the Rust `resolve_register_for_channel` function.
 * Given a base register and YM2151 channel index, returns the per-channel address.
 */
function resolveRegisterForChannel(baseReg: number, channel: number): number {
	if (baseReg >= 0x20 && baseReg <= 0x27) return 0x20 + channel;
	if (baseReg >= 0x28 && baseReg <= 0x2f) return 0x28 + channel;
	if (baseReg >= 0x30 && baseReg <= 0x37) return 0x30 + channel;
	if (baseReg >= 0x38 && baseReg <= 0x3f) return 0x38 + channel;
	if (baseReg >= 0x40) {
		const base = baseReg & 0xe0;
		const slot = baseReg & 0x1f;
		const operator = Math.floor(slot / 8);
		const newSlot = channel + operator * 8;
		return base + newSlot;
	}
	return baseReg;
}

/**
 * For each LFO base register, collect the set of resolved per-channel addresses
 * (for the 8 YM2151 channels) that appear in the event list.
 */
function collectLfoEvents(
	events: YmLogEvent[],
	lfoRegisters: LfoRegisterConfig[],
	channelCount: number,
): Map<
	string,
	{ addr: number; label: string; events: { x: number; data: number }[] }
> {
	const result = new Map<
		string,
		{ addr: number; label: string; events: { x: number; data: number }[] }
	>();

	if (lfoRegisters.length === 0) return result;

	// Build address → (baseKey, label) lookup for all channel-resolved addresses
	const addrToKey = new Map<number, { key: string; label: string }>();
	for (const lfoDef of lfoRegisters) {
		const base = parseHexByte(lfoDef.baseRegister);
		if (base === null) continue;
		const key = lfoDef.baseRegister;
		const label = lfoDef.label ?? `LFO ${lfoDef.baseRegister}`;
		for (let ch = 0; ch < channelCount; ch++) {
			const resolved = resolveRegisterForChannel(base, ch);
			addrToKey.set(resolved, { key, label });
		}
	}

	for (const event of events) {
		const addr = parseHexByte(event.addr);
		const data = parseHexByte(event.data);
		if (addr === null || data === null) continue;
		const entry = addrToKey.get(addr);
		if (!entry) continue;
		if (!result.has(entry.key)) {
			result.set(entry.key, {
				addr: parseHexByte(entry.key) ?? addr,
				label: entry.label,
				events: [],
			});
		}
		result.get(entry.key)!.events.push({ x: event.time, data });
	}

	return result;
}

/**
 * Render an LFO waveform lane below the channel lanes.
 * Events are plotted as dots whose Y position is proportional to the data byte value,
 * scaled between the observed minimum and maximum so the full lane height is used.
 */
function renderLfoLane(
	container: HTMLElement,
	label: string,
	lfoEvts: { x: number; data: number }[],
	trackWidth: number,
): void {
	if (lfoEvts.length === 0) return;

	let minVal = Number.POSITIVE_INFINITY;
	let maxVal = Number.NEGATIVE_INFINITY;
	for (const e of lfoEvts) {
		if (e.data < minVal) minVal = e.data;
		if (e.data > maxVal) maxVal = e.data;
	}

	const lane = createLane(`${label} ${minVal}–${maxVal}`, trackWidth);
	container.appendChild(lane.root);

	const valueRange = maxVal - minVal;
	const usableHeight = TRACK_HEIGHT - EVENT_WIDTH;

	for (const e of lfoEvts) {
		const ratio = valueRange > 0 ? (e.data - minVal) / valueRange : 0.5;
		// High data value → low Y (top of track); low data value → high Y (bottom)
		const top = Math.round((1 - ratio) * usableHeight);

		const dot = document.createElement("div");
		dot.className = "log-visualizer-event log-visualizer-event--lfo";
		dot.style.left = `${Math.max(0, Math.min(trackWidth - EVENT_WIDTH, e.x * PIXELS_PER_SECOND))}px`;
		dot.style.top = `${top}px`;
		dot.title = `t=${e.x.toFixed(3)}s data=0x${e.data.toString(16).padStart(2, "0")}`;
		lane.track.appendChild(dot);
	}
}

function detectChannel(
	addrHex: string,
	dataHex: string,
	channelCount: number,
): number | null {
	const addr = parseHexByte(addrHex);
	if (addr === null) return null;

	if (addr === 0x08) {
		const data = parseHexByte(dataHex);
		if (data !== null) {
			return data & 0x07;
		}
	}

	if (addr >= 0x20) {
		const channel = addr & 0x07;
		return channel < channelCount ? channel : null;
	}

	return null;
}

function buildNoteSegments(
	events: YmLogEvent[],
	channelCount: number,
): NoteSegment[] {
	// YM2151 has exactly 8 channels; KC and KF registers are always 0x28-0x2F and 0x30-0x37.
	const ymChannelCount = Math.min(channelCount, DEFAULT_CHANNELS);
	const channelKC: number[] = Array(channelCount).fill(0);
	const channelKF: number[] = Array(channelCount).fill(0);
	const channelNoteOn: Array<NoteOnState | null> =
		Array(channelCount).fill(null);
	const segments: NoteSegment[] = [];

	for (const event of events) {
		const addr = parseHexByte(event.addr);
		const data = parseHexByte(event.data);
		if (addr === null || data === null) continue;

		if (addr >= KC_REGISTER_BASE && addr < KC_REGISTER_BASE + ymChannelCount) {
			const ch = addr - KC_REGISTER_BASE;
			const newKC = data;
			// If KC changes while a note is held, close the current segment and
			// open a new one with the updated pitch (handles portamento/vibrato).
			if (channelNoteOn[ch] && channelKC[ch] !== newKC) {
				const noteOn = channelNoteOn[ch] as NoteOnState;
				const duration = event.time - noteOn.time;
				if (duration >= MIN_SEGMENT_SECONDS) {
					segments.push({
						startTime: noteOn.time,
						endTime: event.time,
						kc: noteOn.kc,
						kf: noteOn.kf,
						ch,
					});
					channelNoteOn[ch] = {
						time: event.time,
						kc: newKC,
						kf: channelKF[ch],
					};
				} else {
					// Too short to be individually visible; update pitch without splitting.
					channelNoteOn[ch] = { ...noteOn, kc: newKC, kf: channelKF[ch] };
				}
			}
			channelKC[ch] = newKC;
		}

		// KF register (0x30-0x37): fine pitch in 1/64 semitone steps.
		// Track changes alongside KC to show continuous vibrato motion.
		if (addr >= KF_REGISTER_BASE && addr < KF_REGISTER_BASE + ymChannelCount) {
			const ch = addr - KF_REGISTER_BASE;
			const newKF = data;
			if (channelNoteOn[ch] && channelKF[ch] !== newKF) {
				const noteOn = channelNoteOn[ch] as NoteOnState;
				// Avoid creating a zero-duration segment when KC and KF updates
				// occur at the same timestamp (the converter emits them as a pair).
				// Also skip splitting when the accumulated segment is too short to render.
				if (
					noteOn.time !== event.time &&
					event.time - noteOn.time >= MIN_SEGMENT_SECONDS
				) {
					segments.push({
						startTime: noteOn.time,
						endTime: event.time,
						kc: noteOn.kc,
						kf: noteOn.kf,
						ch,
					});
					channelNoteOn[ch] = {
						time: event.time,
						kc: channelKC[ch],
						kf: newKF,
					};
				} else {
					// Coalesce into the current segment (same-timestamp or sub-pixel).
					channelNoteOn[ch] = { ...noteOn, kf: newKF };
				}
			}
			channelKF[ch] = newKF;
		}

		if (addr === 0x08) {
			const ch = data & 0x07;
			const operators = (data >> 3) & 0x0f;
			if (ch >= 0 && ch < channelCount) {
				if (operators !== 0) {
					if (!channelNoteOn[ch]) {
						channelNoteOn[ch] = {
							time: event.time,
							kc: channelKC[ch],
							kf: channelKF[ch],
						};
					}
				} else {
					const noteOn = channelNoteOn[ch];
					if (noteOn) {
						segments.push({
							startTime: noteOn.time,
							endTime: event.time,
							kc: noteOn.kc,
							kf: noteOn.kf,
							ch,
						});
						channelNoteOn[ch] = null;
					}
				}
			}
		}
	}

	const lastTime =
		events.length > 0
			? events[events.length - 1].time + UNCLOSED_NOTE_EXTENSION_S
			: 0;
	for (let ch = 0; ch < channelCount; ch++) {
		const noteOn = channelNoteOn[ch];
		if (noteOn) {
			segments.push({
				startTime: noteOn.time,
				endTime: lastTime,
				kc: noteOn.kc,
				kf: noteOn.kf,
				ch,
			});
		}
	}

	return segments;
}

/**
 * Combined pitch value for visualization.
 *
 * YM2151 KC layout:
 *   - high nibble: YM2151 octave (0–7); each octave starts at C#
 *   - low nibble: note code (from NOTE_TABLE), decoded via NOTE_CODE_TO_SEMITONE
 *
 * We decode KC into a linear semitone index and then add KF (fine pitch,
 * 1/64 semitone per step) so that the result is monotonic with actual pitch
 * and suitable for Y-axis placement.
 */
function notePitch(kc: number, kf: number): number {
	const octave = (kc >> 4) & 0x07;
	const noteCode = kc & 0x0f;
	const semitone = NOTE_CODE_TO_SEMITONE[noteCode] ?? noteCode;
	const linearSemitone = octave * 12 + semitone;
	const fine = kf & 0x3f; // KF is 6-bit, 0-63
	return linearSemitone * 64 + fine;
}

function computePitchRange(segments: NoteSegment[]): {
	min: number;
	max: number;
} {
	if (segments.length === 0) return { min: 0, max: 0 };
	let min = notePitch(segments[0].kc, segments[0].kf);
	let max = min;
	for (const seg of segments) {
		const pitch = notePitch(seg.kc, seg.kf);
		if (pitch < min) min = pitch;
		if (pitch > max) max = pitch;
	}
	return { min, max };
}

function noteYPosition(
	kc: number,
	kf: number,
	minPitch: number,
	maxPitch: number,
): number {
	const pitch = notePitch(kc, kf);
	const range = maxPitch - minPitch;
	if (range === 0) return (TRACK_HEIGHT - NOTE_BAR_HEIGHT) / 2;
	return ((maxPitch - pitch) / range) * (TRACK_HEIGHT - NOTE_BAR_HEIGHT);
}

function normalizeEvents(parsed: unknown): YmLogEvent[] {
	if (!parsed || typeof parsed !== "object") return [];
	const rawEvents = (parsed as { events?: unknown }).events;
	if (!Array.isArray(rawEvents)) return [];

	return rawEvents
		.map((event) => {
			if (!event || typeof event !== "object") return null;
			const e = event as { time?: unknown; addr?: unknown; data?: unknown };
			const time =
				typeof e.time === "number"
					? e.time
					: typeof e.time === "string"
						? Number(e.time)
						: Number.NaN;
			const addr = typeof e.addr === "string" ? e.addr : "";
			const data = typeof e.data === "string" ? e.data : "";
			if (!Number.isFinite(time) || !addr || !data) return null;
			return { time, addr, data };
		})
		.filter((e): e is YmLogEvent => Boolean(e));
}

function laneColor(index: number | null): string {
	if (index === null) return "#8a8a8a";
	const hue = (index * 37) % 360;
	return `hsl(${hue}, 70%, 55%)`;
}

function createLane(label: string, trackWidth: number): LaneElements {
	const root = document.createElement("div");
	root.className = "log-visualizer-lane";

	const labelEl = document.createElement("div");
	labelEl.className = "log-visualizer-label";
	labelEl.textContent = label;
	root.appendChild(labelEl);

	const track = document.createElement("div");
	track.className = "log-visualizer-track";
	track.style.width = `${trackWidth}px`;
	root.appendChild(track);

	return { root, track };
}

function computeTrackWidth(events: YmLogEvent[]): number {
	const maxTime = events.reduce((max, e) => Math.max(max, e.time), 0);
	const width = maxTime * PIXELS_PER_SECOND + 40;
	return Math.min(MAX_TRACK_WIDTH, Math.max(MIN_TRACK_WIDTH, width));
}

export function createLogVisualizer(
	container: HTMLElement | null,
	options?: { channelCount?: number },
): LogVisualizer {
	if (!container) {
		return {
			renderFromJson: () => {
				/* no-op */
			},
			clear: () => {
				/* no-op */
			},
			setLfoRegisters: () => {
				/* no-op */
			},
		};
	}

	const channelCount = Math.max(
		1,
		Math.min(16, options?.channelCount ?? DEFAULT_CHANNELS),
	);

	let lfoRegisters: LfoRegisterConfig[] = [];
	let lastJsonText: string | null | undefined = null;

	const renderEmpty = (message: string) => {
		container.classList.add("log-visualizer", "log-visualizer--empty");
		container.innerHTML = "";
		const empty = document.createElement("div");
		empty.className = "log-visualizer-empty";
		empty.textContent = message;
		container.appendChild(empty);
	};

	const renderFromJson = (jsonText: string | null | undefined) => {
		lastJsonText = jsonText;
		if (!jsonText || jsonText.trim().length === 0) {
			renderEmpty("変換結果がまだありません。");
			return;
		}

		let events: YmLogEvent[] = [];
		try {
			const parsed = JSON.parse(jsonText);
			events = normalizeEvents(parsed);
		} catch {
			renderEmpty("ログ JSON を解釈できませんでした。");
			return;
		}

		if (events.length === 0) {
			renderEmpty("描画できるイベントがありません。");
			return;
		}

		const trackWidth = computeTrackWidth(events);
		const segments = buildNoteSegments(events, channelCount);
		const { min: minPitch, max: maxPitch } = computePitchRange(segments);

		container.classList.add("log-visualizer");
		container.classList.remove("log-visualizer--empty");
		container.innerHTML = "";

		const lanes: Record<string, LaneElements> = {};

		for (let ch = 0; ch < channelCount; ch += 1) {
			const lane = createLane(`CH ${ch}`, trackWidth);
			container.appendChild(lane.root);
			lanes[ch.toString()] = lane;
		}

		let globalLane: LaneElements | null = null;
		const ensureGlobalLane = () => {
			if (globalLane) return globalLane;
			globalLane = createLane("GLOBAL", trackWidth);
			container.appendChild(globalLane.root);
			return globalLane;
		};

		// Render other events as small background dots (rendered first so note bars appear on top)
		// Build a set of all LFO-resolved addresses so they can be suppressed in channel lanes
		const lfoAddrSet = new Set<number>();
		for (const lfoDef of lfoRegisters) {
			const base = parseHexByte(lfoDef.baseRegister);
			if (base === null) continue;
			for (let ch = 0; ch < channelCount; ch++) {
				lfoAddrSet.add(resolveRegisterForChannel(base, ch));
			}
		}

		events.forEach((event, index) => {
			const addr = parseHexByte(event.addr);
			// Suppress KC (0x28-0x2F), KF (0x30-0x37), KEY ON/OFF (0x08), and LFO registers —
			// these are shown via note bars or dedicated LFO lanes.
			const isKcKfOrKeyOn =
				addr !== null &&
				((addr >= KC_REGISTER_BASE &&
					addr < KC_REGISTER_BASE + DEFAULT_CHANNELS) ||
					(addr >= KF_REGISTER_BASE &&
						addr < KF_REGISTER_BASE + DEFAULT_CHANNELS) ||
					addr === 0x08);
			if (isKcKfOrKeyOn) return;
			if (addr !== null && lfoAddrSet.has(addr)) return;

			const channel = detectChannel(event.addr, event.data, channelCount);
			const lane =
				channel !== null && channel >= 0 && channel < channelCount
					? lanes[channel.toString()]
					: ensureGlobalLane();
			const marker = document.createElement("div");
			marker.className = "log-visualizer-event";
			marker.style.left = `${Math.max(0, Math.min(trackWidth - EVENT_WIDTH, event.time * PIXELS_PER_SECOND))}px`;
			marker.style.backgroundColor = laneColor(channel);
			marker.title = `t=${event.time.toFixed(3)}s addr=${event.addr} data=${event.data} (#${index})`;
			lane.track.appendChild(marker);
		});

		// Render note bars on top (piano-roll style: keyon/off + KC pitch)
		for (const seg of segments) {
			const lane = lanes[seg.ch.toString()];
			if (!lane) continue;
			const bar = document.createElement("div");
			bar.className = "log-visualizer-note";
			const x = Math.max(
				0,
				Math.min(
					trackWidth - MIN_NOTE_WIDTH,
					seg.startTime * PIXELS_PER_SECOND,
				),
			);
			const w = Math.max(
				MIN_NOTE_WIDTH,
				(seg.endTime - seg.startTime) * PIXELS_PER_SECOND - NOTE_WIDTH_GAP,
			);
			const y = noteYPosition(seg.kc, seg.kf, minPitch, maxPitch);
			bar.style.left = `${x}px`;
			bar.style.width = `${w}px`;
			bar.style.top = `${y}px`;
			bar.style.backgroundColor = laneColor(seg.ch);
			bar.title = `CH${seg.ch} KC=0x${seg.kc.toString(16).padStart(2, "0")} KF=0x${seg.kf.toString(16).padStart(2, "0")} t=${seg.startTime.toFixed(3)}-${seg.endTime.toFixed(3)}s`;
			lane.track.appendChild(bar);
		}

		// Render LFO waveform lanes (one per configured LFO base register)
		if (lfoRegisters.length > 0) {
			const lfoData = collectLfoEvents(events, lfoRegisters, channelCount);
			for (const [, entry] of lfoData) {
				renderLfoLane(container, entry.label, entry.events, trackWidth);
			}
		}
	};

	renderEmpty("YM2151 ログを変換するとここに描画します。");

	const setLfoRegisters = (registers: LfoRegisterConfig[]) => {
		lfoRegisters = registers;
		// Re-render with the new LFO config if we already have data
		if (lastJsonText != null) {
			renderFromJson(lastJsonText);
		}
	};

	return {
		renderFromJson,
		clear: () => {
			lastJsonText = null;
			renderEmpty("YM2151 ログを変換するとここに描画します。");
		},
		setLfoRegisters,
	};
}

{% endraw %}
```

### demo-library/tone-json-demo.ts
```ts
{% raw %}
import "./style.css";

import { smf_to_ym2151_json_with_attachment } from "smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js";
import {
	ensureWasmInitialized,
	ensureWebYm2151,
	setEventCountDisplay,
	setStatus,
	updateOutput,
} from "./shared-demo";
import { createLogVisualizer } from "./log-visualizer";
import {
	ATTACHMENT_PRESETS,
	YM_LOG_STYLE_PRESET,
	normalizeAttachmentText,
} from "./tone-json-attachment";
import {
	ensureMmlRuntime,
	getMmlParser,
	getParseTreeJsonToSmf,
	treeToJson,
} from "./tone-json-mml";

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;
let mmlDebounce: number | null = null;
let latestMidiRequestId = 0;
let latestAutoPlayId = 0;
let lastMidiSource: "file" | "mml" | null = null;

const toneJsonField = document.getElementById(
	"tone-json",
) as HTMLTextAreaElement | null;
const conversionOutput = document.getElementById(
	"conversion-output",
) as HTMLPreElement | null;
const conversionStatus = document.getElementById("conversion-status");
const attachmentStatus = document.getElementById("attachment-status");
const fileStatus = document.getElementById("file-status");
const mmlStatus = document.getElementById("mml-status");
const eventCount = document.getElementById("event-count");
const jsonEditor = document.getElementById(
	"jsonEditor",
) as HTMLTextAreaElement | null;
const playButton = document.getElementById(
	"play-audio",
) as HTMLButtonElement | null;
const attachmentPresetSelect = document.getElementById(
	"attachment-preset",
) as HTMLSelectElement | null;
const webYmStatus = document.getElementById("web-ym-status");
const mmlInput = document.getElementById(
	"mml-input",
) as HTMLTextAreaElement | null;
const logVisualizer = createLogVisualizer(
	document.getElementById("log-visualizer"),
);

function updateOutputWithState(text: string): void {
	currentOutput = text;
	updateOutput(text, conversionOutput, jsonEditor, () => {
		logVisualizer.renderFromJson(text);
		updatePlayButtonState();
	});
}

function updatePlayButtonState(): void {
	if (!playButton) return;
	playButton.disabled = !currentOutput;
}

async function convertMmlToSmf(trigger: string): Promise<void> {
	if (!mmlInput) return;
	const mmlText = mmlInput.value.trim();
	if (mmlText.length === 0) {
		if (lastMidiSource === "mml") {
			midiBytes = null;
			lastMidiSource = null;
		}
		setStatus(mmlStatus, "MML を入力すると SMF を生成します。");
		return;
	}

	const requestId = ++latestMidiRequestId;
	const initialized = await ensureMmlRuntime(mmlStatus);
	if (!initialized || !getMmlParser() || !getParseTreeJsonToSmf()) {
		return;
	}
	if (requestId !== latestMidiRequestId) {
		return;
	}

	try {
		const parser = getMmlParser()!;
		const smfConverter = getParseTreeJsonToSmf()!;
		const tree = parser.parse(mmlText);
		const treeJson = JSON.stringify(treeToJson(tree.rootNode, mmlText));
		const smfBytes = smfConverter(treeJson, mmlText);
		const midiArray =
			smfBytes instanceof Uint8Array ? smfBytes : new Uint8Array(smfBytes);

		if (requestId !== latestMidiRequestId) {
			return;
		}

		midiBytes = midiArray;
		lastMidiSource = "mml";
		setStatus(
			fileStatus,
			`MML 入力を SMF に変換しました (${midiArray.byteLength} bytes)`,
		);
		setStatus(mmlStatus, "MML から SMF への変換が完了しました。");
		void runConversion(trigger);
	} catch (error) {
		if (requestId !== latestMidiRequestId) {
			return;
		}
		setStatus(
			mmlStatus,
			`MML 変換に失敗しました: ${(error as Error).message}`,
			true,
		);
	}
}

function readAttachmentBytes(): Uint8Array | null {
	if (!toneJsonField) {
		return new Uint8Array();
	}

	const original = toneJsonField.value;
	const normalized = normalizeAttachmentText(original, attachmentStatus);
	if (normalized === null) {
		return null;
	}
	if (normalized.length === 0) {
		return new Uint8Array();
	}

	toneJsonField.value = normalized;
	if (
		attachmentPresetSelect &&
		attachmentPresetSelect.value !== "" &&
		normalized.trim() !== original.trim()
	) {
		attachmentPresetSelect.value = "";
	}
	return new TextEncoder().encode(normalized);
}

async function runConversion(trigger: string): Promise<void> {
	if (!wasmReady) {
		setStatus(conversionStatus, "WASM 初期化中です。少しお待ちください...");
		return;
	}
	if (!midiBytes) {
		setStatus(
			conversionStatus,
			"SMF ファイルを選択するか、MML を入力してください。",
			true,
		);
		return;
	}

	const attachmentBytes = readAttachmentBytes();
	if (attachmentBytes === null) {
		updatePlayButtonState();
		return;
	}

	try {
		const triggerLabel =
			lastMidiSource === "mml"
				? `${trigger} (MML 入力)`
				: lastMidiSource === "file"
					? `${trigger} (SMF ファイル)`
					: trigger;
		setStatus(conversionStatus, `変換中... (${triggerLabel})`);
		const result = smf_to_ym2151_json_with_attachment(
			midiBytes,
			attachmentBytes,
		);
		const parsed = JSON.parse(result);
		const formatted = JSON.stringify(parsed, null, 2);
		setEventCountDisplay(
			eventCount,
			typeof parsed.event_count === "number" ? parsed.event_count : undefined,
		);
		updateOutputWithState(formatted);
		setStatus(conversionStatus, "変換が完了しました。");
		void handlePlay(++latestAutoPlayId);
	} catch (error) {
		updateOutputWithState("");
		setEventCountDisplay(eventCount, undefined);
		setStatus(
			conversionStatus,
			`変換に失敗しました: ${(error as Error).message}`,
			true,
		);
	}
}

async function handlePlay(autoPlayId?: number): Promise<void> {
	if (!currentOutput) {
		setStatus(conversionStatus, "先に SMF を変換してください。", true);
		return;
	}
	setStatus(conversionStatus, "web-ym2151 で再生します...");
	try {
		const api = await ensureWebYm2151();
		if (autoPlayId !== undefined && autoPlayId !== latestAutoPlayId) {
			return;
		}
		api.playAudioWithOverlay();
		setStatus(conversionStatus, "再生コマンドを送信しました。");
	} catch (error) {
		setStatus(
			conversionStatus,
			`再生に失敗しました: ${(error as Error).message}`,
			true,
		);
	}
}

function setupAttachmentEditor(): void {
	if (!toneJsonField) return;
	toneJsonField.value = YM_LOG_STYLE_PRESET;

	if (attachmentPresetSelect) {
		attachmentPresetSelect.innerHTML = "";
		const manualOption = document.createElement("option");
		manualOption.value = "";
		manualOption.textContent = "手動入力";
		attachmentPresetSelect.appendChild(manualOption);
		ATTACHMENT_PRESETS.forEach((preset) => {
			const option = document.createElement("option");
			option.value = preset.id;
			option.textContent = preset.label;
			attachmentPresetSelect.appendChild(option);
		});
		attachmentPresetSelect.value = ATTACHMENT_PRESETS[0]?.id ?? "";
		attachmentPresetSelect.addEventListener("change", () => {
			const preset = ATTACHMENT_PRESETS.find(
				(p) => p.id === attachmentPresetSelect.value,
			);
			if (!preset || !toneJsonField) return;
			toneJsonField.value = preset.value;
			void runConversion(`プリセット選択: ${preset.label}`);
		});
	}

	toneJsonField.addEventListener("input", () => {
		if (attachmentPresetSelect && attachmentPresetSelect.value !== "") {
			attachmentPresetSelect.value = "";
		}
		if (attachmentDebounce) {
			window.clearTimeout(attachmentDebounce);
		}
		attachmentDebounce = window.setTimeout(() => {
			void runConversion("音色 JSON 更新");
		}, 400);
	});
}

function setupMmlInput(): void {
	if (!mmlInput) return;
	mmlInput.addEventListener("input", () => {
		if (mmlDebounce) {
			window.clearTimeout(mmlDebounce);
		}
		mmlDebounce = window.setTimeout(() => {
			void convertMmlToSmf("MML 更新");
		}, 400);
	});
}

function setupMidiInput(): void {
	const midiInput = document.getElementById(
		"midi-input",
	) as HTMLInputElement | null;
	if (!midiInput) return;

	midiInput.addEventListener("change", async (event) => {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file) {
			midiBytes = null;
			lastMidiSource = null;
			latestMidiRequestId += 1;
			updateOutputWithState("");
			setEventCountDisplay(eventCount, undefined);
			setStatus(fileStatus, "SMF ファイルを選択してください。");
			updatePlayButtonState();
			return;
		}

		setStatus(fileStatus, `${file.name} を読み込み中...`);
		try {
			const requestId = ++latestMidiRequestId;
			const arrayBuffer = await file.arrayBuffer();
			if (requestId !== latestMidiRequestId) {
				return;
			}
			midiBytes = new Uint8Array(arrayBuffer);
			lastMidiSource = "file";
			setStatus(
				fileStatus,
				`${file.name} を読み込みました (${midiBytes.byteLength} bytes)`,
			);
			void runConversion("MIDI 更新");
		} catch (error) {
			midiBytes = null;
			lastMidiSource = null;
			setStatus(
				fileStatus,
				`読み込みに失敗しました: ${(error as Error).message}`,
				true,
			);
		}
	});
}

function bootstrapWebYm(): void {
	setStatus(webYmStatus, "web-ym2151 を準備中...");
	ensureWebYm2151()
		.then(() => {
			setStatus(webYmStatus, "web-ym2151 準備完了");
			updatePlayButtonState();
		})
		.catch((error) => {
			setStatus(
				webYmStatus,
				`web-ym2151 の準備に失敗しました: ${(error as Error).message}`,
				true,
			);
		});
}

async function initializeWasm(): Promise<void> {
	wasmReady = await ensureWasmInitialized(
		(message, isError) => setStatus(conversionStatus, message, isError),
		"WASM 初期化完了。MIDI を読み込んでください。",
	);
}

function main(): void {
	setupAttachmentEditor();
	setupMidiInput();
	setupMmlInput();
	updateOutputWithState("");
	updatePlayButtonState();
	bootstrapWebYm();
	void initializeWasm();

	if (playButton) {
		playButton.addEventListener("click", () => {
			void handlePlay();
		});
	}
}

document.addEventListener("DOMContentLoaded", main);

{% endraw %}
```

### issue-notes/171.md
```md
{% raw %}
# issue 添付音色JSONについて、プルダウンでコンパクトnibble形式を選んだあと、MMLでcを入力すると、nibbleが消えてym2151log形式になってしまいuserが混乱する #171
[issues #171](https://github.com/cat2151/smf-to-ym2151log-rust/issues/171)



{% endraw %}
```

### issue-notes/172.md
```md
{% raw %}
# issue 添付音色JSONについて、コンパクトnibble形式の項目名は、CompactToneをやめてregistersにする（web-ym2151に寄せる） #172
[issues #172](https://github.com/cat2151/smf-to-ym2151log-rust/issues/172)



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

### src/ym2151/converter_tests/effects.rs
```rs
{% raw %}
//! Effects tests for YM2151 converter (delay vibrato, portamento, LFO, pop noise, attack)
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

    // Collect KC events emitted strictly after the note-on time (> 0.5s) and within the
    // portamento glide window (up to 0.6s). Using strictly-greater-than excludes the initial
    // note-on KC write at exactly 0.5s emitted by the main converter, so only
    // portamento-driven KC updates are collected. A small epsilon is added to portamento_end
    // to avoid missing the final event that may be emitted at exactly stop_time.
    let note_on_time = 0.5_f64;
    let portamento_end = note_on_time + 0.1; // start_time + PORTAMENTO_TIME_SECONDS
    let kc_events_in_glide: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            e.addr == "0x28"
                && e.time > note_on_time
                && e.time <= portamento_end + f64::EPSILON * portamento_end
        })
        .collect();
    assert!(
        kc_events_in_glide.len() >= 2,
        "Portamento should emit multiple KC steps during the glide"
    );

    let (kc_second, kf_second) = midi_to_kc_kf(67);

    // Glide should include the previous pitch at the start
    assert!(
        kc_events_in_glide
            .iter()
            .any(|e| e.data == format!("0x{:02X}", kc_first)),
        "Glide should include the starting KC from the previous note"
    );
    // Glide must end at the target KC. Verify the LAST KC event in the glide window
    // equals the target, confirming the portamento fully reaches the destination note.
    let last_kc_in_glide = kc_events_in_glide
        .iter()
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    assert_eq!(
        last_kc_in_glide.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kc_second).as_str()),
        "Portamento glide must reach the target KC at the end of the glide"
    );

    // The last KF event in the glide window must also match the target pitch exactly.
    let last_kf_in_glide = result
        .events
        .iter()
        .filter(|e| {
            e.addr == "0x30"
                && e.time > note_on_time
                && e.time <= portamento_end + f64::EPSILON * portamento_end
        })
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    assert_eq!(
        last_kf_in_glide.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kf_second).as_str()),
        "Portamento glide must reach the target KF at the end of the glide"
    );
}

#[test]
fn test_portamento_one_octave_reaches_target() {
    // Verify that a 1-octave portamento (C4 -> C5) always writes the target KC at stop_time.
    // Previously, the loop's time_step didn't align with stop_time, leaving the portamento
    // stuck just below the target note.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60, // C4
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
                note: 72, // C5 (one octave up)
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 0,
                note: 72,
            },
        ],
    };

    let options = ConversionOptions {
        portamento: true,
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let (kc_target, kf_target) = midi_to_kc_kf(72); // C5
    let note_on_time = 0.5_f64;
    let portamento_end = note_on_time + 0.1; // start_time + PORTAMENTO_TIME_SECONDS
                                             // A small epsilon is added to portamento_end to avoid missing the final event
                                             // that is emitted at exactly stop_time due to floating-point boundary effects.
    let portamento_end_with_eps = portamento_end + f64::EPSILON * portamento_end;

    // The last KC event written during the portamento window must be the target (C5).
    let last_portamento_kc = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > note_on_time && e.time <= portamento_end_with_eps)
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    assert_eq!(
        last_portamento_kc.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kc_target).as_str()),
        "1-octave portamento must reach the target KC (C5) at the end of the glide"
    );

    // The last KF event must also match, confirming the exact pitch lands on C5 (KF=0).
    let last_portamento_kf = result
        .events
        .iter()
        .filter(|e| e.addr == "0x30" && e.time > note_on_time && e.time <= portamento_end_with_eps)
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    assert_eq!(
        last_portamento_kf.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kf_target).as_str()),
        "1-octave portamento must reach the target KF (C5) at the end of the glide"
    );
}

#[test]
fn test_register_lfo_triangle_wave_smooth_transitions() {
    // Verify that the triangle LFO produces intermediate values (not just top/center/bottom)
    // and that consecutive register values differ by at most 1.
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
                ticks: 1920, // 2 seconds at 120 BPM
                channel: 0,
                note: 60,
            },
        ],
    };

    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 6.0,
            rate_hz: 4.0,
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let lfo_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();

    assert!(
        !lfo_events.is_empty(),
        "LFO should emit events for the TL register"
    );

    let values: Vec<u8> = lfo_events
        .iter()
        .filter_map(|e| {
            let hex = e.data.strip_prefix("0x")?;
            u8::from_str_radix(hex, 16).ok()
        })
        .collect();

    let unique_count = {
        let mut v = values.clone();
        v.sort_unstable();
        v.dedup();
        v.len()
    };
    assert!(
        unique_count > 3,
        "Triangle LFO with depth=6 should produce more than 3 unique values, got {}",
        unique_count
    );

    // All consecutive LFO value changes should be at most 1 (smooth transitions)
    for window in values.windows(2) {
        let diff = (window[0] as i16 - window[1] as i16).unsigned_abs();
        assert!(
            diff <= 1,
            "Consecutive LFO values should differ by at most 1, got diff={diff} ({} → {})",
            window[0],
            window[1]
        );
    }
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
3c81f4a Merge pull request #170 from cat2151/copilot/implement-linear-interpolation-feature
f71761c fix: use checked_add(1) for next_program to avoid silent u8 wraparound
73f6539 Add issue note for #172 [auto]
b85e46e Add issue note for #171 [auto]
eafa59a refactor: address code review feedback (rename test vars, use TIME_LOOP_EPSILON)
8b4cd1d feat: implement ChangeToNextTone linear interpolation and demo page
417913c Initial plan
583bc3b Add issue note for #169 [auto]
e361356 Add notes for ym2151log format and sound demo
8419ced style: auto-format with cargo fmt

### 変更されたファイル:
demo-library/index.html
demo-library/tone-interpolation-demo.ts
demo-library/tone-interpolation.html
demo-library/vite.config.ts
issue-notes/164.md
issue-notes/166.md
issue-notes/167.md
issue-notes/171.md
issue-notes/172.md
src/lib.rs
src/ym2151/converter.rs
src/ym2151/converter/register_effects.rs
src/ym2151/converter_tests/attachments.rs
tests/integration_program_change.rs


---
Generated at: 2026-03-09 07:08:13 JST

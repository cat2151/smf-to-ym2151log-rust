Last updated: 2026-02-13

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
- .github/actions-tmp/.github_automation/check-large-files/check-large-files.toml.example
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
- .github/actions-tmp/issue-notes/7.md
- .github/actions-tmp/issue-notes/8.md
- .github/actions-tmp/issue-notes/9.md
- .github/actions-tmp/package-lock.json
- .github/actions-tmp/package.json
- .github/actions-tmp/src/main.js
- .github/copilot-instructions.md
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
- demo-library/delay-vibrato-demo.ts
- demo-library/delay-vibrato.html
- demo-library/globals.d.ts
- demo-library/index.html
- demo-library/library-demo.ts
- demo-library/package-lock.json
- demo-library/package.json
- demo-library/style.css
- demo-library/tone-json-demo.ts
- demo-library/tone-json.html
- demo-library/tsconfig.json
- demo-library/vite.config.ts
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
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
b33378c Merge pull request #95 from cat2151/codex/add-json-parameter-for-tone
1458ccf Merge branch 'main' into codex/add-json-parameter-for-tone
6414c14 Fix tone demo playback wiring
b636bb0 Add attachment tone support and tone demo
e99a250 Auto-translate README.ja.md to README.md [auto]
fe1a9d2 Initial plan
2d24bc1 Merge pull request #94 from cat2151/codex/add-delay-vibrato-demo-page
c84927f fix: add timeout/cleanup and accessibility adjustments
38249d5 feat: add delay vibrato demo page
824774c Revise sound data format and add JSON demo page

### 変更されたファイル:
README.ja.md
README.md
WASM_USAGE.md
demo-library/delay-vibrato-demo.ts
demo-library/delay-vibrato.html
demo-library/globals.d.ts
demo-library/index.html
demo-library/library-demo.ts
demo-library/package-lock.json
demo-library/package.json
demo-library/style.css
demo-library/tone-json-demo.ts
demo-library/tone-json.html
demo-library/tsconfig.json
demo-library/vite.config.ts
issue-notes/90.md
issue-notes/93.md
src/lib.rs
src/midi/utils.rs
src/wasm.rs
src/ym2151/converter.rs
src/ym2151/converter_tests.rs
src/ym2151/event_processor.rs


---
Generated at: 2026-02-13 07:12:04 JST

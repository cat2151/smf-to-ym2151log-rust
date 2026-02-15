Last updated: 2026-02-16

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
- .github/actions-tmp/issue-notes/40.md
- .github/actions-tmp/issue-notes/42.md
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
- demo-library/tone-json-demo.ts
- demo-library/tone-json.html
- demo-library/tsconfig.json
- demo-library/vite.config.ts
- generated-docs/project-overview-generated-prompt.md
- googled947dc864c270e07.html
- issue-notes/105.md
- issue-notes/111.md
- issue-notes/112.md
- issue-notes/114.md
- issue-notes/115.md
- issue-notes/117.md
- issue-notes/122.md
- issue-notes/123.md
- issue-notes/125.md
- issue-notes/126.md
- issue-notes/128.md
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

## [Issue #117](../issue-notes/117.md): 添付JSONのdemoの開発用に、TypeScript側のformatterとlinterとしてBiomeを採用する
[issue-notes/117.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/117.md)

...
ラベル: 
--- issue-notes/117.md の内容 ---

```markdown
# issue 添付JSONのdemoの開発用に、TypeScript側のformatterとlinterとしてBiomeを採用する #117
[issues #117](https://github.com/cat2151/smf-to-ym2151log-rust/issues/117)



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

## [Issue #114](../issue-notes/114.md): 添付JSONのdemoはすべてauto playにする。SMF upload後すぐ、プルダウン決定後すぐ、MML入力デバウンス後、JSON編集デバウンス後、である
[issue-notes/114.md](https://github.com/cat2151/smf-to-ym2151log-rust/blob/main/issue-notes/114.md)

...
ラベル: 
--- issue-notes/114.md の内容 ---

```markdown
# issue 添付JSONのdemoはすべてauto playにする。SMF upload後すぐ、プルダウン決定後すぐ、MML入力デバウンス後、JSON編集デバウンス後、である #114
[issues #114](https://github.com/cat2151/smf-to-ym2151log-rust/issues/114)



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

## ドキュメントで言及されているファイルの内容
### .github/actions-tmp/issue-notes/14.md
```md
{% raw %}
# issue Development Status のdocument生成において、最初の小さな一歩 を実現する用のプロンプト生成がされなくなっている #14
[issues #14](https://github.com/cat2151/github-actions/issues/14)

## 何が困るの？
- #11の場合
- 期待値
    - 最初の小さな一歩 : [Issue #11]のtranslateについて、現在の処理フローを確認し、外部プロジェクトから利用する際にどのような情報（翻訳対象のファイルパス、ターゲット言語設定など）が必要となるかを明確にする。これにより、再利用可能なワークフロー設計の基礎を築く。
    - 最初の小さな一歩をagentに実行させるためのプロンプト : 現在のGitHub Actions翻訳ワークフロー（translate-readme.yml、call-translate-readme.yml、translate-readme.cjs）を分析し、外部プロジェクトから利用する際に必要となる設定項目を洗い出してください。具体的には、以下の観点から調査し、markdown形式でまとめてください：1) 必須入力パラメータ（現在はtarget-branchのみ） 2) 必須シークレット（GEMINI_API_KEY） 3) ファイル配置の前提条件（README.ja.md の存在、配置場所） 4) 翻訳対象ファイル名の制約（現在はREADME固定） 5) ブランチ・トリガー設定の制約 6) 外部プロジェクトでの利用時に追加で必要となりそうな設定項目の提案
- 実際の結果
    - 最初の小さな一歩: [Issue #11]のtranslateについて、現在の処理フローを確認し、外部プロジェクトから利用する際にどのような情報（翻訳対象のファイルパス、ターゲット言語設定など）が必要となるかを明確にする。これにより、再利用可能なワークフロー設計の基礎を築く。

## close条件
- 期待値のように、Agent実行プロンプト、が生成されること

## agentに修正させた
- development-status.md を修正させた
- test green

## closeとする

{% endraw %}
```

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

### .github/actions-tmp/issue-notes/17.md
```md
{% raw %}
# issue development-status が生成したmdに誤りがある。issue-note へのlinkがURL誤りで、404となってしまう #17
[issues #17](https://github.com/cat2151/github-actions/issues/17)

# 事例
- 生成したmdのURL：
    - https://github.com/cat2151/github-actions/blob/main/generated-docs/development-status.md
- そのmdをGitHub上でdecodeして閲覧したときのURL、404である：
    - https://github.com/cat2151/github-actions/blob/main/generated-docs/issue-notes/16.md
- そのmdに実際に含まれるURL：
    - issue-notes/16.md
- あるべきURL：
    - https://github.com/cat2151/github-actions/blob/main/issue-notes/16.md
- あるべきURLがmdにどう含まれているべきか：
    - ../issue-notes/16.md

# どうする？
- 案
    - promptを修正する
    - promptの場所は：
        - .github_automation/project_summary/scripts/development/DevelopmentStatusGenerator.cjs
    - 備考、cjs内にpromptがハードコーディングされており、promptをメンテしづらいので別途対処する : [issues #18](https://github.com/cat2151/github-actions/issues/18)

# 結果
- agentにpromptを投げた
    - ※promptは、development-statusで生成したもの
- レビューした
    - agentがフルパスで実装した、ことがわかった
- userが分析し、 ../ のほうが適切と判断した
    - ※「事例」コーナーを、あわせて修正した
- そのように指示してagentに修正させた
- testする

# 結果
- test green
- closeする

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

{% endraw %}
```

### issue-notes/114.md
```md
{% raw %}
# issue 添付JSONのdemoはすべてauto playにする。SMF upload後すぐ、プルダウン決定後すぐ、MML入力デバウンス後、JSON編集デバウンス後、である #114
[issues #114](https://github.com/cat2151/smf-to-ym2151log-rust/issues/114)



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

### issue-notes/117.md
```md
{% raw %}
# issue 添付JSONのdemoの開発用に、TypeScript側のformatterとlinterとしてBiomeを採用する #117
[issues #117](https://github.com/cat2151/smf-to-ym2151log-rust/issues/117)



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

## 最近の変更（過去7日間）
### コミット履歴:
9ed088a Add issue note for #128 [auto]
b69e10a Add issue note for #126 [auto]
d2f06d7 CIが落ちていたのをfmtで修正
e0760c3 CIがclippyで落ちていたので、clippy警告が出ないよう修正
a1c0c7e Add issue note for #125 [auto]
7d36766 CIが落ちていたのをclippyで修正
ef075b5 404で落ちていたので不要変数定義を削除
1b0d281 Update project summaries (overview & development status) [auto]
0f9af24 Modify JSON format for sound program changes
cd606b5 Add issue note for #123 [auto]

### 変更されたファイル:
demo-library/pop-noise-demo.ts
issue-notes/125.md
issue-notes/126.md
issue-notes/128.md
src/lib.rs
src/ym2151/converter/register_effects.rs
src/ym2151/init.rs
tests/integration_tests.rs


---
Generated at: 2026-02-16 07:08:27 JST

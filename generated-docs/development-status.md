Last updated: 2026-02-07

# Development Status

## 現在のIssues
- [Issue #65](../issue-notes/65.md) では、最近分離された`demo-library`の動作を手動で確認する必要があります。
- [Issue #33](../issue-notes/33.md) は、`ym2151-tone-editor`が生成するGM000 variations format JSONを既存トーンより優先して読み込む機能の追加を検討しています。
- [Issue #22](../issue-notes/22.md) では、`ym2151-tone-editor`を使って`tones/`ディレクトリに実際のJSONトーンファイルを配置する手動作業が必要です。

## 次の一手候補
1. [Issue #65](../issue-notes/65.md) demo-libraryの動作確認と現状レポート
   - 最初の小さな一歩: `demo-library/index.html`をブラウザで開き、コンソールエラーの有無と基本的なUIが機能しているかを確認する。
   - Agent実行プロンプト:
     ```
     対象ファイル: demo-library/index.html, demo-library/library-demo.ts, demo-library/package.json

     実行内容: `demo-library`ディレクトリ内のファイル構造と依存関係を分析し、`demo-library`がローカル環境でビルド・実行可能であるかの確認手順を整理してください。特に、`index.html`が正しく`library-demo.ts`をロードしているか、`package.json`の依存関係が適切かを確認してください。

     確認事項: `demo-library`が`npm install`後にエラーなくビルドできるか。ブラウザで`index.html`を開いた際にコンソールにエラーが出ていないか。

     期待する出力: `demo-library`の動作確認に必要な環境設定、ビルド手順、および簡易的な動作確認結果（エラーの有無、主要機能の動作状況）をmarkdown形式で報告してください。
     ```

2. [Issue #33](../issue-notes/33.md) 新規トーンファイル読み込み優先順位変更の設計検討
   - 最初の小さな一歩: `src/ym2151/tone.rs`および関連ファイルで現在のトーンファイル読み込みロジックを特定し、新しいトーンディレクトリをどこに配置し、どのように優先して扱うかの設計方針を検討する。
   - Agent実行プロンプト:
     ```
     対象ファイル: src/ym2151/tone.rs, src/ym2151/mod.rs

     実行内容: `src/ym2151`ディレクトリ内のトーンファイル読み込みロジックを分析し、[Issue #33](../issue-notes/33.md) の「ym2151-tone-editorの出力するGM000 variations format jsonがある場合、従来のtones/より優先して読み込む」という新仕様を実現するための設計案をmarkdown形式で作成してください。設計案には、新しいトーンディレクトリのパス指定方法（例：環境変数、設定ファイル、または固定パス）、既存の`tones/`ディレクトリに対する優先順位決定ロジック、およびsymlinkを用いた検証の実現可能性について具体的に記述してください。

     確認事項: 既存のトーンファイル(`tones/000.json`など)の読み込み処理が新しいロジックによって意図せず変更されたり、壊れたりしないこと。新しいトーンディレクトリがプロジェクトルートからの相対パスでどのように参照されるか。

     期待する出力: 新しいトーンファイル読み込みロジックの設計案をMarkdown形式で出力してください。
     ```

3. [Issue #22](../issue-notes/22.md) ym2151-tone-editorを用いたダミートーンファイルの生成手順文書化
   - 最初の小さな一歩: `ym2151-tone-editor`の利用方法（入手、起動、基本的な操作）を調査し、単一のダミートーンファイル（例: `tones/001.json`）を実際に生成してみる。
   - Agent実行プロンプト:
     ```
     対象ファイル: tones/README.md (新規作成または追記対象), tones/000.json (分析対象)

     実行内容: [Issue #22](../issue-notes/22.md) のタスク達成のため、`ym2151-tone-editor`を利用して`tones/000.json`から`127.json`のようなGM音色ファイルを生成し、現在のプロジェクトの`tones/`ディレクトリに配置するための具体的な手順を調査し、markdown形式で文書化してください。特に、`ym2151-tone-editor`の入手方法、基本的な操作、出力ファイルの形式と内容の確認、`tones/`ディレクトリへの配置方法、および既存の`tones/000.json`との互換性について含めてください。

     確認事項: 生成されるJSONファイルが既存の`tones/000.json`と同じ形式構造を持ち、プロジェクトで利用可能であること。手順が明確で、他の開発者も再現可能であること。

     期待する出力: `ym2151-tone-editor`を使ったトーンファイル生成・配置手順書をmarkdown形式で出力してください。
     ```

---
Generated at: 2026-02-07 07:08:09 JST

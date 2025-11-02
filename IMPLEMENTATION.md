# Implementation Plan: smf-to-ym2151log-rust

## 概要 (Overview)

Standard MIDI Files (SMF) をYM2151レジスタ書き込みログ（JSON形式）に変換するRust実装の計画書です。
Python版の [smf-to-ym2151log](https://github.com/cat2151/smf-to-ym2151log) を参考に、Rustで再実装します。

This document outlines the implementation plan for a Rust version of smf-to-ym2151log, which converts Standard MIDI Files (SMF) to YM2151 register write logs in JSON format.

## 1. 入出力仕様 (Input/Output Specification)

### 入力 (Input)
- **フォーマット**: Standard MIDI File (.mid)
- **対応形式**: SMF Format 0, Format 1
- **コマンドライン引数**: 
  ```bash
  smf-to-ym2151log-rust <midi_file>
  ```

### 出力 (Output)

#### Pass A 出力 (Debug Events JSON)
- **ファイル名**: `<base_name>_events.json`
- **用途**: デバッグ用の中間イベントデータ
- **フォーマット例**:
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
      },
      {
        "type": "note_off",
        "ticks": 480,
        "channel": 0,
        "note": 60
      }
    ]
  }
  ```

#### Pass B 出力 (YM2151 Log JSON)
- **ファイル名**: `<base_name>_ym2151.json`
- **用途**: 最終的なYM2151レジスタ書き込みログ
- **仕様**: [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc) 互換形式
- **フォーマット例**:
  ```json
  {
    "event_count": 50,
    "events": [
      {
        "time": 0,
        "addr": "0x08",
        "data": "0x00"
      },
      {
        "time": 0,
        "addr": "0x28",
        "data": "0x4E"
      }
    ]
  }
  ```

#### フィールド説明
- `time`: サンプル時刻（整数、55930 Hzサンプルレートでの値）
- `addr`: YM2151レジスタアドレス（16進数文字列）
- `data`: 書き込むデータ（16進数文字列）

## 2. アーキテクチャ (Architecture)

### 2段階処理 (2-Pass Processing)

```
┌──────────────┐
│  MIDI File   │
│   (.mid)     │
└──────┬───────┘
       │
       ▼
┌──────────────────────┐
│   Pass A: Parser     │
│  MIDI → Events JSON  │
│  (midi_parser.rs)    │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│  Events JSON (Debug) │
│  <name>_events.json  │
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│  Pass B: Converter   │
│ Events → YM2151 Log  │
│ (ym2151_converter.rs)│
└──────┬───────────────┘
       │
       ▼
┌──────────────────────┐
│   YM2151 Log JSON    │
│ <name>_ym2151.json   │
└──────────────────────┘
```

### モジュール構成 (Module Structure)

```
src/
├── main.rs                  # エントリーポイント、CLI処理
├── lib.rs                   # ライブラリのルート
├── midi/
│   ├── mod.rs              # MIDIモジュールのルート
│   ├── parser.rs           # Pass A: MIDIファイル解析
│   ├── events.rs           # MIDIイベント構造体定義
│   └── utils.rs            # MIDIユーティリティ関数
├── ym2151/
│   ├── mod.rs              # YM2151モジュールのルート
│   ├── converter.rs        # Pass B: YM2151ログ変換
│   ├── events.rs           # YM2151イベント構造体定義
│   ├── init.rs             # YM2151チャンネル初期化
│   └── note_table.rs       # MIDI→YM2151音程変換テーブル
└── error.rs                 # エラー型定義

tests/
├── integration_tests.rs     # 統合テスト
└── test_data/              # テストデータ（MIDIファイルなど）
    └── test.mid
```

## 3. 利用ライブラリ (Dependencies)

### 必須ライブラリ

| ライブラリ | バージョン | 用途 | 理由 |
|-----------|-----------|------|------|
| `midly` | 0.5.x | MIDI解析 | 高速で安全なSMF解析、ゼロコピー設計 |
| `serde` | 1.0.x | シリアライズ | JSON出力に必要、Rust標準的な選択 |
| `serde_json` | 1.0.x | JSON処理 | 最も広く使われているJSONライブラリ |
| `anyhow` | 1.0.x | エラーハンドリング | シンプルで使いやすいエラー処理 |
| `thiserror` | 1.0.x | カスタムエラー型 | ライブラリ用のエラー型定義に最適 |

### 開発・テスト用ライブラリ

| ライブラリ | バージョン | 用途 |
|-----------|-----------|------|
| `criterion` | 0.5.x | ベンチマーク（オプション） |
| `proptest` | 1.0.x | プロパティベーステスト（オプション） |

### Cargo.toml 設定例

```toml
[package]
name = "smf-to-ym2151log-rust"
version = "0.1.0"
edition = "2021"
authors = ["cat2151"]
license = "MIT"
description = "Convert Standard MIDI Files to YM2151 register write log in JSON format"
repository = "https://github.com/cat2151/smf-to-ym2151log-rust"

[dependencies]
midly = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
criterion = { version = "0.5", optional = true }
proptest = { version = "1.0", optional = true }

[[bin]]
name = "smf-to-ym2151log-rust"
path = "src/main.rs"

[lib]
name = "smf_to_ym2151log"
path = "src/lib.rs"
```

## 4. テスト方針 (Testing Strategy)

### 4.1 ユニットテスト (Unit Tests)

各モジュールに対してユニットテストを実装：

#### midi_utils テスト
- [ ] MIDI音程番号からYM2151 KC/KF変換のテスト
  - 各オクターブの代表的な音程
  - 境界値（最小値、最大値）
  - オクターブクランプの検証
- [ ] ティック→秒変換のテスト
- [ ] 秒→サンプル数変換のテスト
- [ ] ティック→サンプル数変換のテスト

#### ym2151_converter テスト
- [ ] チャンネル初期化イベント生成のテスト
- [ ] Note OnイベントからYM2151イベント生成のテスト
- [ ] Note OffイベントからYM2151イベント生成のテスト
- [ ] テンポ変更の反映テスト

#### midi_parser テスト
- [ ] シンプルなMIDIファイル解析のテスト
- [ ] 複数トラックのMIDIファイル解析のテスト
- [ ] テンポ変更を含むMIDIファイルのテスト

### 4.2 統合テスト (Integration Tests)

- [ ] エンドツーエンドテスト：実際のMIDIファイルからJSON出力まで
- [ ] Python版との出力比較テスト（互換性確認）
- [ ] 複数のMIDIファイル形式での動作確認

### 4.3 テストデータ

```rust
// tests/test_data/ にテスト用MIDIファイルを配置
tests/
└── test_data/
    ├── simple_melody.mid    // 単純なメロディ
    ├── scale.mid            // 音階パターン
    ├── tempo_change.mid     // テンポ変更を含む
    └── multi_track.mid      // 複数トラック
```

### 4.4 Python版との互換性テスト

```bash
# Python版で変換
python smf_to_ym2151log.py test.mid
# Rust版で変換
cargo run -- test.mid
# 出力を比較
diff test_ym2151.json test_ym2151.json
```

### 4.5 テストカバレッジ目標

- ユニットテスト: 90%以上のコードカバレッジ
- 統合テスト: 主要なユースケースをカバー
- エラーハンドリング: 異常系テストの実装

## 5. 段階的実装 (Phased Implementation)

### フェーズ1: プロジェクト基盤構築 (Phase 1: Project Foundation)

**目標**: Rustプロジェクトのセットアップと基本構造の構築

**タスク**:
- [ ] Cargoプロジェクトの初期化
- [ ] 基本的なディレクトリ構造の作成
- [ ] 依存ライブラリの追加（Cargo.toml）
- [ ] 基本的なエラー型の定義（error.rs）
- [ ] CI/CD設定（GitHub Actions）
  - ビルドテスト
  - ユニットテスト実行
  - フォーマットチェック（rustfmt）
  - リントチェック（clippy）

**成果物**:
- 動作するRustプロジェクト構造
- エラー型定義
- CI/CDパイプライン

### フェーズ2: MIDI解析実装 (Phase 2: MIDI Parser Implementation)

**目標**: Pass A - MIDIファイルを中間イベントJSONに変換

**タスク**:
- [ ] MIDIイベント構造体の定義（midi/events.rs）
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub enum MidiEvent {
      NoteOn { ticks: u32, channel: u8, note: u8, velocity: u8 },
      NoteOff { ticks: u32, channel: u8, note: u8 },
      Tempo { ticks: u32, tempo_bpm: f64 },
      ProgramChange { ticks: u32, channel: u8, program: u8 },
  }
  ```
- [ ] MIDIファイル解析機能の実装（midi/parser.rs）
  - midlyクレートを使用したSMF解析
  - Note On/Off イベントの抽出
  - テンポイベントの抽出
  - プログラムチェンジイベントの抽出（将来の拡張用）
- [ ] 中間JSON出力機能の実装
- [ ] ユニットテストの作成
  - 基本的なMIDI解析テスト
  - テンポ変更を含むMIDI解析テスト

**成果物**:
- MIDIファイルを解析して中間JSONを出力する機能
- 対応するユニットテスト

### フェーズ3: MIDI→YM2151変換ユーティリティ (Phase 3: MIDI to YM2151 Utilities)

**目標**: MIDI音程からYM2151レジスタ値への変換ロジック実装

**タスク**:
- [ ] YM2151音程テーブルの実装（ym2151/note_table.rs）
  ```rust
  // YM2151 note table (C# to C)
  const NOTE_TABLE: [u8; 12] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14];
  ```
- [ ] MIDI音程→KC/KF変換関数の実装（midi/utils.rs）
  ```rust
  pub fn midi_to_kc_kf(midi_note: u8) -> (u8, u8) {
      // MIDI note to YM2151 KC (Key Code) and KF (Key Fraction)
  }
  ```
- [ ] タイミング変換関数の実装
  - ティック→秒変換
  - 秒→サンプル数変換
  - ティック→サンプル数変換（55930 Hz）
- [ ] 包括的なユニットテストの作成
  - 各オクターブの音程変換テスト
  - 境界値テスト（オクターブクランプ）
  - タイミング変換の精度テスト

**成果物**:
- MIDI→YM2151変換ユーティリティ関数群
- 対応するユニットテスト

### フェーズ4: YM2151変換実装 (Phase 4: YM2151 Converter Implementation)

**目標**: Pass B - 中間イベントからYM2151レジスタログへの変換

**タスク**:
- [ ] YM2151イベント構造体の定義（ym2151/events.rs）
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Ym2151Event {
      pub time: u32,
      pub addr: String,  // "0x08" format
      pub data: String,  // "0x4E" format
  }
  ```
- [ ] YM2151チャンネル初期化の実装（ym2151/init.rs）
  - 全8チャンネルの初期化（time=0でKEY OFF）
  - チャンネル0のデフォルトパラメータ設定
- [ ] YM2151変換コアロジックの実装（ym2151/converter.rs）
  - Note On → YM2151レジスタ書き込み（KC, KF, KEY ON）
  - Note Off → YM2151レジスタ書き込み（KEY OFF）
  - テンポ変更の反映
  - 発音中ノートの管理
- [ ] ユニットテストの作成
  - 初期化イベント生成テスト
  - Note On変換テスト
  - Note Off変換テスト
  - 複数ノートの処理テスト

**成果物**:
- YM2151ログ生成機能
- 対応するユニットテスト

### フェーズ5: メインプログラム統合 (Phase 5: Main Program Integration)

**目標**: コマンドラインインターフェースと2パス処理の統合

**タスク**:
- [ ] コマンドライン引数の解析（main.rs）
- [ ] Pass A + Pass B の統合処理フロー
  ```rust
  fn main() -> Result<()> {
      // 1. コマンドライン引数の取得
      // 2. MIDIファイルの解析 (Pass A)
      // 3. 中間JSON出力
      // 4. YM2151ログ変換 (Pass B)
      // 5. 最終JSON出力
      // 6. 進捗・統計情報の表示
  }
  ```
- [ ] エラーハンドリングの整備
- [ ] 統合テストの作成
  - エンドツーエンドテスト
  - Python版との出力比較テスト

**成果物**:
- 完全に動作するCLIプログラム
- 統合テスト

### フェーズ6: ドキュメントと仕上げ (Phase 6: Documentation and Polish)

**目標**: ドキュメント整備とコード品質の最終確認

**タスク**:
- [ ] README.mdの作成
  - インストール方法
  - 使い方（サンプル付き）
  - ビルド方法
  - テスト実行方法
- [ ] API ドキュメントの整備（docコメント）
- [ ] サンプルMIDIファイルの追加
- [ ] ベンチマーク追加（オプション）
- [ ] コード品質チェック
  - `cargo fmt` の実行
  - `cargo clippy` の実行
  - 未使用コードの削除
- [ ] セキュリティチェック
  - `cargo audit` の実行
  - 依存関係の脆弱性チェック

**成果物**:
- 完成したドキュメント
- クリーンで高品質なコードベース

## 6. Python版との比較 (Comparison with Python Version)

### 類似点 (Similarities)
- 2段階処理アーキテクチャ
- 同じ出力JSON形式
- 同じYM2151音程変換ロジック
- 同じサンプルレート（55930 Hz）

### 相違点 (Differences)

| 項目 | Python版 | Rust版 |
|------|----------|--------|
| **型安全性** | 実行時チェック | コンパイル時チェック |
| **エラーハンドリング** | 例外ベース | Result型ベース |
| **パフォーマンス** | インタープリタ | ネイティブコンパイル |
| **メモリ管理** | GC | 所有権システム |
| **依存ライブラリ** | mido | midly |
| **構造体定義** | dict | struct + enum |

### Rust版の利点 (Advantages of Rust Version)
- **型安全性**: コンパイル時に多くのバグを検出
- **パフォーマンス**: 大きなMIDIファイルの高速処理
- **メモリ効率**: 低メモリフットプリント
- **ゼロコスト抽象化**: 抽象化によるオーバーヘッドなし
- **並行処理**: 将来的な並行処理の実装が容易

## 7. 将来の拡張可能性 (Future Enhancements)

### 短期的な拡張 (Short-term)
- [ ] 複数チャンネルのサポート（ポリフォニック）
- [ ] MIDIコントローラーイベントの対応（ボリューム、パン）
- [ ] プログラムチェンジによる音色選択

### 中期的な拡張 (Mid-term)
- [ ] カスタム音色定義のサポート
- [ ] MIDIトラック選択機能
- [ ] 出力フォーマットの選択肢追加（バイナリ形式など）

### 長期的な拡張 (Long-term)
- [ ] リアルタイムMIDI入力対応
- [ ] GUIアプリケーション化
- [ ] 他のFM音源チップのサポート（OPM、OPN等）

## 8. 実装時の注意事項 (Implementation Notes)

### セキュリティ
- [ ] ファイル入力のバリデーション
- [ ] メモリ制限の設定（大きなMIDIファイル対策）
- [ ] パニックの回避（unwrap()の使用を最小限に）

### パフォーマンス
- [ ] 不要なクローンの回避
- [ ] イテレータの活用
- [ ] ヒープアロケーションの最小化

### コード品質
- [ ] Rustのイディオムに従う
- [ ] エラーメッセージの充実
- [ ] ログ出力の適切な実装
- [ ] テストの充実

## 9. 成功の定義 (Definition of Success)

このプロジェクトは以下の基準を満たした時に成功とみなされます：

1. **機能性**: Python版と同じMIDIファイルから同じYM2151ログJSONを生成
2. **テスト**: 90%以上のコードカバレッジとすべてのテストが合格
3. **ドキュメント**: 完全なREADMEとAPIドキュメント
4. **品質**: `cargo clippy`と`cargo fmt`がクリーン
5. **セキュリティ**: `cargo audit`で脆弱性なし
6. **パフォーマンス**: Python版と同等以上の処理速度

## 10. まとめ (Summary)

このRust実装は、Python版の設計思想を引き継ぎながら、Rustの型安全性とパフォーマンスの利点を活かした実装となります。段階的なアプローチにより、各フェーズで動作確認を行いながら確実に機能を構築していきます。

テスト駆動開発を採用し、各機能に対して包括的なテストを作成することで、高品質で保守性の高いコードベースを維持します。

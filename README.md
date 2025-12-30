# tracing_study

Rust の [`tracing`](https://docs.rs/tracing/latest/tracing/) クレートを使用したログ出力の学習用プロジェクトです。

## 概要

`tracing` と `tracing-subscriber` を使って、様々なログ出力の設定を試すことができます。

## 必要な環境

- Rust (edition 2024)
- Cargo

## セットアップ

```bash
# リポジトリをクローン後、依存関係をインストール
cargo build
```

## 実行方法

```bash
cargo run
```

## 依存クレート

- [tracing](https://crates.io/crates/tracing) - 構造化されたアプリケーションレベルの診断のためのフレームワーク
- [tracing-subscriber](https://crates.io/crates/tracing-subscriber) - `tracing` のサブスクライバを実装するためのユーティリティ

## 主な機能・設定項目

### ログレベル

`with_max_level` で出力するログレベルを指定できます。

- `ERROR` - エラー
- `WARN` - 警告
- `INFO` - 情報
- `DEBUG` - デバッグ
- `TRACE` - トレース

### フォーマット

3種類の出力フォーマットが選択可能です：

| フォーマット | 説明 |
|------------|------|
| `compact()` | 1行でコンパクトに出力（デフォルト） |
| `pretty()` | 複数行で見やすく出力 |
| `json()` | JSON形式で出力（`json` feature が必要） |

### 出力設定

| メソッド | 説明 | デフォルト |
|---------|------|-----------|
| `with_level()` | ログレベルを表示するか | `true` |
| `with_ansi()` | ANSIエスケープコードを出力するか | `true` |
| `with_file()` | ファイルパスを表示するか | `false` |
| `with_line_number()` | 行番号を表示するか | `false` |
| `with_target()` | ターゲット（モジュールパス）を表示するか | `true` |
| `with_thread_ids()` | スレッドIDを表示するか | `false` |
| `with_thread_names()` | スレッド名を表示するか | `false` |
| `with_timer()` | タイムスタンプの形式を指定 | システム時間 |
| `without_time()` | 時刻を出力しない | - |
| `with_writer()` | 出力先を指定（標準出力、ファイルなど） | 標準出力 |

### Span イベント

`with_span_events()` で span のライフサイクルイベントのログ出力を制御できます：

- `FmtSpan::NONE` - 何も出力しない（デフォルト）
- `FmtSpan::NEW` - span 作成時
- `FmtSpan::ENTER` - span に入るとき
- `FmtSpan::EXIT` - span から抜けるとき
- `FmtSpan::CLOSE` - span が閉じられるとき
- `FmtSpan::ACTIVE` - ENTER + EXIT
- `FmtSpan::FULL` - すべて

### JSON フォーマット専用設定

| メソッド | 説明 | デフォルト |
|---------|------|-----------|
| `flatten_event()` | イベントをフラットな構造にする | `false` |
| `with_current_span()` | 現在の span 情報を出力する | `true` |
| `with_span_list()` | ルートからの span リストを出力する | `true` |

## 出力例

```
2025-12-30T15:38:56.396182Z  INFO log_test: Hello world!
```

## 詳細なドキュメント

各設定項目の詳細な動作説明と出力例は [memo.md](memo.md) を参照してください。

## ライセンス

[LICENSE](LICENSE) ファイルを参照してください。
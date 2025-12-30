# tracing によるログ出力

## 概要

[tracing](https://github.com/tokio-rs/tracing) を使用したログ出力を試したので、その内容を記事に残します。  
`tracing` は、Rust のアプリケーションにおける構造化ログ、診断、計装のためのフレームワークです。Tokio プロジェクトの一部として開発されています。

## 環境

- OS: Ubuntu 24.04.2 LTS
- Rust: 1.88.0
- Cargo: 1.88.0
- tracing: 0.1.44
- tracing-subscriber: 0.3.22

## 動作確認

まずは Hello World を出力してみます。  

tracing(と依存パッケージ)を追加します。    
```shell
cargo add tracing tracing-subscriber
```

`src/main.rs`を以下の内容で作成します。  
```rust
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::info!("Hello world!");
}
```

実行してみると、以下のようにログが標準出力に出力されます。  
```
2025-12-30T15:38:56.396182Z  INFO log_test: Hello world!
```

## builder で指定できること

`tracing` は Builder パターンで設計されています。  
メソッドを組み合わせて、出力内容やフォーマットを指定できます。  

builder で指定できる内容を確認したので、記載します。  

### with_max_level

出力するログレベルを指定できます。  
何も指定しない場合 `INFO` 以上のログが出力されました。  

ログレベルには、以下の5種があります。  
- ERROR
- WARN
- INFO
- DEBUG
- TRACE

以下の場合 `DEBUG` 以上のログが出力されます。  
```rust
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
```

### with_level

ログにレベルを出力するか、を指定できます。  
何も指定しない場合、`true` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_level(false)
        .finish();
```

`true` で実行すると、ログレベルが出力されます。  
```
2025-12-30T16:17:29.662610Z ERROR log_test: level is error
2025-12-30T16:17:29.662649Z  WARN log_test: level is warn
2025-12-30T16:17:29.662670Z  INFO log_test: level is info
```

`false` で実行すると、ログレベルが出力されません。
```
2025-12-30T16:16:03.535466Z log_test: level is error
2025-12-30T16:16:03.535528Z log_test: level is warn
2025-12-30T16:16:03.535558Z log_test: level is info
```


### with_ansi

フォーマッタが色やその他のテキスト書式設定用の [ANSI エスケープコード](https://ja.wikipedia.org/wiki/ANSI%E3%82%A8%E3%82%B9%E3%82%B1%E3%83%BC%E3%83%97%E3%82%B3%E3%83%BC%E3%83%89)を発行するかどうかを設定できます。  
何も指定しない場合、`true` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_ansi(false)
        .finish();
```

`true` の場合、エスケープコードが含まれました。  
```
[2m2025-12-30T16:26:30.042366Z[0m [31mERROR[0m [2mlog_test[0m[2m:[0m level is error
[2m2025-12-30T16:26:30.042429Z[0m [33m WARN[0m [2mlog_test[0m[2m:[0m level is warn
[2m2025-12-30T16:26:30.042439Z[0m [32m INFO[0m [2mlog_test[0m[2m:[0m level is info
```

`false` の場合、エスケープコードは含まれませんでした。  
```
2025-12-30T16:26:50.011953Z ERROR log_test: level is error
2025-12-30T16:26:50.012005Z  WARN log_test: level is warn
2025-12-30T16:26:50.012015Z  INFO log_test: level is info
```

### with_file

ログを出力したファイルのパスを出力するかどうか、を指定できます。  
何も指定しない場合、`false` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_file(true)
        .finish();
```

`true` の場合、ファイルパスが含まれます。
```
2025-12-30T16:37:21.404083Z ERROR log_test: src/main.rs: level is error
2025-12-30T16:37:21.404141Z  WARN log_test: src/main.rs: level is warn
2025-12-30T16:37:21.404163Z  INFO log_test: src/main.rs: level is info
```

`false` の場合、ファイルパスが含まれません。
```
2025-12-30T16:39:23.308558Z ERROR log_test: level is error
2025-12-30T16:39:23.308600Z  WARN log_test: level is warn
2025-12-30T16:39:23.308621Z  INFO log_test: level is info
```

### with_line_number

ログを出力したソースコードの行番号を出力するかどうか、を指定できます。  
何も指定しない場合、`false` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .finish();
```

`true` の場合、行番号が含まれません。
```
2025-12-30T16:41:25.179766Z ERROR log_test: 16: level is error
2025-12-30T16:41:25.179804Z  WARN log_test: 17: level is warn
2025-12-30T16:41:25.179823Z  INFO log_test: 18: level is info
```

`false` の場合、行番号が含まれません。
```
2025-12-30T16:43:12.344339Z ERROR log_test: level is error
2025-12-30T16:43:12.344372Z  WARN log_test: level is warn
2025-12-30T16:43:12.344393Z  INFO log_test: level is info
```

上記は動作確認のため、他のオプションはすべてデフォルトの状態で実行していますが、実際は `with_file` とセットで使用することになると思います。  

### with_span_events

[span のライフサイクル](https://docs.rs/tracing/latest/tracing/span/index.html#the-span-lifecycle) のイベントをログに出力するかどうか、を指定できます。  
以下を指定できます。  

- `FmtSpan::NONE` : 何も出力しない（デフォルト）
- `FmtSpan::NEW` : `span` の作成時にログを出力する
- `FmtSpan::ENTER` : `span` に入るときにログを出力する
- `FmtSpan::EXIT` : `span` から抜けるときにログを出力する
- `FmtSpan::CLOSE` : `span` が閉じられるときにログを出力する
- `FmtSpan::ACTIVE` : `span` に入る・抜けるときにログを出力する
- `FmtSpan::FULL` : すべてのポイントでログを出力する

```rust
    let subscriber = FmtSubscriber::builder()
        .with_span_events(FmtSpan::NEW)
        .finish();
```

[span](https://docs.rs/tracing/latest/tracing/span/) は以下のように作成できるようです。  
```rust
    tracing::span!(tracing::Level::INFO, "my_span").in_scope(|| {
        tracing::info!("inside my_span");
    });
```

`span` には以下の４種類のイベントがあるようです
- `new` : `span` が作成されたときのイベント  
- `enter` : `span` のスコープに入ったときのイベント  
- `exit` : `span` のスコープから抜けたときのイベント  
- `close` : `span` が閉じられたときのイベント  

`FmtSpan::FULL` を指定して、すべてのイベントをログに出力した場合、以下のようになりました。  
```
2025-12-30T16:59:35.761308Z  INFO my_span: log_test: new
2025-12-30T16:59:35.761355Z  INFO my_span: log_test: enter
2025-12-30T16:59:35.761378Z  INFO my_span: log_test: inside my_span
2025-12-30T16:59:35.761404Z  INFO my_span: log_test: exit
2025-12-30T16:59:35.761445Z  INFO my_span: log_test: close time.busy=49.3µs time.idle=100µs
```

### with_target

ログのターゲット（筆者環境ではパッケージ名でした）を出力するか、を指定できます。  
何も指定しない場合、`true` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_target(true)
        .finish();
```

`true` の場合、ターゲットが含まれます。
```
2025-12-30T17:07:00.649175Z ERROR log_test: level is error
2025-12-30T17:07:00.649208Z  WARN log_test: level is warn
2025-12-30T17:07:00.649217Z  INFO log_test: level is info
```


`false` の場合、ターゲットが含まれません。
```
2025-12-30T17:06:13.628821Z ERROR level is error
2025-12-30T17:06:13.628857Z  WARN level is warn
2025-12-30T17:06:13.628875Z  INFO level is info
```

### with_thread_ids

スレッドIDを出力するかどうか、を指定できます。  
何も指定しない場合、`false` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_thread_ids(false)
        .finish();
```

`true` の場合、スレッドIDが含まれます。
```
2025-12-30T17:10:21.708795Z ERROR ThreadId(01) log_test: level is error
2025-12-30T17:10:21.708839Z  WARN ThreadId(01) log_test: level is warn
2025-12-30T17:10:21.708861Z  INFO ThreadId(01) log_test: level is info
```

`false` の場合、スレッドIDが含まれません。
```
2025-12-30T17:10:32.732025Z ERROR log_test: level is error
2025-12-30T17:10:32.732062Z  WARN log_test: level is warn
2025-12-30T17:10:32.732070Z  INFO log_test: level is info
```

### with_thread_name

スレッド名を出力するかどうか、を指定できます。  
何も指定しない場合、`false` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .with_thread_names(true)
        .finish();
```

`true` の場合、スレッド名が含まれます。
```
2025-12-30T17:14:56.666556Z ERROR main log_test: level is error
2025-12-30T17:14:56.666594Z  WARN main log_test: level is warn
2025-12-30T17:14:56.666614Z  INFO main log_test: level is info
```

`false` の場合、スレッド名が含まれません。
```
2025-12-30T17:16:15.763413Z ERROR log_test: level is error
2025-12-30T17:16:15.763453Z  WARN log_test: level is warn
2025-12-30T17:16:15.763474Z  INFO log_test: level is info
```

### with_timer

タイムスタンプの出力方法、を指定できます。  

```rust
    // システム時間で表示
    let subscriber = FmtSubscriber::builder()
        .with_timer(tracing_subscriber::fmt::time::SystemTime)
        .finish();

    // 実行時からの経過時間で表示
    let subscriber = FmtSubscriber::builder()
        .with_timer(tracing_subscriber::fmt::time::Uptime::default())
        .finish();
```

システム時間で表示した場合は以下のような出力でした。
```
2025-12-30T18:26:10.581077Z ERROR log_test: level is error
2025-12-30T18:26:10.581102Z  WARN log_test: level is warn
2025-12-30T18:26:10.581107Z  INFO log_test: level is info
```

経過時間で表示した場合、以下のような出力でした。
```
   0.000116306s ERROR log_test: level is error
   0.000147559s  WARN log_test: level is warn
   0.000174541s  INFO log_test: level is info
```



### compact, pretty, json

ログのフォーマットを指定できます。以下の３種が指定できました。  

- `compact`
- `pretty`
- `json`

---

`json` を使用する場合、feature の指定が必要でした。  
```
$ cargo add tracing-subscriber -F json
```

---

何も指定しない場合、`compact` で動作しました。  

```rust
    // compact の場合
    let subscriber = FmtSubscriber::builder()
        .compact()
        .finish();

    // pretty の場合
    let subscriber = FmtSubscriber::builder()
        .pretty()
        .finish();

    // json の場合
    let subscriber = FmtSubscriber::builder()
        .json()
        .finish();
```

`compact` の場合、1行で表示されました。
```
2025-12-30T17:24:05.331855Z ERROR log_test: level is error
2025-12-30T17:24:05.331896Z  WARN log_test: level is warn
2025-12-30T17:24:05.331917Z  INFO log_test: level is info
```


`pretty` の場合、複数行で表示されました。  
（ドキュメントでは「きれいで、人間が読める」フォーマットと記載がありました）
```
  2025-12-30T17:24:21.326312Z ERROR log_test: level is error
    at src/main.rs:21

  2025-12-30T17:24:21.326397Z  WARN log_test: level is warn
    at src/main.rs:22

  2025-12-30T17:24:21.326418Z  INFO log_test: level is info
    at src/main.rs:23

```

`json` の場合、json 形式で表示されました。  
```
{"timestamp":"2025-12-30T17:30:53.320636Z","level":"ERROR","fields":{"message":"level is error"},"target":"log_test"}
{"timestamp":"2025-12-30T17:30:53.320693Z","level":"WARN","fields":{"message":"level is warn"},"target":"log_test"}
{"timestamp":"2025-12-30T17:30:53.320717Z","level":"INFO","fields":{"message":"level is info"},"target":"log_test"}
```

### flatten_event

json の構造をフラットにします。ログフォーマットを `json` にする必要があります  
何も指定しない場合、`false` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .json()
        .flatten_event(true)
        .finish();
```

`true` の場合、`message` が何の子要素でもない状態でした。
```
{"timestamp":"2025-12-30T17:39:53.191545Z","level":"ERROR","message":"level is error","target":"log_test"}
{"timestamp":"2025-12-30T17:39:53.191604Z","level":"WARN","message":"level is warn","target":"log_test"}
{"timestamp":"2025-12-30T17:39:53.191616Z","level":"INFO","message":"level is info","target":"log_test"}
```

`false` の場合、`message`が`fields`の子要素として存在していました。
```
{"timestamp":"2025-12-30T17:40:04.343516Z","level":"ERROR","fields":{"message":"level is error"},"target":"log_test"}
{"timestamp":"2025-12-30T17:40:04.343575Z","level":"WARN","fields":{"message":"level is warn"},"target":"log_test"}
{"timestamp":"2025-12-30T17:40:04.343599Z","level":"INFO","fields":{"message":"level is info"},"target":"log_test"}
```

### with_current_span

現在入っている `span` の情報を出力するか、を指定できます。  
ログフォーマットを `json` にする必要があります  
何も指定しない場合、`true` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .json()
        .with_current_span(false)
        .finish();
```

以下の span で動作確認しました。  
```rust
    tracing::span!(tracing::Level::INFO, "my_span").in_scope(|| {
        tracing::span!(tracing::Level::INFO, "my_inner_span").in_scope(|| {
            tracing::info!("inside my_inner_span");
        });
        tracing::info!("inside my_span");
    });
```

`true` の場合、jsonの中に要素`span`があります。
```
{"timestamp":"2025-12-30T18:01:30.720020Z","level":"INFO","fields":{"message":"inside my_inner_span"},"target":"log_test","span":{"name":"my_inner_span"},"spans":[{"name":"my_span"},{"name":"my_inner_span"}]}
{"timestamp":"2025-12-30T18:01:30.720086Z","level":"INFO","fields":{"message":"inside my_span"},"target":"log_test","span":{"name":"my_span"},"spans":[{"name":"my_span"}]}
```


`false` の場合、要素`span`がありません。  
（`spans` があるためわかりにくいですが）
```
{"timestamp":"2025-12-30T18:01:38.181822Z","level":"INFO","fields":{"message":"inside my_inner_span"},"target":"log_test","spans":[{"name":"my_span"},{"name":"my_inner_span"}]}
{"timestamp":"2025-12-30T18:01:38.181889Z","level":"INFO","fields":{"message":"inside my_span"},"target":"log_test","spans":[{"name":"my_span"}]}
```

### with_span_list

ルートから現在入っているスパンまでのリストを出力するか、を指定できます。  
ログフォーマットを `json` にする必要があります  
何も指定しない場合、`true` で動作しました。  

```rust
    let subscriber = FmtSubscriber::builder()
        .json()
        .with_span_list(false)
        .finish();
```

以下の span で動作確認しました。  
```rust
    tracing::span!(tracing::Level::INFO, "my_span").in_scope(|| {
        tracing::span!(tracing::Level::INFO, "my_inner_span").in_scope(|| {
            tracing::info!("inside my_inner_span");
        });
        tracing::info!("inside my_span");
    });
```


`true` の場合、要素`spans`にルートからのリストがありました。
```
{"timestamp":"2025-12-30T17:55:56.312831Z","level":"INFO","fields":{"message":"inside my_inner_span"},"target":"log_test","span":{"name":"my_inner_span"},"spans":[{"name":"my_span"},{"name":"my_inner_span"}]}
{"timestamp":"2025-12-30T17:55:56.312897Z","level":"INFO","fields":{"message":"inside my_span"},"target":"log_test","span":{"name":"my_span"},"spans":[{"name":"my_span"}]}
```

`false` の場合、要素`spans`がありませんでした。
```
{"timestamp":"2025-12-30T17:56:04.600799Z","level":"INFO","fields":{"message":"inside my_inner_span"},"target":"log_test","span":{"name":"my_inner_span"}}
{"timestamp":"2025-12-30T17:56:04.600850Z","level":"INFO","fields":{"message":"inside my_span"},"target":"log_test","span":{"name":"my_span"}}
```

### fmt_fields

フォーマットを設定できました。  
以下にしてみたところ、`フィールド名=ログ`という形式で出力できました。  
柔軟に設定できそうに見えます。  

```rust
    let subscriber = FmtSubscriber::builder()
        .fmt_fields(
            tracing_subscriber::fmt::format::debug_fn(|writer, field, value| write!(
                writer,
                "{}={:?}",
                field,
                value
        )))
        .finish();
```

出力されたログは以下です。
```
2025-12-30T18:07:30.428844Z ERROR log_test: message=level is error
2025-12-30T18:07:30.428881Z  WARN log_test: message=level is warn
2025-12-30T18:07:30.428902Z  INFO log_test: message=level is info
```

### with_writer

出力先を指定できます。  
何も指定しない場合、標準出力に出力されました。  

以下のようにすれば、標準エラーに出力されました。
```rust
    let subscriber = FmtSubscriber::builder()
        .with_writer(std::io::stderr)
        .finish();
```

引数には `std::io::Write` を返すものを指定できるようです。  
以下のよう`io::Write`を返す関数を定義して引数に渡すことによって、ファイルに書くこともできました。  

```rust
fn log_write() -> impl io::Write {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open("test.log")
        .expect("Failed to open test.log")
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_writer(log_write)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::error!("level is error");
    tracing::warn!("level is warn");
    tracing::info!("level is info");
}
```

### without_time

時刻を出力しないようにできます。  

```rust
    let subscriber = FmtSubscriber::builder()
        .without_time()
        .finish();
```

以下のように、時刻が出力されなくなります。  
```
ERROR log_test: level is error
 WARN log_test: level is warn
 INFO log_test: level is info
```

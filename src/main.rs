use std::fs::OpenOptions;
use std::io;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

/// test.log ファイルへの Writer を返す（追記モード）
#[allow(dead_code)]
fn log_write() -> impl io::Write {
    let filename = "sample.log";
    OpenOptions::new()
        .create(true) // ファイルがなければ作成
        .append(true) // 追記モード
        .open(filename)
        .expect("Failed to open sample.log")
}

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // 出力するログレベルの設定。
        .with_level(true) // ログレベルを表示するかどうか。デフォルト true。そのままにしておくのが無難に見える
        .with_ansi(false) // ANSIエスケープコードを出力するかどうか。ファイルに出力する場合はfalseにするのが無難？
        .with_file(true) // ファイル名を表示するかどうか
        .with_line_number(true) // 行番号を表示するかどうか
        .with_target(true) // ターゲット（モジュールパス）を表示するかどうか。デフォルト true。そのままにしておくのが無難に見える
        .with_thread_ids(true) // スレッドIDを表示するかどうか。とにかく情報を出したい場合、true にする方がいいかも
        .with_thread_names(true) // スレッド名を表示するかどうか。とにかく情報を出したい場合、true にする方がいいかも。マルチスレッドなら true にするのが無難？
        .compact() // コンパクトなフォーマットでログを出力する場合（デフォルトなので、付けなくても同じ動作になる）
        // .pretty() // 見やすいフォーマットでログを出力する場合
        // .json() // JSON形式でログを出力する場合
        // .fmt_fields(
        //     tracing_subscriber::fmt::format::debug_fn(|writer, field, value| write!(
        //         writer,
        //         "{}={:?}",
        //         field,
        //         value
        // ))) // フィールドのフォーマットをカスタマイズ
        // .with_writer(log_write) // ログの出力先をカスタマイズ
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::span!(tracing::Level::INFO, "my_span").in_scope(|| {
        tracing::span!(tracing::Level::INFO, "my_inner_span").in_scope(|| {
            tracing::info!("inside my_inner_span");
        });
        tracing::info!("inside my_span");
    });

    tracing::error!("level is error");
    tracing::warn!("level is warn");
    tracing::info!("level is info");
    tracing::debug!("level is debug");
    tracing::trace!("level is trace");
}

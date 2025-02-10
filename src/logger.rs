use chrono::Local;
use fern::Dispatch;
use fern::colors::{Color, ColoredLevelConfig};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// ログをファイルおよびコンソールに出力するロガーを初期化します。
///
/// ログファイルは `./logs/YYYY-MM-DD.log` に保存され、
/// コンソール出力は色付きでフォーマットされます。
pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    // ログディレクトリを作成（存在しない場合）
    let log_dir: &Path = Path::new("logs");
    if !log_dir.exists() {
        fs::create_dir_all(log_dir)?;
    }
    // ログファイルのパス：logs/YYYY-MM-DD.log
    let log_file: PathBuf  = log_dir.join(format!("{}.log", Local::now().format("%Y-%m-%d")));
    
    // コンソール出力用の色設定（Python の colorlog の設定に似せる）
    let colors: ColoredLevelConfig = ColoredLevelConfig::new()
        .debug(Color::Cyan)
        .info(Color::BrightBlue)
        .warn(Color::Yellow)
        .error(Color::Red);
    
    // fern の Dispatch を使ってロガーを設定
    Dispatch::new()
        // ログレベルを Debug 以上に設定
        .level(log::LevelFilter::Info)
        // ファイル出力（非カラー、フォーマット: "YYYY-MM-DD HH:MM:SS LEVEL   target message"）
        .chain(
            Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{} {:<8} {} {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file(log_file)?)
        )
        // コンソール出力（カラー付き）
        .chain(
            Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} {:<8} {} {}{}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        colors.color(record.level()),
                        record.target(),
                        message,
                        "\x1B[0m" // ANSIリセットコードを追加
                    ))
                })
                .chain(std::io::stderr())
        )
        .apply()?;
    Ok(())
}

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
    
    // コンソール出力用の色設定
    let colors: ColoredLevelConfig = ColoredLevelConfig::new()
        .debug(Color::Cyan)
        .info(Color::BrightBlue)
        .warn(Color::Yellow)
        .error(Color::Red);
    
    // ビルドモードに応じてログレベルを切り替え
    let log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    // ロガーを設定
    Dispatch::new()
        .level(log_level)
        // ファイル出力
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
        // コンソール出力
        .chain(
            Dispatch::new()
                .format(move |out, message, record| {
                    out.finish(format_args!(
                        "{} {} {} {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        colors.color(record.level()),
                        record.target(),
                        message
                    ))
                })
                .chain(std::io::stdout())
        )
        .apply()?;
    Ok(())
}

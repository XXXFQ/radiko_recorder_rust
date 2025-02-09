mod recorder;
mod auth_handler;
//mod config;

use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;

use chrono::{Local, DateTime, NaiveDateTime, TimeZone};
use clap::{ArgAction, Parser, CommandFactory};
use regex::Regex;

use crate::recorder::RadikoPlayer;
//use crate::config::RADIKO_AREA_ID;

/// コマンドライン引数を表す構造体
#[derive(Parser, Debug)]
#[command(author, version, about = "Radiko Recorder", long_about = None)]
struct Args {
    /// エリアID (例: JP13, JP27, etc.)
    #[arg(short, long, default_value = "JP13")]
    area_id: String,

    /// 放送局リストを表示する
    #[arg(short, long, action = ArgAction::SetTrue)]
    station_list: bool,

    /// 放送局ID (録音時は必須)
    station_id: Option<String>,

    /// 録音開始時刻 (YYYYMMDDHHMMSS形式、録音時は必須)
    start_time: Option<String>,

    /// 録音時間（分）
    #[arg(default_value_t = 60)]
    duration_minutes: i32,
}

/// エリアIDの正規表現パターンに合致しているかチェックする
fn is_valid_area_id(area_id: &str) -> bool {
    // パターン例: JP13, JP27, … JP47 など（JP + 数値）
    let re = Regex::new(r"^JP([1-9]|[1-3][0-9]|4[0-7])$").unwrap();
    re.is_match(area_id)
}

/// 放送局リストを表示する
fn show_station_list(area_id: &str) -> Result<(), Box<dyn Error>> {
    if !is_valid_area_id(area_id) {
        return Err(format!("Invalid area ID: {}", area_id).into());
    }

    let player = RadikoPlayer::new(area_id);
    // get_station_list は Result<Vec<Station>, Error> を返すと仮定
    let station_list = player.get_station_list()?;
    // 各放送局の ID と名前を出力する
    for station in station_list {
        println!(
            "Station: id={}, name={}, ascii_name={}, ruby={}",
            station.id, station.name, station.ascii_name, station.ruby
        );
    }
    Ok(())
}

/// ラジオを録音する処理
fn record_radio(area_id: &str, station_id: &str, start_time_str: &str, duration_minutes: i64) -> Result<(), Box<dyn Error>> {
    if !is_valid_area_id(area_id) {
        return Err(format!("Invalid area ID: {}", area_id).into());
    }

    // 出力ディレクトリ "output" を作成（存在しなければ）
    let output_dir = Path::new("output");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }
    // 現在時刻を付与して出力ファイル名を生成
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let output_file = output_dir.join(format!("{}_{}.aac", station_id, timestamp));

    // 開始時刻の文字列をパースする
    let naive_dt = NaiveDateTime::parse_from_str(start_time_str, "%Y%m%d%H%M%S")?;
    let start_time: DateTime<Local> = Local.from_local_datetime(&naive_dt)
        .single()
        .ok_or("Failed to convert start time")?;

    let player = RadikoPlayer::new(area_id);
    player.record(station_id, start_time, duration_minutes, output_file.to_str().unwrap())?;
    Ok(())
}

fn main() {
    // コマンドライン引数を解析
    let args = Args::parse();

    if args.station_list {
        if let Err(e) = show_station_list(&args.area_id) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        return;
    }

    // 録音モードの場合、必須の引数（station_id, start_time）が与えられているかチェック
    if args.station_id.is_none() || args.start_time.is_none() {
        eprintln!("Station ID, start time, and duration minutes are required unless using the --station-list option.");
        eprintln!("{}", Args::command().render_usage());
        process::exit(1);
    }

    let station_id = args.station_id.unwrap();
    let start_time = args.start_time.unwrap();
    let duration_minutes = args.duration_minutes as i64;

    if let Err(e) = record_radio(&args.area_id, &station_id, &start_time, duration_minutes) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

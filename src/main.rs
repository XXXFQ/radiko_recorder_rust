use chrono::Local;
use log::info;
use std::error::Error;

mod recorder;
mod auth_handler;

fn main() -> Result<(), Box<dyn Error>> {
    // ログ初期化（環境変数 RUST_LOG でログレベルを指定可能）
    env_logger::init();

    // エリアIDを指定してRadikoPlayerを作成
    let area_id = "JP13"; // 例：関東エリアの場合（実際は適切なIDに変更）
    let player = recorder::RadikoPlayer::new(area_id);

    // 放送局リストの取得例
    let stations = player.get_station_list()?;
    for station in &stations {
        info!(
            "Station: id={}, name={}, ascii_name={}, ruby={}",
            station.id, station.name, station.ascii_name, station.ruby
        );
    }

    // 録音の実行例
    // （ここでは現在時刻から5分間の録音を "output.aac" として保存）
    let station_id = "TBS"; // 例：放送局ID（適切なものに変更）
    let start_time = Local::now();
    let duration_minutes = 5;
    let output_path = "output.aac";

    player.record(station_id, start_time, duration_minutes, output_path)?;

    Ok(())
}
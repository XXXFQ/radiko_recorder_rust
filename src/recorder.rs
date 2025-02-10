use chrono::{DateTime, Duration, Local};
use log::{debug, info};
use quick_xml::de::from_str;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::process::Command;
use std::error::Error;
use std::process::ExitStatus;

use crate::auth_handler::RadikoAuthHandler;

/// 放送局情報
#[derive(Debug, Deserialize)]
pub struct Station {
    pub id: String,
    pub name: String,
    pub ascii_name: String,
    pub ruby: String,
}

/// XMLのルート要素として放送局リストを受け取るための構造体
#[derive(Debug, Deserialize)]
struct StationList {
    #[serde(rename = "station")]
    stations: Vec<Station>,
}

/// Radikoプレイヤー
pub struct RadikoPlayer {
    area_id: String,
    headers: HashMap<String, String>,
}

impl RadikoPlayer {
    /// コンストラクタ  
    /// 
    /// # 引数
    /// - `area_id`: RadikoのエリアID
    pub fn new(area_id: &str) -> Self {
        let headers: HashMap<String, String> = Self::make_headers(area_id);
        Self {
            area_id: area_id.to_string(),
            headers,
        }
    }

    /// 指定した放送局のストリームを録音してファイルに保存する  
    /// 
    /// # 引数
    /// - `station_id`: 放送局ID
    /// - `start_time`: 録音開始日時（Localタイムゾーン）
    /// - `duration_minutes`: 録音時間（分）
    /// - `output_path`: 出力先ファイルパス
    pub fn record(
        &self,
        station_id: &str,
        start_time: DateTime<Local>,
        duration_minutes: i64,
        output_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        // 開始時刻、終了時刻をフォーマット
        let ft: String = Self::format_datetime(start_time);
        let end_time: DateTime<Local> = start_time + Duration::minutes(duration_minutes);
        let to: String = Self::format_datetime(end_time);

        // ストリームURLの作成
        let stream_url: String = format!(
            "https://radiko.jp/v2/api/ts/playlist.m3u8?station_id={}&l=15&ft={}&to={}",
            station_id, ft, to
        );

        // ffmpeg用のヘッダー（ここではX-Radiko-AuthTokenを指定）
        let auth_token: &String = self
            .headers
            .get("X-Radiko-AuthToken")
            .ok_or("Missing X-Radiko-AuthToken")?;
        let header_arg: String = format!("X-RADIKO-AUTHTOKEN: {}", auth_token);

        info!("Recording {}...", output_path);

        // ffmpegコマンドを実行して録音
        let status: ExitStatus = Command::new("ffmpeg")
            .args(&[
                "-headers",
                &header_arg,
                "-i",
                &stream_url,
                "-acodec",
                "copy",
                "-y",
                output_path,
            ])
            .status()?;

        if !status.success() {
            return Err(format!("ffmpeg exited with status: {:?}", status).into());
        }

        info!("Successfully recorded {}", output_path);
        Ok(())
    }

    /// 指定エリアの放送局リストを取得する  
    /// 
    /// # 戻り値
    /// 放送局情報のベクター
    pub fn get_station_list(&self) -> Result<Vec<Station>, Box<dyn Error>> {
        let url: String = format!("https://radiko.jp/v3/station/list/{}.xml", self.area_id);
        let resp: reqwest::blocking::Response = reqwest::blocking::get(&url)?;
        let content: String = resp.text()?;

        // XMLパース
        let station_list: StationList = from_str(&content)?;
        Ok(station_list.stations)
    }

    /// 認可済みのヘッダを取得する
    fn make_headers(area_id: &str) -> HashMap<String, String> {
        let auth_handler: RadikoAuthHandler = RadikoAuthHandler::new(area_id)
            .expect("Radiko authentication failed");
        let mut headers: HashMap<String, String> = auth_handler.get_authenticated_headers();
        headers.insert("Connection".to_string(), "keep-alive".to_string());
        debug!("headers: {:?}", headers);
        headers
    }

    /// 日時を "YYYYMMDDHHMMSS" 形式にフォーマットする  
    fn format_datetime(dt: DateTime<Local>) -> String {
        dt.format("%Y%m%d%H%M%S").to_string()
    }
}
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;

use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use base64::{engine::general_purpose, Engine as _};
use log::{debug, warn};

/// Radiko API の認可ハンドラ
pub struct RadikoAuthHandler {
    headers: HashMap<String, String>,
}

impl RadikoAuthHandler {
    const AUTH1_URL: &'static str = "https://radiko.jp/v2/api/auth1";
    const AUTH2_URL: &'static str = "https://radiko.jp/v2/api/auth2";
    /// Radiko の認可キー（固定値）
    const RADIKO_AUTH_KEY: &'static [u8] = b"bcd151073c03b352e1ef2fd66c32209da9ca0afa";

    /// コンストラクタ
    /// `area_id` に指定されたエリアIDを使い、認可処理を実行する。
    pub fn new(area_id: &str) -> Result<Self, Box<dyn Error>> {
        // 初期ヘッダの設定
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("User-Agent".to_string(), "python3.7".to_string());
        headers.insert("Accept".to_string(), "*/*".to_string());
        headers.insert("X-Radiko-App".to_string(), "pc_html5".to_string());
        headers.insert("X-Radiko-App-Version".to_string(), "0.0.1".to_string());
        headers.insert("X-Radiko-User".to_string(), "dummy_user".to_string());
        headers.insert("X-Radiko-Device".to_string(), "pc".to_string());
        headers.insert("X-Radiko-AuthToken".to_string(), "".to_string());
        headers.insert("X-Radiko-Partialkey".to_string(), "".to_string());
        headers.insert("X-Radiko-AreaId".to_string(), area_id.to_string());

        let mut handler: RadikoAuthHandler = RadikoAuthHandler { headers };
        // 認可処理（auth1 → auth2）を実行
        handler.auth()?;
        Ok(handler)
    }

    /// 認可済みのヘッダを取得する
    pub fn get_authenticated_headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    /// 内部で認可処理を行う  
    ///  
    /// 1. AUTH1 API を呼び出し、認可用トークンと部分鍵を取得する。  
    /// 2. 取得した情報をヘッダに設定後、AUTH2 API を呼び出す。
    fn auth(&mut self) -> Result<(), Box<dyn Error>> {
        // AUTH1 API 呼び出し
        let res: Response = self.call_auth_api(Self::AUTH1_URL)?;
        // レスポンスから認可用トークンと部分鍵を取得
        let auth_token: String = self.get_auth_token(&res)?;
        let partial_key: String = self.get_partial_key(&res)?;
        self.headers.insert("X-Radiko-AuthToken".to_string(), auth_token);
        self.headers.insert("X-Radiko-Partialkey".to_string(), partial_key);

        // AUTH2 API 呼び出し（認可トークンが設定されたヘッダを利用）
        let res2 = self.call_auth_api(Self::AUTH2_URL)?;
        debug!("authenticated headers: {:?}", self.headers);
        debug!("auth2 response headers: {:?}", res2.headers());
        let content = res2.text()?;
        debug!("auth2 response content: {}", content);
        Ok(())
    }

    /// RadikoAPIに認可リクエストを送信する
    /// タイムアウトは 5 秒、リクエスト後に 1 秒のスリープを行う。
    fn call_auth_api(&self, api_url: &str) -> Result<Response, Box<dyn Error>> {
        // タイムアウト付きのクライアントを作成
        let client: Client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()?;
        // self.headers (HashMap) を HeaderMap に変換
        let mut header_map: HeaderMap = HeaderMap::new();
        for (key, value) in &self.headers {
            header_map.insert(
                HeaderName::from_bytes(key.as_bytes())?,
                HeaderValue::from_str(value)?
            );
        }
        // GET リクエストを送信
        let res: Response = client.get(api_url)
            .headers(header_map)
            .send()?;
        // リクエスト後、1 秒待機
        sleep(Duration::from_secs(1));
        if !res.status().is_success() {
            warn!("failed in {}.", api_url);
            warn!("status code: {}", res.status());
            let text = res.text()?;
            warn!("content: {}", text);
            return Err(format!("failed in {}.", api_url).into());
        }
        debug!("auth in {} is success.", api_url);
        Ok(res)
    }

    /// レスポンスヘッダから認可用トークン（X-Radiko-AUTHTOKEN）を取得する
    fn get_auth_token(&self, response: &Response) -> Result<String, Box<dyn Error>> {
        match response.headers().get("X-Radiko-AUTHTOKEN") {
            Some(val) => Ok(val.to_str()?.to_string()),
            None => Err("Missing X-Radiko-AUTHTOKEN header".into()),
        }
    }

    /// レスポンスヘッダから部分鍵用の情報を取得し、  
    /// 固定の認可キーから指定範囲のバイト列を Base64 エンコードして返す
    fn get_partial_key(&self, response: &Response) -> Result<String, Box<dyn Error>> {
        let key_length: usize = match response.headers().get("X-Radiko-KeyLength") {
            Some(val) => val.to_str()?.parse::<usize>()?,
            None => return Err("Missing X-Radiko-KeyLength header".into()),
        };
        let key_offset: usize = match response.headers().get("X-Radiko-KeyOffset") {
            Some(val) => val.to_str()?.parse::<usize>()?,
            None => return Err("Missing X-Radiko-KeyOffset header".into()),
        };

        if key_offset + key_length > Self::RADIKO_AUTH_KEY.len() {
            return Err("Key offset and length out of bounds".into());
        }
        let slice: &[u8] = &Self::RADIKO_AUTH_KEY[key_offset .. key_offset + key_length];
        let partial_key: String = general_purpose::STANDARD.encode(slice);
        Ok(partial_key)
    }
}

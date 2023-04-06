use std::collections::HashMap;
use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

use urlencoding::encode;
extern crate tokio;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Serialize, Deserialize)]
struct ClientConfig {
    seoul_url: String,
    file_type: String,
    service_name: String,
    start_index: String,
    end_index: String,
}

#[derive(Deserialize, Debug)]
struct RealTimeArrival {
    subwayId: Option<String>,
    updnLine: Option<String>,
    trainLineNm: Option<String>,
    statnFid: Option<String>,
    statnTid: Option<String>,
    statnId: Option<String>,
    statnNm: Option<String>,
    trnsitCo: Option<String>,
    ordkey: Option<String>,
    subwayList: Option<String>,
    statnList: Option<String>,
    btrainSttus: Option<String>,
    barvlDt: Option<String>,
    btrainNo: Option<String>,
    bstatnId: Option<String>,
    bstatnNm: Option<String>,
    recptnDt: Option<String>,
    arvlMsg2: Option<String>,
    arvlMsg3: Option<String>,
    arvlCd: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ServerMessage {
    status: u8,
    code: Option<String>,
    message: Option<String>,
    link: Option<String>,
    developerMessage: Option<String>,
    total: u8,
}

#[derive(Deserialize, Debug)]
struct ClientResponse {
    errorMessage: ServerMessage,
    realtimeArrivalList: Vec<RealTimeArrival>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // read api-key and make request url
    let api_key = get_public_api_key("src/public_subway_api_key.yml");
    let client_config: ClientConfig = get_client_config("src/client_config.yaml");

    let url = make_url(api_key, client_config, "동천".to_string());
    let response = reqwest::get(url).await?.json::<ClientResponse>().await?;

    println!("{:?}", response);
    return Ok(());
}

fn get_public_api_key(api_key_path: &str) -> String {
    let f = File::open(api_key_path).expect("Could not open file.");
    let api_key: HashMap<String, String> =
        serde_yaml::from_reader(f).expect("Could not read values.");
    return api_key["API_KEY"].to_string();
}

fn get_client_config(clien_config_path: &str) -> ClientConfig {
    let f = File::open(clien_config_path).expect("Could not open file.");
    let client_config: ClientConfig = serde_yaml::from_reader(f).expect("Could hot read values");
    return client_config;
}

fn make_url(
    api_key: String,
    client_config: ClientConfig,
    station_name: String, //station name is KOREAN. have to be converted to ASCII and encoded UTF-8.
) -> String {
    let encodec_station_name = encode(&station_name);
    let full_url = format!(
        "{}/{}/{}/{}/{}/{}/{}",
        client_config.seoul_url,
        api_key,
        client_config.file_type,
        client_config.service_name,
        client_config.start_index,
        client_config.end_index,
        encodec_station_name
    );

    return full_url;
}

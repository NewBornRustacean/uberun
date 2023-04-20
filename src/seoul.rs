use std::collections::HashMap;
use std::fs::File;

use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Serialize, Deserialize)]
pub struct ClientConfig {
    pub seoul_url: String,
    pub file_type: String,
    pub service_name: String,
    pub start_index: String,
    pub end_index: String,
}

#[derive(Deserialize, Debug)]
pub struct RealTimeArrival {
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
pub struct ServerMessage {
    status: u8,
    code: Option<String>,
    message: Option<String>,
    link: Option<String>,
    developerMessage: Option<String>,
    total: u8,
}

#[derive(Deserialize, Debug)]
pub struct ClientResponse {
    errorMessage: ServerMessage,
    realtimeArrivalList: Vec<RealTimeArrival>,
}

pub fn get_public_api_key(api_key_path: &str) -> String {
    let f = File::open(api_key_path).expect("Could not open file.");
    let api_key: HashMap<String, String> =
        serde_yaml::from_reader(f).expect("Could not read values.");
    return api_key["API_KEY"].to_string();
}

pub fn get_client_config(clien_config_path: &str) -> ClientConfig {
    let f = File::open(clien_config_path).expect("Could not open file.");
    let client_config: ClientConfig = serde_yaml::from_reader(f).expect("Could hot read values");
    return client_config;
}

pub fn make_url(
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

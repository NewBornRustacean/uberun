use serde::{Deserialize, Serialize};

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

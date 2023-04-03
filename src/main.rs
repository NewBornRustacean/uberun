use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

use hyper::{Client, Uri};
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

#[tokio::main]
async fn main() -> Result<()> {
    // read api-key and make request url
    let api_key = get_public_api_key("D:/RustWorkspace/uberun/src/public_subway_api_key.yml");
    let client_config: ClientConfig =
        get_client_config("D:/RustWorkspace/uberun/src/client_config.yaml");

    let url = make_url(api_key, client_config, "동천".to_string());
    println!("{}", url);
    let uri: Uri = Uri::from_str(url.as_str()).unwrap();

    let client = Client::new();
    let res = client.get(uri).await?;

    // And then, if the request gets a response...
    println!("status: {}", res.status());

    // Concatenate the body stream into a single buffer...
    let buf = hyper::body::to_bytes(res).await?;

    println!("body: {:?}", buf);
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

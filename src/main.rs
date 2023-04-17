use std::collections::HashMap;
use std::fs::File;

use serde_yaml::{self};
use teloxide::{prelude::*, utils::command::BotCommands};
use urlencoding::encode;
extern crate tokio;

mod artifacts;
use artifacts::{ClientConfig, ClientResponse};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // read api-key and make request url
    let api_key = get_public_api_key("src/public_subway_api_key.yml");
    let client_config: artifacts::ClientConfig = get_client_config("src/client_config.yaml");

    let url = make_url(api_key, client_config, "동천".to_string());
    let response = reqwest::get(url).await?.json::<ClientResponse>().await?;

    println!("{:?}", response);

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer_from_bot).await;
    return Ok(());
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start to watch")]
    Go(String),
    #[command(description = "stop to watch ")]
    Stop,
}

async fn answer_from_bot(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        Command::Go(station) => {
            bot.send_message(msg.chat.id, format!("watching {station}.."))
                .await?
        }
        Command::Stop => {
            bot.send_message(msg.chat.id, format!("stop to watching."))
                .await?
        }
    };

    Ok(())
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

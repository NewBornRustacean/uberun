use serde_yaml::{self};
extern crate tokio;
use teloxide::prelude::*;

mod seoul;
use seoul::{get_client_config, get_public_api_key, make_url, ClientResponse};

mod telebot;
use telebot::{answer_from_bot, Command};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // read api-key and make request url
    let api_key = get_public_api_key("src/public_subway_api_key.yml");
    let client_config: seoul::ClientConfig = get_client_config("src/client_config.yaml");

    let url = make_url(api_key, client_config, "동천".to_string());
    let response = reqwest::get(url).await?.json::<ClientResponse>().await?;

    println!("{:?}", response);

    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer_from_bot).await;
    return Ok(());
}

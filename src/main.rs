use serde_yaml::{self};
extern crate tokio;
use teloxide::prelude::*;

mod seoul;
mod telebot;
use telebot::{answer_from_bot, Command};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer_from_bot).await;
    return Ok(());
}

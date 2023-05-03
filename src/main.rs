use serde_yaml::{self};
extern crate tokio;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

mod seoul;
use seoul::{get_client_config, get_public_api_key, ClientConfig};
mod telebot;
use telebot::{receive_station, start, State};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveStation].endpoint(receive_station)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}

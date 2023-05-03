use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

use crate::seoul::{
    get_arrival_time_in_second, get_client_config, get_public_api_key, make_url, ClientConfig,
    SeoulResponse,
};

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveStation,
}

pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "어느역이 궁금? 역이름만 보내셈 e.g. 동천")
        .await?;
    dialogue.update(State::ReceiveStation).await?;
    Ok(())
}

pub async fn receive_station(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(station_name) => {
            let api_key: String = get_public_api_key("src/public_subway_api_key.yml");
            let client_config: ClientConfig = get_client_config("src/client_config.yaml");

            let url = make_url(api_key, client_config, station_name.to_string());
            let response: SeoulResponse = reqwest::get(url).await?.json::<SeoulResponse>().await?;
            let time_sec = get_arrival_time_in_second(response);

            bot.send_message(msg.chat.id, format!("{time_sec}")).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "역이름! 입력하라그").await?;
        }
    }

    Ok(())
}

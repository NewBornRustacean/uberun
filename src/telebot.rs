use teloxide::{prelude::*, utils::command::BotCommands};

use crate::seoul::{
    get_arrival_time_in_second, get_client_config, get_public_api_key, make_url, ClientConfig,
    ClientResponse,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "start to watch")]
    Go(String),
    #[command(description = "stop to watch ")]
    Stop,
}

pub async fn answer_from_bot(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // read api-key and make request url
    let api_key: String = get_public_api_key("src/public_subway_api_key.yml");
    let client_config: ClientConfig = get_client_config("src/client_config.yaml");

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }

        Command::Go(station) => {
            let url: String = make_url(api_key, client_config, station.trim().to_string());
            let response: ClientResponse =
                reqwest::get(url).await?.json::<ClientResponse>().await?;
            let arrival_msg: String = get_arrival_time_in_second(response);

            bot.send_message(msg.chat.id, format!("{arrival_msg}"))
                .await?
        }
        Command::Stop => {
            bot.send_message(msg.chat.id, format!("stop to watching."))
                .await?
        }
    };

    Ok(())
}

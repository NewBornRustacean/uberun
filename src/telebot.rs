use teloxide::{prelude::*, utils::command::BotCommands};

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

use std::str::FromStr;

use chrono::Duration;

use teloxide::{
    payloads::KickChatMemberSetters,
    prelude::{Requester, ResponseResult},
    types::Message,
    utils::command::BotCommands,
    Bot,
};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", parse_with = "split", description = "Supported commands:")]
pub enum Command {
    Help,
    Ban { time: u64, unit: UnitOfTime },
}

#[derive(Clone)]
enum UnitOfTime {
    Seconds,
    Minutes,
    Hours,
}

impl FromStr for UnitOfTime {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "h" | "hours" => Ok(UnitOfTime::Hours),
            "m" | "minutes" => Ok(UnitOfTime::Minutes),
            "s" | "seconds" => Ok(UnitOfTime::Seconds),
            _ => Err("Allowed units: h, m, s"),
        }
    }
}

pub async fn command_answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Ban { time, unit } => ban_user(bot, msg, calc_restrict_time(time, unit)).await?,
    };

    Ok(())
}

async fn ban_user(bot: Bot, msg: Message, time: Duration) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            bot.kick_chat_member(
                msg.chat.id,
                replied.from.as_ref().expect("Must be MessageKind::Common").id,
            )
            .until_date(msg.date + time)
            .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Use this command in a reply to another message!")
                .await?;
        }
    }
    Ok(())
}

fn calc_restrict_time(time: u64, unit: UnitOfTime) -> Duration {
    match unit {
        UnitOfTime::Hours => Duration::try_hours(time as i64).unwrap(),
        UnitOfTime::Minutes => Duration::try_minutes(time as i64).unwrap(),
        UnitOfTime::Seconds => Duration::try_seconds(time as i64).unwrap(),
    }
}

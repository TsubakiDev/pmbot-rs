mod commands;
mod config;

use std::{env, path::Path, process::exit};

use reqwest::Error;
use serde::Deserialize;
use teloxide::{repls::CommandReplExt, Bot};

const VERSION_CODE: &str = "1.0.0";
const GITHUB_RELEASE_URL: &str = "https://api.github.com/repos/TsubakiDev/pmbot-rs/releases/latest";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting telegram pm bot...");

    if !Path::new("config.json").exists() {
        config::init_default_config();
        log::info!(
            "A default configuration file has been generated for you, please modify the fields."
        );
        exit(0)
    }

    let loaded_config = config::load_config().unwrap();

    match env::var("TELOXIDE_TOKEN") {
        Ok(_) => (),
        Err(_) => env::set_var("TELOXIDE_TOKEN", loaded_config.bot_token),
    }

    let bot = Bot::from_env();

    commands::Command::repl(bot, commands::command_answer).await;
    log::info!("Commands loaded.");

    match check_for_update(VERSION_CODE).await {
        Ok(_) => (),
        Err(e) => println!("Error checking for updates: {}", e),
    }
}

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    html_url: String,
}

async fn check_for_update(current_version: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let response = client
        .get(&*GITHUB_RELEASE_URL)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Linux; Android 14; Pixel 9 Pro) AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/112.0.0.0 Mobile Safari/537.36",
        )
        .send()
        .await?
        .json::<Release>()
        .await?;

    if *response.tag_name > *current_version {
        log::warn!("Update available: {}", response.html_url);
    } else {
        log::info!("You are on the latest version.");
    }

    Ok(())
}

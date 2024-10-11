use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
// use serde::{Deserialize, Serialize,};
// use core::error;
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use token_overview::{TokenOverview, TokenOverviewData};

pub mod token_overview;


#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display help message")]
    Help,
    #[command(description = "Send the welcome message")]
    Start,
    #[command(description = "get token overview")]
    S,
    #[command(description = "Send the web app")]
    Jito,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    let bot = Bot::from_env();   
   
    let bot_commands = Command::bot_commands();
    if bot.set_my_commands(bot_commands).await.is_err() {
        log::warn!("Could not set up the commands.");
    }

    Dispatcher::builder(
        bot,
        dptree::entry().branch(Update::filter_message().endpoint(message_handler)),
    )
    .build()
    .dispatch()
    .await;

    Ok(())
}

async fn get_token_overview(token_address: &str, api_key: &str) -> Result<TokenOverview, reqwest::Error> {
    let url = format!("https://public-api.birdeye.so/defi/token_overview?address={}", token_address);
    let client = Client::new();
    
    let response = client
        .get(&url)
        .header("X-API-KEY", api_key)
        .header("x-chain", "sui")
        .send()
        .await?;
    let text = response.text().await.expect("Not available to get response body");
    let object: TokenOverview = serde_json::from_str(&text).expect("Invalid response parameters!!");

    Ok(object)
}

async fn make_token_overview_message(token_data: &TokenOverviewData) -> Result<String, reqwest::Error> {
    let token_address = &token_data.address;
    let name = &token_data.name;
    let symbol = &token_data.symbol;
    let liquidity = token_data.liquidity;
    let price = token_data.price;
    let price_change_30m_percent = token_data.price_change_30m_percent.expect("Invalid operation to get 30m price change percent");
    let price_change_1h_percent = token_data.price_change_1h_percent.expect("Invalid operation to get 1h price change percent");
    let price_change_24h_percent = token_data.price_change_24h_percent.expect("Invalid operation to get 24h price change percent");
    let buy_1h_change_percent = token_data.buy_1h_change_percent.expect("Invalid operation to get 1h buy percent");
    let buy_24h_change_percent = token_data.buy_24h_change_percent.expect("Invalid operation to get 24h buy percent");
    let volume_1h = token_data.history_1h_price;
    let volume_6h = token_data.history_6h_price;
    let volume_24h = token_data.history_24h_price;
    
    let text = format!("
    symbol: {name} ({symbol})
    ⛓ SUI
    👥 Socials: 🌐💬🐦
    ➖➖➖➖➖➖
    🔎 Top 10 holders: 21.36% 🚨, no mint, liquidity burned, no blacklist
    
    {token_address}
    
    📊 MCap:   | ATH: $386.07K
    🏷 Price: ${price}
    💧 Liq: ${liquidity} 
            └🔥 100.00% Burned
    
    📉 Price Changes:
            30m: {price_change_30m_percent}% | 1h: {price_change_1h_percent}% | 24h: {price_change_24h_percent}%
    🎚 Volume:
            1h: ${volume_1h} | 6h: ${volume_6h} | 24h: ${volume_24h}
    🔄 Buys/Sells:
            1h: 0/0 | 24h: 0/3
    🔄 Buy percentage:
            1h: {buy_1h_change_percent} | 24h: {buy_24h_change_percent}
    
    🧳 Holders: 320
    ⏳ Age: 154d 5h 3m
    📡 Check for Calls ❎ Search on 𝕏
    
    🎯 PIRBX | Maestro | Maestro Pro
    ");

    Ok(text)

}

async fn answer(bot: Bot, msg: Message, cmd: Command, token_adr: &str, api_key: &str ) -> ResponseResult<()> {

    let username = msg.chat.username().unwrap();

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {           
            bot.send_message(msg.chat.id, format!("Welcome to Here {username}! 🎉"))              
                .await?
        },
        Command::S => {                
            let token_overview = get_token_overview(token_adr, api_key).await.expect("Failed to get token_overview");                    
            let token_data = token_overview.data;            
            let text = make_token_overview_message(&token_data).await?;

            bot.send_message(msg.chat.id, text)              
                .await?
        },
        Command::Jito => {
            bot.send_message(msg.chat.id, "Welcome to HyperLoop! 🎉")              
                .await?
        }
    };

    Ok(())
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let token_address =  env::var("TOKEN_ADDRESS").expect("API_KEY not set");

    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd, &token_address, &api_key).await?;
        }
    }

    Ok(())
}


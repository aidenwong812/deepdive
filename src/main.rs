pub mod token_overview;
pub mod token_social;
pub mod token_top50_holders;

use dotenv::dotenv;
use log::{error, info};
use reqwest::Client;
use std::env;
use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
use tokio::time;
use serde_json;
use token_overview::{TokenOverview, TokenOverviewData};
use token_social::TokenSocial;
use token_top50_holders::TokenTopHolders;

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
    #[command(description = "Get token overview\n\tEntry type: /s ****(token address)")]
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

async fn get_token_overview(token_address: &str) -> Result<TokenOverview, serde_json::Error> {
    let birdeye_api_key = env::var("Birdeye_API_KEY").expect("API_KEY not set");
    let url = format!(
        "https://public-api.birdeye.so/defi/token_overview?address={}",
        token_address
    );
    let client = Client::new().clone();

    let response = client
        .get(&url)
        .header("X-API-KEY", birdeye_api_key)
        .header("x-chain", "sui")
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(e) => Err(e),
    }
}

async fn get_token_social_link(client: Client, api_key:&str, token_address: &str) -> Result<TokenSocial, serde_json::Error> {
    let url = format!(
        "https://api.blockberry.one/sui/v1/coins/metadata/{}",
        token_address
    );

    let response = client
        .get(&url)
        .header("X-API-KEY", api_key)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(e) => Err(e),
    }
}

async fn get_top_50_holders(client: Client, api_key: &str, token_address: &str) -> Result<TokenTopHolders, serde_json::Error> {
    let url = format!("https://api.blockberry.one/sui/v1/coins/{}/holders?page=0&size=50&orderBy=DESC&sortBy=AMOUNT", token_address);
   
    let response = client
        .get(&url)
        .header("X-API-KEY", api_key)
        .send()
        .await
        .unwrap();

    let text = response.text().await.unwrap();
    match serde_json::from_str(&text) {
        Ok(obj) => Ok(obj),
        Err(e) => Err(e),
    }

}

// async fn get_token_creation_info(token_address: &str, api_key: &str) -> Result<TokenOverview, reqwest::Error> {
//     let url = format!("https://public-api.birdeye.so/defi/token_creation_info?address={}", token_address);
//     let client = Client::new();

//     let response = client
//         .get(&url)
//         .header("X-API-KEY", api_key)
//         .header("x-chain", "sui")
//         .send()
//         .await?;
//     let text = response.text().await.expect("Not available to get response body");
//     let object: TokenOverview = serde_json::from_str(&text).expect("Invalid response parameters!!");

//     Ok(object)
// }

async fn make_token_overview_message(
    token_data: &TokenOverviewData,
    token_socials: &TokenSocial,
    token_holders: &TokenTopHolders
) -> Result<String, reqwest::Error> {
    let token_address = &token_data.address;
    let name = &token_data.name;
    let symbol = &token_data.symbol;
    let price = token_data.price;
    let liquidity = controll_big_float(token_data.liquidity).await?;

    let price_change_1h_percent = num_floating_point(
        &(token_data
            .price_change_1h_percent
            .expect("Invalid operation to get 1h price change percent")),
        4,
    )
    .await?;
    let price_change_6h_percent = num_floating_point(
        &(token_data
            .price_change_6h_percent
            .expect("Invalid operation to get 6h price change percent")),
        4,
    )
    .await?;
    let price_change_24h_percent = num_floating_point(
        &(token_data
            .price_change_24h_percent
            .expect("Invalid operation to get 24h price change percent")),
        4,
    )
    .await?;

    let buy_trade_1h = token_data.buy_1h;
    let sell_trade_1h = token_data.sell_1h;
    let buy_trade_24h = token_data.buy_24h;
    let sell_trade_24h = token_data.sell_24h;

    let volume_1h = controll_big_float(token_data.volume_1h).await?;
    let volume_6h = controll_big_float(token_data.volume_6h).await?;
    let volume_24h = controll_big_float(token_data.volume_24h).await?;

    //tokenSocail
    let social_website = token_socials.social_website.clone().unwrap_or_default();
    let social_discord = token_socials.social_discord.clone().unwrap();
    let social_telegram = token_socials.social_telegram.clone().unwrap_or_default();
    let social_twitter = token_socials.social_twitter.clone().unwrap();

    //top holders Info
    let holders_count = token_holders.holders_count;
    let mut index_on_a_line = 0;
    let mut num_whale = 0;
    let mut num_largefish = 0;
    let mut num_bigfish = 0;
    let mut num_smallfish = 0;
    let mut num_shrimp = 0;
    let mut holders_text = String::from("");
    for holder in &(token_holders.content) {
        index_on_a_line += 1;
        let holder_address = &holder.holder_address;
        let holder_usd_amount = holder.usd_amount;
        let holder_stock_percentage = holder.percentage;

        let mut symbol = String::from("");
        if holder_usd_amount > 100000.0 { symbol = format!("🐳"); num_whale += 1; }
        else if holder_usd_amount > 50000.0  {symbol = format!("🦈"); num_largefish += 1; }
        else if holder_usd_amount > 10000.0  {symbol = format!("🐬"); num_bigfish += 1; }
        else if holder_usd_amount > 1000.0  {symbol = format!("🐟"); num_smallfish += 1; }
        else { symbol = format!("🦐"); num_shrimp += 1; }

        let link = format!("<a href=\"https://suiscan.xyz/mainnet/account/{holder_address}?percentage={holder_stock_percentage}\"> {symbol} </a>");
        if index_on_a_line == 10 { holders_text = holders_text + &link + "\n"; index_on_a_line = 0; }
        else { holders_text = holders_text + &link; }

    }
    
    let text = format!("
⛓ SUI

🪙 <a href=\"{social_website}\"> {name} </a>  ({symbol})
👥 Socials: <a href=\"{social_discord}\">🌐</a> <a href=\"{social_telegram}\">💬</a> <a href=\"{social_twitter}\">𝕏</a>

{token_address}
➖➖➖➖➖➖

🏷 Price: ${price}
💧 Liq: ${liquidity} 

📉 Price Changes:
        1h: {price_change_1h_percent}%   |   6h: {price_change_6h_percent}%   |   24h: {price_change_24h_percent}%
🎚 Volume:
        1h:  ${volume_1h}  |  6h:  ${volume_6h}  |  24h:  ${volume_24h}
🔄 Buys / Sells:
        1h:  {buy_trade_1h} / {sell_trade_1h}   |   24h:  {buy_trade_24h} / {sell_trade_24h}

🧳 Holders:  {holders_count}

{holders_text}
🐳 ( > $100K ) :  {num_whale}
🦈 ( $50K - $100K ) :  {num_largefish}
🐬 ( $10K - $50K ) :  {num_bigfish}
🐟 ( $1K - $10K ) :  {num_smallfish}
🦐 ( $0 - $1K ) :  {num_shrimp}

❎ <a href=\"https://twitter.com/search?q={token_address}=typed_query&f=live\"> Search on 𝕏 </a>

📈 <a href=\"https://dexscreener.com/sui/{token_address}\"> DexS </a>

");

    Ok(text)
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    let username = msg.chat.username().unwrap();
    let message_text = msg.text().unwrap();

    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Start => {
            bot.send_message(msg.chat.id, format!("Welcome to Here {username}! 🎉"))
                .await?
        }
        Command::S => {
            let token_adr = message_text.replace("/s ", "");
            info!("Received command /s for token: {}", token_adr);

            match get_token_overview(&token_adr).await {
                Ok(token_overview) => {
                    let blockberry_client = Client::new();
                    let blockberry_api_key = env::var("Blockberry_API_KEY").expect("API_KEY not set");
                    let token_social = get_token_social_link(blockberry_client.clone(), &blockberry_api_key, &token_adr).await.unwrap();
                    
                    tokio::time::sleep(time::Duration::from_secs(3)).await;  //delay for 3 sec to avoid conflict request

                    let token_holders = get_top_50_holders(blockberry_client.clone(), &blockberry_api_key, &token_adr).await.unwrap();
                    
                    let token_data = token_overview.data;
                    let text = make_token_overview_message(&token_data, &token_social, &token_holders).await?;
                    bot.send_message(msg.chat.id, text)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?
                }
                Err(e) => {
                    error!("Error fetching token overview: {}", e);
                    bot.send_message(msg.chat.id, "Invalid token address")
                        .await?
                }
            }
        }
        Command::Jito => {
            bot.send_message(msg.chat.id, "Welcome to HyperLoop! 🎉")
                .await?
        }
    };

    Ok(())
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    dotenv().ok();

    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd).await?;
        }
    }

    Ok(())
}

async fn num_floating_point(num: &f64, length: i32) -> Result<f64, reqwest::Error> {
    let num_floating = ((num * 10_f64.powi(length as i32)).round()) / 10_f64.powi(length as i32);
    Ok(num_floating)
}

async fn controll_big_float(num: f64) -> Result<String, reqwest::Error> {
    let mut result_text = String::from("");
    if num > 1000000.0 {
        result_text = format!("{:.1}M", num / 1000000.0);
    } else if num > 1000.0 {
        result_text = format!("{:.2}K", num / 1000.0);
    }

    Ok(result_text)
}

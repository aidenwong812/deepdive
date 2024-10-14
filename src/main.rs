use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
use reqwest::Client;
use std::env;
use dotenv::dotenv;
use token_overview::{TokenOverview, TokenOverviewData};
use log::{error, info};

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
    #[command(description = "Get token overview\n\tEntry type: /s *******(token adr)")]
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

async fn get_token_overview(token_address: &str, api_key: &str) -> Result<TokenOverview, serde_json::Error> {
    let url = format!("https://public-api.birdeye.so/defi/token_overview?address={}", token_address);
    let client = Client::new();

    let response = client.get(&url)
        .header("X-API-KEY", api_key)
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


async fn make_token_overview_message(token_data: &TokenOverviewData) -> Result<String, reqwest::Error> {
    let token_address = &token_data.address;
    let name = &token_data.name;
    let symbol = &token_data.symbol;
    let liquidity = token_data.liquidity;
    let price = token_data.price;
    let price_change_1h_percent = num_floating_point(&(token_data.price_change_1h_percent.expect("Invalid operation to get 1h price change percent")), 4).await?;
    let price_change_6h_percent = num_floating_point(&(token_data.price_change_6h_percent.expect("Invalid operation to get 6h price change percent")), 4).await?;
    let price_change_24h_percent = num_floating_point(&(token_data.price_change_24h_percent.expect("Invalid operation to get 24h price change percent")), 4).await?;

    let buy_trade_1h = token_data.buy_1h;
    let sell_trade_1h = token_data.sell_1h;
    let buy_trade_24h = token_data.buy_24h;
    let sell_trade_24h = token_data.sell_24h;

    let volume_1h = controll_big_float(token_data.volume_1h).await?;
    let volume_6h = controll_big_float(token_data.volume_6h).await?;
    let volume_24h = controll_big_float(token_data.volume_24h).await?;

    let text_1 = format!("
⛓ SUI

🪙 {name}        ({symbol})
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

🧳 Holders: 320
⏳ Age: 154d 5h 3m

📡<a href=\"https://t.me/callanalyserbot\">Check for Calls</a>  ❎ <a href=\"https://twitter.com/search?q={token_address}=typed_query&f=live\"> Search on 𝕏 </a>

📈<a href=\"https://dexscreener.com/sui/{token_address}\"> DexS </a>

Ads: <a href=\"https://bit.ly/3TYlUWc\"> Comment & repost memes - get USD₮! 🤑 Launch & trade memecoins - claim $100. NO launch & tx fees, 10% ref bonus. 💰Earn on Meme Money Maker - multichain memepad!</a>
    ");
        
    Ok(text_1)

}

async fn answer(bot: Bot, msg: Message, cmd: Command, api_key: &str ) -> ResponseResult<()> {

    let username = msg.chat.username().unwrap();
    let message_text = msg.text().unwrap();
    let token_adr= message_text.replace("/s ", "");
    println!("{}\n{}", message_text, token_adr);
    
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
            let token_adr = message_text.replace("/s ", "");
            info!("Received command /s for token: {}", token_adr);
    
            match get_token_overview(&token_adr, api_key).await {
                Ok(token_overview) => {
                    let token_data = token_overview.data;
                    let text = make_token_overview_message(&token_data).await?;
                    bot.send_message(msg.chat.id, text)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .await?
                },
                Err(e) => {
                    error!("Error fetching token overview: {}", e);
                    bot.send_message(msg.chat.id, "Invalid token address.\nRewrite toekn address")
                        .await?
                }
            }
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

    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd, &api_key).await?;
        }
    }

    Ok(())
}

async fn num_floating_point(num: &f64, length: i32) -> Result<f64, reqwest::Error> {
    let num_floating = ((num * 10_f64.powi(length as i32)).round()) / 10_f64.powi(length as i32);    
    Ok(num_floating)
}

async fn controll_big_float(num: f64) -> Result<String, reqwest::Error> {
    
    let mut result_num = 0.0;
    let mut result_text = String::from("");
    if num > 1000000.0 {
        result_text = format!("{:.1}M", num / 1000000.0);
    } else if num > 1000.0 {
        result_text = format!("{:.2}K", num / 1000.0);
    }

    Ok(result_text)
}


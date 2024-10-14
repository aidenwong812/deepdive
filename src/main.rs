use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
};
// use serde::{Deserialize, Serialize,};
// use core::error;
use reqwest::{Client, Url};
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

    let url_1 = "https://t.me/callanalyserbot";
    
    
    let text_1 = format!("⛓ SUI
    👥 Socials: 🌐💬🐦
    ➖➖➖➖➖➖
    
    {token_address}
    
    🏷 Price: ${price}
    💧 Liq: ${liquidity} 
    
    📉 Price Changes:
            1h: {price_change_1h_percent}%   |   6h: {price_change_6h_percent}%   |   24h: {price_change_24h_percent}%
    🎚 Volume:
            1h: ${volume_1h}  |  6h:  ${volume_6h}  |  24h:  ${volume_24h}
    🔄 Buys/Sells:
            1h: {buy_trade_1h}/{sell_trade_1h}   |   24h: {buy_trade_24h}/{sell_trade_24h}
    
    🧳 Holders: 320
    ⏳ Age: 154d 5h 3m
    
    📡 Check for Calls {url_1} ❎ Search on 𝕏
    
    🎯 PIRBX | Maestro | Maestro Pro");
        
    // let text = format!(
    //     "{}\n\n[{}]({})\n{}",
    //     text_1,
    //     "📡 Check for Calls",
    //     "https://t.me/callanalyserbot",
    //     "
    // 📡 Check for Calls ❎ Search on 𝕏
    
    // 🎯 PIRBX | Maestro | Maestro Pro
    // ");

    Ok(text_1)

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

            bot.send_message(msg.chat.id, 
                text                
                )                
            // .markdown_v2()
            // .reply_markup(ReplyMarkup::InlineKeyboard(vec![
            //     InlineKeyboardButton::callback("Open Link", "open_link")
            // ]))
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

async fn num_floating_point(num: &f64, length: i32) -> Result<f64, reqwest::Error> {
    let num_floating = ((num * 10_f64.powi(length as i32)).round()) / 10_f64.powi(length as i32);    
    Ok(num_floating)
}

async fn controll_big_float(num: f64) -> Result<String, reqwest::Error> {
    
    let mut result_num = 0.0;
    let mut result_text = String::from("");
    if num > 1000000.0 {
        // result_num = num_floating_point(&(num / 1000000.0), 1).await?;
        result_text = format!("{:.1}M", num / 1000000.0);
    } else if num > 1000.0 {
        result_text = format!("{:.2}K", num / 1000.0);
    }

    // Ok(result_num);
    Ok(result_text)
}


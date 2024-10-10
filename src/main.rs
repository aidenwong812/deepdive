use teloxide::{
    prelude::*,
    types::{Me, MessageKind},
    utils::command::BotCommands,
    // RequestError,
    // ApiError,
};
use birdeye_rs::{
    // types::PriceOverviewData, 
    Client};
use std::sync::Arc;
use std::env;
use dotenv::dotenv;
use anyhow::Result;
// use serde::{Deserialize, Serialize,};


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

    // let token_data = (&birdeye_client).token_overview(token_address).await.expect("You can't get this token info. Please try agin a bit later");

    Ok(())
}


async fn answer(bot: Bot, msg: Message, cmd: Command, client: &Arc<Client>, token_adr: &str ) -> ResponseResult<()> {

    let username = msg.chat.username().unwrap();
    // println!("token_price resuilt: {:?}", (&client).price(token_adr).await);

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
            // let token_overview = match (&client).token_overview(token_adr).await {
            //     Ok(data) => data,
            //     Err(e) => {
            //         log::error!("💥Failed to get token overview💥: {}", e);
            //         // return Err(RequestError::Api(ApiError::Unknown(e.to_string())));
            //         return Err(RequestError::Api(ApiError::CantParseEntities(e.to_string())));
            //     }
            // };                        
            // let token_data = token_overview.data;
            
            // let token_address = token_data.address;
            // let symbol = token_data.symbol; 
            // let mcap = token_data.mc;
            // let liquidity = token_data.liquidity;
            // let supply = token_data.supply;
            // let liquidity_percent = liquidity / supply;
            // let price_change_30m_percent = token_data.price_change_30m_percent.expect("Invalid operation to get 30m price change percent");
            // let price_change_1h_percent = token_data.price_change_1h_percent.expect("Invalid operation to get 1h price change percent");
            // let price_change_24h_percent = token_data.price_change_24h_percent.expect("Invalid operation to get 24h price change percent");
            // let buy_1h_change_percent = token_data.buy_1h_change_percent.expect("Invalid operation to get 1h buy percent");
            // let buy_24h_change_percent = token_data.buy_24h_change_percent.expect("Invalid operation to get 24h buy percent");
            // let volume_1h = token_data.history_1h_price;
            // let volume_6h = token_data.history_6h_price;
            // let volume_24h = token_data.history_24h_price;
           
            // let text = format!("
            // {username}
            // symbol: {symbol} ()
            // ⛓ SOLANA
            // 👥 Socials: 🌐💬🐦
            // ➖➖➖➖➖➖
            // 🔎 Top 10 holders: 21.36% 🚨, no mint, liquidity burned, no blacklist
            
            // {token_address}
            
            // 📊 MCap: ${mcap} | ATH: $386.07K
            // 🏷 Price: $0.0000226500
            // 💧 Liq: ${liquidity} ({liquidity_percent}%)
            //         └🔥 100.00% Burned
            
            // 📉 Price Changes:
            //        30m: {price_change_30m_percent}% | 1h: {price_change_1h_percent}% | 24h: {price_change_24h_percent}%
            // 🎚 Volume:
            //        1h: ${volume_1h} | 6h: ${volume_6h} | 24h: ${volume_24h}
            // 🔄 Buys/Sells:
            //        1h: 0/0 | 24h: 0/3
            // 🔄 Buy percentage:
            //        1h: {buy_1h_change_percent} | 24h: {buy_24h_change_percent}
            
            // 🧳 Holders: 320
            // ⏳ Age: 154d 5h 3m
            // 📡 Check for Calls ❎ Search on 𝕏
            
            // 🎯 PIRBX | Maestro | Maestro Pro
            // ");

            // public API_Key using test
            let token_price_value = client.clone().price(token_adr).await.expect("Invalid operation 0").data.value;
            let token_price_update_human_time = client.clone().price(token_adr).await.expect("Invalid operation 1").data.update_human_time;
            let token_price_update_unix_time = client.clone().price(token_adr).await.expect("Invalid operation 2").data.update_unix_time;
            // let price_history = (&client).historical_price(&token_adr, 20241001,20251009).await.expect("Invalid operation");
            // let priceOverviewData_Vec = price_history.data.items;

            // for(item in priceOverviewData_Vec)


            let text = format!("
            token_price_value: {token_price_value} || token_price_update_human_time: {token_price_update_human_time}  || token_price_update_unix_time: {token_price_update_unix_time}
            ");
            bot.send_message(msg.chat.id, text)              
                .await?
        },
        Command::Jito => {
            bot.send_message(msg.chat.id, "Welcome to HyperLoop! 🎉")              
                .await?
        }

        // =>{
        //     println!("You sould type here valid command");
        // }
    };

    Ok(())
}

async fn message_handler(bot: Bot, msg: Message, me: Me) -> ResponseResult<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set");
    let token_address =  env::var("TOKEN_ADDRESS").expect("API_KEY not set");

    let birdeye_client = birdeye_rs::Client::new(&api_key).expect("Invalid API key !");
    // println!("{}", birdeye_client);

    if let MessageKind::WebAppData(data) = msg.kind {
        bot.send_message(msg.chat.id, data.web_app_data.data)
            .await?;
    } else if let Some(text) = msg.text() {
        if let Ok(cmd) = Command::parse(text, me.username()) {
            answer(bot, msg, cmd, &birdeye_client, &token_address).await?;
        }
    }

    Ok(())
}

// fn get_web_app_keyboard(chat_id: ChatId, user_name: &str) -> InlineKeyboardMarkup {
//     // let user_name = me.username.as_deref().unwrap_or(&me.first_name);
//     let web_app = WebAppInfo {
//         url: format!("https://hyperloop-nine.vercel.app/?id={chat_id}&username={user_name}").parse().unwrap(),
//     };
//     println!("{:?}", format!("https://hyperloop-nine.vercel.app/?id={chat_id}&username={user_name}"));
//     InlineKeyboardMarkup::new(vec![vec![
//         InlineKeyboardButton::new("Open HyperLoop", InlineKeyboardButtonKind::WebApp(web_app))
//     ]])
// }

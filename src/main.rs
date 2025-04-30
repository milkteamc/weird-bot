use std::env;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!banid") {
            let parts: Vec<&str> = msg.content.splitn(2, ' ').collect();
            if parts.len() > 1 {
                let content_after_command = parts[1];
                let response = format!("https://bans.milkteamc.org/bans/{}", content_after_command);
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    println!("Error sending message: {why:?}");
                }
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Please provide ban ID!")
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        }
        if msg.content.starts_with("!wiki") {
            let parts: Vec<&str> = msg.content.splitn(2, ' ').collect();
            if parts.len() > 1 {
                let content_after_command = parts[1];
                let response = format!("https://wiki.milkteamc.org/%E5%8A%9F%E8%83%BD%E6%B8%85%E5%96%AE/{}/", content_after_command);
                if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                    println!("Error sending message: {why:?}");
                }
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Please provide a name!")
                    .await
                {
                    println!("Error sending message: {why:?}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().expect("Failed to load .env file");

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

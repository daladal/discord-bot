use std::env;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

mod commands;
mod config;

use config::{ConfigMap, create_config_map, get_prefix};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("Bot is ready! Logged in as {}", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let prefix = get_prefix(
            &ctx.data.read().await.get::<ConfigMap>().expect("ConfigMap not found"), 
            msg.guild_id
            ).await;

        if !msg.content.starts_with(&prefix) {
            return;
        }

        let mut parts = msg.content[prefix.len()..].split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => return,
        };
        let args: Vec<&str> = parts.collect();

        commands::handle_command(&ctx, &msg, command, args).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN in environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ConfigMap>(create_config_map());
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


use std::env;
use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

mod commands;
mod config;
mod utils;

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

        let content_without_prefix = &msg.content[prefix.len()..];
        let mut parsed = utils::parse_args(content_without_prefix);

        if parsed.is_empty() {
            return;
        }

        let command = parsed.remove(0);
        let args: Vec<String> = parsed;
        commands::handle_command(&ctx, &msg, &command, args).await;
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


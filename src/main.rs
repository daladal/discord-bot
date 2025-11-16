use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use std::env;
use std::sync::Arc;

mod commands;
mod config;
mod utils;
mod database;
mod user_cache;
mod riot;

use config::{ConfigMap, DatabaseContainer, create_config_map, get_prefix};
use user_cache::{UserLinkCache, create_user_cache};
use database::Database;

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

        let data = ctx.data.read().await;
        let config_map = data.get::<ConfigMap>().expect("ConfigMap not found"); 
        let prefix = get_prefix(config_map, msg.guild_id); 

        if !msg.content.starts_with(&prefix) {
            return;
        }

        let content_without_prefix = &msg.content[prefix.len()..];
        let mut parsed = utils::parse_args(content_without_prefix);

        if parsed.is_empty() {
            return;
        }

        let command = parsed.remove(0);
        let args = parsed;
        commands::handle_command(&ctx, &msg, &command, args).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN in environment");

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:bot.db?mode=rwc".to_string());

    let db = Database::new(&database_url)
        .await
        .expect("Failed to initialize database");

    let config_map = create_config_map();
    match db.load_all_configs().await {
        Ok(configs) => {
            for (guild_id, config) in configs {
                config_map.insert(guild_id, config);
            }
            println!("Loaded {} guild configs from database", config_map.len());
        }
        Err(e) => {
            eprintln!("Failed to load configs from database: {}", e);
        }
    }

    let user_cache = create_user_cache();

    let intents = GatewayIntents::GUILD_MESSAGES 
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILDS;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ConfigMap>(config_map);
        data.insert::<DatabaseContainer>(Arc::new(db));
        data.insert::<UserLinkCache>(user_cache);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


use serenity::prelude::*;
use serenity::model::channel::Message;
use crate::config::{ConfigMap, DatabaseContainer, ServerConfig};

pub async fn handle_command(ctx: &Context, msg: &Message, command: &str, args: Vec<String>) {
    match command {
        "ping" => ping(ctx, msg).await,
        "help" => help(ctx, msg).await,
        "prefix" => prefix(ctx, msg, args).await,
        _ => {}
    }
}

async fn ping(ctx: &Context, msg: &Message) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}

async fn help(ctx: &Context, msg: &Message) {
    let help_text = "Available commands:\n\
        ping - Responds with Pong!\n\
        help - Shows this message\n\
        prefix [new_prefix] - View or set command prefix for this server\n\
        \n\
        Tip: Use quotes for multi-word arguments: `!command \"multi word arg\"`";

    if let Err(why) = msg.channel_id.say(&ctx.http, help_text).await {
        println!("Error sending message: {:?}", why);
    }
}

async fn prefix(ctx: &Context, msg: &Message, args: Vec<String>) {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            let _ = msg.channel_id.say(&ctx.http, "This command only works in servers!").await;
            return;
        }
    };

    let data = ctx.data.read().await;
    let config_map = data.get::<ConfigMap>().expect("ConfigMap not found"); 
    let db = data.get::<DatabaseContainer>().expect("Database not found");

    if args.is_empty() {
        let current_prefix = config_map.get(&guild_id)
            .map(|entry| entry.prefix.clone())
            .unwrap_or_else(|| "!".to_string());

        let response = format!("Current prefix: {}", current_prefix);
        let _ = msg.channel_id.say(&ctx.http, response).await;
        return;
    }

    let new_config = ServerConfig {
        prefix: args[0].clone(),
    };

    config_map.insert(guild_id, new_config.clone());

    if let Err(e) = db.save_config(guild_id, &new_config).await {
        eprintln!("Failed to save config to database: {}", e);
        let _ = msg.channel_id.say(&ctx.http, "Warning: Config saved to memory but failed to save to database!").await;
    }

    let response = format!("Prefix changed to: {}", args[0]);
    let _ = msg.channel_id.say(&ctx.http, response).await;
}

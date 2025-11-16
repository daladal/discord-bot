use serenity::prelude::*;
use serenity::model::channel::Message;
use crate::config::{ConfigMap, DatabaseContainer, ServerConfig};
use crate::user_cache::UserLinkCache;
use crate::database::UserLink;

pub async fn handle_command(ctx: &Context, msg: &Message, command: &str, args: Vec<String>) {
    match command {
        "ping" => ping(ctx, msg).await,
        "help" => help(ctx, msg).await,
        "prefix" => prefix(ctx, msg, args).await,
        "link" => link(ctx, msg, args).await,
        "unlink" => unlink(ctx, msg).await,
        "me" => me(ctx, msg).await,
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
        link <Name#TAG> <region>\n\
        unlink \n\
        me\n\
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

async fn link(ctx: &Context, msg: &Message, args: Vec<String>) {
    if args.len() < 2 {
        let _ = msg.channel_id.say(&ctx.http, "Usage: link <Name#TAG> <region>\n").await;
    }

    let riot_id = &args[0];
    let region = args[1].to_lowercase();

    let parts: Vec<&str> = riot_id.split('#').collect();
    if parts.len() != 2 {
        let _ = msg.channel_id.say(&ctx.http, "Invalid Riot ID format.\n").await;
        return;
    }

    let summoner_name = parts[0].to_string();
    let summoner_tag = parts[1].to_string();

    let valid_regions = ["na", "euw", "eune", "kr", "br", "lan", "las", "oce", "ru", "tr", "jp", "ph", "sg", "th", "tw", "vn"];
    if !valid_regions.contains(&region.as_str()) {
        let _ = msg.channel_id.say(&ctx.http,
            format!("Invalid region: {}. Valid regions: {}", region, valid_regions.join(", "))
        ).await;
        return;
    }

    let user_link = UserLink {
        discord_user_id: msg.author.id,
        summoner_name: summoner_name.clone(),
        summoner_tag: summoner_tag.clone(),
        region: region.clone(),
        riot_puuid: None,
    };

    let data = ctx.data.read().await;
    let db = data.get::<DatabaseContainer>().expect("Database not found");
    let cache = data.get::<UserLinkCache>().expect("UserLinkCache not found");

    if let Err(e) = db.save_user_link(&user_link).await {
        eprintln!("Failed to save user link: {}", e);
        let _ = msg.channel_id.say(&ctx.http, "Failed to link accounts.").await;
        return;
    }

    cache.insert(msg.author.id, user_link);

    let response = format!(
        "✅ Linked to **{}#{}** in region **{}**",
        summoner_name, summoner_tag, region.to_uppercase()
    );
    let _ = msg.channel_id.say(&ctx.http, response).await;
}

async fn unlink(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let db = data.get::<DatabaseContainer>().expect("Database not found");
    let cache = data.get::<UserLinkCache>().expect("UserLinkCache not found");

    if cache.get(&msg.author.id).is_none() {
        match db.get_user_link(msg.author.id).await {
            Ok(Some(_)) => {},
            Ok(None) => {
                let _ = msg.channel_id.say(&ctx.http, "You don't have a linked Riot account").await;
                return;
            }
            Err(e) => {
                eprintln!("Failed to check user link: {}", e);
                let _ = msg.channel_id.say(&ctx.http, "Failed to check your link. Please try again later").await;
                return;
            }
        }
    }

    match db.delete_user_link(msg.author.id).await {
        Ok(true) => {
            cache.remove(&msg.author.id);
            let _ = msg.channel_id.say(&ctx.http, "✅ Your account has been unlinked.").await;
        }
        Ok(false) => {
            let _ = msg.channel_id.say(&ctx.http, "You don't have a linked Riot account").await;
        }
        Err(e) => {
            eprintln!("Failed to delete user link: {}", e);
        }
    }   
}

async fn me(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let db = data.get::<DatabaseContainer>().expect("Database not found");
    let cache = data.get::<UserLinkCache>().expect("UserLinkCache not found");

    if let Some(link) = cache.get(&msg.author.id) {
        let response = format!(
            "**Your linked account:**\n **{}#{}**\n Region: **{}**",
            link.summoner_name, link.summoner_tag, link.region.to_uppercase()
        );
        let _ = msg.channel_id.say(&ctx.http, response).await;
        return;
    }

    match db.get_user_link(msg.author.id).await {
        Ok(Some(link)) => {
            cache.insert(msg.author.id, link.clone());

            let response = format!(
                "**Your linked account:**\n **{}#{}**\n Region: **{}**",
                link.summoner_name, link.summoner_tag, link.region.to_uppercase()
            );
            let _ = msg.channel_id.say(&ctx.http, response).await;
        }
        Ok(None) => {
            let _ = msg.channel_id.say(&ctx.http, "You don't have a linked Riot account").await;
        }
        Err(e) => {
            eprintln!("Failed to get user link: {}", e);
        }

    }
}














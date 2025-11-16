use serenity::prelude::*;
use serenity::model::channel::Message;
use crate::config::{DatabaseContainer, RiotClientContainer};
use crate::user_cache::UserLinkCache;
use crate::database::models::UserLink;
use crate::cache::{CachedData, ttl};

pub async fn link(ctx: &Context, msg: &Message, args: Vec<String>) {
    if args.len() < 2 {
        let _ = msg.channel_id.say(&ctx.http, "Usage: `link <Name#TAG> <region>`\nExample: `link Faker#KR1 kr`").await;
        return;
    }

    let riot_id = &args[0];
    let region = args[1].to_lowercase();

    let parts: Vec<&str> = riot_id.split('#').collect();
    if parts.len() != 2 {
        let _ = msg.channel_id.say(&ctx.http, "Invalid Riot ID format. Use `Name#TAG` (e.g., `Faker#KR1`)").await;
        return;
    }

    let summoner_name = parts[0].to_string();
    let summoner_tag = parts[1].to_string();

    let valid_regions = ["na", "euw", "eune", "kr", "br", "lan", "las", "oce", "ru", "tr", "jp", "ph", "sg", "th", "tw", "vn"];
    if !valid_regions.contains(&region.as_str()) {
        let _ = msg.channel_id.say(&ctx.http, 
            format!("Invalid region: `{}`. Valid regions: {}", region, valid_regions.join(", "))
        ).await;
        return;
    }

    let verify_msg = msg.channel_id.say(&ctx.http, "Verifying summoner with Riot API...").await;

    let data = ctx.data.read().await;
    let riot_client = data.get::<RiotClientContainer>().expect("RiotClient not found");

    let riot_account = match riot_client.get_account_by_riot_id(&summoner_name, &summoner_tag, &region).await {
        Ok(account) => account,
        Err(e) => {
            let error_msg = match e {
                crate::riot::RiotApiError::NotFound => {
                    format!("❌ Summoner **{}#{}** not found in region **{}**. Please check the name and tag.",
                        summoner_name, summoner_tag, region.to_uppercase())
                }
                crate::riot::RiotApiError::RateLimited => {
                    "Rate limited by Riot API. Please try again in a moment.".to_string()
                }
                crate::riot::RiotApiError::Unauthorized => {
                    "❌ API authentication error. Please contact the bot administrator.".to_string() 
                }
                _ => {
                    format!("❌ Failed to verify summoner: {}. Please try again later", e)
                }
            };

            if let Ok(mut verify_msg) = verify_msg {
                let _ = verify_msg.edit(&ctx.http, serenity::builder::EditMessage::new().content(&error_msg)).await;
            } else {
                let _ = msg.channel_id.say(&ctx.http, error_msg).await;
            }
            return; 
        }
    };

    let user_link = UserLink {
        discord_user_id: msg.author.id,
        summoner_name: riot_account.game_name.clone(),
        summoner_tag: riot_account.tag_line.clone(),
        region: region.clone(),
        riot_puuid: Some(riot_account.puuid.clone()),
    };

    let db = data.get::<DatabaseContainer>().expect("Database not found");
    let cache = data.get::<UserLinkCache>().expect("UserLinkCache not found");

    if let Err(e) = db.save_user_link(&user_link).await {
        eprintln!("Failed to save user link: {}", e);

        if let Ok(mut verify_msg) = verify_msg {
            let _ = verify_msg.edit(&ctx.http, serenity::builder::EditMessage::new().content("❌ Failed to save your link. Please try again later.")).await;
        }
        return;
    }

    cache.insert(msg.author.id, CachedData::new(user_link));

    let success_msg = format!(
        "✅ Linked your account to **{}#{}** in region **{}**",
        riot_account.game_name, riot_account.tag_line, region.to_uppercase()
    );

    if let Ok(mut verify_msg) = verify_msg {
        let _ = verify_msg.edit(&ctx.http, serenity::builder::EditMessage::new().content(&success_msg)).await;
    } else {
        let _ = msg.channel_id.say(&ctx.http, success_msg).await;
    }

}

pub async fn unlink(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let db = data.get::<DatabaseContainer>().expect("Database not found");
    let cache = data.get::<UserLinkCache>().expect("UserLinkCache not found");

    let has_cached = cache.get(&msg.author.id).is_some();

    if !has_cached {
        match db.get_user_link(msg.author.id).await {
            Ok(Some(_)) => {},
            Ok(None) => {
                let _ = msg.channel_id.say(&ctx.http, "You don't have a linked LoL account.").await;
                return;
            }
            Err(e) => {
                eprintln!("Failed to check user link: {}", e);
                let _ = msg.channel_id.say(&ctx.http, "Failed to check your link. Please try again later.").await;
                return;
            }
        }
    }

    match db.delete_user_link(msg.author.id).await {
        Ok(true) => {
            cache.remove(&msg.author.id);
            let _ = msg.channel_id.say(&ctx.http, "✅ Your LoL account has been unlinked.").await;
        }
        Ok(false) => {
            let _ = msg.channel_id.say(&ctx.http, "You don't have a linked LoL account.").await;
        }
        Err(e) => {
            eprintln!("Failed to delete user link: {}", e);
            let _ = msg.channel_id.say(&ctx.http, "Failed to unlink your account. Please try again later.").await;
        }
    }
}

pub async fn me(ctx: &Context, msg: &Message) {
    let data = ctx.data.read().await;
    let db = data.get::<DatabaseContainer>().expect("Database not found");
    let cache = data.get::<UserLinkCache>().expect("UserLinkCache not found");

    if let Some(cached) = cache.get(&msg.author.id) {
        if cached.is_stale(ttl::USER_LINK) {
            drop(cached);

            match db.get_user_link(msg.author.id).await {
                Ok(Some(fresh_link)) => {
                    cache.insert(msg.author.id, CachedData::new(fresh_link.clone()));

                    let response = format!(
                        "**Your linked account:**\n🎮 **{}#{}**\n🌍 Region: **{}**",
                       fresh_link.summoner_name, fresh_link.summoner_tag, fresh_link.region.to_uppercase()
                    );
                    let _ = msg.channel_id.say(&ctx.http, response).await;
                }
                Ok(None) => {
                    cache.remove(&msg.author.id);
                    let _ = msg.channel_id.say(&ctx.http, "You don't have a linked Riot account.\nUse `link <Name#TAG> <region>` to link one.").await;

                } 
                Err(e) => {
                    eprintln!("Failed to refresh  user link: {}", e);
                    let _ = msg.channel_id.say(&ctx.http, "Failed to retrieve your link. Please try again later.").await; 
                }
            }
        } else {
            let response = format!(
                "**Your linked account:**\n🎮 **{}#{}**\n🌍 Region: **{}**",
                cached.data.summoner_name, cached.data.summoner_tag, cached.data.region.to_uppercase()
            );
            let _ = msg.channel_id.say(&ctx.http, response).await;
        }
        return;
    }

    match db.get_user_link(msg.author.id).await {
        Ok(Some(link)) => {
            cache.insert(msg.author.id, CachedData::new(link.clone()));

            let response = format!(
                "**Your linked account:**\n🎮 **{}#{}**\n🌍 Region: **{}**",
                link.summoner_name, link.summoner_tag, link.region.to_uppercase()
            );
            let _ = msg.channel_id.say(&ctx.http, response).await;
        }
        Ok(None) => {
            let _ = msg.channel_id.say(&ctx.http, "You don't have a linked LoL account.\nUse `link <Name#TAG> <region>` to link one.").await;
        }
        Err(e) => {
            eprintln!("Failed to get user link: {}", e);
            let _ = msg.channel_id.say(&ctx.http, "Failed to retrieve your link. Please try again later.").await;
        }
    }
}


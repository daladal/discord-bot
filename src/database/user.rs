use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use serenity::model::id::UserId;
use super::models::UserLink;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn get_user_link(pool: &SqlitePool, user_id: UserId) -> Result<Option<UserLink>, sqlx::Error> {
    let user_id_str = user_id.to_string();

    let row = sqlx::query(
        "SELECT discord_user_id, summoner_name, summoner_tag, region, riot_puuid 
         FROM user_links 
         WHERE discord_user_id = ?1"
    )
    .bind(&user_id_str)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| UserLink {
        discord_user_id: UserId::new(r.get::<String, _>("discord_user_id").parse().unwrap()),
        summoner_name: r.get("summoner_name"),
        summoner_tag: r.get("summoner_tag"),
        region: r.get("region"),
        riot_puuid: r.get("riot_puuid"),
    }))
}

pub async fn save_user_link(pool: &SqlitePool, link: &UserLink) -> Result<(), sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let user_id_str = link.discord_user_id.to_string();

    sqlx::query(
        "INSERT INTO user_links (discord_user_id, summoner_name, summoner_tag, region, riot_puuid, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)
         ON CONFLICT(discord_user_id)
         DO UPDATE SET 
            summoner_name = excluded.summoner_name,
            summoner_tag = excluded.summoner_tag,
            region = excluded.region,
            riot_puuid = excluded.riot_puuid,
            updated_at = excluded.updated_at"
    )
    .bind(&user_id_str)
    .bind(&link.summoner_name)
    .bind(&link.summoner_tag)
    .bind(&link.region)
    .bind(&link.riot_puuid)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_user_link(pool: &SqlitePool, user_id: UserId) -> Result<bool, sqlx::Error> {
    let user_id_str = user_id.to_string();

    let result = sqlx::query("DELETE FROM user_links WHERE discord_user_id = ?1")
        .bind(&user_id_str)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}

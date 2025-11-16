use sqlx::sqlite::SqlitePool;
use sqlx::Row;
use serenity::model::id::GuildId;
use super::models::ServerConfig;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn load_all_configs(pool: &SqlitePool) -> Result<Vec<(GuildId, ServerConfig)>, sqlx::Error> {
    let rows = sqlx::query("SELECT guild_id, prefix FROM guild_configs")
        .fetch_all(pool)
        .await?;

    let configs = rows
        .into_iter()
        .map(|row| {
            let guild_id_str: String = row.get("guild_id");
            let guild_id = GuildId::new(guild_id_str.parse().unwrap());
            let prefix: String = row.get("prefix");
            
            (guild_id, ServerConfig { prefix })
        })
        .collect();

    Ok(configs)
}

pub async fn save_config(pool: &SqlitePool, guild_id: GuildId, config: &ServerConfig) -> Result<(), sqlx::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let guild_id_str = guild_id.to_string();

    sqlx::query(
        "INSERT INTO guild_configs (guild_id, prefix, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?3)
         ON CONFLICT(guild_id) 
         DO UPDATE SET prefix = excluded.prefix, updated_at = excluded.updated_at"
    )
    .bind(&guild_id_str)
    .bind(&config.prefix)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

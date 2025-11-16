use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row; 
use serenity::model::id::GuildId;
use serenity::model::id::UserId;
use crate::config::ServerConfig;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Database {
    pool: SqlitePool,
}

#[derive(Clone, Debug)]
pub struct UserLink {
    pub discord_user_id: UserId,
    pub summoner_name: String,
    pub summoner_tag: String,
    pub region: String,
    pub riot_puuid: Option<String>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        if database_url.starts_with("sqlite:") {
            let path = database_url.strip_prefix("sqlite:").unwrap();

            if let Some(parent) = std::path::Path::new(path).parent() {
                std::fs::create_dir_all(parent).ok();
            }   
        }
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        sqlx::query(include_str!("../migrations/001_init.sql"))
            .execute(&pool)
            .await?;

        sqlx::query(include_str!("../migrations/002_user_links.sql"))
            .execute(&pool)
            .await?;

        Ok(Database { pool })
    }

    pub async fn load_all_configs(&self) -> Result<Vec<(GuildId, ServerConfig)>, sqlx::Error> {
        let rows = sqlx::query("SELECT guild_id, prefix FROM guild_configs")
            .fetch_all(&self.pool)
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

    pub async fn save_config(&self, guild_id: GuildId, config: &ServerConfig) -> Result<(), sqlx::Error> {
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
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_user_link(&self, user_id: UserId) -> Result<Option<UserLink>, sqlx::Error> {
        let user_id_str = user_id.to_string();

        let row = sqlx::query(
            "SELECT discord_user_id, summoner_name, summoner_tag, region, riot_puuid
            FROM user_links
            WHERE discord_user_id = ?1"
            )
            .bind(&user_id_str)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| UserLink {
            discord_user_id: UserId::new(r.get::<String, _>("discord_user_id").parse().unwrap()),
            summoner_name: r.get("summoner_name"),
            summoner_tag: r.get("summoner_tag"),
            region: r.get("region"),
            riot_puuid: r.get("riot_puuid"),
        }))
    }

    pub async fn save_user_link(&self, link: &UserLink) -> Result<(), sqlx::Error> {
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
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn delete_user_link(&self, user_id: UserId) -> Result<bool, sqlx::Error> {
        let user_id_str = user_id.to_string();

        let result = sqlx::query("DELETE FROM user_links WHERE discord_user_id = ?1")
            .bind(&user_id_str)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}


















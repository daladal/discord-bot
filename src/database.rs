use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row; 
use serenity::model::id::GuildId;
use crate::config::ServerConfig;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Database {
    pool: SqlitePool,
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

    pub async fn get_config(&self, guild_id: GuildId) -> Result<Option<ServerConfig>, sqlx::Error> {
        let guild_id_str = guild_id.to_string();

        let row = sqlx::query("SELECT prefix FROM guild_configs WHERE guild_id = ?1")
            .bind(&guild_id_str)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| ServerConfig {
            prefix: r.get("prefix"),
        }))
    }
}


















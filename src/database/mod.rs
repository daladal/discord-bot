mod guild;
mod user;

pub mod models;

use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use serenity::model::id::{GuildId, UserId};
pub use models::{ServerConfig, UserLink};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        sqlx::query(include_str!("../../migrations/001_init.sql"))
            .execute(&pool)
            .await?;
        
        sqlx::query(include_str!("../../migrations/002_user_links.sql"))
            .execute(&pool)
            .await?;

        Ok(Database { pool })
    }

    pub async fn load_all_configs(&self) -> Result<Vec<(GuildId, ServerConfig)>, sqlx::Error> {
        guild::load_all_configs(&self.pool).await
    }

    pub async fn save_config(&self, guild_id: GuildId, config: &ServerConfig) -> Result<(), sqlx::Error> {
        guild::save_config(&self.pool, guild_id, config).await
    }

    pub async fn get_user_link(&self, user_id: UserId) -> Result<Option<UserLink>, sqlx::Error> {
        user::get_user_link(&self.pool, user_id).await
    }

    pub async fn save_user_link(&self, link: &UserLink) -> Result<(), sqlx::Error> {
        user::save_user_link(&self.pool, link).await
    }

    pub async fn delete_user_link(&self, user_id: UserId) -> Result<bool, sqlx::Error> {
        user::delete_user_link(&self.pool, user_id).await
    }
}

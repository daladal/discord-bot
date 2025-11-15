use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct ServerConfig {
    pub prefix: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            prefix: "!".to_string(),
        }
    }
}

pub struct ConfigMap;

impl TypeMapKey for ConfigMap {
    type Value = Arc<RwLock<HashMap<GuildId, ServerConfig>>>;
}

pub fn create_config_map() -> Arc<RwLock<HashMap<GuildId, ServerConfig>>> {
    Arc::new(RwLock::new(HashMap::new()))
}

pub async fn get_prefix(config_map: &Arc<RwLock<HashMap<GuildId, ServerConfig>>>, guild_id: Option<GuildId>) -> String {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return "!".to_string(),
    };

    let configs = config_map.read().await;
    configs.get(&guild_id)
        .map(|c| c.prefix.clone())
        .unwrap_or_else(|| "!".to_string())
}

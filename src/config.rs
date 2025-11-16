use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;
use dashmap::DashMap;
use std::sync::Arc;
use crate::database::models::ServerConfig;

pub struct ConfigMap;

impl TypeMapKey for ConfigMap {
    type Value = Arc<DashMap<GuildId, ServerConfig>>;
}

pub struct DatabaseContainer;

impl TypeMapKey for DatabaseContainer {
    type Value = Arc<crate::database::Database>;
}

pub struct RiotClientContainer;

impl TypeMapKey for RiotClientContainer {
    type Value = Arc<crate::riot::RiotClient>;
}

pub fn create_config_map() -> Arc<DashMap<GuildId, ServerConfig>> {
    Arc::new(DashMap::new())
}

pub fn get_prefix(config_map: &Arc<DashMap<GuildId, ServerConfig>>, guild_id: Option<GuildId>) -> String {
    let guild_id = match guild_id {
        Some(id) => id,
        None => return "!".to_string(),
    };
    
    config_map.get(&guild_id)
        .map(|entry| entry.prefix.clone())
        .unwrap_or_else(|| "!".to_string())
}

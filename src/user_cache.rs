use dashmap::DashMap;
use serenity::model::id::UserId;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use crate::database::models::UserLink;
use crate::cache::CachedData;

pub struct UserLinkCache;

impl TypeMapKey for UserLinkCache {
    type Value = Arc<DashMap<UserId, CachedData<UserLink>>>;
}

pub fn create_user_cache() -> Arc<DashMap<UserId, CachedData<UserLink>>> {
    Arc::new(DashMap::new())
}

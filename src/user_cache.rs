use dashmap::DashMap;
use serenity::model::id::UserId;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use crate::database::models::UserLink;

pub struct UserLinkCache;

impl TypeMapKey for UserLinkCache {
    type Value = Arc<DashMap<UserId, UserLink>>;
}

pub fn create_user_cache() -> Arc<DashMap<UserId, UserLink>> {
    Arc::new(DashMap::new())
}

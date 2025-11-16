use std::time::{Duration, SystemTime};

#[derive(Clone, Debug)]
pub struct CachedData<T> {
    pub data: T,
    cached_at: SystemTime,
}

impl<T> CachedData<T> {
    pub fn new(data: T) -> Self {
        CachedData {
            data,
            cached_at: SystemTime::now(),
        }
    }

    pub fn is_stale(&self, ttl: Duration) -> bool {
        SystemTime::now()
            .duration_since(self.cached_at)
            .map(|age| age > ttl)
            .unwrap_or(true)
    }

    pub fn age(&self) -> Option<Duration> {
        SystemTime::now().duration_since(self.cached_at).ok()
    }

    pub fn update(&mut self, data: T) {
        self.data = data;
        self.cached_at = SystemTime::now();
    }
}

pub mod ttl {
    use std::time::Duration;

    pub const USER_LINK: Duration = Duration::from_secs(86400);
    pub const SUMMONER_PROFILE: Duration = Duration::from_secs(3600); 
    pub const MATCH_HISTORY: Duration = Duration::from_secs(3600); 
}



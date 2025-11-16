use serenity::model::id::UserId;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct UserLink {
    pub discord_user_id: UserId,
    pub summoner_name: String,
    pub summoner_tag: String,
    pub region: String,
    pub riot_puuid: Option<String>,
}

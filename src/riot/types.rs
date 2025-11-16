use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiotAccount {
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Region {
    Americas,
    Europe,
    Asia,
    Sea,
}

impl Region {
    pub fn from_game_region(region: &str) -> Self {
        match region.to_lowercase().as_str() {
            "na" | "br" | "lan" | "las" => Region::Americas,
            "euw" | "eune" | "tr" | "ru" => Region::Europe,
            "kr" | "jp" => Region::Asia,
            _ => Region::Americas,
        }
    }

    pub fn api_base_url(&self) -> &'static str {
        match self {
            Region::Americas => "https://americas.api.riotgames.com",
            Region::Europe => "https://europe.api.riotgames.com",
            Region::Asia => "https://asia.api.riotgames.com",
            Region::Sea => "https://sea.api.riotgames.com",
        }
    }
}

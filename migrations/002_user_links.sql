CREATE TABLE IF NOT EXISTS user_links (
    discord_user_id TEXT PRIMARY KEY NOT NULL,
    summoner_name TEXT NOT NULL,
    summoner_tag TEXT NOT NULL,
    region TEXT NOT NULL,
    riot_puuid TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_user_links_summoner
ON user_links(summoner_name, summoner_tag, region);


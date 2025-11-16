CREATE TABLE IF NOT EXISTS guild_configs (
    guild_id TEXT PRIMARY KEY NOT NULL,
    prefix TEXT NOT NULL DEFAULT '!',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_guild_configs_updated
ON guild_configs(updated_at);


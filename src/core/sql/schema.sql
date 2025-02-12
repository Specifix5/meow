PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS interaction (
    -- Discord Snowflake --
    user_id TEXT NOT NULL PRIMARY KEY,
    -- Interactions --
    hug INTEGER DEFAULT 0,
    kiss INTEGER DEFAULT 0,
    slap INTEGER DEFAULT 0,
    bite INTEGER DEFAULT 0,
    cuddle INTEGER DEFAULT 0,
    pat INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS interaction_pair (
    -- UserID A_UserID B --
    users_id TEXT NOT NULL,
    -- Interactions --
    hug INTEGER DEFAULT 0,
    kiss INTEGER DEFAULT 0,
    slap INTEGER DEFAULT 0,
    bite INTEGER DEFAULT 0,
    cuddle INTEGER DEFAULT 0,
    pat INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
)
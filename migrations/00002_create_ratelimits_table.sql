CREATE TABLE ratelimits (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    api_key_id INTEGER NOT NULL,
    period INTEGER NOT NULL,
    `limit` INTEGER NOT NULL,
    created_at DATETIME NOT NULL DEFAULT NOW(),
    updated_at DATETIME NOT NULL DEFAULT NOW()
);
CREATE INDEX ratelimits_api_key_id_index ON ratelimits (api_key_id);

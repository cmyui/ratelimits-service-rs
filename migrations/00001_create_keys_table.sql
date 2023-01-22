CREATE TABLE api_keys (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    api_key VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL DEFAULT 'active',
    created_at DATETIME NOT NULL DEFAULT NOW(),
    updated_at DATETIME NOT NULL DEFAULT NOW()
);
CREATE INDEX api_keys_name_index ON api_keys (name);
CREATE INDEX api_keys_api_key_index ON api_keys (api_key);

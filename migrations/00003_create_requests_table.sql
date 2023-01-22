CREATE TABLE requests (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
    api_key_id INTEGER NOT NULL,
    method VARCHAR(255) NOT NULL,
    path VARCHAR(255) NOT NULL,
    created_at DATETIME NOT NULL DEFAULT NOW()
);
CREATE INDEX requests_api_key_id_index ON requests (api_key_id);

CREATE TABLE IF NOT EXISTS user(
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username        TEXT NOT NULL UNIQUE CHECK (length(username) >= 5 AND length(username) <= 20),
    password_hash   TEXT NOT NULL,
    created_at      INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS session(
    id          TEXT NOT NULL PRIMARY KEY,
    user_id     INTEGER NOT NULL,
    -- user_agent  TEXT NOT NULL,
    -- created_at  INTEGER NOT NULL,
    -- expires_at  INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

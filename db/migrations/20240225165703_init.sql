CREATE TABLE IF NOT EXISTS user(
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username    TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL,
    created_at  INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS session(
    id          TEXT NOT NULL PRIMARY KEY,
    user_id     INTEGER NOT NULL,
    -- user agent
    -- created_at
    -- expires_at
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

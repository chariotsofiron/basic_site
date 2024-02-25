create table user(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    username    TEXT NOT NULL UNIQUE,
    password    TEXT NOT NULL,
    created_at  INTEGER
);

create table session(
    id          TEXT PRIMARY KEY,
    user_id     INTEGER NOT NULL,
    -- user agent
    -- created_at
    -- expires_at
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

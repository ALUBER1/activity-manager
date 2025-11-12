-- Your SQL goes here
CREATE TABLE IF NOT EXISTS events(
    uuid TEXT PRIMARY KEY,
    name TEXT,
    date TEXT,
    time TEXT,
    notified_at TEXT
) WITHOUT ROWID;
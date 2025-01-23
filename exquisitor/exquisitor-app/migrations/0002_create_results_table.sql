CREATE TABLE results
(
    result_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    success       INTEGER NOT NULL CHECK (success in (0, 1)),
    filepath      TEXT
);

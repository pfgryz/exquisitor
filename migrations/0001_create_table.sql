CREATE TABLE experiments
(
    experiment_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT NOT NULL CHECK (length(name) <= 255),
    filepath      TEXT NOT NULL,
    status        TEXT NOT NULL CHECK (status in ('QUEUED', 'IN_PROGRESS', 'DONE')),
    result_id     INTEGER,
    FOREIGN KEY (result_id) REFERENCES results (result_id)
);

CREATE TABLE results
(
    result_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    organism_name  TEXT NOT NULL,
    organism_count REAL NOT NULL
);
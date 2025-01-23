CREATE TABLE orders
(
    order_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    name          TEXT NOT NULL CHECK (length(name) <= 255),
    filepath      TEXT NOT NULL,
    status        TEXT NOT NULL CHECK (status in ('QUEUED', 'IN_PROGRESS', 'DONE', 'FAILED')),
    result_id     INTEGER,
    FOREIGN KEY (result_id) REFERENCES results (result_id)
);

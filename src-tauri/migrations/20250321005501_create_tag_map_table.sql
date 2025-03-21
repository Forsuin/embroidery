-- Add migration script here
CREATE TABLE IF NOT EXISTS tag_map(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY(pattern_id) REFERENCES patterns(id),
    FOREIGN KEY(tag_id) REFERENCES tag(id)
);
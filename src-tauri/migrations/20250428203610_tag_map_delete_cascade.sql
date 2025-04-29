-- Add migration script here
ALTER TABLE tag_map RENAME TO _tag_map_old;

CREATE TABLE IF NOT EXISTS tag_map(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    pattern_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY(pattern_id) REFERENCES patterns(id) ON DELETE CASCADE,
    FOREIGN KEY(tag_id) REFERENCES tag(id) ON DELETE CASCADE
);

INSERT INTO tag_map SELECT * FROM _tag_map_old;
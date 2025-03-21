-- Add migration script here
CREATE TABLE IF NOT EXISTS patterns(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    pattern_num INTEGER NULL,
    thread_count INTEGER NULL
);

CREATE INDEX idx_patterns_name ON patterns(name);
CREATE INDEX idx_pattenrs_id ON patterns(id);
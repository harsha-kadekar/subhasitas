PRAGMA foreign_keys = ON;

CREATE TABLE categories (
    id TEXT PRIMARY KEY NOT NULL,
    category TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE subhasitas (
    id TEXT PRIMARY KEY NOT NULL,
    category_id TEXT NOT NULL,
    content TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT '',
    author TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (category_id) REFERENCES categories(id)
        ON DELETE RESTRICT
        ON UPDATE CASCADE
);

CREATE INDEX idx_subhasitas_category ON subhasitas(category_id);

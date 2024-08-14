CREATE TABLE IF NOT EXISTS projects (
    id          TEXT NOT NULL PRIMARY KEY,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name        TEXT NOT NULL,
    description TEXT NOT NULL,
    repository  TEXT NOT NULL
);

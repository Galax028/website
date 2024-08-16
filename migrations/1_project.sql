CREATE TABLE project (
    id          TEXT    NOT NULL PRIMARY KEY,
    created_at  TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TEXT    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    name        TEXT    NOT NULL,
    description TEXT    NOT NULL,
    starred     INTEGER NOT NULL DEFAULT 0,
    showcase    TEXT,
    repository  TEXT    NOT NULL
);

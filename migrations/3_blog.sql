CREATE TABLE blog (
    id         TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    title      TEXT NOT NULL,
    slug       TEXT NOT NULL UNIQUE,
    content    TEXT NOT NULL,

    PRIMARY KEY (id, slug, title)
);

CREATE TABLE blog_tag (
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    blog_id    TEXT NOT NULL REFERENCES blog (id) ON DELETE CASCADE,
    tag_id     TEXT NOT NULL REFERENCES tag (id)  ON DELETE CASCADE,

    PRIMARY KEY (blog_id, tag_id)
);

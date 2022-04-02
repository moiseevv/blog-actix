CREATE TABLE posts(
    id INTEGER PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users (id),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    pubished BOOLEAN NOT NULL DEFAULT 0
)
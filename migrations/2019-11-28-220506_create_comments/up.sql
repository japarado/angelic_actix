-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    body VARCHAR NOT NULL,
    post_id INTEGER NOT NULL REFERENCES posts(id),
    user_id INTEGER NOT NULL REFERENCES users(id)
)

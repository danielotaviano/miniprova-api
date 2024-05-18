-- Your SQL goes here

CREATE TABLE avatars (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    "url" TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    FOREIGN KEY (user_id) REFERENCES users(id),
    CONSTRAINT unique_user_id UNIQUE (user_id)
);
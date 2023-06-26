-- Add migration script here
CREATE TABLE IF NOT EXISTS user
(
    name TEXT NOT NULL,
    email TEXT PRIMARY KEY UNIQUE NOT NULL,
    password TEXT NOT NULL
);
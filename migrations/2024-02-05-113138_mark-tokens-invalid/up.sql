-- Your SQL goes here
ALTER TABLE tokens
    ADD COLUMN invalidated tinyint(1) NOT NULL DEFAULT 0;
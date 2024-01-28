-- Add up migration script here

CREATE EXTENSION pgcrypto;

-- user
ALTER TABLE users ADD CONSTRAINT unique_email UNIQUE (email);

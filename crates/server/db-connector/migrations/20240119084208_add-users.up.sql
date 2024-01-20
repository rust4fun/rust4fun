-- Add up migration script here

CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL PRIMARY KEY,
    name VARCHAR(255),
    email BYTEA,
    password BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_automatic_updating
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_automatic_updating_updated_at();

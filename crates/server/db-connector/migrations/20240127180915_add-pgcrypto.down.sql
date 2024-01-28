-- Add down migration script here

DROP EXTENSION pgcrypto;

-- user
ALTER TABLE users DROP CONSTRAINT unique_email;

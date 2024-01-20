-- Add up migration script here

CREATE TABLE IF NOT EXISTS articles (
    id UUID NOT NULL PRIMARY KEY,
    url varchar(2048) NOT NULL,
    title varchar(255),
    description varchar(255),
    image_url varchar(2048) NOT NULL DEFAULT 'https://www.rust-lang.org/static/images/rust-social-wide.jpg',
    registered_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_automatic_updating
BEFORE UPDATE ON articles
FOR EACH ROW
EXECUTE FUNCTION update_automatic_updating_updated_at();

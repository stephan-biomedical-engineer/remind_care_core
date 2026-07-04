-- Add migration script here
ALTER TABLE users 
ADD COLUMN email TEXT UNIQUE,
ADD COLUMN password_hash TEXT;

DELETE FROM users;

ALTER TABLE users ALTER COLUMN email SET NOT NULL;
ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;
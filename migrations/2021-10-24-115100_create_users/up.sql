CREATE EXTENSION IF NOT EXISTS citext WITH SCHEMA public;

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email citext unique not null,
  password_hash varchar(255) not null,
  inserted_at TIMESTAMP (0) NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
  updated_at TIMESTAMP (0) NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC')
);

SELECT diesel_manage_updated_at('users');

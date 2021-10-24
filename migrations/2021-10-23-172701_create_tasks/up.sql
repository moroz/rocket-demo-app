-- Your SQL goes here

CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  done BOOLEAN NOT NULL DEFAULT FALSE
);

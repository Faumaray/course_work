-- Add migration script here
CREATE TABLE games (
  id SERIAL PRIMARY KEY NOT NULL,
  game_name VARCHAR NOT NULL
)
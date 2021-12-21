-- Add migration script here
CREATE TABLE locations (
  id SERIAL PRIMARY KEY,
  gameid SERIAL NOT NULL,
  location_name VARCHAR NOT NULL,
  descr VARCHAR,
  on_map BYTEA,
  CONSTRAINT fk_game FOREIGN KEY(gameid) REFERENCES games(id)
)
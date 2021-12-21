-- Add migration script here
CREATE TABLE mobs (
  id SERIAL PRIMARY KEY,
  locationid SERIAL NOT NULL,
  mob_name VARCHAR NOT NULL,
  desct VARCHAR,
  preview BYTEA,
  CONSTRAINT fk_location FOREIGN KEY(locationid) REFERENCES locations(id)
)
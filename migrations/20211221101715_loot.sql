-- Add migration script here
CREATE TABLE loot (
  id SERIAL PRIMARY KEY,
  mobid SERIAL,
  locationid SERIAL,
  loot_name VARCHAR NOT NULL,
  descr VARCHAR,
  preview BYTEA,
  CONSTRAINT fk_mob FOREIGN KEY(mobid) REFERENCES mobs(id),
  CONSTRAINT fk_location FOREIGN KEY(locationid) REFERENCES locations(id)
)
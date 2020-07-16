-- Your SQL goes here
CREATE TABLE company (
  id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  dividend DATE NOT NULL,
  shares INTEGER NOT NULL,
  stdev FLOAT NOT NULL
)
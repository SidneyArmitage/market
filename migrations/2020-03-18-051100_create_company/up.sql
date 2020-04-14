-- Your SQL goes here
CREATE TABLE company (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  dividend DATE NOT NULL,
  shares INTEGER NOT NULL,
  alpha FLOAT NOT NULL,
  stdev FLOAT NOT NULL
)
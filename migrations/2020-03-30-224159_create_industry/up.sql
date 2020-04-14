-- Your SQL goes here
CREATE TABLE industry (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  beta FLOAT NOT NULL,
  stdev FLOAT NOT NULL
)
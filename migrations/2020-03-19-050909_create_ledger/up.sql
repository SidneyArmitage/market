-- Your SQL goes here
CREATE TABLE ledger (
  id SERIAL PRIMARY KEY,
  company INTEGER NOT NULL REFERENCES company(id),
  is_buy BOOLEAN NOT NULL,
  price MONEY NOT NULL,
  quantity INTEGER NOT NULL,
  buyer INTEGER NOT NULL
)
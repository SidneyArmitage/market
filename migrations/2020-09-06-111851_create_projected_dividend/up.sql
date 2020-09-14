-- Your SQL goes here
CREATE TABLE projected_dividend (
  company INTEGER NOT NULL REFERENCES company(id),
  index FLOAT NOT NULL,
  value_date DATE NOT NULL,
  PRIMARY KEY (company, value_date)
)
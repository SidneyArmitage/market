-- Your SQL goes here
CREATE TABLE industry_value (
  industry INTEGER REFERENCES industry(id),
  date DATE NOT NULL,
  value FLOAT NOT NULL,
  PRIMARY KEY (industry, date)
)
-- Your SQL goes here
CREATE TABLE industry_map (
  industry INTEGER REFERENCES industry(id) NOT NULL,
  company INTEGER REFERENCES company(id) NOT NULL,
  beta FLOAT NOT NULL,
  weight FLOAT NOT NULL,
  PRIMARY KEY (industry, company)
)
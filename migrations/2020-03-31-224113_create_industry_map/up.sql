-- Your SQL goes here
CREATE TABLE industry_map (
  industry INTEGER REFERENCES industry(id),
  company INTEGER REFERENCES company(id),
  beta FLOAT NOT NULL,
  PRIMARY KEY (industry, company)
)
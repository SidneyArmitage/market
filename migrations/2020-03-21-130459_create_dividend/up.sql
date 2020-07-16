-- Your SQL goes here
CREATE TABLE dividend (
  company INTEGER NOT NULL REFERENCES company(id) NOT NULL,
  payment_date DATE NOT NULL,
  announcement_date DATE NOT NULL,
  exdividend_date DATE NOT NULL,
  payment FLOAT NOT NULL,
  PRIMARY KEY (company, payment_date)
)
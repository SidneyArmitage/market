-- Your SQL goes here
CREATE TABLE dividend (
  company INTEGER NOT NULL REFERENCES company(id),--company
  payment_date DATE NOT NULL,--date dividend is payed
  announcement_date DATE NOT NULL,--date dividend is announced
  exdividend_date DATE NOT NULL,--date needed to own in order to receive
  payment FLOAT NOT NULL,--payment (value of dividend)
  PRIMARY KEY (company, payment_date)
)
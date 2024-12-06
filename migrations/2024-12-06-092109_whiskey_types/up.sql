-- Your SQL goes here

CREATE TABLE whiskey_type (
  id SERIAL NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT NOT NULL,
  annual_sale_in_liters BIGINT NOT NULL
)

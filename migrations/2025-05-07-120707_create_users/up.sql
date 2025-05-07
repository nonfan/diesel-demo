-- Your SQL goes here
CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   username VARCHAR NOT NULL,
   remark TEXT NOT NULL
)
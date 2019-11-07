-- Your SQL goes here
CREATE TABLE url_status (
  id SERIAL PRIMARY KEY,
  url_id SERIAL REFERENCES urls (id),
  created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (now() at time zone 'utc'),
  http_status INT NOT NULL
)

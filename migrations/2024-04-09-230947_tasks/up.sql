CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  created_at timestamp default current_timestamp NOT NULL,
  updated_at timestamp default current_timestamp NOT NULL,
  message VARCHAR NOT NULL
)

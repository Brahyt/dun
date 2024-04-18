CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  created_at timestamp default current_timestamp,
  updated_at timestamp,
  message VARCHAR NOT NULL
)

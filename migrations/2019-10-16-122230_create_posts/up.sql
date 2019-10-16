CREATE TABLE posts (
  uuid UUID PRIMARY KEY,
  author UUID NOT NULL,
  description TEXT NOT NULL,
  photo UUID NOT NULL
)

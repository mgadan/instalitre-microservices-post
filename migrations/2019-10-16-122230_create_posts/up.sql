CREATE TABLE posts (
  id UUID PRIMARY KEY,
  author UUID NOT NULL,
  description TEXT NOT NULL,
  photo TEXT NOT NULL
)

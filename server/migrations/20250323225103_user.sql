CREATE TABLE "user" (
  id UUID PRIMARY KEY,
  name VARCHAR(40) NOT NULL,
  email VARCHAR(254) NOT NULL UNIQUE,
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now()
);
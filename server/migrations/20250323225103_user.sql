CREATE TABLE "user" (
  id UUID PRIMARY KEY,
  name VARCHAR(40) NOT NULL,
  email VARCHAR(455) NOT NULL,
  created_at timestamp NOT NULL DEFAULT now(),
  updated_at timestamp NOT NULL DEFAULT now()
);
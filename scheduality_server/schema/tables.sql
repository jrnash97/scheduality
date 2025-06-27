CREATE TABLE IF NOT EXISTS Artist (
  id serial PRIMARY KEY,
  Name varchar(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS Label (
  id serial PRIMARY KEY,
  Name varchar(255)
);

CREATE TABLE IF NOT EXISTS Release (
  id serial PRIMARY KEY,
  uid uuid NOT NULL DEFAULT gen_random_uuid(),
  Name varchar(255) NOT NULL,
  Artist integer NOT NULL REFERENCES Artist ON DELETE RESTRICT,
  Label integer REFERENCES Label ON DELETE SET NULL,
  ReleaseDate date NOT NULL
);

CREATE TABLE IF NOT EXISTS Entity (
  id serial PRIMARY KEY,
  uid uuid NOT NULL DEFAULT gen_random_uuid()
);

CREATE TABLE IF NOT EXISTS Tag (
  id serial PRIMARY KEY,
  Data varchar(128) NOT NULL
);

CREATE TABLE IF NOT EXISTS EntityTag (
  id serial PRIMARY KEY,
  Entity integer NOT NULL REFERENCES Entity ON DELETE CASCADE,
  Tag integer NOT NULL REFERENCES Tag ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS EntityRelease (
  id serial PRIMARY KEY,
  Entity integer NOT NULL REFERENCES Entity ON DELETE CASCADE,
  EntityTag integer REFERENCES EntityTag ON DELETE SET NULL,
  Release integer NOT NULL REFERENCES Release ON DELETE CASCADE
);

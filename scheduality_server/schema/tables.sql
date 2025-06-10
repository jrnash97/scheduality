CREATE TABLE IF NOT EXISTS Artist (
  id serial PRIMARY KEY,
  Name text NOT NULL
);

CREATE TABLE IF NOT EXISTS Label (
  id serial PRIMARY KEY,
  Name text
);

CREATE TABLE IF NOT EXISTS Release (
  id serial PRIMARY KEY,
  uid uuid NOT NULL DEFAULT gen_random_uuid(),
  Name text NOT NULL,
  Artist integer NOT NULL REFERENCES Artist ON DELETE RESTRICT,
  Label integer REFERENCES Label ON DELETE SET NULL,
  ReleaseDate date NOT NULL
);

CREATE TABLE IF NOT EXISTS Client (
  id serial PRIMARY KEY,
  uid uuid NOT NULL DEFAULT gen_random_uuid()
);

CREATE TABLE IF NOT EXISTS ClientRelease (
  id serial PRIMARY KEY,
  Client integer NOT NULL REFERENCES Client ON DELETE CASCADE,
  Release integer NOT NULL REFERENCES Release ON DELETE CASCADE
);

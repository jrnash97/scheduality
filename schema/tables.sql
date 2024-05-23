DROP TABLE IF EXISTS GuildPrivilegedRole CASCADE;
DROP TABLE IF EXISTS GuildBannedUser CASCADE;
DROP TABLE IF EXISTS GuildUpdatePolicy CASCADE;
DROP TABLE IF EXISTS GuildRelease CASCADE;
DROP TABLE IF EXISTS Release CASCADE;
DROP TABLE IF EXISTS Artist CASCADE;
DROP TABLE IF EXISTS Label CASCADE;
DROP TABLE IF EXISTS Guild CASCADE;
DROP TABLE IF EXISTS GuildUser CASCADE;

CREATE TABLE Artist (
  id serial PRIMARY KEY,
  Name text NOT NULL
);

CREATE TABLE Label (
  id serial PRIMARY KEY,
  Name text
);

CREATE TABLE Release (
  id serial PRIMARY KEY,
  Name text NOT NULL,
  Artist integer NOT NULL REFERENCES Artist ON DELETE RESTRICT,
  Label integer REFERENCES Label ON DELETE SET NULL,
  ReleaseDate date NOT NULL
);

CREATE TABLE GuildUser (
  id serial PRIMARY KEY,
  Snowflake bigint NOT NULL
);

CREATE TABLE Guild (
  id serial PRIMARY KEY,
  Snowflake bigint UNIQUE NOT NULL,
  Owner int NOT NULL REFERENCES GuildUser ON DELETE CASCADE,
  Joined timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE GuildRelease (
  id serial PRIMARY KEY,
  GuildId integer NOT NULL REFERENCES Guild ON DELETE CASCADE,
  GuildUserId integer REFERENCES GuildUser ON DELETE SET NULL,
  ReleaseId integer NOT NULL REFERENCES Release ON DELETE RESTRICT,
  SubmittedAt timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE GuildPrivilegedRole (
  id serial PRIMARY KEY,
  integer integer NOT NULL REFERENCES Guild ON DELETE CASCADE,
  Snowflake bigint NOT NULL
);

CREATE TABLE GuildBannedUser (
  id serial PRIMARY KEY,
  GuildId integer NOT NULL REFERENCES Guild ON DELETE CASCADE,
  UserId integer NOT NULL REFERENCES GuildUser,
  IsBanned bit NOT NULL,
  BannedBy integer NOT NULL REFERENCES GuildUser
);

CREATE TABLE GuildUpdatePolicy (
  GuildId serial PRIMARY KEY REFERENCES Guild ON DELETE CASCADE,
  UpdateRole bigint,
  UpdateChannel bigint
);

DROP TABLE IF EXISTS GuildPrivilegedRoles;
DROP TABLE IF EXISTS GuildBannedUsers;
DROP TABLE IF EXISTS GuildUpdatePolicies;
DROP TABLE IF EXISTS GuildReleases;
DROP TABLE IF EXISTS Releases;
DROP TABLE IF EXISTS Artists;
DROP TABLE IF EXISTS Labels;
DROP TABLE IF EXISTS Guilds;
DROP TABLE IF EXISTS Users;

DROP TYPE IF EXISTS Period;
DROP TYPE IF EXISTS Day;

CREATE TYPE Period AS Enum('Daily', 'Weekly', 'Monthly');
CREATE TYPE Day AS Enum('Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday');


CREATE TABLE Artists (
  id serial PRIMARY KEY,
  Name text NOT NULL
);

CREATE TABLE Labels (
  id serial PRIMARY KEY,
  Name text
);

CREATE TABLE Releases (
  id serial PRIMARY KEY,
  Name text NOT NULL,
  Artist integer NOT NULL REFERENCES Artists ON DELETE RESTRICT,
  Label integer REFERENCES Labels ON DELETE SET NULL,
  ReleaseDate date NOT NULL
);

CREATE TABLE Users (
  id serial PRIMARY KEY,
  Snowkflake bigint NOT NULL
);

CREATE TABLE Guilds (
  id serial PRIMARY KEY,
  Snowflake bigint UNIQUE NOT NULL,
  Joined timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE GuildReleases (
  id serial PRIMARY KEY,
  GuildId integer NOT NULL REFERENCES Guilds ON DELETE CASCADE,
  UserId integer REFERENCES Users ON DELETE SET NULL,
  ReleaseId integer NOT NULL REFERENCES Releases ON DELETE RESTRICT,
  Submitted timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS GuildPrivilegedRoles (
  id serial PRIMARY KEY,
  integer integer NOT NULL REFERENCES Guilds ON DELETE CASCADE,
  Snowflake bigint NOT NULL
);

CREATE TABLE IF NOT EXISTS GuildBannedUsers (
  id serial PRIMARY KEY,
  GuildId integer NOT NULL REFERENCES Guilds ON DELETE CASCADE,
  UserId integer NOT NULL REFERENCES Users,
  IsBanned bit NOT NULL,
  BannedBy integer NOT NULL REFERENCES Users
);

CREATE TABLE IF NOT EXISTS GuildUpdatePolicies (
  GuildId serial PRIMARY KEY REFERENCES Guilds ON DELETE CASCADE,
  UpdatePeriod Period NOT NULL DEFAULT 'Weekly',
  UpdateDay Day NOT NULL DEFAULT 'Friday',
  UpdateRole bigint,
  UpdateChannel bigint
);

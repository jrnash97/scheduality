CREATE OR REPLACE FUNCTION fetch_or_insert_artist(text) RETURNS integer 
AS $$
DECLARE ArtistId integer;
BEGIN
  IF (SELECT count(*) FROM Artist WHERE Name = $1) >= 1 THEN
    SELECT id INTO ArtistId FROM Artist WHERE Name=$1 LIMIT 1;
  ELSE
    INSERT INTO Artist (Name) VALUES ($1) RETURNING id INTO ArtistId;
  END IF;
  RETURN ArtistId;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION fetch_or_insert_label(text) RETURNS integer 
AS $$
DECLARE LabelId integer;
BEGIN
  IF (SELECT count(*) FROM Label WHERE Name = $1) >= 1 THEN
    SELECT id INTO LabelId FROM Label WHERE Name=$1 LIMIT 1;
  ELSE
    INSERT INTO Label (Name) VALUES ($1) RETURNING id INTO LabelId;
  END IF;
  RETURN LabelId;
END;
$$
LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION fetch_or_insert_guild_user(bigint) RETURNS integer 
AS $$
DECLARE UserId integer;
BEGIN
  IF (SELECT count(*) FROM GuildUser WHERE Snowflake = $1) >= 1 THEN
    SELECT id INTO UserId FROM GuildUser WHERE Name=$1 LIMIT 1;
  ELSE
    INSERT INTO GuildUser (Snowflake) VALUES ($1) RETURNING id INTO UserId;
  END IF;
  RETURN UserId;
END;
$$
LANGUAGE plpgsql;


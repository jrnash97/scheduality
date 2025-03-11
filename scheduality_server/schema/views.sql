CREATE OR REPLACE VIEW ReleaseData AS
  SELECT 
    Release.id AS id,
    Artist.Name AS Artist,
    Release.Name AS Name,
    Label.Name AS Label,
    Release.ReleaseDate AS ReleaseDate
  FROM
    Release 
    INNER JOIN Artist ON Release.Artist = Artist.id
    LEFT OUTER JOIN Label ON Release.Label = Label.id;


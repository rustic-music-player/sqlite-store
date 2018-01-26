CREATE TABLE albums
(
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  title VARCHAR(255) NOT NULL,
  artist_id INTEGER,
  coverart TEXT,
  column_5 INT,
  CONSTRAINT albums_artists_id_fk FOREIGN KEY (artist_id) REFERENCES artists (id) ON DELETE SET NULL
);
CREATE UNIQUE INDEX albums_id_uindex ON albums (id);
#[macro_use]
extern crate diesel;

extern crate rustic_core as core;

mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

enum SqliteError {
    ConnectionError(diesel::ConnectionError)
}

impl From<diesel::ConnectionError> for SqliteError {
    fn from(err: diesel::ConnectionError) -> Self {
        SqliteError::ConnectionError(err)
    }
}

impl From<diesel::result::Error> for SqliteError {
    fn from(_: diesel::result::Error) -> Self {
        unimplemented!()
    }
}

impl From<SqliteError> for core::RusticError {
    fn from(_: SqliteError) -> Self {
        unimplemented!()
    }
}

struct SqliteStore {
    connection: SqliteConnection
}

impl SqliteStore {
    pub fn new(url: String) -> Result<SqliteStore, SqliteError> {
        let connection = SqliteConnection::establish(&url)?;

        Ok(SqliteStore {
            connection
        })
    }

    fn load_tracks(&self) -> Result<Vec<TrackEntity>, SqliteError> {
        use schema::tracks::dsl::*;

        let track_list = tracks.load::<TrackEntity>(&self.connection)?;

        Ok(track_list)
    }
}

impl core::LibraryStore for SqliteStore {
    fn store(&mut self, library: &core::Library) -> Result<(), core::RusticError> {
        unimplemented!()
    }

    fn load(&self) -> Result<core::Library, core::RusticError> {
        let tracks = self.load_tracks()?;

        Err(core::RusticError::LibraryLoadError(String::new()))
    }
}

#[derive(Queryable)]
pub struct TrackEntity {
    pub id: i32,
    pub title: String,
    pub artist_id: Option<i32>,
    pub album_id: Option<i32>,
    pub stream_url: String,
    pub uri: String,
    pub coverart: Option<String>,
    pub duration: Option<i32>
}

#[derive(Queryable)]
pub struct AlbumEntity {
    pub id: i32,
    pub title: String,
    pub artist_id: Option<i32>,
    pub coverart: Option<String>,
}

#[derive(Queryable)]
pub struct ArtistEntity {
    pub id: i32,
    pub name: String,
}
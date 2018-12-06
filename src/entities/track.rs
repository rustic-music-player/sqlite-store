use core::Track;
use entities::provider::int_to_provider;
use failure::Error;

#[derive(Queryable)]
pub struct TrackEntity {
    pub id: i32,
    pub title: String,
    pub artist_id: Option<i32>,
    pub album_id: Option<i32>,
    pub stream_url: String,
    pub provider: i32,
    pub uri: String,
    pub image_url: Option<String>,
    pub duration: Option<i32>,
}

impl TrackEntity {
    pub fn into_track(self) -> Result<Track, Error> {
        Ok(Track {
            id: Some(self.id as usize),
            title: self.title,
            artist_id: self.artist_id.map(|id| id as usize),
            artist: None,
            album_id: self.album_id.map(|id| id as usize),
            album: None,
            stream_url: self.stream_url,
            provider: int_to_provider(self.provider)?,
            uri: self.uri,
            image_url: self.image_url,
            duration: self.duration.map(|duration| duration as u64),
        })
    }
}

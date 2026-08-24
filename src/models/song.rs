use url::Url;

use crate::models::{ArtistPreview, SongWriterPreview};

#[derive(Debug, PartialEq)]
pub struct SongPreview {
    pub id: String,
    pub title: String,
    pub artist: ArtistPreview,
}

pub struct Song {
    pub id: String,
    pub title: String,
    pub image: Url,
    pub artist: ArtistPreview,
    pub lyricists: Vec<SongWriterPreview>,
    pub composers: Vec<SongWriterPreview>,
    pub arrangers: Vec<SongWriterPreview>,
    pub tags: Vec<String>,
    pub lyrics: String,
}

use crate::models::SongPreview;

#[derive(Debug, PartialEq)]
pub struct ArtistPreview {
    pub id: String,
    pub name: String,
}

pub struct Artist {
    pub id: String,
    pub name: String,
    pub songs: Vec<SongPreview>,
}

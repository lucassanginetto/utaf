use crate::models::SongPreview;

#[derive(Clone, Debug, PartialEq)]
pub struct ArtistPreview {
    pub id: String,
    pub name: String,
}
impl From<Artist> for ArtistPreview {
    fn from(value: Artist) -> Self {
        ArtistPreview {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub songs: Vec<SongPreview>,
}

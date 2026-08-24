use super::SongPreview;

pub struct SongWriterPreview {
    pub id: String,
    pub name: String,
}

pub struct SongWriter {
    pub id: String,
    pub name: String,
    pub songs: Vec<SongPreview>,
}

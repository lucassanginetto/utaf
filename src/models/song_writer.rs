use super::SongPreview;

pub struct SongWriterPreview {
    pub id: String,
    pub name: String,
}

pub struct SongWriter {
    pub id: String,
    pub name: String,
    pub songs_as_lyricist: Vec<SongPreview>,
    pub songs_as_composer: Vec<SongPreview>,
    pub songs_as_arranger: Vec<SongPreview>,
}

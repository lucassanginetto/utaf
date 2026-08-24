use std::sync::LazyLock;

use scraper::{Html, Selector};

use crate::{
    models::SongWriter,
    scrapers::{ScrapeError, scrape_id_from_og_url_meta, scrape_songs_table},
};

static SONG_WRITER_NAME_H1: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("h1.contentBox__title").unwrap());

pub fn scrape_song_writer(html: &Html) -> Result<SongWriter, ScrapeError<'_>> {
    let id = scrape_id_from_og_url_meta(html, 1)?;
    let name = {
        let selector = &SONG_WRITER_NAME_H1;
        html.select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .and_then(|h2| {
                h2.text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .map(|text| {
                        let trimmed_text = text.trim();
                        trimmed_text
                            .strip_suffix("の作詞・作曲・編曲歌詞一覧")
                            .unwrap_or(trimmed_text)
                    })
                    .map(|name_str| name_str.to_string())
            })
    }?;
    let songs = scrape_songs_table(html)?;

    Ok(SongWriter { id, name, songs })
}

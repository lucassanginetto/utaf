use std::sync::LazyLock;

use scraper::{Html, Selector};

use crate::{
    models::SongWriter,
    scrapers::{ScrapeError, scrape_id_from_og_url_meta, scrape_songs_table},
};

static SONG_WRITER_NAME_H1_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("h1.contentBox__title").unwrap());

static TABLE_SONG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("p.searchResult__title > a").unwrap());

static TABLE_ARTIST_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.searchResult__artist > p > a").unwrap());

pub fn scrape_song_writer(html: &Html) -> Result<SongWriter, ScrapeError<'_>> {
    let id = scrape_id_from_og_url_meta(html, 1)?;
    let name = {
        let selector = &SONG_WRITER_NAME_H1_SELECTOR;
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
    let songs = scrape_songs_table(html, &TABLE_SONG_A_SELECTOR, &TABLE_ARTIST_A_SELECTOR)?;

    Ok(SongWriter { id, name, songs })
}

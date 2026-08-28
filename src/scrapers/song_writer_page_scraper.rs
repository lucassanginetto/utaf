use std::sync::LazyLock;

use scraper::{Html, Selector};

use crate::{
    models::SongWriter,
    scrapers::{ScrapeError, scrape_id_from_og_url_meta, scrape_songs_table},
};

static SONG_WRITER_NAME_H1_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("h1.contentBox__title").unwrap());

static AS_LYRICIST_TABLE_SONG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#lyricistTab p.searchResult__title > a").unwrap());

static AS_LYRICIST_TABLE_ARTIST_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#lyricistTab td.searchResult__artist > p > a").unwrap());

static AS_COMPOSER_TABLE_SONG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#composerTab p.searchResult__title > a").unwrap());

static AS_COMPOSER_TABLE_ARTIST_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#composerTab td.searchResult__artist > p > a").unwrap());

static AS_ARRANGER_TABLE_SONG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#arrangerTab p.searchResult__title > a").unwrap());

static AS_ARRANGER_TABLE_ARTIST_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("#arrangerTab td.searchResult__artist > p > a").unwrap());

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
    let songs_as_lyricist = {
        scrape_songs_table(
            html,
            &AS_LYRICIST_TABLE_SONG_A_SELECTOR,
            &AS_LYRICIST_TABLE_ARTIST_A_SELECTOR,
        )
    }?;
    let songs_as_composer = {
        scrape_songs_table(
            html,
            &AS_COMPOSER_TABLE_SONG_A_SELECTOR,
            &AS_COMPOSER_TABLE_ARTIST_A_SELECTOR,
        )
    }?;
    let songs_as_arranger = {
        scrape_songs_table(
            html,
            &AS_ARRANGER_TABLE_SONG_A_SELECTOR,
            &AS_ARRANGER_TABLE_ARTIST_A_SELECTOR,
        )
    }?;

    Ok(SongWriter {
        id,
        name,
        songs_as_lyricist,
        songs_as_composer,
        songs_as_arranger,
    })
}

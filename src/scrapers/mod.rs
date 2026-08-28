use std::sync::LazyLock;

use scraper::{Html, Selector, selector::ToCss};
use thiserror::Error;
use url::Url;

use crate::models::{ArtistPreview, SongPreview};

mod artist_page_scraper;
mod lyrics_page_scraper;
mod search_page_scraper;
mod song_writer_page_scraper;

pub use artist_page_scraper::*;
pub use lyrics_page_scraper::*;
pub use search_page_scraper::*;
pub use song_writer_page_scraper::*;

#[derive(Clone, Copy, Debug, Error)]
pub enum ScrapeError<'a> {
    #[error("selector \"{}\" failed to find element", selector.to_css_string())]
    MissingElement { selector: &'static Selector },

    #[error("element found by selector \"{}\" has no text", selector.to_css_string())]
    MissingText { selector: &'static Selector },

    #[error("element found by selector \"{}\" has invalid text \"{text}\"", selector.to_css_string())]
    InvalidText {
        selector: &'static Selector,
        text: &'a str,
    },

    #[error("element found by selector \"{}\" has no \"{attr}\" attribute", selector.to_css_string())]
    MissingAttribute {
        selector: &'static Selector,
        attr: &'static str,
    },

    #[error("element found by selector \"{}\" has invalid value \"{value}\" for attribute \"{attr}\"", .selector.to_css_string())]
    InvalidAttribute {
        selector: &'static Selector,
        attr: &'static str,
        value: &'a str,
    },
}

/* Scraping functions used by multiple scrapers */

static OG_URL_META_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("meta[property=\"og:url\"]").unwrap());

fn scrape_id_from_og_url_meta(
    html: &Html,
    id_path_segment_index: usize,
) -> Result<String, ScrapeError<'_>> {
    let selector = &OG_URL_META_SELECTOR;
    let attr = "content";
    html.select(selector)
        .next()
        .ok_or(ScrapeError::MissingElement { selector })
        .and_then(|meta| {
            meta.attr(attr)
                .ok_or(ScrapeError::MissingAttribute { selector, attr })
        })
        .and_then(|value| {
            let invalid_attribute_error = ScrapeError::InvalidAttribute {
                selector,
                attr,
                value,
            };
            Url::parse(value)
                .map_err(|_| invalid_attribute_error)
                .and_then(|url| {
                    url.path_segments()
                        .ok_or(invalid_attribute_error)
                        .and_then(|mut segments| {
                            segments
                                .nth(id_path_segment_index)
                                .ok_or(invalid_attribute_error)
                        })
                        .map(|id| id.to_string())
                })
        })
}

fn scrape_songs_table<'a>(
    html: &'a Html,
    table_song_a_selector: &'static Selector,
    table_artist_a_selector: &'static Selector,
) -> Result<Vec<SongPreview>, ScrapeError<'a>> {
    {
        let selector = table_artist_a_selector;
        html.select(selector).map(|a| {
            let id = {
                let attr = "href";
                a.attr(attr)
                    .ok_or(ScrapeError::MissingAttribute { selector, attr })
                    .and_then(|value| {
                        value
                            .split('/')
                            .nth(2)
                            .ok_or(ScrapeError::InvalidAttribute {
                                selector,
                                attr,
                                value,
                            })
                            .map(|id_str| id_str.to_string())
                    })
            }?;
            let name = a
                .text()
                .next()
                .ok_or(ScrapeError::MissingText { selector })
                .map(|text| text.to_string())?;
            Ok(ArtistPreview { id, name })
        })
    }
    .zip({
        let selector = table_song_a_selector;
        html.select(selector).map(|a| {
            let id = {
                let attr = "href";
                a.attr(attr)
                    .ok_or(ScrapeError::MissingAttribute { selector, attr })
                    .and_then(|value| {
                        value
                            .split('/')
                            .nth(2)
                            .ok_or(ScrapeError::InvalidAttribute {
                                selector,
                                attr,
                                value,
                            })
                            .map(|id_str| id_str.to_string())
                    })
            }?;
            let title = a
                .text()
                .next()
                .ok_or(ScrapeError::MissingText { selector })
                .map(|text| text.to_string())?;
            Ok((id, title))
        })
    })
    .map(|(artist_result, id_and_title_result)| {
        let (id, title) = id_and_title_result?;
        let artist = artist_result?;
        Ok(SongPreview { id, title, artist })
    })
    .collect::<Result<Vec<_>, ScrapeError>>()
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_song_preview_scraping() {
        let song_preview_vec = SongPreview::scrape_search_page(&Html::parse_document(
            r##"
                <table class="searchResult artistLyricList">
                    <tbody>
                        <tr>
                        </tr>
                        <tr>
                            <td>
                                <h3>
                                    <a href="/lyric/test_song_1/">Test Song 1</a>
                                </h3>
                            </td>
                            <td class="searchResult__artist">
                                <p>
                                    <a href="/artist/test_artist_1/">Test Artist 1</a>
                                </p>
                                <div class="searchResult__lyricist">
                                </div>
                            </td>
                            <td>
                            </td>
                        </tr>
                        <tr>
                            <td>
                                <h3>
                                    <a href="/lyric/test_song_2/">Test Song 2</a>
                                </h3>
                            </td>
                            <td class="searchResult__artist">
                                <p>
                                    <a href="/artist/test_artist_2/">Test Artist 2</a>
                                </p>
                                <div class="searchResult__lyricist">
                                </div>
                            </td>
                            <td>
                            </td>
                        </tr>
                    </tbody>
                </table>
            "##,
        ))
        .unwrap();

        assert_eq!(song_preview_vec.len(), 2);

        assert_eq!(
            song_preview_vec[0],
            SongPreview {
                id: "test_song_1".to_string(),
                name: "Test Song 1".to_string(),
                artist: ArtistPreview {
                    id: "test_artist_1".to_string(),
                    name: "Test Artist 1".to_string()
                }
            }
        );

        assert_eq!(
            song_preview_vec[1],
            SongPreview {
                id: "test_song_2".to_string(),
                name: "Test Song 2".to_string(),
                artist: ArtistPreview {
                    id: "test_artist_2".to_string(),
                    name: "Test Artist 2".to_string()
                }
            }
        );
    }
}
*/

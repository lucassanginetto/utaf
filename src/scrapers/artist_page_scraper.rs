use std::sync::LazyLock;

use scraper::{Html, Selector};

use crate::{
    models::Artist,
    scrapers::{ScrapeError, scrape_id_from_og_url_meta, scrape_songs_table},
};

static ARTIST_NAME_H2_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("div.contentBox__title h2").unwrap());

static TABLE_SONG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("table.searchResult h3 a").unwrap());

static TABLE_ARTIST_A_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("table.searchResult td.searchResult__artist > p > a").unwrap()
});

pub fn scrape_artist(html: &Html) -> Result<Artist, ScrapeError<'_>> {
    let id = scrape_id_from_og_url_meta(html, 2)?;
    let name = {
        let selector = &ARTIST_NAME_H2_SELECTOR;
        html.select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .and_then(|h2| {
                h2.text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .map(|text| text.strip_suffix("の歌詞一覧").unwrap_or(text).to_string())
            })
    }?;
    let songs = scrape_songs_table(html, &TABLE_SONG_A_SELECTOR, &TABLE_ARTIST_A_SELECTOR)?;
    Ok(Artist { id, name, songs })
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_scraping() {
        scrape_artist(&Html::parse_document(
            r#"
                <html lang="ja">
                    <head>
                        <meta property="og:url" content="https://utaten.com/artist/lyric/test_artist">
                    </head>
                    <body>
                        <div class="contentBox__title">
                            <h2>Test Artistの歌詞一覧</h2>
                        </div>
                        <div class="contentBox__body">
                            <table class="searchResult artistLyricList">
                                <tbody>
                                    <tr>
                                    </tr>
                                    <tr>
                                        <td>
                                            <h3>
                                                <a href="/lyric/test_song_1">
                                                    Test Song 1
                                                </a>
                                            </h3>
                                        </td>
                                        <td class="searchResult__artist">
                                            <p>
                                                <a href="/artist/test_artist/">Test Artist</a>
                                            </p>
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>
                                            <h3>
                                                <a href="/lyric/test_song_1">
                                                    Test Song 2
                                                </a>
                                            </h3>
                                        </td>
                                        <td class="searchResult__artist">
                                            <p>
                                                <a href="/artist/test_artist/">Test Artist</a>
                                            </p>
                                        </td>
                                    </tr>
                                </tbody>
                            </table>
                        </div>
                    </body>
                </html>
            "#,
        )).unwrap();
    }
}
*/

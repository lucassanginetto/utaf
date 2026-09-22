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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artist_scraping() {
        let artist = scrape_artist(&Html::parse_document(
            r#""<html lang="ja">
            <head>
                ...
                <title>Artistの歌詞一覧｜歌詞検索サイト【UtaTen】</title>
                ...
                <meta property="og:url" content="https://utaten.com/artist/lyric/artist_id">
                ...
                <link rel="canonical" href="https://utaten.com/artist/lyric/artist_id">
                ...
            </head>
            <body>
                ...
                <div id="container">
                    <div id="contents">
                        <main>
                            ...
                            <section class="contentBox">
                                <div class="contentBox__title">
                                    <div class="contentBox__title__ruby">よみ：あーてぃすと</div>
                                    <h2>Artistの歌詞一覧</h2>
                                </div>
                                <div class="lyricsInfo">...</div>
                                <div class="contentBox__body">
                                    <div class="switchTab">...</div>
                                    <table class="searchResult artistLyricList"><tbody>
                                        <tr>
                                            <th class="searchResult__head">楽曲・タイトル</th>
                                            <th class="searchResult__artist">アーティスト</th>
                                            <th>歌詞・歌い出し</th>
                                        </tr>
                                        <tr>
                                            <td>
                                                <p class="searchResult__title"></p>
                                                <h3><a href="/lyric/song_1_id/">Song 1</a></h3>
                                                <p></p>
                                            </td>
                                            <td class="searchResult__artist">
                                                <p>
                                                    <a href="/artist/artist_id/">Artist</a>
                                                </p>
                                                <div class="searchResult__lyricist">...</div>
                                            </td>
                                            <td class="artistList__beginning">...</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <p class="searchResult__title"></p>
                                                <h3><a href="/lyric/song_2_id/">Song 2</a></h3>
                                                <p></p>
                                            </td>
                                            <td class="searchResult__artist">
                                                <p>
                                                    <a href="/artist/artist_id/">Artist</a>
                                                </p>
                                                <div class="searchResult__lyricist">...</div>
                                            </td>
                                            <td class="artistList__beginning">...</td>
                                        </tr>
                                    </tbody></table>
                                </div>
                                <div class="contentBox__backLink" style="margin-top: 0;">
                                    <a href="/artist/artist_id/">アーティストのページへ</a>
                                </div>
                            </section>
                        </main>
                    </div>
                    ...
                </div>
                ...
            </body>
            </html>""#,
        ))
        .unwrap();

        assert_eq!(artist.id, "artist_id");
        assert_eq!(artist.name, "Artist");
        assert_eq!(artist.songs.len(), 2);

        assert_eq!(artist.songs[0].id, "song_1_id");
        assert_eq!(artist.songs[1].id, "song_2_id");

        assert_eq!(artist.songs[0].title, "Song 1");
        assert_eq!(artist.songs[1].title, "Song 2");

        let artist_preview = artist.clone().into();
        assert_eq!(artist.songs[0].artist, artist_preview);
        assert_eq!(artist.songs[1].artist, artist_preview);
    }
}

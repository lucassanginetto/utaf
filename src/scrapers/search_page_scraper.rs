use std::sync::LazyLock;

use scraper::{Html, Selector};

use crate::{
    models::{SearchParams, SearchResults},
    scrapers::{ScrapeError, scrape_songs_table},
};

static TITLE_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"title\"]").unwrap());

static ARTIST_NAME_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"artist_name\"]").unwrap());

static SUB_TITLE_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"sub_title\"]").unwrap());

static LYRICIST_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"lyricist\"]").unwrap());

static COMPOSER_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"composer\"]").unwrap());

static BEGINNING_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"beginning\"]").unwrap());

static BODY_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"body\"]").unwrap());

static TAG_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"tag\"]").unwrap());

static SORT_INPUT_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("input[name=\"sort\"]").unwrap());

static CURRENT_PAGE_SPAN_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("li.pager__item--current span").unwrap());

fn scrape_search_params(html: &Html) -> Result<SearchParams, ScrapeError<'_>> {
    let scrape_form_input = |selector| {
        let attr = "value";
        html.select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .map(|input| input.attr(attr).unwrap_or("").to_string())
    };
    let scrape_sort_hidden_form_input = || {
        let attr = "value";
        let selector = &SORT_INPUT_SELECTOR;
        html.select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .map(|input| input.attr(attr).unwrap_or(""))
            .and_then(|value| {
                value.parse().map_err(|_| ScrapeError::InvalidAttribute {
                    selector,
                    attr,
                    value,
                })
            })
    };

    let title = scrape_form_input(&TITLE_INPUT_SELECTOR)?;
    let artist_name = scrape_form_input(&ARTIST_NAME_INPUT_SELECTOR)?;
    let sub_title = scrape_form_input(&SUB_TITLE_INPUT_SELECTOR)?;
    let lyricist = scrape_form_input(&LYRICIST_INPUT_SELECTOR)?;
    let composer = scrape_form_input(&COMPOSER_INPUT_SELECTOR)?;
    let beginning = scrape_form_input(&BEGINNING_INPUT_SELECTOR)?;
    let body = scrape_form_input(&BODY_INPUT_SELECTOR)?;
    let tag = scrape_form_input(&TAG_INPUT_SELECTOR)?;
    let sort = scrape_sort_hidden_form_input()?;
    let page = {
        let selector = &CURRENT_PAGE_SPAN_SELECTOR;
        html.select(selector).next().map_or_else(
            || Ok(1),
            |span| {
                span.text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .and_then(|text| {
                        text.parse()
                            .map_err(|_| ScrapeError::InvalidText { selector, text })
                    })
            },
        )
    }?;

    Ok(SearchParams {
        title,
        artist_name,
        sub_title,
        lyricist,
        composer,
        beginning,
        body,
        tag,
        sort,
        page,
    })
}

/*
static ARTIST_ID_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("div.bxslider div.searchSlide__item a").unwrap());

static ARTIST_NAME_P_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("div.bxslider div.searchSlide__item a p.searchSlide__name").unwrap()
});
*/

static TABLE_SONG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("p.searchResult__title > a").unwrap());

static TABLE_ARTIST_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("td.searchResult__artist > p > a").unwrap());

static LAST_PAGE_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("li.pager__item--last a").unwrap());

pub fn scrape_search_results(html: &Html) -> Result<SearchResults, ScrapeError<'_>> {
    let params = scrape_search_params(html)?;
    /*
    let artists = {
        html.select(&ARTIST_ID_A_SELECTOR)
            .zip(html.select(&ARTIST_NAME_P_SELECTOR))
            .map(|(a, p)| {
                let id = {
                    let selector = &ARTIST_ID_A_SELECTOR;
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
                        })
                        .map(|id_str| id_str.to_string())
                }?;
                let name = {
                    let selector = &ARTIST_NAME_P_SELECTOR;
                    p.text()
                        .next()
                        .ok_or(ScrapeError::MissingText { selector })
                        .map(|text| text.to_string())
                }?;

                Ok(ArtistPreview { id, name })
            })
            .collect::<Result<Vec<_>, ScrapeError>>()
    }?;
    */
    let songs = scrape_songs_table(html, &TABLE_SONG_A_SELECTOR, &TABLE_ARTIST_A_SELECTOR)?;
    let total_pages = {
        let selector = &LAST_PAGE_A_SELECTOR;
        html.select(selector).next().map_or(Ok(1), |a| {
            let attr = "href";
            a.attr(attr)
                .ok_or(ScrapeError::MissingAttribute { selector, attr })
                .and_then(|value| {
                    value
                        .split('/')
                        .filter_map(|param_str| match param_str.split_once('=') {
                            Some((key, value)) if key == "page" => value.parse().ok(),
                            _ => None,
                        })
                        .next()
                        .ok_or(ScrapeError::InvalidAttribute {
                            selector,
                            attr,
                            value,
                        })
                })
        })
    }?;

    Ok(SearchResults {
        params,
        songs,
        total_pages,
    })
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_results_scraping() {
        let search_results = SearchResults::scrape_search_page(&Html::parse_document(
            r##"
                <main>
                    <div class="contentBox__body">
                        <form action="/lyric/search" id="search">
                            <input type="hidden" name="sort" value="popular_sort_asc">
                            <table>
                            <tbody><tr>
                                <th><h3>歌手名</h3></th>
                                <td><input type="text" name="artist_name" placeholder="歌手名" value=""></td>
                                <th><h3>曲名</h3></th>
                                <td><input type="text" name="title" placeholder="曲名" value=""></td>
                            </tr>
                            <tr>
                                <th><h3>歌い出し</h3></th>
                                <td><input type="text" name="beginning" placeholder="歌い出し" value=""></td>
                                <th><h3>歌詞の一部</h3></th>
                                <td><input type="text" name="body" placeholder="歌詞の一部" value=""></td>
                            </tr>
                            <tr>
                                <th><h3>作詞家</h3></th>
                                <td><input type="text" name="lyricist" placeholder="作詞家" value=""></td>
                                <th><h3>作曲家</h3></th>
                                <td><input type="text" name="composer" placeholder="作曲家" value=""></td>
                            </tr>
                            <tr>
                                <th><h3>タイアップ</h3></th>
                                <td><input type="text" name="sub_title" placeholder="タイアップ" value=""></td>
                                <th><h3>ハッシュタグ</h3></th>
                                <td>
                                <div class="custom-autocomplete">
                                    <input type="text" name="tag" id="hashtag-input" placeholder="ハッシュタグ" value="" autocomplete="off">
                                    <div class="hashtag-dropdown-menu" id="hashtag-dropdown" style="display: none;">
                                    <div class="hashtag-dropdown-section">
                                        <div class="hashtag-section-label" id="popular-hashtag-text">人気のハッシュタグ</div>
                                        <ul class="hashtag-list" id="hashtag-list">
                                        </ul>
                                        <div class="hashtag-dropdown-footer" id="hashtag-index">
                                        <a href="/tag" class="all-tags-link">ハッシュタグ一覧を見る</a>
                                        </div>
                                    </div>
                                    </div>
                                </div>
                                </td>
                            </tr>
                            </tbody></table>
                            <p class="formButton">
                            <input type="hidden" name="show_artists" value="1">
                            <button type="submit">検索</button>
                            </p>
                        </form>
                    </div>
                    <div class="contentBox__body">
                        <div class="searchSlide">
                            <div class="bxslider">
                                <div class="searchSlide__item">
                                    <a href="/artist/test_artist_1">
                                        <p class="searchSlide__name">Test Artist 1</p>
                                    </a>
                                </div>
                                <div class="searchSlide__item">
                                    <a href="/artist/test_artist_2">
                                        <p class="searchSlide__name">Test Artist 2</p>
                                    </a>
                                </div>
                            </div>
                        </div>
                        <nav>
                            <ul class="pager__inner">
                                <li class="pager__item pager__item--first"></li>
                                <li class="pager__item pager__item--current"><span>1</span></li>
                                <li class="pager__item pager__item--last"></li>
                            </ul>
                        </nav>
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
                    </div>
                </main>
            "##,
        ))
        .unwrap();

        assert_eq!(search_results.songs.len(), 2);
        assert_eq!(search_results.artists.len(), 2);
    }
}
*/

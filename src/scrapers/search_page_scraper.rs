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
                            Some(("page", value)) => value.parse().ok(),
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

#[cfg(test)]
mod tests {
    use crate::models::SortBy;

    use super::*;

    #[test]
    fn test_search_results_scraping() {
        let search_results = scrape_search_results(&Html::parse_document(
            r##"<main>
                <ol class="path" itemscope="" itemtype="http://schema.org/BreadcrumbList">...</ol>
                <section class="contentBox">
                    <h1 class="contentBox__title">歌詞検索 曲詳細</h1>
                    <div class="contentBox__body">
                        <div class="box lyricSearchForm">
                            <form action="/lyric/search" id="search">
                            <input type="hidden" name="sort" value="popular_sort_asc">
                            <table>
                                <tbody>
                                    <tr>
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
                                                    <div class="hashtag-dropdown-menu" id="hashtag-dropdown">...</div>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                                <p class="formButton">
                                    <input type="hidden" name="show_artists" value="1">
                                    <button type="submit">検索</button>
                                </p>
                            </form>
                        </div>
                        <script>...</script>
                    </div>
                </section>
                <section class="contentBox">
                    <h2 class="contentBox__title">歌詞検索結果</h2>
                    <div class="contentBox__body">
                        <nav class="pager">
                            <ul class="pager__inner">
                                <li class="pager__item pager__item--first">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=1/">First</a>
                                </li>

                                <li class="pager__item pager__item--current"><span>1</span></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=2/">2</a></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=3/">3</a></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=4/">4</a></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=5/">5</a></li>

                                <li class="pager__item pager__item--last">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=100/">Last</a>
                                </li>
                            </ul>
                        </nav>
                        <div class="switchTab">
                            <ul class="switchTab__inner">
                                <li class="switchTab__item switchTab__item--current">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/">
                                        人気順          </a>
                                </li>
                                <li class="switchTab__item">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=release_date_desc/">
                                        発売新順          </a>
                                </li>
                                <li class="switchTab__item">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=release_date_asc/">
                                        発売古順          </a>
                                </li>
                                <li class="switchTab__item">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=title_kana_asc/">
                                        あ ⇒ わ          </a>
                                </li>
                                <li class="switchTab__item">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=title_kana_desc/">
                                        わ ⇒ あ          </a>
                                </li>
                            </ul>
                        </div>
                        <table class="searchResult artistLyricList"><tbody>
                            <tr>
                                <th class="searchResult__head">楽曲・タイトル</th>
                                <th class="searchResult__artist">アーティスト</th>
                                <th>歌詞・歌い出し</th>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song1/">          Song 1        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist1/">        Artist 1        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song2/">          Song 2        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist2/">        Artist 2        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song3/">          Song 3        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist3/">        Artist 3        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song4/">          Song 4        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist4/">        Artist 4        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song5/">          Song 5    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist5/">        Artist 5        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song6/">          Song 6    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist6/">        Artist 6        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song7/">          Song 7    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist7/">        Artist 7        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song8/">          Song 8    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist8/">        Artist 8        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song9/">          Song 9    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist9/">        Artist 9        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song10/">          Song 10    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist10/">        Artist 10        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song11/">          Song 11        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist11/">        Artist 11        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song12/">          Song 12        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist12/">        Artist 12        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song13/">          Song 13        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist13/">        Artist 13        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song14/">          Song 14        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist14/">        Artist 14        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song15/">          Song 15    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist15/">        Artist 15        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song16/">          Song 16    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist16/">        Artist 16        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song17/">          Song 17    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist17/">        Artist 17        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song18/">          Song 18    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist18/">        Artist 18        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song19/">          Song 19    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist19/">        Artist 19        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song20/">          Song 20    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist20/">        Artist 20        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song21/">          Song 21        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist21/">        Artist 21        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song22/">          Song 22        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist22/">        Artist 22        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song23/">          Song 23        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist23/">        Artist 23        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song24/">          Song 24        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist24/">        Artist 24        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song25/">          Song 25    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist25/">        Artist 25        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song26/">          Song 26    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist26/">        Artist 26        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song27/">          Song 27    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist27/">        Artist 27        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song28/">          Song 28    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist28/">        Artist 28        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song29/">          Song 29    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist29/">        Artist 29        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song30/">          Song 30    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist30/">        Artist 30        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song31/">          Song 31        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist31/">        Artist 31        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song32/">          Song 32        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist32/">        Artist 32        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song33/">          Song 33        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist33/">        Artist 33        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song34/">          Song 34        </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist34/">        Artist 34        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song35/">          Song 35    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist35/">        Artist 35        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song36/">          Song 36    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist36/">        Artist 36        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song37/">          Song 37    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist37/">        Artist 37        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song38/">          Song 38    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist38/">        Artist 38        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song39/">          Song 39    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist39/">        Artist 39        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                            <tr>
                                <td>
                                    <p class="searchResult__title">
                                        <a href="/lyric/song40/">          Song 40    </a>
                                        <span class="popularIcon">人気</span>
                                    </p>
                                </td>
                                <td class="searchResult__artist">
                                    <p>
                                        <a href="/artist/artist40/">        Artist 40        </a>
                                    </p>
                                    <div class="searchResult__lyricist">...</div>
                                </td>
                                <td class="lyricList__beginning">...</td>
                            </tr>
                        </tbody></table>
                        <nav class="pager">
                                <ul class="pager__inner">
                                <li class="pager__item pager__item--first">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=1/">First</a>
                                </li>

                                <li class="pager__item pager__item--current"><span>1</span></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=2/">2</a></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=3/">3</a></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=4/">4</a></li>
                                <li class="pager__item"><a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=5/">5</a></li>

                                <li class="pager__item pager__item--last">
                                    <a href="/search/=/title=/artist_name=/sub_title=/lyricist=/composer=/beginning=/body=/tag=/sort=popular_sort_asc/page=100/">Last</a>
                                </li>
                            </ul>
                        </nav>
                    </div>
                </section>
                <script type="text/javascript" src="/js/user/lyric/search.js"></script>
            </main>"##,
        ))
        .unwrap();

        assert_eq!(search_results.params.title, "");
        assert_eq!(search_results.params.artist_name, "");
        assert_eq!(search_results.params.sub_title, "");
        assert_eq!(search_results.params.lyricist, "");
        assert_eq!(search_results.params.composer, "");
        assert_eq!(search_results.params.beginning, "");
        assert_eq!(search_results.params.body, "");
        assert_eq!(search_results.params.tag, "");
        assert_eq!(search_results.params.sort, SortBy::Popularity);
        assert_eq!(search_results.params.page, 1);

        assert_eq!(search_results.total_pages, 100);
        assert_eq!(search_results.songs.len(), 40);
    }
}

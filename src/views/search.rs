use templr::{templ, templ_ret};

use crate::models::SearchResults;

use super::layout;

fn search_pagination_nav(results: &SearchResults) -> templ_ret!['_] {
    templ! {
        <nav id="search_pagination_nav">
            #for page in (1..=results.total_pages).filter(|&x| {
                x == 1
                || x == results.params.page.saturating_sub(2)
                || x == results.params.page-1
                || x == results.params.page
                || x == results.params.page+1
                || x == results.params.page+2
                || x == results.total_pages
            }) {
                #if page == results.params.page {
                    <span class="search_page_button current">{ page }</span>
                } else {
                    <a class="search_page_button" href={
                        format!(
                            "/search?artist_name={}&title={}&beginning={}&body={}&lyricist={}&composer={}&sub_title={}&tag={}&page={page}",
                            &results.params.artist_name,
                            &results.params.title,
                            &results.params.beginning,
                            &results.params.body,
                            &results.params.lyricist,
                            &results.params.composer,
                            &results.params.sub_title,
                            &results.params.tag
                        )
                    }>{ page }</a>
                }
            }
        </nav>
    }
}

pub fn search_page(results: &SearchResults) -> templ_ret!['_] {
    templ! {
        #layout("Search - UTAF", true) {
            <main id="search_main">
                <h1>Advanced search</h1>
                <form id="search_form" method="GET">
                    <div id="search_form_input_grid_div">
                        <input type="text" name="artist_name" class="search_input" placeholder="Artist name" value={results.params.artist_name}/>
                        <input type="text" name="title" class="search_input" placeholder="Song title" value={results.params.title}/>
                        <input type="text" name="beginning" class="search_input" placeholder="Lyrics begin with" value={results.params.beginning}/>
                        <input type="text" name="body" class="search_input" placeholder="Lyrics contain" value={results.params.body}/>
                        <input type="text" name="lyricist" class="search_input" placeholder="Lyricist name" value={results.params.lyricist}/>
                        <input type="text" name="composer" class="search_input" placeholder="Composer name" value={results.params.composer}/>
                        <input type="text" name="sub_title" class="search_input" placeholder="Song subtitle" value={results.params.sub_title}/>
                        <input type="text" name="tag" class="search_input" placeholder="Hashtag" value={results.params.tag}/>
                    </div>
                    <button id="search_button" type="submit">Search</button>
                </form>
                <h2>Songs</h2>
                #if results.total_pages > 1 { #search_pagination_nav(results); }
                #if !results.songs.is_empty() {
                    <ul id="search_results_ul">
                        #for song in results.songs.iter() {
                            <li class="search_results_li">
                                <a href={ format!("/lyric/{}/", song.id) }>
                                    { song.title }
                                </a>
                                <p>by { song.artist.name }</p>
                            </li>
                        }
                    </ul>
                } else {
                    <p>No songs found</p>
                }
                #if results.total_pages > 1 { #search_pagination_nav(results); }
            </main>
        }
    }
}

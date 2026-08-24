use templr::{templ, templ_ret};

use crate::models::Artist;

use super::layout;

pub fn artist_page(artist: &Artist) -> templ_ret!['_] {
    templ! {
        #layout(&artist.name, false) {
            <main id="artist_main">
                <h1 id="artist_name">{ artist.name }</h1>
                <h2>Songs</h2>
                <ul>
                    #for song in artist.songs.iter() {
                        <li>
                            <a href={ format!("/lyric/{}/", song.id) }>
                                { song.title }
                            </a>
                        </li>
                    }
                </ul>
            </main>
        }
    }
}

use templr::{templ, templ_ret};

use crate::models::SongWriter;

use super::layout;

pub fn song_writer_page(song_writer: &SongWriter) -> templ_ret!['_] {
    templ! {
        #layout(&song_writer.name, false) {
            <main id="song_writer_main">
                <h1 id="song_writer_name">{ song_writer.name }</h1>
                <h2>Songs</h2>
                <ul>
                    #for song in song_writer.songs.iter() {
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

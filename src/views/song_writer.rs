use templr::{templ, templ_ret};

use crate::models::SongWriter;

use super::layout;

pub fn song_writer_page(song_writer: &SongWriter) -> templ_ret!['_] {
    templ! {
        #layout(&song_writer.name, false) {
            <main id="song_writer_main">
                <h1 id="song_writer_name">{ song_writer.name }</h1>
                <h2>Songs</h2>
                #if song_writer.songs_as_lyricist.len() > 0 {
                    <h3>As Lyricist</h3>
                    <ul>
                        #for song in song_writer.songs_as_lyricist.iter() {
                            <li>
                                <a href={ format!("/lyric/{}/", song.id) }>
                                    { song.title } by { song.artist.name }
                                </a>
                            </li>
                        }
                    </ul>
                }
                #if song_writer.songs_as_composer.len() > 0 {
                    <h3>As Composer</h3>
                    <ul>
                        #for song in song_writer.songs_as_composer.iter() {
                            <li>
                                <a href={ format!("/lyric/{}/", song.id) }>
                                    { song.title } by { song.artist.name }
                                </a>
                            </li>
                        }
                    </ul>
                }
                #if song_writer.songs_as_arranger.len() > 0 {
                    <h3>As Arranger</h3>
                    <ul>
                        #for song in song_writer.songs_as_arranger.iter() {
                            <li>
                                <a href={ format!("/lyric/{}/", song.id) }>
                                    { song.title } by { song.artist.name }
                                </a>
                            </li>
                        }
                    </ul>
                }
            </main>
        }
    }
}

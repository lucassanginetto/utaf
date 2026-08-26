use templr::{Trust, templ, templ_ret};

use crate::models::Song;

use super::layout;

pub fn lyrics_page(song: &Song) -> templ_ret!['_] {
    templ! {
        #layout(&format!("{} - {} lyrics", song.artist.name, song.title), false) {
            <main id="lyrics_main">
                <div id="song_metadata_div">
                    <img id="song_image" src={song.image} alt="Song image" width="100" height="100"/>
                    <div>
                        <h1>{ song.title }</h1>
                        <p>by <a href={ format!("/artist/lyric/{}", song.artist.id) }>{ song.artist.name }</a></p>
                        <p>released on { song.release_date }</p>

                        #if song.tags.len() > 0 {
                            <h2>Tags</h2>
                            <ul>
                                #for tag in song.tags.iter() {
                                    <li><a href={format!("/search?tag={tag}")}>{format!("#{tag}")}</a></li>
                                }
                            </ul>
                        }

                        #if song.lyricists.len() > 0 || song.composers.len() > 0 || song.arrangers.len() > 0 {
                            <h2>Song writers</h2>
                            #if song.lyricists.len() > 0 {
                                <h3>{if song.lyricists.len() > 1 {"Lyricists"} else {"Lyricist"}}</h3>
                                #for lyricist in song.lyricists.iter() {
                                    <a href={format!("/songWriter/{}/", lyricist.id)}>{lyricist.name}</a>
                                }
                            }
                            #if song.composers.len() > 0 {
                                <h3>{if song.composers.len() > 1 {"Composers"} else {"Composer"}}</h3>
                                #for composer in song.composers.iter() {
                                    <a href={format!("/songWriter/{}/", composer.id)}>{composer.name}</a>
                                }
                            }
                            #if song.arrangers.len() > 0 {
                                <h3>{if song.arrangers.len() > 1 {"Arrangers"} else {"Arranger"}}</h3>
                                #for arranger in song.arrangers.iter() {
                                    <a href={format!("/songWriter/{}/", arranger.id)}>{arranger.name}</a>
                                }
                            }
                        }
                    </div>
                </div>
                <div id="lyrics_div">
                    { Trust(song.lyrics.clone()) }
                </div>
            </main>
        }
    }
}

use std::{convert::identity, sync::LazyLock};

use scraper::{Element, ElementRef, Html, Node, Selector};
use url::{ParseError, Url};

use crate::{
    models::{ArtistPreview, Song, SongWriterPreview},
    scrapers::{OG_URL_META_SELECTOR, ScrapeError},
};

static TITLE_H2_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("h2.newLyricTitle__main").unwrap());

static IMAGE_IMG_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("div.lyricData__sub img").unwrap());

static ARTIST_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("dt.newLyricWork__name a").unwrap());

static SONG_WRITER_A_SELECTOR: LazyLock<Selector> = LazyLock::new(|| {
    Selector::parse("dt.newLyricWork__title + dd.newLyricWork__body > a").unwrap()
});

static TAG_A_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("div.newLyricTag a").unwrap());

static LYRICS_DIV_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("div.hiragana").unwrap());

static RUBY_ANNOTATED_SPAN_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("span.rb").unwrap());

static RUBY_ANNOTATION_SPAN_SELECTOR: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse("span.rt").unwrap());

pub fn scrape_song(html: &Html) -> Result<Song, ScrapeError<'_>> {
    let id = {
        let selector = &OG_URL_META_SELECTOR;
        let attr = "content";
        let value = html
            .select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .and_then(|meta| {
                meta.attr(attr)
                    .ok_or(ScrapeError::MissingAttribute { selector, attr })
            })?;
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
                    .and_then(|mut segments| segments.nth(1).ok_or(invalid_attribute_error))
                    .map(|id_str| id_str.to_string())
            })
    }?;

    let title = {
        let selector = &TITLE_H2_SELECTOR;
        html.select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .and_then(|h2| {
                h2.text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .map(|text| text.trim().to_string())
            })
    }?;

    let image = {
        let selector = &IMAGE_IMG_SELECTOR;
        let attr = "src";
        html.select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })
            .and_then(|img| {
                img.attr(attr)
                    .ok_or(ScrapeError::MissingAttribute { selector, attr })
            })
            .and_then(|value| {
                let invalid_attribute_error = ScrapeError::InvalidAttribute {
                    selector,
                    attr,
                    value,
                };
                Url::parse(value)
                    .map_err(|parse_error| match parse_error {
                        ParseError::RelativeUrlWithoutBase => {
                            Url::parse(&format!("https://utaten.com{value}"))
                                .map_err(|_| invalid_attribute_error)
                        }
                        _ => Err(invalid_attribute_error),
                    })
                    .or_else(identity) // flatten result
            })
    }?;

    let artist = {
        let selector = &ARTIST_A_SELECTOR;
        let artist_a = html
            .select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })?;
        let id = {
            let attr = "href";
            artist_a
                .attr(attr)
                .ok_or(ScrapeError::MissingAttribute { selector, attr })
                .and_then(|value| {
                    value
                        .split('/')
                        .last()
                        .ok_or(ScrapeError::InvalidAttribute {
                            selector,
                            attr,
                            value,
                        })
                        .map(|str| str.to_string())
                })
        }?;
        let name = artist_a
            .text()
            .next()
            .ok_or(ScrapeError::MissingText { selector })
            .map(|text| text.trim().to_string())?;

        ArtistPreview { id, name }
    };

    let lyricists = {
        let selector = &SONG_WRITER_A_SELECTOR;
        html.select(selector)
            .filter(|a| {
                a.parent_element()
                    .expect("<a> should have a parent <dd>")
                    .prev_sibling_element()
                    .expect("<dd> should have a previous sibling <dt>")
                    .text()
                    .next()
                    .map(|text| text == "作詞")
                    .unwrap_or(false)
            })
            .map(|a| {
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
                                .map(|id| id.to_string())
                        })
                }?;
                let name = a
                    .text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .map(|text| text.to_string())?;
                Ok(SongWriterPreview { id, name })
            })
            .collect::<Result<Vec<_>, ScrapeError>>()
    }?;

    let composers = {
        let selector = &SONG_WRITER_A_SELECTOR;
        html.select(selector)
            .filter(|a| {
                a.parent_element()
                    .expect("<a> should have a parent <dd>")
                    .prev_sibling_element()
                    .expect("<dd> should have a previous sibling <dt>")
                    .text()
                    .next()
                    .map(|text| text == "作曲")
                    .unwrap_or(false)
            })
            .map(|a| {
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
                                .map(|id| id.to_string())
                        })
                }?;
                let name = a
                    .text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .map(|text| text.to_string())?;
                Ok(SongWriterPreview { id, name })
            })
            .collect::<Result<Vec<_>, ScrapeError>>()
    }?;

    let arrangers = {
        let selector = &SONG_WRITER_A_SELECTOR;
        html.select(selector)
            .filter(|a| {
                a.parent_element()
                    .expect("<a> should have a parent <dd>")
                    .prev_sibling_element()
                    .expect("<dd> should have a previous sibling <dt>")
                    .text()
                    .next()
                    .map(|text| text == "編曲")
                    .unwrap_or(false)
            })
            .map(|a| {
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
                                .map(|id| id.to_string())
                        })
                }?;
                let name = a
                    .text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })
                    .map(|text| text.to_string())?;
                Ok(SongWriterPreview { id, name })
            })
            .collect::<Result<Vec<_>, ScrapeError>>()
    }?;

    let tags = {
        let selector = &TAG_A_SELECTOR;
        html.select(selector)
            .map(|a| {
                let text = a
                    .text()
                    .next()
                    .ok_or(ScrapeError::MissingText { selector })?;
                Ok(text.trim()[1..].to_string())
            })
            .collect::<Result<Vec<_>, ScrapeError>>()
    }?;

    let lyrics = {
        let selector = &LYRICS_DIV_SELECTOR;
        let mut lyrics_buf = String::new();
        for child in html
            .select(selector)
            .next()
            .ok_or(ScrapeError::MissingElement { selector })?
            .children()
        {
            match child.value() {
                Node::Text(text) => {
                    lyrics_buf.push_str(text);
                }
                Node::Element(element) if element.name() == "span" => {
                    let element_ref =
                        ElementRef::wrap(child).expect("child should be element to get here");

                    lyrics_buf.push_str("<ruby>");
                    lyrics_buf.push_str({
                        let selector = &RUBY_ANNOTATED_SPAN_SELECTOR;
                        element_ref
                            .select(selector)
                            .next()
                            .ok_or(ScrapeError::MissingElement { selector })
                            .and_then(|ruby| {
                                ruby.text()
                                    .next()
                                    .ok_or(ScrapeError::MissingText { selector })
                            })?
                    });
                    lyrics_buf.push_str("<rp>(</rp><rt>");
                    lyrics_buf.push_str({
                        let selector = &RUBY_ANNOTATION_SPAN_SELECTOR;
                        element_ref
                            .select(selector)
                            .next()
                            .ok_or(ScrapeError::MissingElement { selector })
                            .and_then(|rt| {
                                rt.text()
                                    .next()
                                    .ok_or(ScrapeError::MissingText { selector })
                            })?
                    });
                    lyrics_buf.push_str("</rt><rp>)</rp></ruby>");
                }
                Node::Element(element) if element.name() == "br" => lyrics_buf.push_str("<br/>"),
                _ => {}
            }
        }
        lyrics_buf.trim().to_string()
    };

    Ok(Song {
        id,
        title,
        image,
        artist,
        lyricists,
        composers,
        arrangers,
        tags,
        lyrics,
    })
}

use axum::{
    extract::{Path, Query},
    http::StatusCode,
};
use scraper::Html;
use templr::{Response as TemplrResponse, TemplateExt};
use url::Url;

use crate::{
    models::RawSearchParams,
    scrapers::{self, ScrapeError},
    views,
};

/* Type aliases */

type TemplrErrorResponse = (StatusCode, TemplrResponse);
type TemplrResponseResult = Result<TemplrResponse, TemplrErrorResponse>;

/* Handlers */

pub async fn robots_txt() -> String {
    "User-agent: *\nDisallow: /\n".to_string()
}

/** Page handlers **/

pub async fn home() -> TemplrResponse {
    views::home_page().response(&())
}

pub async fn artist(Path(artist_id): Path<String>) -> TemplrResponseResult {
    scrape_data(
        format!("http://utaten.com/artist/lyric/{artist_id}"),
        scrapers::scrape_artist,
    )
    .await
    .map(|artist| views::artist_page(&artist).response(&()))
}

pub async fn lyrics(Path(song_id): Path<String>) -> TemplrResponseResult {
    scrape_data(
        format!("http://utaten.com/lyric/{song_id}/"),
        scrapers::scrape_song,
    )
    .await
    .map(|song| views::lyrics_page(&song).response(&()))
}

pub async fn song_writer(Path(song_writer_id): Path<String>) -> TemplrResponseResult {
    scrape_data(
        format!("http://utaten.com/songWriter/{song_writer_id}/"),
        scrapers::scrape_song_writer,
    )
    .await
    .map(|song_writer| views::song_writer_page(&song_writer).response(&()))
}

pub async fn search(Query(params): Query<RawSearchParams>) -> TemplrResponseResult {
    let utaten_url = {
        let mut url =
            Url::parse("https://utaten.com/search").expect("hardcoded URL should be valid");

        if let Some(text) = params.layout_search_text {
            url.query_pairs_mut()
                .append_pair("layout_search_text", &text);
        }
        if let Some(r#type) = params.layout_search_type {
            url.query_pairs_mut()
                .append_pair("layout_search_type", &r#type.to_string());
        }
        if let Some(title) = params.title {
            url.query_pairs_mut().append_pair("title", &title);
        }
        if let Some(name) = params.artist_name {
            url.query_pairs_mut().append_pair("artist_name", &name);
        }
        if let Some(sub_title) = params.sub_title {
            url.query_pairs_mut().append_pair("sub_title", &sub_title);
        }
        if let Some(lyricist) = params.lyricist {
            url.query_pairs_mut().append_pair("lyricist", &lyricist);
        }
        if let Some(composer) = params.composer {
            url.query_pairs_mut().append_pair("composer", &composer);
        }
        if let Some(beginning) = params.beginning {
            url.query_pairs_mut().append_pair("beginning", &beginning);
        }
        if let Some(body) = params.body {
            url.query_pairs_mut().append_pair("body", &body);
        }
        if let Some(tag) = params.tag {
            url.query_pairs_mut().append_pair("tag", &tag);
        }
        if let Some(sort) = params.sort {
            url.query_pairs_mut().append_pair("sort", &sort.to_string());
        }
        if let Some(page) = params.page {
            url.query_pairs_mut().append_pair("page", &page.to_string());
        }

        url
    };

    scrape_data(utaten_url.to_string(), scrapers::scrape_search_results)
        .await
        .map(|search_results| views::search_page(&search_results).response(&()))
}

pub async fn not_found(Path(_): Path<String>) -> TemplrErrorResponse {
    let status_code = StatusCode::NOT_FOUND;
    (
        status_code,
        views::error_page(status_code, "Page not found").response(&()),
    )
}

/** Subhandlers **/

async fn scrape_data<T, S>(url: String, scraper_fn: S) -> Result<T, TemplrErrorResponse>
where
    S: Fn(&Html) -> Result<T, ScrapeError>,
{
    reqwest::get(url)
        .await
        .map_err(request_error)
        .and_then(|response| response.error_for_status().map_err(upstream_error))
        .map(|response| response.text())?
        .await
        .map_err(reading_error)
        .and_then(|text| scraper_fn(&Html::parse_document(&text)).map_err(scraping_error))
}

fn request_error(error: reqwest::Error) -> TemplrErrorResponse {
    tracing::error!("{error}");

    let status_code = StatusCode::INTERNAL_SERVER_ERROR;
    (
        status_code,
        views::error_page(status_code, "Failed to reach UtaTen servers").response(&()),
    )
}

fn upstream_error(error: reqwest::Error) -> TemplrErrorResponse {
    tracing::error!("{error}");

    let status_code = error
        .status()
        .expect("the error should should come from Response::error_for_status");

    (
        status_code,
        views::error_page(
            status_code,
            match status_code {
                StatusCode::NOT_FOUND => "Page not found",
                _ => "Received error response from UtaTen",
            },
        )
        .response(&()),
    )
}

fn reading_error(error: reqwest::Error) -> TemplrErrorResponse {
    tracing::error!("{error}");

    let status_code = StatusCode::INTERNAL_SERVER_ERROR;
    (
        status_code,
        views::error_page(status_code, "Unable to read response from UtaTen").response(&()),
    )
}

fn scraping_error(error: ScrapeError) -> TemplrErrorResponse {
    tracing::error!("{error}");

    let status_code = StatusCode::BAD_GATEWAY;
    (
        status_code,
        views::error_page(status_code, "Unable to scrape response from UtaTen").response(&()),
    )
}

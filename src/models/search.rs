use std::{fmt::Display, str::FromStr};

use serde::Deserialize;

use crate::models::SongPreview;

#[derive(Debug, Deserialize)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Debug, Deserialize)]
pub enum SortBy {
    Popularity,
    ReleaseDate(Order),
    TitleKana(Order),
}
impl Default for SortBy {
    fn default() -> Self {
        Self::Popularity
    }
}
impl Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::Popularity => write!(f, "popular_sort_asc"),
            SortBy::ReleaseDate(Order::Descending) => write!(f, "release_date_desc"),
            SortBy::ReleaseDate(Order::Ascending) => write!(f, "release_date_asc"),
            SortBy::TitleKana(Order::Ascending) => write!(f, "title_kana_asc"),
            SortBy::TitleKana(Order::Descending) => write!(f, "title_kana_desc"),
        }
    }
}
impl FromStr for SortBy {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "popular_sort_asc" => Ok(SortBy::Popularity),
            "release_date_desc" => Ok(SortBy::ReleaseDate(Order::Descending)),
            "release_date_asc" => Ok(SortBy::ReleaseDate(Order::Ascending)),
            "title_kana_asc" => Ok(SortBy::TitleKana(Order::Ascending)),
            "title_kana_desc" => Ok(SortBy::TitleKana(Order::Descending)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LayoutSearchType {
    Artist,
    Title,
}
impl Display for LayoutSearchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LayoutSearchType::Artist => write!(f, "artist"),
            LayoutSearchType::Title => write!(f, "title"),
        }
    }
}
impl FromStr for LayoutSearchType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(LayoutSearchType::Artist),
            "title" => Ok(LayoutSearchType::Title),
            _ => Err(()),
        }
    }
}

pub struct SearchParams {
    pub title: String,
    pub artist_name: String,
    pub sub_title: String,
    pub lyricist: String,
    pub composer: String,
    pub beginning: String,
    pub body: String,
    pub tag: String,
    pub sort: SortBy,
    pub page: u8,
}

#[derive(Debug, Deserialize)]
pub struct RawSearchParams {
    pub layout_search_text: Option<String>,
    pub layout_search_type: Option<LayoutSearchType>,

    pub title: Option<String>,
    pub artist_name: Option<String>,
    pub sub_title: Option<String>,
    pub lyricist: Option<String>,
    pub composer: Option<String>,
    pub beginning: Option<String>,
    pub body: Option<String>,
    pub tag: Option<String>,
    pub sort: Option<SortBy>,
    pub page: Option<u32>,
}

pub struct SearchResults {
    pub params: SearchParams,
    pub songs: Vec<SongPreview>,
    pub total_pages: u8,
}

use crate::core::meta::{CountryCode, Language, MediaLanguage, MediaRating};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use url::Url;

pub(crate) enum MediaType {
    TVShow,
    TCEpisode,
    Movie,
    MovieSet,
    Subtitle,
}

pub(crate) struct SearchAndScrapeOption {
    media_type: MediaType,
    language: MediaLanguage,
    cert_country: CountryCode,
    ids: HashMap<String, String>,
    release_date_country: String,
    search_query: String,
    search_year: i32,
}

pub(crate) enum MediaArtworkType {
    Background,
    Banner,
    Poster,
    Actor,
    SeasonPoster,
    SeasonFanArt,
    SeasonBanner,
    SeasonThumb,
    Thumb,
    ClearArt,
    KeyArt,
    CharacterArt,
    Disc,
    Logo,
    ClearLogo,
    All,
}

pub(crate) struct ImageInfo {
    width: i32,
    height: i32,
    url: Url,
}

pub(crate) struct MediaArtwork {
    provider_id: String,
    art_type: MediaArtworkType,
    imdb_id: String,
    tmdb_id: i64,
    season: i8,
    preview_url: Url,
    default_url: Url,
    original_url: Url,
    language: String,
}

#[derive(Debug)]
pub(crate) struct MediaMetadata {
    provider_id: String,
    title: String,
    original_title: String,
    original_language: String,
    year: i32,
    plot: String,
    tag_line: String,
    top250: i16,
    collection_name: String,
    release_date: DateTime<Utc>,
    runtime: i32,
    episode_number: i8,
    season_number: i8,
    dvd_episode_number: i8,
    dvd_season_number: i8,
    display_episode_number: i8,
    display_season_number: i8,
    absolute_number: i8,

    ratings: Vec<MediaRating>,
}

// pub const KeyIMDB: &'static str = "imdb";



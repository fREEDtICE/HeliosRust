use crate::core::meta::MediaRating;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use url::Url;

pub trait PropertyHolder {}

#[derive(Debug)]
pub(crate) struct MediaMetadata {
    provider_id: String,
    title: String,
    original_title: String,
    original_language: String,
    overview: String,
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

pub(crate) struct MediaTrailer {
    id: String,
    name: String,
    url: Url,
    quality: String,
    provider: String,
    in_NFO: bool,
    date: DateTime<Utc>,
}

impl MediaTrailer {
    pub fn get_url(&self) -> &Url {
        &self.url
    }

    pub fn get_quality(&self) -> &str {
        &self.quality
    }

    pub fn is_in_NFO(&self) -> bool {
        self.in_NFO
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

use crate::core::meta::MediaRating;
use crate::scraper::entities::artwork::{ImageSize, MediaArtwork, MediaArtworkType};
use crate::scraper::entities::country_lang::{CountryCode, MediaLanguage};
use crate::scraper::entities::provider_info::MediaProviderInfo;

use std::collections::HashMap;

pub(crate) enum MediaType {
    TVShow,
    TVEpisode,
    Movie,
    MovieSet,
    Subtitle,
}

pub(crate) enum ScraperType {
    Movie,
    TVShow,
    MovieSet,
    MovieArtwork,
    TVShowArtwork,
    MovieTrailer,
    TvShowTrailer,
    MovieSubtitle,
    TVShowSubtitle,
    Album,
    Artist,
    MusicVideo,
    Library,
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

pub(crate) struct ArtworkSearchAndScrapeOption<'a> {
    search_info: SearchAndScrapeOption,
    artwork_type: MediaArtworkType,
    artwork_size: ImageSize<'a>,
}

impl MediaType {
    pub fn to_media_type(id: String) -> &'static MediaType {
        match id.as_str() {
            "movie" | "movies" => &MediaType::Movie,
            "movieSet" | "set" => &MediaType::MovieSet,
            "tv" | "tvShow" => &MediaType::TVShow,
            "episode" | "tvEpisode" => &MediaType::TVEpisode,
            _ => &MediaType::Subtitle,
        }
    }

    pub fn get_scraper_type_for_media_type(media: &MediaType) -> &'static ScraperType {
        match media {
            MediaType::Movie => &ScraperType::Movie,
            MediaType::MovieSet => &ScraperType::MovieSet,
            MediaType::TVEpisode | MediaType::TVShow => &ScraperType::TVShow,
            _ => &ScraperType::Movie,
        }
    }
}

pub(crate) trait MediaProvider {
    fn get_media_provider_info(&self) -> &MediaProviderInfo;
    fn get_id(&self) -> &str {
        self.get_media_provider_info().get_id()
    }
}

pub(crate) trait MediaArtworkProvider: MediaProvider {
    fn get_artworks(options: ArtworkSearchAndScrapeOption) -> Vec<MediaArtwork>;
}

pub(crate) trait RatingProvider {
    fn get_ratings() -> Vec<MediaRating>;
}

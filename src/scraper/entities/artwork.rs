use url::Url;

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

pub(crate) struct ImageSize<'a>{
  text_size: &'a str,
  order: i16,
}

impl<'a> ImageSize<'a> {
  pub const SmallPoster:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 1,
  };

  pub const MediumPoster: ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 2,
  };

  pub const BigPoster:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 4,
  };
  
  pub const LargePoster:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 8,
  };
  
  pub const XLargePoster:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 16,
  };

  pub const SmallFanArt:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 1,
  };

  pub const MediumFanArt: ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 2,
  };
  
  pub const LargeFanArt:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 8,
  };
  
  pub const XLargeFanArt:ImageSize<'static> = ImageSize {
    text_size:"", 
    order: 16,
  };

  pub fn get_text_size(&self) -> &str {
      &self.text_size
  }

  pub fn get_order(&self) -> i16 {
    self.order
  }
}

pub(crate) struct MediaArtwork {
    pub provider_id: String,
    pub art_type: MediaArtworkType,
    pub imdb_id: String,
    pub tmdb_id: i64,
    pub season: i8,
    pub likes: i32,
    pub preview_url: Url,
    pub default_url: Url,
    pub original_url: Url,
    pub language: String,
    urls: Vec<ImageInfo>,
}

impl MediaArtwork {
  // TODO
  pub fn get_preview_url(&self) -> &Url{
     &self.preview_url
  }

  // pub fn add_image_info(width: i32, height: i32, url: String) {
    
  // }
}

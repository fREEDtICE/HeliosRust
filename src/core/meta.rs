use url::Url;

pub(crate) enum MediaRatingType {
    NFO,
    Default,
    User,
}

#[derive(Debug)]
pub(crate) struct MediaRating {
    id: String,
    rating: f32,
    pub vote: i32,
    pub max_value: i32,
}

impl MediaRating {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn rating(&self) -> f32 {
        self.rating
    }

    pub fn get_normalized(&self) -> f32 {
        if self.max_value != 0 {
            return (self.rating / self.max_value as f32) * 10.0;
        }
        return 0.0;
    }

    pub fn set_normalized(&mut self, r: f32) {
        if self.rating < 0.0 || self.rating > 10.0 {
            return;
        }

        self.rating = r;
        self.max_value = 10;
    }
}

pub(crate) enum PersonRoleType {
    Actor,
    Director,
    Writer,
    Producer,
    Other,
}

pub(crate) struct Person {
    person_type: PersonRoleType,
    name: String,
    role: String,
    thumb: Url,
    profile: Url,
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Language {
    name: &'static str,
    code: &'static str,
}

impl Language {
    pub const Afrikaans: Language = Language {
        name: "Afrikaans",
        code: "af",
    };
    pub const Amharic: Language = Language {
        name: "አማርኛ",
        code: "am",
    };
    pub const English: Language = Language {
        name: "English",
        code: "en",
    };

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn code(&self) -> &'static str {
        self.code
    }
}



#[derive(Eq, PartialEq, Debug)]
pub(crate) struct MediaGenre {
    code_name: &'static str,
    code: i8,
    default_display_name: &'static str,
}

impl MediaGenre {
    pub const Action: MediaGenre = MediaGenre {
        code_name: "ACTION",
        code: 0,
        default_display_name: "Action",
    };

    pub fn get_locale_alternate_names(&self) -> Vec<&'static str> {
        Vec::new()
    }
}

// String enumName, int ordinal, String title, String pattern
pub(crate) struct MediaSource {
    code_name: &'static str,
    code: i8,
    title: &'static str,
    pattern: &'static str,
}

impl MediaSource {
    pub const UHDBlueray: MediaSource = MediaSource {
        code_name: "UHD_BLURAY",
        code: 0,
        title: "UHD Blu-ray",
        pattern: "(uhd|ultrahd)[ .\\-]?(bluray|blueray|bdrip|brrip|dbrip|bd25|bd50|bdmv|blu\\-ray)",
    };

    pub const Blueray: MediaSource = MediaSource {
        code_name: "BLURAY",
        code: 1,
        title: "Blu-ray",
        pattern: "(bluray|blueray|bdrip|brrip|dbrip|bd25|bd50|bdmv|blu\\-ray)",
    };

    pub const DVD: MediaSource = MediaSource {
        code_name: "DVD",
        code: 2,
        title: "DVD",
        pattern: "(dvd|video_ts|dvdrip|dvdr)",
    };

    pub const HDDVD: MediaSource = MediaSource {
        code_name: "HD_DVD",
        code: 3,
        title: "HD DVD",
        pattern: "(hddvd|hddvdrip)",
    };

    pub const TV: MediaSource = MediaSource {
        code_name: "TV",
        code: 4,
        title: "TV",
        pattern: "(tv|hdtv|pdtv|dsr|dtb|dtt|dttv|dtv|hdtvrip|tvrip|dvbrip)",
    };

    pub const VHS: MediaSource = MediaSource {
        code_name: "VHS",
        code: 5,
        title: "VHS",
        pattern: "(vhs|vhsrip)",
    };

    pub fn code_name(&self) -> &'static str {
        self.code_name
    }

    pub fn code(&self) -> i8 {
        self.code
    }

    pub fn title(&self) -> &'static str {
        self.title
    }

    pub fn pattern(&self) -> &'static str {
        self.pattern
    }
}

use crate::scraper::entities::country_lang::CountryCode;

pub(crate) struct MediaCertification {
    country_code: CountryCode,
    name: &'static str,
    notations: &'static [&'static str],
}

impl MediaCertification {
    pub const US_G: MediaCertification = MediaCertification {
        country_code: CountryCode::US,
        name: "G",
        notations: &["G", "Rated G"],
    };

    pub const US_PG: MediaCertification = MediaCertification {
        country_code: CountryCode::US,
        name: "PG",
        notations: &["PG", "Rated PG"],
    };

    pub const US_PG13: MediaCertification = MediaCertification {
        country_code: CountryCode::US,
        name: "PG13",
        notations: &["PG-13", "Rated PG-13"],
    };

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_country_code(&self) -> CountryCode {
        self.country_code
    }
}

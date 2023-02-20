use url::Url;

pub(crate) struct CountryCode {
    pub display: &'static str,
    pub alpha: &'static str,
    pub numeric: i16,
}

impl CountryCode {
    pub const AU: CountryCode = CountryCode {
        display: "Australia",
        alpha: "AUS",
        numeric: 36,
    };
}

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

pub(crate) enum MediaLanguage {
    Afrikaans,
    Amharic,
    Arabic,
    Mapudungun,
    Assamese,
    Azerbaijani,
    Bashkir,
    Belarusian,
    Bengali,
    Tibetan,
    Breton,
    Bosnian,
    Catalan,
    Corsican,
    Czech,
    Welsh,
    Danish,
    German,
    LowerSorbian,
    Divehi,
    Greek,
    English,
    Spanish,
    Estonian,
    Basque,
    Persian,
    Finnish,
    Filipino,
    Faroese,
    French,
    Frisian,
    Irish,
    ScottishGaelic,
    Galician,
    Alsatian,
    Gujarati,
    Hausa,
    Hebrew,
    Hindi,
    Croatian,
    UpperSorbian,
    Hungarian,
    Armenian,
    Indonesian,
    Igbo,
    Yi,
    Icelandic,
    Italian,
    Inuktitut,
    Japanese,
    Georgian,
    Kazakh,
    Greenlandic,
    Khmer,
    Kannada,
    Korean,
    Konkani,
    Kyrgyz,
    Luxembourgish,
    Lao,
    Lithuanian,
    Latvian,
    Maori,
    Macedonian,
    Malayalam,
    Mongolian,
    Mohawk,
    Marathi,
    Malay,
    Maltese,
    Burmese,
    Norwegian,
    Nepali,
    Dutch,
    Sesotho,
    Occitan,
    Odia,
    Punjabi,
    Polish,
    Dari,
    Pashto,
    Portuguese,
    Quechua,
}

#[derive(Eq, PartialEq, Debug)]
pub(crate) struct MediaGenre {
  code_name:&'static str,
  code: i8,
  default_display_name:&'static str,
}

impl MediaGenre {
  pub const Action:MediaGenre = MediaGenre {code_name: "ACTION", code: 0, default_display_name:"Action"};

  pub fn get_locale_alternate_names(&self) -> Vec<&'static str> {
    Vec::new()
  }
}

// String enumName, int ordinal, String title, String pattern
struct MediaSource {
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
// K'iche	K'iche	quc
// Romansh	Rumantsch	rm
// Romanian	română	ro
// Russian	русский	ru
// Kinyarwanda	Kinyarwanda	rw
// Sanskrit	संस्कृत	sa
// Yakut	саха	sah
// Sami (Northern)	davvisámegiella	se
// Sinhala	සිංහල	si
// Slovak	slovenčina	sk
// Slovenian	slovenski	sl
// Sami (Southern)	åarjelsaemiengiele	sma
// Sami (Lule)	julevusámegiella	smj
// Sami (Inari)	sämikielâ	smn
// Sami (Skolt)	sääm´ǩiõll	sms
// Albanian	shqipe	sq
// Serbian	srpski/српски	sr
// Swedish	svenska	sv
// Kiswahili	Kiswahili	sw
// Syriac	ܣܘܪܝܝܐ	syc
// Tamil	தமிழ்	ta
// Telugu	తెలుగు	te
// Tajik	Тоҷикӣ	tg
// Thai	ไทย	th
// Turkmen	türkmençe	tk
// Tswana	Setswana	tn
// Turkish	Türkçe	tr
// Tatar	Татарча	tt
// Tamazight	Tamazight	tzm
// Uyghur	ئۇيغۇرچە	ug
// Ukrainian	українська	uk
// Urdu	اُردو	ur
// Uzbek	U'zbek/Ўзбек	uz
// Vietnamese	Tiếng Việt/㗂越	vi
// Wolof	Wolof	wo
// Xhosa	isiXhosa	xh
// Yoruba	Yoruba	yo
// Chinese	中文	zh
// Zulu	isiZulu	zu
impl MediaLanguage {
    pub fn title(self) -> String {
        match self {
            Afrikaans => "Afrikaans".to_string(),
            Amharic => "አማርኛ".to_string(),
            Arabic => "العربية".to_string(),
            Mapudungun => "Mapudungun".to_string(),
            Assamese => "অসমীয়া".to_string(),
            Azerbaijani => "Azərbaycan­lı".to_string(),
            Bashkir => "Башҡорт".to_string(),
            Belarusian => "беларуская".to_string(),
            Bulgarian => "български".to_string(),
            Bengali => "বাংলা".to_string(),
            Tibetan => "བོད་ཡིག".to_string(),
            Breton => "brezhoneg".to_string(),
            Bosnian => "bosanski/босански".to_string(),
            Catalan => "català".to_string(),
            Corsican => "Corsu".to_string(),
            Czech => "čeština".to_string(),
            Welsh => "Cymraeg".to_string(),
            Danish => "dansk".to_string(),
            German => "Deutsch".to_string(),
            LowerSorbian => "dolnoserbšćina".to_string(),
            Divehi => "ދިވެހިބަސް".to_string(),
            Greek => "ελληνικά".to_string(),
            English => "English".to_string(),
            Spanish => "español".to_string(),
            Estonian => "eesti".to_string(),
            Basque => "euskara".to_string(),
            Persian => "فارسى".to_string(),
            Finnish => "suomi".to_string(),
            Filipino => "Filipino".to_string(),
            Faroese => "føroyskt".to_string(),
            French => "français".to_string(),
            Frisian => "Frysk".to_string(),
            Irish => "Gaeilge".to_string(),
            ScottishGaelic => "Gàidhlig".to_string(),
            Galician => "galego".to_string(),
            Alsatian => "Elsässisch".to_string(),
            Gujarati => "ગુજરાતી".to_string(),
            Hausa => "Hausa".to_string(),
            Hebrew => "עברית".to_string(),
            Hindi => "हिंदी".to_string(),
            Croatian => "hrvatski".to_string(),
            UpperSorbian => "hornjoserbšćina".to_string(),
            Hungarian => "magyar".to_string(),
            Armenian => "Հայերեն".to_string(),
            Indonesian => "Bahasa Indonesia".to_string(),
            Igbo => "Igbo".to_string(),
            Yi => "ꆈꌠꁱꂷ".to_string(),
            Icelandic => "íslenska".to_string(),
            Italian => "italiano".to_string(),
            Inuktitut => "Inuktitut /ᐃᓄᒃᑎᑐᑦ (ᑲᓇᑕ)".to_string(),
            Japanese => "日本語".to_string(),
            Georgian => "ქართული".to_string(),
            Kazakh => "Қазақша".to_string(),
            Greenlandic => "kalaallisut".to_string(),
            Khmer => "ខ្មែរ".to_string(),
            Kannada => "ಕನ್ನಡ".to_string(),
            Korean => "한국어/韓國語|조선말/朝鮮말".to_string(),
            Konkani => "कोंकणी".to_string(),
            Kyrgyz => "Кыргыз".to_string(),
            Luxembourgish => "Lëtzebuergesch".to_string(),
            Lao => "ລາວ".to_string(),
            Lithuanian => "lietuvių".to_string(),
            Latvian => "latviešu".to_string(),
            Maori => "Reo Māori".to_string(),
            Macedonian => "македонски јазик".to_string(),
            Malayalam => "മലയാളം".to_string(),
            Mongolian => "Монгол хэл/ᠮᠤᠨᠭᠭᠤᠯ ᠬᠡᠯᠡ".to_string(),
            Mohawk => "Kanien'kéha".to_string(),
            Marathi => "मराठी".to_string(),
            Malay => "Bahasa Malaysia".to_string(),
            Maltese => "Malti".to_string(),
            Burmese => "Myanmar".to_string(),
            Norwegian => "norsk".to_string(),
            Nepali => "नेपाली (नेपाल)".to_string(),
            Dutch => "Nederlands".to_string(),
            Sesotho => "Sesotho sa Leboa".to_string(),
            Occitan => "Occitan".to_string(),
            Odia => "ଓଡ଼ିଆ".to_string(),
            Punjabi => "ਪੰਜਾਬੀ".to_string(),
            Polish => "polski".to_string(),
            Dari => "درى".to_string(),
            Pashto => "پښتو".to_string(),
            Portuguese => "Português".to_string(),
            Quechua => "runasimi".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            _ => "English".to_string(),
        }
    }

    pub fn code(&self) -> String {
        match self {
            Afrikaans => "af".to_string(),
            Amharic => "am".to_string(),
            Arabic => "ar".to_string(),
            Mapudungun => "arn".to_string(),
            Assamese => "as".to_string(),
            Azerbaijani => "az".to_string(),
            Bashkir => "ba".to_string(),
            Belarusian => "be".to_string(),
            Bulgarian => "bg".to_string(),
            Bengali => "bn".to_string(),
            Tibetan => "bo".to_string(),
            Breton => "br".to_string(),
            Bosnian => "bs".to_string(),
            Catalan => "ca".to_string(),
            Corsican => "co".to_string(),
            Czech => "cs".to_string(),
            Welsh => "cy".to_string(),
            Danish => "da".to_string(),
            German => "de".to_string(),
            LowerSorbian => "dsb".to_string(),
            Divehi => "dv".to_string(),
            Greek => "el".to_string(),
            English => "en".to_string(),
            Spanish => "es".to_string(),
            Estonian => "et".to_string(),
            Basque => "eu".to_string(),
            Persian => "fa".to_string(),
            Finnish => "fi".to_string(),
            Filipino => "fil".to_string(),
            Faroese => "fo".to_string(),
            French => "fr".to_string(),
            Frisian => "fy".to_string(),
            Irish => "ga".to_string(),
            ScottishGaelic => "gd".to_string(),
            Galician => "gl".to_string(),
            Alsatian => "gsw".to_string(),
            Gujarati => "gu".to_string(),
            Hausa => "ha".to_string(),
            Hebrew => "he".to_string(),
            Hindi => "hi".to_string(),
            Croatian => "hr".to_string(),
            UpperSorbian => "hsb".to_string(),
            Hungarian => "hu".to_string(),
            Armenian => "hy".to_string(),
            Indonesian => "id".to_string(),
            Igbo => "ig".to_string(),
            Yi => "ii".to_string(),
            Icelandic => "is".to_string(),
            Italian => "it".to_string(),
            Inuktitut => "iu".to_string(),
            Japanese => "ja".to_string(),
            Georgian => "ka".to_string(),
            Kazakh => "kk".to_string(),
            Greenlandic => "kl".to_string(),
            Khmer => "km".to_string(),
            Kannada => "kn".to_string(),
            Korean => "ko".to_string(),
            Konkani => "kok".to_string(),
            Kyrgyz => "ky".to_string(),
            Luxembourgish => "lb".to_string(),
            Lao => "lo".to_string(),
            Lithuanian => "lt".to_string(),
            Latvian => "lv".to_string(),
            Maori => "mi".to_string(),
            Macedonian => "mk".to_string(),
            Malayalam => "ml".to_string(),
            Mongolian => "mn".to_string(),
            Mohawk => "moh".to_string(),
            Marathi => "mr".to_string(),
            Malay => "ms".to_string(),
            Maltese => "mt".to_string(),
            Burmese => "my".to_string(),
            Norwegian => "no".to_string(),
            Nepali => "ne".to_string(),
            Dutch => "nl".to_string(),
            Sesotho => "st".to_string(),
            Occitan => "oc".to_string(),
            Odia => "or".to_string(),
            Punjabi => "pa".to_string(),
            Polish => "pl".to_string(),
            Dari => "prs".to_string(),
            Pashto => "ps".to_string(),
            Portuguese => "pt".to_string(),
            Quechua => "qu".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            // Belarusian => "".to_string(),
            _ => "English".to_string(),
        }
    }
}

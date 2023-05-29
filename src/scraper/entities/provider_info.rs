use url::Url;

pub(crate) struct MediaProviderInfo {
    id: String,
    sub_id: String,
    name: String,
    description: String,
    logo: Url,
    pub priority: i32,
    pub version: String,
    // TODO MediaConfig
}

impl MediaProviderInfo {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_sub_id(&self) -> &str {
        &self.sub_id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_logo(&self) -> &Url {
        &self.logo
    }
}

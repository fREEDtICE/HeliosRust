pub(crate) mod meta;

use std::path::Path;

use crate::core::meta::{MediaSource};

pub fn parse_media_source(path_str:String) -> MediaSource {
    let path = Path::new(path_str);
    
}
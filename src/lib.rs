extern crate id3;
extern crate liquery;

use liquery::Queryable;
use id3::Tag;
use std::path::Path;

/// Queryable instace of a ID3 Tag
pub struct QueryId3 {
    tag: Tag,
}

impl QueryId3 {
    /// Create a QueryTag from a file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ()> {
        // TODO: return a proper error.
        match Tag::read_from_path(path) {
            Ok(tag) => Ok(QueryId3 { tag: tag }),
            Err(e) => Err(()),
        }
    }
}

impl Queryable for QueryId3 {
    fn query(&self, key: &str) -> Option<String> {
        match key {
            "title" => self.tag.title().map(|s| s.to_owned()),
            "artist" => self.tag.artist().map(|s| s.to_owned()),
            "album" => self.tag.album().map(|s| s.to_owned()),
            "album artist" => self.tag.album_artist().map(|s| s.to_owned()),
            "year" => self.tag.year().map(|s| format!("{}", s)),
            "disc number" => self.tag.disc().map(|s| format!("{}", s)),
            "total discs" => self.tag.total_discs().map(|s| format!("{}", s)),
            "total tracks" => self.tag.total_tracks().map(|s| format!("{}", s)),
            "track number" => self.tag.track().map(|s| format!("{}", s)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryId3;

    #[test]
    fn fails_on_non_existing_file() {
        assert!(QueryId3::new("non-existing-file").is_err());
    }
}

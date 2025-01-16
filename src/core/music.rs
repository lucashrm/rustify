use std::io::Error;
use id3::{Tag, TagLike};

#[derive(Debug)]
pub struct MusicMetaData {
    title: String,
    artist: String,
    album: String,
}

impl MusicMetaData {
    fn new(filepath: &str) -> Result<Self, Error> {
        let tag = Tag::read_from_path(filepath).expect("error while reading path");

        Ok(Self {
            artist: tag.artist().unwrap_or_else(|| "unknown artist").to_string(),
            title: tag.title().unwrap_or_else(|| "untitled").to_string(),
            album: tag.album().unwrap_or_else(|| "no album").to_string(),
        })
    }
}

pub struct Music {
    is_playing: bool,
    filepath: String,
    pub metadata: MusicMetaData
}

impl Music {
    pub fn new(filepath: &str) -> Result<Self, Error> {
        let metadata = MusicMetaData::new(filepath).unwrap();

        println!("{}", metadata.artist);

        Ok(Self {
            is_playing: false,
            filepath: filepath.to_string(),
            metadata,
        })
    }
}
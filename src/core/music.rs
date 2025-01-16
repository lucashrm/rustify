use std::io::Error;
use id3::{Tag, TagLike};
use std::path::Path;
use mp3_duration;
use vizia::prelude::*;

#[derive(Debug)]
pub struct MusicMetaData {
    title: String,
    artist: String,
    album: String,
    length: f32,
}

impl MusicMetaData {
    fn new(filepath: &str) -> Result<Self, Error> {
        let tag = Tag::read_from_path(filepath).expect("error while reading path");

        let path = Path::new(filepath);
        let duration = mp3_duration::from_path(&path).unwrap();

        Ok(Self {
            artist: tag.artist().unwrap_or_else(|| "unknown artist").to_string(),
            title: tag.title().unwrap_or_else(|| "untitled").to_string(),
            album: tag.album().unwrap_or_else(|| "no album").to_string(),
            length: duration.as_secs_f32()
        })
    }

    pub fn get_infos(&self) -> (&str, &str, &str) {
        (&self.title.as_str(), &self.album.as_str(), &self.artist.as_str())
    }

    pub fn get_duration(&self) -> f32 {
        self.length
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
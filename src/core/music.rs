use std::io::Error;
use id3::{Tag, TagLike};
use std::path::Path;
use mp3_duration;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use vizia::prelude::*;
use vizia::vg::gpu::gl::Format;

#[derive(Debug, Clone)]
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

#[derive(Clone)]
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

    pub fn play(&self) {
        let src = std::fs::File::open(self.filepath.as_str()).expect("failed to open media");

        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();
        hint.with_extension("mp3");

        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts).expect("unsupported format");

    }
}
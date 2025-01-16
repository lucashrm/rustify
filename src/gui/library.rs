use vizia::prelude::*;
use std::{fs, io};
use crate::LIBRARY_PATH;
use std::path::Path;
use crate::core::music::Music;
use std::io::Error;

pub struct Library {}

impl View for Library {}

impl Library {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        let musics = get_all_library_musics(LIBRARY_PATH)
            .expect("Error while loading a music");

        for music in musics {
            println!("{:?}", music.metadata);
        }

        Self {}.build(cx, |cx| {
            Label::new(cx, "Your library");
        })
    }
}

fn get_all_library_musics(dir_path: &str) -> Result<Vec<Music>, Error> {
    let dir: &Path = Path::new(dir_path);

    let paths = fs::read_dir(dir)?.map(|x| {
        x.map(|e| e.path())
    }).collect::<Result<Vec<_>, io::Error>>()?;

    let mut musics: Vec<Music> = vec![];

    for path in paths {
        if path.is_dir() {
            musics.append(&mut get_all_library_musics(path.to_str().unwrap())?);
        } else {
            musics.push(Music::new(path.to_str().unwrap())?);
        }
    }

    Ok(musics)

    // let mut iter_musics: Vec<Music> = vec![];
    //
    // let mut musics = paths.iter().map(|x| {
    //     if x.is_dir() {
    //         iter_musics.append(&mut get_all_library_musics(x.to_str().unwrap())?);
    //     } else {
    //     Music::new(x.to_str().unwrap())
    //
    // }).collect::<Result<Vec<_>, Error>>()?;
    //
    // musics.append(&mut iter_musics);
    //
    // Ok(musics)
}
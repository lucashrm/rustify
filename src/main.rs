mod gui;
mod core;

use std::{fs, io};
use std::io::Error;
use std::path::Path;
use vizia::prelude::*;
use crate::gui::music_player::*;
use crate::gui::library::*;
use crate::core::music::{Music};

const LIBRARY_PATH: &str = "assets/musics/";

#[derive(Lens)]
pub struct AppData {
    music_list: Vec<Music>,
    selected_music: Music,
}

impl Model for AppData {}

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
}

fn play(cx: &mut EventContext, music: &Music) {
    println!("{}", music.metadata.get_infos().0);
}

fn main() -> Result<(), ApplicationError> {
    let musics = get_all_library_musics(LIBRARY_PATH)
        .expect("Error while loading a music");
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("assets/css/style.css"))
            .expect("Failed to load stylesheet");

        MusicPlayer::new(cx);
        Library::new(cx);
        for music in musics {
            MusicLine::new(cx, music).on_play(play);
        }

    })
    .title("rustify")
    .inner_size((1920, 1080))
    .run()
}

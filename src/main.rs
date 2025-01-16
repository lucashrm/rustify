mod gui;
mod core;

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

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("assets/css/style.css"))
            .expect("Failed to load stylesheet");

        MusicPlayer::new(cx);
        Library::new(cx);
    })
    .title("rustify")
    .inner_size((1920, 1080))
    .run()
}

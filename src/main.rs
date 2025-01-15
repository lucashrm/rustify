mod gui;

use vizia::prelude::*;
use crate::gui::music_player::*;

#[derive(Lens)]
pub struct AppData {
    music_timer: f32,
}

impl Model for AppData {}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("assets/css/style.css"))
            .expect("Failed to load stylesheet");

        MusicPlayer::new(cx);
    })
    .title("rustify")
    .inner_size((1920, 1080))
    .run()
}

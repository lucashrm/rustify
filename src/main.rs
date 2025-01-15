use vizia::icons::ICON_PLAYER_PLAY;
use vizia::prelude::*;
#[derive(Lens)]
pub struct AppData {
    music_timer: f32,
}

impl Model for AppData {}

pub struct MusicPlayer {}

impl View for MusicPlayer {}

impl MusicPlayer {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Svg::new(cx, ICON_PLAYER_PLAY));
            });
        })
    }
}

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

use vizia::icons::ICON_PLAYER_PLAY;
use vizia::prelude::*;

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

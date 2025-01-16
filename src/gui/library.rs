use std::fmt::Debug;
use vizia::icons::ICON_PLAYER_PLAY_FILLED;
use vizia::prelude::*;
use crate::core::music::Music;

pub struct MusicLine {
    on_play: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl View for MusicLine {}

impl MusicLine {
    pub fn new(cx: &mut Context, music: &Music) {
        let duration = format!("{}:{}",
                               (music.metadata.get_duration() / 60.0) as i32,
                               music.metadata.get_duration() % 60.0);

        HStack::new(cx, |cx| {
            Label::new(cx, music.metadata.get_infos().0);
            Label::new(cx, music.metadata.get_infos().1);
            Label::new(cx, music.metadata.get_infos().2);
            Label::new(cx, duration);
            Button::new(cx, |cx| {
                Svg::new(cx, ICON_PLAYER_PLAY_FILLED)
            });
        });
    }
}

pub struct Library {}

impl View for Library {}

impl Library {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            Label::new(cx, "Your library");
        })
    }
}
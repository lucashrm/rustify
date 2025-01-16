use vizia::prelude::*;
use crate::core::music::Music;

pub struct MusicLine {}

impl View for MusicLine {}

impl MusicLine {
    pub fn new(cx: &mut Context, music: &Music) {
        HStack::new(cx, |cx| {
            Label::new(cx, music.metadata.get_infos().0);
            Label::new(cx, music.metadata.get_infos().1);
            Label::new(cx, music.metadata.get_infos().2);
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


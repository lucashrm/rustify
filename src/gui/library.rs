use std::fmt::Debug;
use vizia::icons::ICON_PLAYER_PLAY_FILLED;
use vizia::prelude::*;
use crate::core::music::Music;

pub enum MusicEvent {
    Play
}

pub struct MusicLine {
    on_play: Option<Box<dyn Fn(&mut EventContext, &Music)>>,
    music: Music
}

impl View for MusicLine {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|music_event, meta| match music_event {
            MusicEvent::Play => {
                if let Some(callback) = &self.on_play {
                    (callback)(cx, &self.music)
                }
            }
        })
    }
}

pub trait MusicModifiers {
    fn on_play<F: Fn(&mut EventContext, &Music) + 'static>(self, callback: F) -> Self;
}

impl<'a> MusicModifiers for Handle<'a, MusicLine> {
    fn on_play<F: Fn(&mut EventContext, &Music) + 'static>(self, callback: F) -> Self {
        self.modify(|music_line| music_line.on_play = Some(Box::new(callback)))
    }
}

impl MusicLine {
    pub fn new<F: Fn(&mut EventContext, &Music) + 'static>(cx: &mut Context, music: Music, callback: F) -> Handle<Self> {
        let duration = format!("{}:{}",
                               (music.metadata.get_duration() / 60.0) as i32,
                               music.metadata.get_duration() % 60.0);
        Self {
            on_play: Some(Box::new(callback)),
            music: music.clone()
        }.build(cx, |cx| {
            HStack::new(cx, |cx| {
                Label::new(cx, music.metadata.get_infos().0);
                Label::new(cx, music.metadata.get_infos().1);
                Label::new(cx, music.metadata.get_infos().2);
                Label::new(cx, duration);
                Button::new(cx, |cx| {
                    Svg::new(cx, ICON_PLAYER_PLAY_FILLED)
                }).on_press(|ex| ex.emit(MusicEvent::Play));
            });
        })
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
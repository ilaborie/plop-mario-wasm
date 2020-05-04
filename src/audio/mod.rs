use crate::audio::musics::{MusicPlayer, Track};

pub mod musics;
pub mod sounds;

pub struct MusicController {
    music_player: MusicPlayer,
}

impl MusicController {
    pub fn new(music_player: MusicPlayer) -> Self {
        Self { music_player }
    }

    pub fn play(&self, track: Track) {
        self.music_player.play(track)
    }
}

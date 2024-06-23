use std::collections::HashMap;

use raylib::prelude::*;

pub const ROTATE_SOUND_BYTES: &[u8] = include_bytes!("../rotate.ogg");
pub const WRONG_MOVE_SOUND_BYTES: &[u8] = include_bytes!("../wrong_move.ogg");

#[derive(Debug)]
pub struct AudioBox<'aud> {
    audio_subsystem: RaylibAudio,
    sounds: HashMap<String, (Wave<'aud>, Sound<'aud>)>,
}

impl<'aud> AudioBox<'aud> {
    pub fn new() -> Self {
        let audio_subsystem = RaylibAudio::init_audio_device();
        Self {
            audio_subsystem: audio_subsystem.unwrap(),
            sounds: HashMap::new(),
        }
    }

    pub fn load_sound_from_bytes(&'aud mut self, name: &str, bytes: &[u8], filetype: &str) {
        let wav = self
            .audio_subsystem
            .new_wave_from_memory(filetype, bytes)
            .unwrap();
        let sound = self.audio_subsystem.new_sound_from_wave(&wav).unwrap();
        self.sounds.insert(name.to_owned(), (wav, sound));
    }

    pub fn play_sound(&self, name: &str) -> bool {
        if let Some((_, sound)) = self.sounds.get(name) {
            sound.play();
            return true;
        }

        false
    }
}

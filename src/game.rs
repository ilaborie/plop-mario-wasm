use crate::level::Level;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::AudioContext;

pub struct GameContext {
    audio_context: Rc<AudioContext>,
    level: Rc<RefCell<Level>>,
    dt: f64,
}

#[allow(dead_code)]
impl GameContext {
    pub fn new(audio_context: Rc<AudioContext>, level: Rc<RefCell<Level>>, dt: f64) -> Self {
        Self {
            audio_context,
            level,
            dt,
        }
    }

    pub fn dt(&self) -> f64 {
        self.dt
    }

    pub fn level(&self) -> Rc<RefCell<Level>> {
        self.level.clone()
    }

    pub fn audio_context(&self) -> &AudioContext {
        &self.audio_context
    }
}

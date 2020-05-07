use crate::entity::events::EventBuffer;
use crate::entity::player_env::PlayerEnv;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::AudioContext;

pub struct GameContext {
    audio_context: Rc<AudioContext>,
    player_env: Rc<RefCell<PlayerEnv>>,
    event_buffer: Rc<RefCell<EventBuffer>>,
    dt: f64,
}

#[allow(dead_code)]
impl GameContext {
    pub fn new(
        audio_context: Rc<AudioContext>,
        event_buffer: Rc<RefCell<EventBuffer>>,
        player_env: Rc<RefCell<PlayerEnv>>,
        dt: f64,
    ) -> Self {
        Self {
            audio_context,
            player_env,
            event_buffer,
            dt,
        }
    }

    pub fn dt(&self) -> f64 {
        self.dt
    }

    pub fn emitter(&self) -> Rc<RefCell<EventBuffer>> {
        self.event_buffer.clone()
    }

    pub fn player_env(&self) -> Rc<RefCell<PlayerEnv>> {
        self.player_env.clone()
    }

    pub fn audio_context(&self) -> &AudioContext {
        &self.audio_context
    }
}

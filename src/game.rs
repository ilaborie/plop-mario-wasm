use crate::events::EventBuffer;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{AudioContext, CanvasRenderingContext2d};

#[derive(Clone, Debug)]
pub struct PlayerInfo {
    name: String,
    lives: u32,
    score: u32,
    coins: u32,
}

impl PlayerInfo {
    pub fn new(name: &str, lives: u32, score: u32, coins: u32) -> Self {
        let name = String::from(name);
        Self {
            name,
            lives,
            score,
            coins,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn lives(&self) -> u32 {
        self.lives
    }
    pub fn score(&self) -> u32 {
        self.score
    }
    pub fn coins(&self) -> u32 {
        self.coins
    }
}

#[derive(Clone)]
pub struct GameContext {
    audio_context: Rc<AudioContext>,
    video_context: Rc<CanvasRenderingContext2d>,
    event_buffer: Rc<RefCell<EventBuffer>>,
    dt: f64,
}

impl GameContext {
    pub fn new(
        audio_context: Rc<AudioContext>,
        video_context: Rc<CanvasRenderingContext2d>,
        event_buffer: Rc<RefCell<EventBuffer>>,
        dt: f64,
    ) -> Self {
        Self {
            audio_context,
            video_context,
            event_buffer,
            dt,
        }
    }

    pub fn dt(&self) -> f64 {
        self.dt
    }

    pub fn video_context(&self) -> Rc<CanvasRenderingContext2d> {
        self.video_context.clone()
    }

    pub fn emitter(&self) -> Rc<RefCell<EventBuffer>> {
        self.event_buffer.clone()
    }

    pub fn audio_context(&self) -> &AudioContext {
        &self.audio_context
    }
}

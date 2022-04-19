use std::cell::RefCell;
use std::rc::Rc;

use web_sys::{AudioContext, CanvasRenderingContext2d};

use crate::assets::Assets;
use crate::events::{Event, EventBuffer};
use crate::game::{GameContext, PlayerInfo};
use crate::input::Keyboard;
use crate::scene::SceneRunner;
use crate::utils::log;

pub struct System {
    audio_context: Rc<AudioContext>,
    video_context: Rc<CanvasRenderingContext2d>,
    event_buffer: Rc<RefCell<EventBuffer>>,
    scene_runner: Rc<RefCell<SceneRunner>>,
}

impl System {
    pub fn new(assets: Assets, video_context: Rc<CanvasRenderingContext2d>) -> Self {
        // Events
        let event_buffer: Rc<RefCell<EventBuffer>> = Rc::default();

        // Keyboard
        let mut input = Keyboard::new(&assets.configuration());
        input.listen();
        let input = Rc::new(RefCell::new(input));

        // Scenes
        let scene_runner = SceneRunner::new(assets, input);
        let scene_runner = Rc::new(RefCell::new(scene_runner));

        // Audio
        let audio_context = AudioContext::new().unwrap();
        let audio_context = Rc::new(audio_context);

        Self {
            audio_context,
            video_context,
            event_buffer,
            scene_runner,
        }
    }

    pub fn start(&mut self, player_name: &str) {
        let player_info = PlayerInfo::new(player_name, 3, 0, 0);
        self.scene_runner
            .borrow_mut()
            .run_level("1-1", &player_info);
    }

    pub fn update(&mut self, dt: f64) {
        let context = GameContext::new(
            self.audio_context.clone(),
            self.video_context.clone(),
            self.event_buffer.clone(),
            dt,
        );

        // Update scene
        self.scene_runner.borrow().update(&context);

        // Process events
        let sr = self.scene_runner.clone();
        let system_events = self.event_buffer.borrow_mut().drain_system();
        self.event_buffer.borrow_mut().clear();
        for event in system_events.iter() {
            match event {
                Event::SceneComplete => sr.borrow_mut().run_next(),
                Event::GotoLevel { level, player } => {
                    log(&format!("Goto <{level}> with {player:?}"));
                    sr.borrow_mut().run_level(level, player);
                    return;
                }
                _ => {} // Skip other events
            }
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        let _ = self.audio_context.close();
    }
}

use crate::assets::Assets;
use crate::game::{GameContext, PlayerInfo};
use crate::input::Keyboard;
use crate::scene::level::Level;
use crate::scene::wait_scene::WaitScene;
use crate::utils::log;
use core::cell::RefCell;
use std::rc::Rc;

pub mod level;
pub mod wait_scene;

pub trait Scene {
    fn update_soft(&self, context: &GameContext);
    fn update(&mut self, context: &GameContext);
    fn draw(&mut self, context: &GameContext);

    fn pause(&mut self) {
        log("Pause");
    }
}

pub struct SceneRunner {
    assets: Assets,
    input: Rc<RefCell<Keyboard>>,
    current_index: Option<usize>,
    scenes: Vec<Rc<RefCell<dyn Scene>>>,
}

impl SceneRunner {
    pub fn new(assets: Assets, input: Rc<RefCell<Keyboard>>) -> Self {
        let current_index = None;
        let scenes = vec![];

        Self {
            assets,
            input,
            current_index,
            scenes,
        }
    }

    fn create_level(&self, level: &str) -> Rc<RefCell<Level>> {
        let level = Level::new(level, self.assets.clone());

        Rc::new(RefCell::new(level))
    }

    fn current(&self) -> Option<Rc<RefCell<dyn Scene>>> {
        self.current_index.and_then(|i| self.scenes.get(i)).cloned()
    }

    pub fn run_level(&mut self, level_name: &str, player_info: &PlayerInfo) {
        // Pause current
        if let Some(current) = self.current() {
            current.borrow_mut().pause();
        }
        self.scenes.clear();
        self.current_index = None;

        // Level
        let level = self.create_level(level_name);

        // Progress
        let progress = WaitScene::new(self.assets.font(), level.clone());
        self.scenes.push(Rc::new(RefCell::new(progress)));
        self.scenes.push(level.clone());

        // Player
        level
            .borrow_mut()
            .start_or_resume(player_info, self.input.clone());

        self.run_next();
    }

    pub fn run_next(&mut self) {
        self.current_index = if let Some(i) = self.current_index {
            Some(i + 1)
        } else {
            Some(0)
        };
    }

    pub fn update(&self, context: &GameContext) {
        if let Some(scene) = self.current() {
            scene.borrow().update_soft(context);
            scene.borrow_mut().update(context);
            scene.borrow_mut().draw(context);
        }
    }
}

use crate::assets::font::Font;
use crate::entity::player_env::PlayerEnv;
use crate::game::GameContext;
use crate::scene::level::Level;
use core::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct Dashboard {
    font: Rc<Font>,
}

impl Dashboard {
    pub fn new(font: Rc<Font>) -> Self {
        Self { font }
    }

    pub fn draw_info(
        &self,
        context: Rc<CanvasRenderingContext2d>,
        level_name: &str,
        player_env: Rc<RefCell<PlayerEnv>>,
    ) {
        let line1 = self.font.size() as f64;
        let line2 = (2 * self.font.size()) as f64;

        // World
        self.font.print(context.clone(), "WORLD", 152., line1);
        let lvl = format!("{:^width$}", level_name, width = "WORLD".len());
        self.font.print(context.clone(), lvl.as_str(), 152., line2);

        // Player
        let name = player_env.borrow().name().to_uppercase();
        self.font.print(context.clone(), name.as_str(), 16., line1);
        let score = format!(
            "{:0>width$}",
            player_env.borrow().score().get(),
            width = name.len() + 1
        );
        self.font.print(context.clone(), score.as_str(), 16., line2);

        // let lives = format!(" x{:>2}", player_env.borrow().lives().get());
        // self.font.print(context.clone(), lives.as_str(), 96., line1);

        let coins = format!("@x{:0>2}", player_env.borrow().coins().get());
        self.font.print(context.clone(), coins.as_str(), 96., line2);

        // Time
        self.font.print(context.clone(), "TIME", 208., line1);
        let t = format!(
            "{:>width$}",
            player_env.borrow().time().get().floor() as u32,
            width = "TIME".len()
        );
        self.font.print(context, t.as_str(), 208., line2);
    }

    pub fn draw(&self, game_context: &GameContext, level: &Level) {
        let context = game_context.video_context();
        if let Some(player_env) = level.find_player() {
            self.draw_info(context, level.name(), player_env);
        }
    }
}

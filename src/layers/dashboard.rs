use crate::assets::font::Font;
use crate::entity::player_env::PlayerEnv;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::CanvasRenderingContext2d;

pub struct Dashboard {
    font: Font,
    level: String,
    player_env: Rc<RefCell<PlayerEnv>>,
}

impl Dashboard {
    pub fn new(font: Font, level: String, player_env: Rc<RefCell<PlayerEnv>>) -> Self {
        Self {
            font,
            level,
            player_env,
        }
    }

    pub(crate) fn draw(&mut self, context: &CanvasRenderingContext2d) {
        let line1 = self.font.size() as f64;
        let line2 = (2 * self.font.size()) as f64;

        let name = self.player_env.borrow().name();
        self.font.print(context, name.as_str(), 16., line1);
        let score = format!(
            "{:0>width$}",
            self.player_env.borrow().score().get(),
            width = name.len() + 1
        );
        self.font.print(context, score.as_str(), 16., line2);

        let coins = format!("@x{:0>2}", self.player_env.borrow().coins().get());
        self.font.print(context, coins.as_str(), 96., line2);

        self.font.print(context, "WORLD", 152., line1);
        let lvl = format!("{:^width$}", self.level, width = "WORLD".len());
        self.font.print(context, lvl.as_str(), 152., line2);

        self.font.print(context, "TIME", 208., line1);
        let t = format!(
            "{:>width$}",
            self.player_env.borrow().time().get().floor() as u32,
            width = "TIME".len()
        );
        self.font.print(context, t.as_str(), 208., line2);
    }
}

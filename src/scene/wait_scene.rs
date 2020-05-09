use crate::assets::font::Font;
use crate::game::GameContext;
use crate::layers::colors::ColorsLayer;
use crate::layers::dashboard::Dashboard;
use crate::layers::player_progress::PlayerProgress;
use crate::scene::level::Level;
use crate::scene::Scene;
use core::cell::RefCell;
use std::rc::Rc;

pub struct WaitScene {
    count_down: f64,
    color: ColorsLayer,
    dashboard: Dashboard,
    progress: PlayerProgress,
    level: Rc<RefCell<Level>>,
}

impl WaitScene {
    pub fn new(font: Rc<Font>, level: Rc<RefCell<Level>>) -> Self {
        let count_down = 2.;

        // Layers
        let color = ColorsLayer::new("#000");
        let dashboard = Dashboard::new(font.clone());
        let progress = PlayerProgress::new(font, level.clone());

        Self {
            level,
            count_down,
            color,
            dashboard,
            progress,
        }
    }
}

impl Scene for WaitScene {
    fn update_soft(&self, _context: &GameContext) {}

    fn update(&mut self, context: &GameContext) {
        self.count_down -= context.dt();
        if self.count_down < 0. {
            context.emitter().borrow_mut().scene_complete();
        }
    }

    fn draw(&mut self, context: &GameContext) {
        let context = context.video_context();

        if let Some(player) = self.level.borrow().find_player() {
            self.color.draw(context.clone());
            self.dashboard
                .draw_info(context.clone(), self.level.borrow().name(), player);
            self.progress.draw(context);
        }
    }
}

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use crate::assets::{Sprite, SpriteSheet};
use crate::levels::Level;
use crate::compositor::{Compositor, Position};
use std::rc::Rc;
use std::cell::RefCell;
use crate::layers::sprite::SpriteLayer;
use crate::layers::backgrounds::BackgroundsLayer;

#[wasm_bindgen]
pub struct System {
    compositor: Compositor,
    player_layer: Rc<RefCell<SpriteLayer>>,
}

#[wasm_bindgen]
impl System {
    pub fn new(level: Level, sprites: SpriteSheet, player: SpriteSheet) -> Self {
        // Background
        let bg = BackgroundsLayer::new(level.backgrounds(), &sprites);

        // Player
        let position = Position::new(64, 64);
        let player_layer = SpriteLayer::new(Sprite::MarioIdle, position, Box::new(player));
        let player_layer = Rc::new(RefCell::new(player_layer));
        let player_layer_rc = player_layer.clone();

        // Compositor
        let mut compositor = Compositor::new();
        compositor.add_layer(Rc::new(move |ctx| bg.draw(ctx)));
        compositor.add_layer(Rc::new(move |ctx| player_layer_rc.borrow().draw(ctx)));

        Self { compositor, player_layer }
    }

    pub fn draw_all(&self, context: &CanvasRenderingContext2d) {
        self.compositor.draw(context);
    }

    pub fn move_player(&mut self) {
        self.player_layer.borrow_mut().update_position(2, 2);
    }
}


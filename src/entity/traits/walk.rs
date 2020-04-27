use crate::entity::traits::EntityTrait;
use crate::entity::{Entity, ObstructionSide};
use crate::physics::rectangle::BoundingBox;

pub struct Walk {}

impl EntityTrait for Walk {
    fn update(&mut self, entity: &mut Entity, dt: f64) {
        // Move X
        entity.apply_velocity_x(dt);
    }

    fn obstruct(&mut self, entity: &mut Entity, side: ObstructionSide, rect: &BoundingBox) {
        match side {
            ObstructionSide::Right => {
                let dx = entity.dx;
                let x = rect.left() - entity.size.width as f64;
                entity.set_x(x, -dx);
            }
            ObstructionSide::Left => {
                let dx = entity.dx;
                let x = rect.right();
                entity.set_x(x, -dx);
            }
            ObstructionSide::Top => {
                let y = rect.bottom();
                entity.set_y(y, 0.);
            }
            ObstructionSide::Bottom => {
                let y = rect.top() - entity.size.height as f64;
                entity.set_y(y, 0.);
            }
        }
    }
}

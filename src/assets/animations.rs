use crate::entity::animation::AnimationEntity;
use crate::physics::motion::Direction;
use crate::utils::create_image_buffer;
use core::fmt;
use fmt::Formatter;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

#[derive(Serialize, Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationName {
    Mario,
    Chance,
}

#[derive(Serialize, Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frame {
    Idle,
    Break,
    Jump,
    // Runs
    Run1,
    Run2,
    Run3,
    // Chance
    Chance1,
    Chance2,
    Chance3,
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "🐙")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize)]
struct FrameDefinition {
    frame: Frame,
    rect: Rectangle,
}

#[derive(Serialize, Deserialize)]
pub(in crate::assets) struct AnimationDefinition {
    pub(crate) name: AnimationName,
    #[serde(alias = "speedRatio")]
    speed_ratio: f64,
    idle: Frame,
    stop: Option<Frame>,
    jump: Option<Frame>,
    frames: Vec<Frame>,
    sprites: Vec<FrameDefinition>,
}

pub struct Animation {
    image: Rc<RefCell<HtmlImageElement>>,
    frames: HashMap<(Frame, Direction), HtmlCanvasElement>,
    idle: Frame,
    stop: Option<Frame>,
    jump: Option<Frame>,
    speed: f64,
    key_frames: Vec<Frame>,
}

impl Animation {
    pub(in crate::assets) fn build(
        definition: &AnimationDefinition,
        image: Rc<RefCell<HtmlImageElement>>,
    ) -> Animation {
        let mut result = Animation::new(image.clone(), definition);

        // Define all sprites
        for d in [Direction::Left, Direction::Stop, Direction::Right].iter() {
            for frame_def in definition.sprites.iter() {
                let r = frame_def.rect;
                result.define(frame_def.frame, *d, r.x, r.y, r.width, r.height);
            }
        }

        result
    }

    fn new(image: Rc<RefCell<HtmlImageElement>>, definition: &AnimationDefinition) -> Self {
        let frames = HashMap::new();
        let idle = definition.idle;
        let stop = definition.stop;
        let jump = definition.jump;
        let speed = definition.speed_ratio;
        let key_frames = definition.frames.clone();

        Self {
            image,
            frames,
            idle,
            stop,
            jump,
            speed,
            key_frames,
        }
    }

    fn define(
        &mut self,
        frame: Frame,
        direction: Direction,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    ) {
        let mirror = direction == Direction::Left;
        let buffer = create_image_buffer(
            self.image.clone(),
            x as f64,
            y as f64,
            width,
            height,
            mirror,
        );
        self.frames.insert((frame, direction), buffer);
    }

    pub(in crate::assets) fn frame(&self, distance: f64) -> Frame {
        let index = (distance.round() / self.speed) as usize % self.key_frames.len();
        self.key_frames[index]
    }

    pub(in crate::assets) fn entity_frame(&self, entity: Rc<RefCell<AnimationEntity>>) -> Frame {
        if self.jump.is_some() && entity.borrow().is_jumping() {
            return self.jump.unwrap();
        }
        let distance = entity.borrow().distance();
        if distance > 0. {
            let (dx, _dy) = entity.borrow().velocity();
            let dir = entity.borrow().direction();
            if self.stop.is_some()
                && ((dx > 0. && dir == Direction::Left) || (dx < 0. && dir == Direction::Right))
            {
                self.stop.unwrap()
            } else {
                self.frame(distance)
            }
        } else {
            self.idle
        }
    }

    pub(in crate::assets) fn draw_frame(
        &self,
        context: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        frame: Frame,
        direction: Direction,
    ) {
        let buffer = self.frames.get(&(frame, direction)).unwrap();
        context
            .draw_image_with_html_canvas_element(&buffer, x as f64, y as f64)
            .unwrap();
    }
}

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
pub struct AnimationDefinition {
    pub(crate) name: AnimationName,
    #[serde(alias = "speedRatio")]
    speed_ratio: f64,
    idle: FrameDefinition,
    frames: Vec<FrameDefinition>,
}

pub struct Animation {
    image: Rc<RefCell<HtmlImageElement>>,
    frames: HashMap<(Frame, Direction), HtmlCanvasElement>,
    idle: Frame,
    speed: f64,
    key_frames: Vec<Frame>,
}

impl Animation {
    pub(crate) fn build(
        definition: &AnimationDefinition,
        image: Rc<RefCell<HtmlImageElement>>,
    ) -> Animation {
        let idle = definition.idle.frame;
        let speed_ratio = definition.speed_ratio;
        let key_frames = definition.frames.iter().map(|fd| fd.frame).collect();

        let mut result = Animation::new(image.clone(), idle, speed_ratio, key_frames);

        // Define all sprites
        for d in [Direction::Left, Direction::Stop, Direction::Right].iter() {
            let r = definition.idle.rect;
            result.define(definition.idle.frame, *d, r.x, r.y, r.width, r.height);
            for frame_def in definition.frames.iter() {
                let r = frame_def.rect;

                result.define(frame_def.frame, *d, r.x, r.y, r.width, r.height);
            }
        }

        result
    }

    fn new(
        image: Rc<RefCell<HtmlImageElement>>,
        idle: Frame,
        speed: f64,
        key_frames: Vec<Frame>,
    ) -> Self {
        let frames = HashMap::new();
        Self {
            image,
            frames,
            idle,
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

    fn route_frame(&self, distance: f64) -> Frame {
        if distance == 0. {
            self.idle
        } else {
            let index = (distance.round() / self.speed) as usize % self.key_frames.len();
            self.key_frames[index]
        }
    }

    pub(crate) fn draw_frame(
        &self,
        context: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        direction: Direction,
        distance: f64,
    ) {
        let frame = self.route_frame(distance);
        let buffer = self.frames.get(&(frame, direction)).unwrap();
        context
            .draw_image_with_html_canvas_element(&buffer, x as f64, y as f64)
            .unwrap();
    }
}

use crate::assets::sprites::{Rectangle, Sprite};
use crate::physics::Direction;
use crate::utils::create_image_buffer;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

#[derive(Deserialize, Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationName {
    #[serde(alias = "run")]
    Run,
    #[serde(alias = "walk")]
    Walk,
    #[serde(alias = "wake")]
    Wake,
    #[serde(alias = "chance")]
    Chance,
    #[serde(alias = "bullet")]
    Bullet,
    #[serde(alias = "coin")]
    Coin,
}

#[derive(Deserialize)]
pub struct AnimationDefinition {
    name: AnimationName,
    #[serde(alias = "frameLen")]
    frame_len: f64,
    frames: Vec<Sprite>,
}

impl AnimationDefinition {
    pub fn name(&self) -> AnimationName {
        self.name
    }
    pub fn frames(&self) -> &[Sprite] {
        &self.frames.as_slice()
    }
}

pub struct Animation {
    name: AnimationName,
    image: Rc<HtmlImageElement>,
    frames: HashMap<(Sprite, Direction), HtmlCanvasElement>,
    frame_len: f64,
    key_frames: Vec<Sprite>,
}

impl Animation {
    pub fn build(
        name: AnimationName,
        animation_def: &AnimationDefinition,
        image: Rc<HtmlImageElement>,
    ) -> Self {
        let frames = HashMap::default();
        let frame_len = animation_def.frame_len;
        let key_frames = animation_def.frames.clone();

        Self {
            name,
            image,
            frames,
            frame_len,
            key_frames,
        }
    }

    pub fn name(&self) -> AnimationName {
        self.name
    }
    pub fn frames(&self) -> &HashMap<(Sprite, Direction), HtmlCanvasElement> {
        &self.frames
    }

    pub fn define(&mut self, frame: Sprite, direction: Direction, rect: &Rectangle) {
        let mirror = direction == Direction::Left;
        let buffer = create_image_buffer(self.image.clone(), rect, mirror);
        self.frames.insert((frame, direction), buffer);
    }

    pub fn frame(&self, distance: f64) -> Sprite {
        // log(&format!("{:?} distance {}", self.name, distance).to_string());
        let index = (distance / self.frame_len) as usize % self.key_frames.len();
        self.key_frames[index]
    }

    pub fn draw_frame(
        &self,
        context: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        frame: Sprite,
        direction: Direction,
    ) {
        let buffer = self.frames.get(&(frame, direction)).unwrap_or_else(|| {
            let found: Vec<Sprite> = self
                .frames
                .keys()
                .filter(|it| it.1 == direction)
                .map(|t| t.0)
                .collect();
            panic!(
                "[{:?}] Frame ({:?},{:?}) not found!, got {:?}",
                self.name, frame, direction, found
            )
        });
        context
            .draw_image_with_html_canvas_element(&buffer, x.floor(), y.floor())
            .unwrap();
    }
}

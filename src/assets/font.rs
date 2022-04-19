use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::assets::load_image;

const CHARS: &str = " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Clone)]
pub struct Font {
    image: Rc<HtmlImageElement>,
    map: HashMap<char, (usize, usize)>,
    size: u32,
}

impl Font {
    pub async fn load() -> Result<Font, JsValue> {
        let image = load_image("assets/images/font.png").await?;
        let row_len = image.width() as usize;
        let mut map = HashMap::default();

        let size = 8;

        for (index, ch) in CHARS.char_indices() {
            let x = index * size % row_len;
            let y = (index * size / row_len) * size;
            map.insert(ch, (x, y));
        }

        let result = Self {
            image,
            map,
            size: size as u32,
        };
        Ok(result)
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn print(&self, context: Rc<CanvasRenderingContext2d>, text: &str, x: f64, y: f64) {
        let size = self.size as f64;
        for (index, ch) in text.char_indices() {
            let (sx, sy) = self
                .map
                .get(&ch)
                .unwrap_or_else(|| panic!("Char '{}' not registered!", ch));
            let sx = *sx as f64;
            let sy = *sy as f64;

            let dx = x + (index as f64 * size) as f64;
            let dy = y;
            context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &self.image.clone(),
                    sx,
                    sy,
                    size,
                    size,
                    dx,
                    dy,
                    size,
                    size,
                )
                .unwrap();
        }
    }
}

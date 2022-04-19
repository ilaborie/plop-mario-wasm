use std::collections::HashMap;
use std::rc::Rc;

use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Request, Response};

use crate::assets::audio::musics::MusicPlayer;
use crate::assets::audio::sounds::AudioBoard;
use crate::assets::config::Configuration;
use crate::assets::font::Font;
use crate::assets::levels::{LevelDefinition, LevelSpec};
use crate::assets::sprites::SpriteSheet;
use crate::utils::window;

pub mod animations;
pub mod audio;
pub mod config;
pub mod font;
pub mod levels;
pub mod patterns;
pub mod sprites;
pub mod tiles;

pub const TILE_SIZE: u32 = 16;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = loader)]
    fn loadImage(path: &str) -> Promise;
}

async fn load_image(path: &str) -> Result<Rc<HtmlImageElement>, JsValue> {
    let promise = loadImage(path);
    let result = JsFuture::from(promise).await?;
    let image = result.dyn_into::<HtmlImageElement>().unwrap();
    let image = Rc::new(image);

    Ok(image)
}

async fn load_json(url: &str) -> Result<JsValue, JsValue> {
    let request = Request::new_with_str(url)?;
    let resp_value = JsFuture::from(window().fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    JsFuture::from(resp.json()?).await
}

/// I choose to have
#[derive(Clone)]
pub struct Assets {
    configuration: Configuration,
    levels: HashMap<String, Rc<LevelSpec>>,
    spite_sheets: HashMap<String, Rc<SpriteSheet>>,
    music_players: HashMap<String, Rc<MusicPlayer>>,
    audio_boards: HashMap<String, Rc<AudioBoard>>,
    font: Rc<Font>,
}

impl Assets {
    pub async fn load() -> Result<Assets, JsValue> {
        let loading_levels = vec!["1-1", "1-2"];
        let loading_sprites = vec!["mario", "luigi", "bullet", "cannon", "goomba", "koopa"];
        let loading_musics = vec!["overworld", "underworld", "silent"];

        // Configuration
        let configuration = Configuration::load().await?;

        // Levels
        let mut levels = HashMap::new();
        for &level_name in loading_levels.iter() {
            let level_def = LevelDefinition::load(level_name).await?;
            let level = level_def.build().await?;
            levels.insert(String::from(level_name), Rc::new(level));
        }

        // Sprites
        let mut spite_sheets = HashMap::new();
        for &sheet in loading_sprites.iter() {
            let spite_sheet = SpriteSheet::load(sheet).await?;
            spite_sheets.insert(String::from(sheet), Rc::new(spite_sheet));
        }

        // Music
        let mut music_players = HashMap::new();
        for &music in loading_musics.iter() {
            let music_player = MusicPlayer::load_music(music, configuration.sounds.music).await?;
            music_players.insert(String::from(music), Rc::new(music_player));
        }

        // Audio
        let mut audio_boards = HashMap::new();
        for &sheet in loading_sprites.iter() {
            if let Ok(audio) = AudioBoard::load_sounds(sheet, configuration.sounds.fx).await {
                audio_boards.insert(String::from(sheet), Rc::new(audio));
            }
        }

        // Font
        let font = Font::load().await?;
        let font = Rc::new(font);

        let result = Self {
            configuration,
            levels,
            spite_sheets,
            audio_boards,
            music_players,
            font,
        };
        Ok(result)
    }

    pub fn configuration(&self) -> Configuration {
        self.configuration.clone()
    }

    pub fn level(&self, name: &str) -> Rc<LevelSpec> {
        self.levels
            .get(name)
            .unwrap_or_else(|| panic!("Level {} not found!", name))
            .clone()
    }

    pub fn spite_sheet(&self, name: &str) -> Rc<SpriteSheet> {
        self.spite_sheets
            .get(name)
            .unwrap_or_else(|| panic!("SpriteSheet {} not found!", name))
            .clone()
    }

    pub fn audio_board(&self, name: &str) -> Option<Rc<AudioBoard>> {
        self.audio_boards.get(name).cloned()
    }

    pub fn music_player(&self, name: &str) -> Rc<MusicPlayer> {
        self.music_players
            .get(name)
            .unwrap_or_else(|| panic!("MusicSheet {} not found!", name))
            .clone()
    }

    pub fn font(&self) -> Rc<Font> {
        self.font.clone()
    }
}

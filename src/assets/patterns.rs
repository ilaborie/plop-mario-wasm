use std::collections::HashMap;
use std::slice::Iter;

use serde::Deserialize;
use wasm_bindgen::JsValue;

use crate::assets::load_json;
use crate::assets::tiles::TilesDefinition;
use crate::utils::log;

#[derive(Deserialize)]
pub struct PatternDefinition {
    tiles: Vec<TilesDefinition>,
}

impl PatternDefinition {
    pub fn tiles(&self) -> Iter<'_, TilesDefinition> {
        self.tiles.iter()
    }
}

pub async fn load_patterns(name: &str) -> Result<HashMap<String, PatternDefinition>, JsValue> {
    log(&format!("Loading patterns {}", name));
    let url = format!("assets/sprites/patterns/{}.json", name);
    let patterns = load_json(url.as_str())
        .await?
        .into_serde::<HashMap<String, PatternDefinition>>()
        .expect("Error during patter loading");

    Ok(patterns)
}

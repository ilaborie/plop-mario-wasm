use crate::audio::player::AudioBoard;
use wasm_bindgen::JsValue;

pub mod player;

pub async fn create_player_audio_board(player: &str) -> Result<AudioBoard, JsValue> {
    let board = AudioBoard::load(player).await?;

    Ok(board)
}

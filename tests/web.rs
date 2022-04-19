//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use plop_mario_wasm::assets::config::Configuration;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn create_system_and_update_twice() {
    let _config = Configuration::load()
        .await
        .unwrap_or_else(|x| panic!("Cannot load the configuration because {x:?}"));

    // let mut sys = System::create(&config, "1-1", "mario")
    //     .await
    //     .unwrap_or_else(|_x| panic!("Cannot create the system"));

    // sys.update(DELTA_TIME);

    // sys.update(DELTA_TIME);
}

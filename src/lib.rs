use async_std::task::block_on;
use nannou::glam::vec3;
use wasm_bindgen::prelude::wasm_bindgen;

use sketch::{run_app, Model};

mod sketch;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let mut p = vec![];
    for x in -20..20 {
        for y in -20..20 {
            p.push(vec3(x as f32, y as f32, 0.0));
        }
    }
    let noise = nannou::noise::OpenSimplex::new();
    let model = Model {
        points: p,
        noise: noise,
    };
    block_on(async {
        run_app(model).await;
    });
}

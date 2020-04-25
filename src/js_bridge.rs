
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/canvas-writer.js")]
extern {
    pub fn init(canvas_id: &str, height: i32, width: i32);
    pub fn clear(color_code: &str);
    #[wasm_bindgen(js_name=drawCircle)]
    pub fn draw_circle(center_x: i32, center_y: i32, radius: i32, color_r: u8, color_g: u8, color_b: u8, color_a: f32);
}
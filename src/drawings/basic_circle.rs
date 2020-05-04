use core::f64::consts::PI;

use wasm_bindgen::prelude::*;
use web_sys::{ console, CanvasRenderingContext2d };

pub fn draw(context: CanvasRenderingContext2d) {
    clear(&context, "#aaaaaa");
    draw_circle(&context, 500.0, 500.0, 200.0, 255, 128, 128, 1.0);
}

pub fn clear(context: &CanvasRenderingContext2d, color_code: &str) {
    context.set_fill_style(&JsValue::from_str(color_code));
    //TODO: The last 2 values are the canvas width and height which probably shouldn't be hard coded long term.
    context.fill_rect(0.0, 0.0, 1000.0, 1000.0);
}

pub fn draw_circle(context: &CanvasRenderingContext2d, center_x: f64, center_y: f64, radius: f64, color_r: u8, color_g: u8, color_b: u8, color_a: f32) {
    context.begin_path();
    context.arc(center_x, center_y, radius, 0.0, PI * 2.0);
    let fill_style = &JsValue::from_str(&format!("rgba({}, {}, {}, {})", color_r, color_g, color_b, color_a));
    console::log_1(fill_style);
    context.set_fill_style(fill_style);
    context.fill();
}
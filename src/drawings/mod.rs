use wasm_bindgen::prelude::*;
use web_sys::{ console, Window };

mod canvas_interface;
mod basic_circle;
mod random_spray;

use canvas_interface::CanvasInterface;

pub fn draw(window: Window) {
    let document = window.document().expect("no global window exist");
    
    let drawing_requested = decode_request(window);
    console::log_1(&JsValue::from_str(&drawing_requested));

    let canvas_interface = CanvasInterface::new(&document, "canvas1", 1000, 1000);

    match drawing_requested.as_str() {
        "basic_circle" => basic_circle::draw(canvas_interface),
        "random_spray" => random_spray::draw(canvas_interface),
        _ => println!("no match")
    }   
}

fn decode_request(window: Window) -> std::string::String {
    let location = window.location();
    let search_str = location.search().expect("no search exists");
    let search_str = search_str.trim_start_matches('?');
    search_str.to_owned()
}
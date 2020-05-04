use crate::js_bridge;
use wasm_bindgen::prelude::*;
use web_sys::{ console, Window };

mod basic_circle;

pub fn draw(window: Window) {
    let drawing_requested = decode_request(window);
    console::log_1(&JsValue::from_str(&drawing_requested));

    js_bridge::init("canvas1", 1000, 1000);
    
    match drawing_requested.as_str() {
        "basic_circle" => basic_circle::draw(),
        _ => println!("no match")
    }   
}

fn decode_request(window: Window) -> std::string::String {
    let location = window.location();
    let search_str = location.search().expect("no search exists");
    let search_str = search_str.trim_start_matches('?');
    search_str.to_owned()
}
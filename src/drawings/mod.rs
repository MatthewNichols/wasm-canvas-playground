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
    let document = window.document().expect("no global window exist");
    let location = document.location().expect("no location exists");
    let raw_search = location.search().expect("no search exists");
    let search_str = raw_search.trim_start_matches("?");
    format!("{}", search_str)
}
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::{ console, Window, Document, Element, 
        HtmlCanvasElement, CanvasRenderingContext2d };

mod basic_circle;

pub fn draw(window: Window) {
    let document = window.document().expect("no global window exist");
    
    let drawing_requested = decode_request(window);
    console::log_1(&JsValue::from_str(&drawing_requested));

    let context = init(&document, "canvas1", 1000, 1000);

    match drawing_requested.as_str() {
        "basic_circle" => basic_circle::draw(context),
        _ => println!("no match")
    }   
}

fn decode_request(window: Window) -> std::string::String {
    let location = window.location();
    let search_str = location.search().expect("no search exists");
    let search_str = search_str.trim_start_matches('?');
    search_str.to_owned()
}

pub fn init(document: &Document, canvas_id: &str, height: u32, width: u32) -> CanvasRenderingContext2d {
    let canvas_el: Element = document.get_element_by_id(canvas_id).expect("Canvas Element does not exist");
    let canvas = canvas_el.dyn_into::<HtmlCanvasElement>().expect("Element is not a Canvas?");
    canvas.set_height(height);
    canvas.set_width(width);

    let context = match canvas.get_context("2d") {
        Ok(c) => c.unwrap().dyn_into::<CanvasRenderingContext2d>(),
        Err(error) => panic!("No Context")
    };

    match context {
        Ok(c) => c,
        Err(error) => panic!("No Context")
    }
    //let context = context.dyn_into::<CanvasRenderingContext2d>()
    //See: https://rustwasm.github.io/wasm-bindgen/examples/paint.html
}
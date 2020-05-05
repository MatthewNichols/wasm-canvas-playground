use core::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ console, Document, Element, 
    HtmlCanvasElement, CanvasRenderingContext2d };

pub struct CanvasInterface {
    context: CanvasRenderingContext2d,
    height: u32,
    width: u32
}

impl CanvasInterface {
    pub fn new(document: &Document, canvas_id: &str, height: u32, width: u32) -> CanvasInterface {
        let canvas_el: Element = document.get_element_by_id(canvas_id).expect("Canvas Element does not exist");
        let canvas = canvas_el.dyn_into::<HtmlCanvasElement>().expect("Element is not a Canvas?");
        canvas.set_height(height);
        canvas.set_width(width);

        let context = match canvas.get_context("2d") {
            Ok(c) => c.unwrap().dyn_into::<CanvasRenderingContext2d>(),
            Err(_error) => panic!("No Context")
        };

        let context = match context {
            Ok(c) => c,
            Err(_error) => panic!("No Context")
        };

        CanvasInterface {
            context: context,
            height: height,
            width: width
        }
    }

    pub fn clear(&self, color_code: &str) {
        self.context.set_fill_style(&JsValue::from_str(color_code));
        self.context.fill_rect(0.0, 0.0, f64::from(self.width), f64::from(self.height));
    }
    
    pub fn draw_circle(&self, center_x: f64, center_y: f64, radius: f64, color_r: u8, color_g: u8, color_b: u8, color_a: f32) {
        self.context.begin_path();
        self.context.arc(center_x, center_y, radius, 0.0, PI * 2.0).ok();
        let fill_style = &JsValue::from_str(&format!("rgba({}, {}, {}, {})", color_r, color_g, color_b, color_a));
        self.context.set_fill_style(fill_style);
        self.context.fill();
    }
}
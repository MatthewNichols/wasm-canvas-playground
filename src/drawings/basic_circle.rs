use crate::js_bridge;

pub fn draw() {
    js_bridge::draw_circle(500, 500, 200, 255, 128, 128, 1.0);
}
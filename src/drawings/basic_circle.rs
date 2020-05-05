use crate::drawings::canvas_interface::CanvasInterface;

pub fn draw(context: CanvasInterface) {
    &context.clear("#aaaaaa");
    &context.draw_circle(500.0, 500.0, 200.0, 255, 128, 128, 1.0);
}
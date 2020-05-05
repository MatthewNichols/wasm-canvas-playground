use js_sys::Math;

use crate::drawings::canvas_interface::CanvasInterface;

pub fn draw(context: CanvasInterface) {
    let mut spray = RandomSpray::new(context, 500., 500., 200.);
    spray.draw();
}

struct RandomSpray {
    context: CanvasInterface,
    colors: Vec<Color>,
    sizes: Vec<CircleSize>,
    circles: Vec<Circle>,
    center: Coordinates,
    initial_max_distance_from_center: f64,
}

impl RandomSpray {
    pub fn new(context: CanvasInterface, center_x: f64, center_y: f64, initial_max_distance_from_center: f64) -> RandomSpray {

        RandomSpray {
            context,
            colors: vec![
                Color::new(20, 151, 162, 1.0),
                Color::new(74, 59, 142, 1.0),
                Color::new(21, 119, 78, 1.0),
            ],
            sizes: vec![
                CircleSize { radius: 17., weight: 10 },
                CircleSize { radius: 12., weight: 10 },
                CircleSize { radius: 8., weight: 12 },
                CircleSize { radius: 5., weight: 12 },
            ],
            circles: vec![],
            center: Coordinates(center_x, center_y),
            initial_max_distance_from_center
        }
    }

    pub fn draw(&mut self) {
        &self.context.clear("#aaaaaa");

        for _ in 0..200 {
            let new_circle = self.get_random_circle();
            self.draw_circle(&new_circle);
            self.circles.push(new_circle);
        }
    }

    fn draw_circle(&self, circle: &Circle) {
        &self.context.draw_circle(circle.center_x, circle.center_y, circle.radius, 
            circle.color.red, circle.color.green, circle.color.blue, circle.color.alpha);
    }

    fn get_random_circle(&self) -> Circle {
        let angle = random_angle();
        let distance_from_center = Math::random() * self.initial_max_distance_from_center;

        let x = (distance_from_center * angle.cos()) + self.center.0;
        let y = (distance_from_center * angle.sin()) + self.center.1;
        Circle::new(x, y, self.sizes[random_index(self.sizes.len())].radius, self.pick_random_color())
    }

    fn pick_random_color(&self) -> Color {
        let index = random_index(self.colors.len());
        self.colors[index]
    } 
}

fn random_angle() -> f64 {
    Math::random() * 2.0 * std::f64::consts::PI 
}

fn random_index(max: usize) -> usize {
    (Math::random() * (max as f64)) as usize
}

#[derive(Clone, Copy)]
pub struct Coordinates(f64, f64);

#[derive(Clone, Copy)]
pub struct Circle {
    center_x: f64, 
    center_y: f64, 
    radius: f64, 
    color: Color,
}

#[derive(Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32
}

impl Circle {
    pub fn new(center_x: f64, center_y: f64, radius: f64, color: Color) -> Circle {
        Circle { center_x, center_y, radius, color }
    }

    /// Returns the distance between the center of this circle and the specified Coordinates
    pub fn distance_to_another_center(&self, other: Coordinates) -> f64 {
        let x_diff = self.center_x - other.0;
        let y_diff = self.center_y - other.1;
        ((x_diff.powf(2.0) + y_diff.powf(2.0)) as f64).sqrt() 
    }

    /// Returns the distance between the edge of this circle and the specified Coordinates
    pub fn distance_from_edge_to_point(&self, other: Coordinates) -> f64 {
        let distance_to_center = self.distance_to_another_center(other) as f64;
        distance_to_center - self.radius
    }
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Color { red, green, blue, alpha }
    }
}

#[derive(Clone, Copy)]
pub struct CircleSize {
    radius: f64,
    weight: i32
}
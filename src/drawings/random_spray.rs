use js_sys::Math;

use crate::drawings::canvas_interface::CanvasInterface;

pub fn draw(context: CanvasInterface) {
    let mut spray = RandomSpray::new(context, 500., 500., 400.);
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

        self.setup_initial_positions();
        self.draw_circles();
        //set up animations next
    }

    pub fn setup_initial_positions(&mut self) {
        for _ in 0..200 {
            let new_circle = self.get_random_circle();
            self.circles.push(new_circle);
        }
    }

    fn draw_circles(&self) {
        for circle in self.circles.iter() {
            self.draw_circle(&circle);
        }
    }

    fn draw_circle(&self, circle: &Circle) {
        &self.context.draw_circle(circle.get_center_x(), circle.get_center_y(), circle.radius, 
            circle.color.red, circle.color.green, circle.color.blue, circle.color.alpha);
    }

    fn get_random_circle(&self) -> Circle {
        let angle = random_angle();
        let distance_from_center = Math::random() * self.initial_max_distance_from_center;

        let x = (distance_from_center * angle.cos()) + self.center.0;
        let y = (distance_from_center * angle.sin()) + self.center.1;
        Circle::new(self.center, angle, distance_from_center, self.sizes[random_index(self.sizes.len())].radius, self.pick_random_color())
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
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: f32
}


#[derive(Clone, Copy)]
pub struct Circle {
    center_ref: Coordinates,
    angle: f64, 
    radial_distance: f64, 
    radius: f64, 
    color: Color,
}

impl Circle {
    pub fn new(center_ref: Coordinates, angle: f64, radial_distance: f64, radius: f64, color: Color) -> Circle {
        Circle { center_ref, angle, radial_distance, radius, color }
    }

    pub fn get_center_x(&self) -> f64 {
        (self.radial_distance * self.angle.cos()) + self.center_ref.0
    }

    pub fn get_center_y(&self) -> f64 {
        (self.radial_distance * self.angle.sin()) + self.center_ref.1
    }

    /// Returns the distance between the center of this circle and the specified Coordinates
    pub fn distance_to_another_center(&self, other: Coordinates) -> f64 {
        let x_diff = self.get_center_x() - other.0;
        let y_diff = self.get_center_y() - other.1;
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
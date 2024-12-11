use std::f64::consts::PI;

pub trait Shape: Send + Sync {
    fn get_area(&self) -> f64;
    fn origin(&self) -> (f64, f64);
    fn set_origin(&mut self, origin: (f64, f64));
}

pub struct Circle {
    pub radius: f64,
    pub origin: (f64, f64),
}

impl Shape for Circle {
    fn get_area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn origin(&self) -> (f64, f64) {
        self.origin
    }

    fn set_origin(&mut self, origin: (f64, f64)) {
        self.origin = origin;
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub origin: (f64, f64),
}

impl Shape for Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn origin(&self) -> (f64, f64) {
        self.origin
    }

    fn set_origin(&mut self, origin: (f64, f64)) {
        self.origin = origin;
    }
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
    pub origin: (f64, f64),
}

impl Shape for Triangle {
    fn get_area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn origin(&self) -> (f64, f64) {
        self.origin
    }

    fn set_origin(&mut self, origin: (f64, f64)) {
        self.origin = origin;
    }
}

use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
    time: f64,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }

    pub fn origin(&self) -> Vector3<f64> {
        self.origin
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }
    pub fn time(&self) -> f64 {
        self.time
    }
}

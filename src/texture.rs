use nalgebra::Vector3;

use crate::perlin::Perlin;

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64>;
}

pub struct SolidColor {
    color_value: Vector3<f64>,
}

impl SolidColor {
    pub fn new(color_value: Vector3<f64>) -> Self {
        SolidColor { color_value }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vector3<f64>) -> Vector3<f64> {
        self.color_value
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: impl Texture + 'static, even: impl Texture + 'static) -> CheckerTexture {
        CheckerTexture {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vector3<f64>) -> Vector3<f64> {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    /// Creates a new [`NoiseTexture`].
    pub fn new(sc: f64) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale: sc,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vector3<f64>) -> Vector3<f64> {
        Vector3::new(1.0, 1.0, 1.0) * 0.5 * (self.noise.noise(&(p * self.scale)) + 1.0)
    }
}

use std::sync::Arc;

use crate::material::Material;

use super::hitable::{HitRecord, Hitable};
use super::ray::Ray;
use nalgebra::Vector3;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: impl Material + 'static) -> Sphere {
        Sphere {
            center,
            radius,
            material: Arc::new(material),
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vector3<f64> = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant: f64 = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(p, normal, t, self.material.clone()));
            };
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(p, normal, t, self.material.clone()));
            }
        }
        None
    }
}

pub struct MovingSphere {
    center0: Vector3<f64>,
    center1: Vector3<f64>,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn new(
        center0: Vector3<f64>,
        center1: Vector3<f64>,
        time0: f64,
        time1: f64,
        radius: f64,
        material: impl Material + 'static,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: Arc::new(material),
        }
    }
    pub fn center(&self, time: f64) -> Vector3<f64> {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0))
                .min(1.0)
                .max(0.0)
                * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vector3<f64> = r.origin() - self.center(r.time());
        let a = r.direction().dot(&r.direction());
        let b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant: f64 = b.powi(2) - a * c;
        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let t = (-b - sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center(r.time())) / self.radius;
                return Some(HitRecord::new(p, normal, t, self.material.clone()));
            };
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center(r.time())) / self.radius;
                return Some(HitRecord::new(p, normal, t, self.material.clone()));
            }
        }
        None
    }
}

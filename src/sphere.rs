use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::{surrounding_box, AABB};

use super::material::Material;

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

    /// 输入单位圆表面的点
    /// 计算球坐标的方位角phi和极角theta
    /// 输出归一化映射u,v
    pub fn get_sphere_uv(p: &Vector3<f64>) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let sin_theta = f64::sin(theta);
        let sin_phi = p.z / sin_theta;
        let cos_phi = p.x / (-sin_theta);
        let mut phi = sin_phi.atan2(cos_phi);
        if phi < 0.0 {
            phi = -phi
        }
        (phi / (2.0 * PI), theta / PI)
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
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(p, normal, t, self.material.clone(), u, v));
            };
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(p, normal, t, self.material.clone(), u, v));
            }
        }
        None
    }
    /// 返回sphere的包围盒
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        let offset = Vector3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - offset, self.center + offset))
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
    /// 输入单位圆表面的点
    /// 计算球坐标的方位角phi和极角theta
    /// 输出归一化映射u,v
    pub fn get_sphere_uv(p: &Vector3<f64>) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let sin_theta = f64::sin(theta);
        let sin_phi = p.z / sin_theta;
        let cos_phi = p.x / (-sin_theta);
        let mut phi = sin_phi.atan2(cos_phi);
        if phi < 0.0 {
            phi = -phi
        }
        (phi / (2.0 * PI), theta / PI)
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
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(p, normal, t, self.material.clone(), u, v));
            };
            let t = (-b + sqrt_discriminant) / a;
            if t < t_max && t > t_min {
                let p = r.at(t);
                let normal = (p - self.center(r.time())) / self.radius;
                let (u, v) = Sphere::get_sphere_uv(&normal);
                return Some(HitRecord::new(p, normal, t, self.material.clone(), u, v));
            }
        }
        None
    }
    /// 返回moving sphere的包围盒
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let offset = Vector3::new(self.radius, self.radius, self.radius);
        let box0 = AABB::new(self.center(time0) - offset, self.center(time0) + offset);
        let box1 = AABB::new(self.center(time1) - offset, self.center(time1) + offset);
        Some(surrounding_box(&box0, &box1))
    }
}

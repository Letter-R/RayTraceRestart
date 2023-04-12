use nalgebra::Vector3;

use super::ray::Ray;

pub struct HitRecord {
    point: Vector3<f64>,  //交点
    normal: Vector3<f64>, //交点法线
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(point: Vector3<f64>, normal: Vector3<f64>, t: f64, front_face: bool) -> HitRecord {
        HitRecord {
            point: point,
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }
    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }
    pub fn point(&self) -> Vector3<f64> {
        self.point
    }
    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitableList(Vec<Box<dyn Hitable>>);

impl HitableList {
    pub fn push(&mut self, hitable: impl Hitable + 'static) {
        self.0.push(Box::new(hitable));
    }

    pub fn new() -> HitableList {
        HitableList(Vec::new())
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;
        for hitable in &self.0 {
            if let Some(HitRecord) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = HitRecord.t;
                hit_anything = Some(HitRecord);
            }
        }
        hit_anything
    }
}

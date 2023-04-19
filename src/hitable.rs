use crate::aabb::{surrounding_box, AABB};

use super::material::Material;
use nalgebra::Vector3;
use rayon::prelude::IntoParallelIterator;
use std::sync::Arc;

use super::ray::Ray;

pub struct HitRecord {
    point: Vector3<f64>,         //交点
    normal: Vector3<f64>,        //交点法线
    time: f64,                   //命中时间
    u: f64,                      //uv坐标系横坐标
    v: f64,                      //uv坐标系纵坐标
    material: Arc<dyn Material>, //命中材质
}

impl HitRecord {
    pub fn new(
        point: Vector3<f64>,
        normal: Vector3<f64>,
        time: f64,
        material: Arc<dyn Material>,
        u: f64,
        v: f64,
    ) -> HitRecord {
        HitRecord {
            point,
            normal,
            time,
            material,
            u,
            v,
        }
    }
    pub fn normal(&self) -> Vector3<f64> {
        self.normal
    }
    pub fn point(&self) -> Vector3<f64> {
        self.point
    }
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn material(&self) -> Arc<dyn Material> {
        self.material.clone()
    }
    pub fn u(&self) -> f64 {
        self.u
    }
    pub fn v(&self) -> f64 {
        self.v
    }
}

pub trait Hitable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
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
            if let Some(hit_record) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.time();
                hit_anything = Some(hit_record);
            }
        }
        hit_anything
    }
    /// 计算所有物体的包围盒
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let mut outbox = self.0[0].bounding_box(time0, time1).unwrap();
        for obj in &self.0 {
            if let Some(objbox) = obj.bounding_box(time0, time1) {
                outbox = surrounding_box(&outbox, &objbox);
            }
        }
        Some(outbox)
    }
}

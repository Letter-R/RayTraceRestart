use std::cmp::min;

use nalgebra::Vector3;

use super::ray::Ray;

// AABB.0是左下角的坐标点
// AABB.1是右上角的坐标点
#[derive(Clone, Copy)]
pub struct AABB(Vector3<f64>, Vector3<f64>);

impl AABB {
    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> AABB {
        AABB(min, max)
    }
    pub fn min(&self) -> Vector3<f64> {
        self.0
    }
    pub fn max(&self) -> Vector3<f64> {
        self.1
    }
    /// 返回是否和box相交
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i];
            let mut t0 = (self.min()[i] - r.origin()[i]) * inv_d;
            let mut t1 = (self.max()[i] - r.origin()[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max < t_min {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vector3::new(
        f64::min(box0.min()[0], box1.min()[0]),
        f64::min(box0.min()[1], box1.min()[1]),
        f64::min(box0.min()[2], box1.min()[2]),
    );
    let max = Vector3::new(
        f64::max(box0.max()[0], box1.max()[0]),
        f64::max(box0.max()[1], box1.max()[1]),
        f64::max(box0.max()[2], box1.max()[2]),
    );
    AABB(min, max)
}

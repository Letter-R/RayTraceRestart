use nalgebra::Vector3;
use rand::Rng;

use crate::ray::Ray;

pub struct Camera {
    origin: Vector3<f64>,
    lower_left_corner: Vector3<f64>,
    horizontal: Vector3<f64>,
    vertical: Vector3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    lens_radius: f64,
}
impl Camera {
    pub fn new(
        lookfrom: Vector3<f64>,
        lookat: Vector3<f64>,
        vup: Vector3<f64>,
        vfov: f64,         // 垂直方向视场角
        aspect_ratio: f64, // 长比高
        aperture: f64,     // 光圈
        focus_dist: f64,   // 对焦距离
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin: Vector3<f64> = lookfrom;
        let horizontal: Vector3<f64> = focus_dist * viewport_width * u;
        let vertical: Vector3<f64> = focus_dist * viewport_height * v;
        let lower_left_corner: Vector3<f64> =
            origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    /// 输入viewport中的相对坐标
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_dsk();
        let origin = self.origin + self.u * rd[0] + self.v * rd[1];
        let direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;
        Ray::new(origin, direction)
    }
}

fn random_in_unit_dsk() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.norm() < 1.0 {
            return p;
        }
    }
}

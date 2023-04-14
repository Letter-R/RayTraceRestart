mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;

use std::sync::Arc;

use camera::Camera;
use hitable::{Hitable, HitableList};
use na::Vector3;
use nalgebra as na;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;

use crate::material::{Dielectric, Lambertian, Metal};
fn ray_color(r: Ray, world: &HitableList, depth: usize) -> Vector3<f64> {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    };
    if let Some(rec) = world.hit(&r, 0.001, f64::MAX) {
        if let Some((sactter, albedo)) = rec.material().scatter(&r, &rec) {
            return albedo.component_mul(&ray_color(sactter, world, depth - 1));
        }
        Vector3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction: Vector3<f64> = r.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        return (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    // 图像参数
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 20;

    //物体
    let mut world = HitableList::new();

    let material_ground = Lambertian::new(Vector3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vector3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0);

    let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, material_left);

    let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, material_right);

    world.push(sphere_ground);
    world.push(sphere_center);
    world.push(sphere_left);
    world.push(sphere_right);

    let material_left = Dielectric::new(1.5);
    let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.4, material_left);
    world.push(sphere_left);

    //相机
    let camera = Camera::new();

    // 计算像素颜色
    let image: Vec<u8> = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|image_y| {
            (0..IMAGE_WIDTH)
                .into_par_iter()
                .flat_map(|image_x| {
                    let mut color: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
                    let mut rng = rand::thread_rng();
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = ((image_x as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0))
                            .min(1.0);
                        let v = ((image_y as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0))
                            .min(1.0);

                        color += ray_color(camera.get_ray(u, v), &world, MAX_DEPTH);
                    }
                    color
                        .iter()
                        .map(|f| (((*f / SAMPLES_PER_PIXEL as f64).sqrt() * 255.99) as u8))
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    //打印
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", "255");

    for p in image.chunks(3) {
        println!("{} {} {}", p[0], p[1], p[2]);
    }
}

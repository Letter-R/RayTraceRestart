mod camera;
mod hitable;
mod material;
mod ray;
mod sphere;

use crate::material::{Dielectric, Lambertian, Metal};
use camera::Camera;
use hitable::{Hitable, HitableList};
use na::Vector3;
use nalgebra as na;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use sphere::{MovingSphere, Sphere};

fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();
    let origin = Vector3::new(4.0, 0.2, 0.0);
    let mut world = HitableList::new();
    world.push(Sphere::new(
        Vector3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Vector3::new(0.5, 0.5, 0.5)),
    ));
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );
            if (center - origin).magnitude() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    world.push(MovingSphere::new(
                        center,
                        center + Vector3::new(0.0, rng.gen_range(0.0..0.5), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Lambertian::new(Vector3::new(
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                            rng.gen::<f64>() * rng.gen::<f64>(),
                        )),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    world.push(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vector3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            0.5 * rng.gen::<f64>(),
                        ),
                    ));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }
    world.push(Sphere::new(
        Vector3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Vector3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Vector3::new(0.4, 0.2, 0.1)),
    ));
    world.push(Sphere::new(
        Vector3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0),
    ));
    world
}

fn ray_color(r: Ray, world: &HitableList, depth: usize) -> Vector3<f64> {
    if depth == 0 {
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
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // 图像参数
    const IMAGE_WIDTH: usize = 500;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as usize;
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const SAMPLES_PER_PIXEL: usize = 50;
    const MAX_DEPTH: usize = 5;

    //物体
    let world = random_scene();

    //相机
    let camera = Camera::new(
        Vector3::new(13.0, 2.0, 3.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        0.0,
        1.0,
    );

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
    println!("255");

    for p in image.chunks(3) {
        println!("{} {} {}", p[0], p[1], p[2]);
    }
}

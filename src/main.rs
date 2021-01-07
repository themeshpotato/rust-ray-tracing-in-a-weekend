mod math;
mod ray;
mod camera;
mod hittable;
mod material;

use math::*;
use ray::*;
use camera::*;
use hittable::*;
use material::*;

use std::io::{self, Write};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - *center;
    let a = ray.direction.length_squared();
    let half_b = Vector3::dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discrimant = half_b * half_b - a * c;

    if discrimant < 0.0 {
        -1.0
    } else {
        (-half_b - discrimant.sqrt() ) / a
    }
}

fn ray_color(ray: &Ray, hittables: &Vec<Hittable>, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = hit_hittables(hittables, ray, 0.001, INFINITY) {
        let target = rec.point + rec.normal + Vector3::random_in_hemisphere(&rec.normal);
        let ray = Ray::new(rec.point, target - rec.point);
        return 0.5 * ray_color(&ray, hittables, depth - 1);
    }

    let normalized_dir = Vector3::normalize(&ray.direction);
    let t = 0.5 * (normalized_dir.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world: Vec<Hittable> = Vec::new();
    world.push(Hittable::Sphere { mat_handle: MaterialHandle(0), center: Point3::new(0.0, 0.0, -1.0), radius: 0.5 });
    world.push(Hittable::Sphere { mat_handle: MaterialHandle(0), center: Point3::new(0.0, -100.5, -1.0), radius: 100.0 });

    // Camera
    let camera = Camera::new(); 

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..=image_height - 1).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + random_double()) / (image_height as f64 - 1.0);

                let r = camera.get_ray(u, v);

                pixel_color += ray_color(&r, &world, max_depth);
            }

            pixel_color.write_color(samples_per_pixel);
        }
    }
    
    eprintln!("Done");
}

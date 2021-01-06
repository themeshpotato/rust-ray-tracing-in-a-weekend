mod math;
mod ray;

use math::*;
use ray::*;
use std::io::{self, Write};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - *center;
    let a = Vector3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vector3::dot(&oc, &ray.direction);
    let c = Vector3::dot(&oc, &oc) - radius * radius;
    let discrimant = b * b - 4.0 * a * c;
    
    (discrimant > 0.0)
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        let normalized_dir = Vector3::normalize(&ray.direction);
        let t = 0.5 * (normalized_dir.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
    let vertical = Vector3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height - 1).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let pixel_color = ray_color(&r);
            pixel_color.write_color();
        }
    }
    
    eprintln!("Done");
}

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

fn ray_color(ray: &Ray, hittables: &Vec<Hittable>, depth: i32, materials: &Vec<Material>) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = hit_hittables(hittables, ray, 0.001, INFINITY) {
        let material = &materials[rec.mat_handle.0 - 1];

        if let Some((scattered, attenuation)) = material.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, hittables, depth - 1, materials);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let normalized_dir = Vector3::normalize(&ray.direction);
    let t = 0.5 * (normalized_dir.y + 1.0);

    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

struct World {
    pub materials: Vec<Material>,
    pub hittables: Vec<Hittable>
}

impl World {
    pub fn register_material(&mut self, material: Material) -> MaterialHandle {
        self.materials.push(material);
        MaterialHandle(self.materials.len())
    }
}

fn random_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let ground_material = world.register_material(Material::Lambertian { albedo: Color::new(0.5, 0.5, 0.5) });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, -1000.0, 0.0), radius: 1000.0 });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                
                if choose_mat  < 0.8 {
                    let albedo = Color::random();
                    let sphere_material = world.register_material(Material::Lambertian { albedo });
                    world.hittables.push(Hittable::Sphere { mat_handle: sphere_material, center, radius: 0.2 });
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0); 
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_material = world.register_material(Material::Metal { albedo, fuzz });
                    world.hittables.push(Hittable::Sphere { mat_handle: sphere_material, center, radius: 0.2 });
                } else {
                    let sphere_material = world.register_material(Material::Dielectric { ir: 1.5 });
                    world.hittables.push(Hittable::Sphere { mat_handle: sphere_material, center, radius: 0.2 });
                }
            }
        }
    }

    world
}

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World

    let mut world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; 
    let aperture = 0.1;

    let camera = Camera::new(&look_from, &look_at, &vup, 20.0, aspect_ratio, aperture, dist_to_focus);

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

                pixel_color += ray_color(&r, &world.hittables, max_depth, &world.materials);
            }

            pixel_color.write_color(samples_per_pixel);
        }
    }
    
    eprintln!("Done");
}

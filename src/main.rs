mod math;
mod ray;
mod camera;
mod hittable;
mod material;
mod aabb;
mod texture;
mod perlin;

//use aabb::*;
use math::*;
use ray::*;
use camera::*;
use hittable::*;
use material::*;
use texture::*;
use perlin::*;

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

fn two_spheres_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let ground_material = world.register_material(Material::Lambertian { albedo: Texture::Checker(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)) });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, -10.0, 0.0), radius: 10.0 });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, 10.0, 0.0), radius: 10.0 });

    world
}

fn two_perlin_spheres_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let ground_material = world.register_material(Material::Lambertian { albedo: Texture::Noise(Perlin::new(), 4.0) });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, -1000.0, 0.0), radius: 1000.0 });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, 2.0, 0.0), radius: 2.0 });

    world
}

fn earth_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let earth_texture = Texture::load_image("textures/earthmap.jpg");
    let earth_material = world.register_material(Material::Lambertian { albedo: earth_texture });
    world.hittables.push(Hittable::Sphere { mat_handle: earth_material, center: Point3::new(0.0, 0.0, 0.0), radius: 2.0 });
    
    world
}

fn random_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let ground_material = world.register_material(Material::Lambertian { albedo: Texture::Checker(Color::new(0.2, 0.5, 0.5), Color::new(0.9, 0.9, 0.9)) });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, -1000.0, 0.0), radius: 1000.0 });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                
                if choose_mat  < 0.8 {
                    let albedo = Color::random();
                    let sphere_material = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(albedo) });
                    let center2 = center + Vector3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.hittables.push(Hittable::MovingSphere { mat_handle: sphere_material, center_0: center, center_1: center2, time_0: 0.0, time_1: 1.0, radius: 0.2 });
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

    let material1 = world.register_material(Material::Dielectric { ir: 1.5 });
    world.hittables.push(Hittable::Sphere { mat_handle: material1, center: Point3::new(0.0, 1.0, 0.0), radius: 1.0 });

    let material2 = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.4, 0.2, 0.1)) });
    world.hittables.push(Hittable::Sphere { mat_handle: material2, center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0 });

    let material3 = world.register_material(Material::Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 });
    world.hittables.push(Hittable::Sphere { mat_handle: material3, center: Point3::new(4.0, 1.0, 0.0), radius: 1.0 });

    world
}

struct PixelChunk {
    pub x: usize,
    pub y: usize
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0; 
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    let thread_count = 10;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; 

    let (world, look_from, look_at, _vfov, aperture) = match 3 {

        0 => {
            let world = Arc::new(random_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.1;

            (world, look_from, look_at, 20.0, aperture)
        },
        1 => {
            let world = Arc::new(two_spheres_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.1;

            (world, look_from, look_at, 20.0, aperture)
        },
        2 => {
            let world = Arc::new(two_perlin_spheres_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.1;

            (world, look_from, look_at, 20.0, aperture)
        },
        3 => {
            let world = Arc::new(earth_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);
            let aperture = 0.1;

            (world, look_from, look_at, 20.0, aperture)
        },
        _ => {
            panic!("Unsupported scene selected")
        }
    };


    let camera = Arc::new(Camera::new(&look_from, &look_at, &vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0));

    // Render
    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    use std::{time, thread};
    use std::sync::{Arc, Mutex};

    let pixel_colors = Arc::new(Mutex::new(vec![vec![Color::new(0.0, 0.0, 0.0); IMAGE_HEIGHT]; IMAGE_WIDTH]));
    let mut remaining_pixel_list: Vec<PixelChunk> = Vec::new();

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            remaining_pixel_list.push(PixelChunk { x, y });
        }
    }

    let mut thread_handles = Vec::new();
    let remaining_pixels = Arc::new(Mutex::new(remaining_pixel_list));
    let pixels_to_process_count = IMAGE_WIDTH * IMAGE_HEIGHT;
    let pixel_count = Arc::new(Mutex::new(pixels_to_process_count));

    eprintln!(
        "Rendering {}x{} ({} pixels) image with {} samples per pixel and a max depth of {}, using {} threads", 
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        IMAGE_WIDTH * IMAGE_HEIGHT,
        samples_per_pixel,
        max_depth,
        thread_count
        );

    use std::time::Instant;
    
    let now = Instant::now();

    for _i in 0..thread_count {
        let pixel_colors = Arc::clone(&pixel_colors);
        let world = Arc::clone(&world);
        let camera = Arc::clone(&camera);
        let remaining_pixels = Arc::clone(&remaining_pixels);
        let pixel_count = Arc::clone(&pixel_count);

        let handle = thread::spawn(move || {
            loop {
                let mut remaining_pixels = remaining_pixels.lock().unwrap();

                if let Some(pixel) = remaining_pixels.pop() {
                    drop(remaining_pixels);

                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = (pixel.x as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.0);
                        let v = (pixel.y as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.0);

                        let r = camera.get_ray(u, v);

                        pixel_color += ray_color(&r, &world.hittables, max_depth, &world.materials);
                    }

                    let mut pixels = pixel_colors.lock().unwrap();
                    pixels[pixel.x][pixel.y] = pixel_color; 

                    let mut pixel_count = pixel_count.lock().unwrap();
                    *pixel_count -= 1;
                } else {
                    break;
                }
            }
        });

        thread_handles.push(handle);
    }
        
    let pixel_count = Arc::clone(&pixel_count);

    let one_second = time::Duration::from_secs(1);

    let handle = thread::spawn(move || {
        loop {
            let count = {
                let pixel_count = pixel_count.lock().unwrap();
                *pixel_count
            };

            eprint!("\rProgress: {:.2}%", 100.0 - (count as f64 / pixels_to_process_count as f64) * 100.0);

            if count > 0 {
                thread::sleep(one_second); // Sleep one second
            } else {
                break;
            }
        }
    });

    thread_handles.push(handle);


    for handle in thread_handles {
        handle.join().unwrap();
    }

    for j in (0..=IMAGE_HEIGHT - 1).rev() {
        for i in 0..IMAGE_WIDTH {
            let colors = pixel_colors.lock().unwrap();
            colors[i][j].write_color(samples_per_pixel);
        }
    }

    eprintln!("Rendering finished in {} seconds", now.elapsed().as_secs());
}

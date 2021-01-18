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

fn ray_color(ray: &Ray, background_color: &Color, hittables: &Vec<Hittable>, depth: i32, materials: &Vec<Material>) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = hit_hittables(hittables, ray, 0.001, INFINITY) {
        let material = &materials[rec.mat_handle.0 - 1];
        
        let emitted = material.emitted(rec.u, rec.v, &rec.point);

        if let Some((scattered, attenuation)) = material.scatter(ray, &rec) {
            return emitted + attenuation * ray_color(&scattered, background_color, hittables, depth - 1, materials);
        } else {
            return emitted;
        }
    } 

    *background_color
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

fn simple_light_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let ground_material = world.register_material(Material::Lambertian { albedo: Texture::Noise(Perlin::new(), 4.0) });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, -1000.0, 0.0), radius: 1000.0 });
    world.hittables.push(Hittable::Sphere { mat_handle: ground_material, center: Point3::new(0.0, 2.0, 0.0), radius: 2.0 });

    let diff_light = world.register_material(Material::DiffuseLight { emit: Texture::SolidColor(Color::new(4.0, 4.0, 4.0)) });
    world.hittables.push(Hittable::XYRect { mat_handle: diff_light, x0: 3.0, x1: 5.0, y0: 1.0, y1: 3.0, k: -2.0 });

    world
}

fn cornell_box_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let red = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.65, 0.05, 0.05)) });
    let white = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.73, 0.73, 0.73)) });
    let green = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.12, 0.45, 0.15)) });
    let light = world.register_material(Material::DiffuseLight { emit: Texture::SolidColor(Color::new(15.0, 15.0, 15.0)) });

    world.hittables.push(Hittable::YZRect { mat_handle: green, y0: 0.0,     y1: 555.0, z0: 0.0,     z1: 555.0, k: 555.0 });
    world.hittables.push(Hittable::YZRect { mat_handle: red,   y0: 0.0,     y1: 555.0, z0: 0.0,     z1: 555.0, k: 0.0 });
    world.hittables.push(Hittable::XZRect { mat_handle: light, x0: 213.0,   x1: 343.0, z0: 227.0,   z1: 332.0, k: 554.0 });
    world.hittables.push(Hittable::XZRect { mat_handle: white, x0: 0.0,     x1: 555.0, z0: 0.0,     z1: 555.0, k: 0.0 });
    world.hittables.push(Hittable::XZRect { mat_handle: white, x0: 0.0,     x1: 555.0, z0: 0.0,     z1: 555.0, k: 555.0 });
    world.hittables.push(Hittable::XYRect { mat_handle: white, x0: 0.0,     x1: 555.0, y0: 0.0,     y1: 555.0, k: 555.0 });

    let box1 = Hittable::new_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white);
    let box1 = Hittable::new_rotate_y(15.0, box1);
    let box1 = Hittable::Translate { offset: Vector3::new(265.0, 0.0, 295.0), ptr: Box::new(box1) };
    world.hittables.push(box1);

    let box2 = Hittable::new_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white);
    let box2 = Hittable::new_rotate_y(-18.0, box2);
    let box2 = Hittable::Translate { offset: Vector3::new(130.0, 0.0, 65.0), ptr: Box::new(box2) };
    world.hittables.push(box2);

    world
}

fn cornell_box_smoke_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let red = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.65, 0.05, 0.05)) });
    let white = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.73, 0.73, 0.73)) });
    let green = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.12, 0.45, 0.15)) });
    let light = world.register_material(Material::DiffuseLight { emit: Texture::SolidColor(Color::new(7.0, 7.0, 7.0)) });

    world.hittables.push(Hittable::YZRect { mat_handle: green, y0: 0.0,     y1: 555.0, z0: 0.0,     z1: 555.0, k: 555.0 });
    world.hittables.push(Hittable::YZRect { mat_handle: red,   y0: 0.0,     y1: 555.0, z0: 0.0,     z1: 555.0, k: 0.0 });
    world.hittables.push(Hittable::XZRect { mat_handle: light, x0: 113.0,   x1: 443.0, z0: 127.0,   z1: 432.0, k: 554.0 });
    world.hittables.push(Hittable::XZRect { mat_handle: white, x0: 0.0,     x1: 555.0, z0: 0.0,     z1: 555.0, k: 0.0 });
    world.hittables.push(Hittable::XZRect { mat_handle: white, x0: 0.0,     x1: 555.0, z0: 0.0,     z1: 555.0, k: 555.0 });
    world.hittables.push(Hittable::XYRect { mat_handle: white, x0: 0.0,     x1: 555.0, y0: 0.0,     y1: 555.0, k: 555.0 });

    let box1_phase = world.register_material(Material::Isotropic { albedo: Texture::SolidColor(Color::new(0.0, 0.0, 0.0)) });
    let box1 = Hittable::new_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 330.0, 165.0), white);
    let box1 = Hittable::new_rotate_y(15.0, box1);
    let box1 = Hittable::Translate { offset: Vector3::new(265.0, 0.0, 295.0), ptr: Box::new(box1) };
    let box1 = Hittable::new_constant_medium(box1, 0.01, box1_phase);
    world.hittables.push(box1);
    
    let box2_phase = world.register_material(Material::Isotropic { albedo: Texture::SolidColor(Color::new(1.0, 1.0, 1.0)) });
    let box2 = Hittable::new_box(Point3::new(0.0, 0.0, 0.0), Point3::new(165.0, 165.0, 165.0), white);
    let box2 = Hittable::new_rotate_y(-18.0, box2);
    let box2 = Hittable::Translate { offset: Vector3::new(130.0, 0.0, 65.0), ptr: Box::new(box2) };
    let box2 = Hittable::new_constant_medium(box2, 0.01, box2_phase);
    world.hittables.push(box2);

    world
}

fn final_scene() -> World {
    let mut world = World {
        materials: Vec::new(),
        hittables: Vec::new()
    };

    let mut boxes1 = Vec::new();
    let ground = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.48, 0.83, 0.53)) });

    const BOXES_PER_SIDE: usize = 20;

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Hittable::new_box(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground));
        }
    }

    world.hittables.push(Hittable::new_bvh_node(&boxes1, 0, boxes1.len(), 0.0, 1.0));

    let light = world.register_material(Material::DiffuseLight { emit: Texture::SolidColor(Color::new(7.0, 7.0, 7.0)) });
    world.hittables.push(Hittable::XZRect { mat_handle: light, x0: 123.0, x1: 423.0, z0: 147.0, z1: 412.0, k: 554.0 });

    let center_1 = Point3::new(400.0, 400.0, 200.0);
    let center_2 = center_1 + Vector3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.7, 0.3, 0.1)) });
    world.hittables.push(Hittable::MovingSphere { mat_handle: moving_sphere_material, center_0: center_1, center_1: center_2, time_0: 0.0, time_1: 1.0, radius: 50.0 });

    let dielectric = world.register_material(Material::Dielectric { ir: 1.5 });
    world.hittables.push(Hittable::Sphere { mat_handle: dielectric, center: Point3::new(260.0, 150.0, 45.0), radius: 50.0 });

    let metal = world.register_material(Material::Metal { albedo: Color::new(0.8, 0.8, 0.9), fuzz: 1.0 });
    world.hittables.push(Hittable::Sphere { mat_handle: metal, center: Point3::new(0.0, 150.0, 145.0), radius: 50.0 });

    let boundary = Hittable::Sphere { mat_handle: dielectric, center: Point3::new(360.0, 150.0, 145.0), radius: 70.0 };
    world.hittables.push(boundary.clone());
    let phase = world.register_material(Material::Isotropic { albedo: Texture::SolidColor(Color::new(0.2, 0.4, 0.9)) });
    world.hittables.push(Hittable::new_constant_medium(boundary, 0.2, phase));

    let boundary = Hittable::Sphere { mat_handle: dielectric, center: Point3::new(0.0, 0.0, 0.0), radius: 5000.0 };
    let phase = world.register_material(Material::Isotropic { albedo: Texture::SolidColor(Color::new(1.0, 1.0, 1.0)) });
    world.hittables.push(Hittable::new_constant_medium(boundary, 0.0001, phase));

    let emat = world.register_material(Material::Lambertian { albedo: Texture::load_image("textures/earthmap.jpg") });
    world.hittables.push(Hittable::Sphere { mat_handle: emat, center: Point3::new(400.0, 200.0, 400.0), radius: 100.0 });
    let pertext = world.register_material(Material::Lambertian { albedo: Texture::Noise(Perlin::new(), 0.1) });
    world.hittables.push(Hittable::Sphere { mat_handle: pertext, center: Point3::new(220.0, 280.0, 300.0), radius: 80.0 });

    let mut boxes2 = Vec::new();
    let white = world.register_material(Material::Lambertian { albedo: Texture::SolidColor(Color::new(0.73, 0.73, 0.73)) });
    let ns = 1000;

    for j in 0..ns {
        boxes2.push(Hittable::Sphere { mat_handle: white, center: Point3::random_range(0.0, 165.0), radius: 10.0 });
    }

    world.hittables.push(Hittable::Translate {
                    offset: Vector3::new(-100.0, 270.0, 395.0),
                    ptr: Box::new(Hittable::new_rotate_y(15.0, Hittable::new_bvh_node(&boxes2, 0, boxes2.len(), 0.0, 1.0)))
                }
    );

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

struct Scene {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub background: Color,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vfov: f64,
    pub world: std::sync::Arc<World>
}

fn main() {
    // Image
    let thread_count = 10; // Find maximum thread count for CPU
    let max_depth = 50;
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0; 

    let scene = match 7 {

        0 => {
            let world = Arc::new(random_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);

            Scene {
                aspect_ratio: 16.0 / 9.0,
                image_width: 400,
                samples_per_pixel: 100,
                background: Color::new(0.7, 0.8, 1.0),
                look_from,
                look_at,
                vfov: 20.0,
                world
            }
        },
        1 => {
            let world = Arc::new(two_spheres_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);

            Scene {
                aspect_ratio: 16.0 / 9.0,
                image_width: 400,
                samples_per_pixel: 100,
                background: Color::new(0.7, 0.8, 1.0),
                look_from,
                look_at,
                vfov: 20.0,
                world
            }
        },
        2 => {
            let world = Arc::new(two_perlin_spheres_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);

            Scene {
                aspect_ratio: 16.0 / 9.0,
                image_width: 400,
                samples_per_pixel: 100,
                background: Color::new(0.7, 0.8, 1.0),
                look_from,
                look_at,
                vfov: 20.0,
                world
            }
        },
        3 => {
            let world = Arc::new(earth_scene());

            // Camera
            let look_from = Point3::new(13.0, 2.0, 3.0);
            let look_at = Point3::new(0.0, 0.0, 0.0);

            Scene {
                aspect_ratio: 16.0 / 9.0,
                image_width: 400,
                samples_per_pixel: 100,
                background: Color::new(0.7, 0.8, 1.0),
                look_from,
                look_at,
                vfov: 20.0,
                world
            }
        },
        4 => {
            let world = Arc::new(simple_light_scene());

            // Camera
            let look_from = Point3::new(26.0, 3.0, 6.0);
            let look_at = Point3::new(0.0, 2.0, 0.0);

            Scene {
                aspect_ratio: 16.0 / 9.0,
                image_width: 400,
                samples_per_pixel: 100,
                background: Color::new(0.0, 0.0, 0.0),
                look_from,
                look_at,
                vfov: 20.0,
                world
            }
        },
        5 => {
            let world = Arc::new(cornell_box_scene());

            // Camera
            let look_from = Point3::new(278.0, 278.0, -800.0);
            let look_at = Point3::new(278.0, 278.0, 0.0);

            Scene {
                aspect_ratio: 1.0,
                image_width: 600,
                samples_per_pixel: 200,
                background: Color::new(0.0, 0.0, 0.0),
                look_from,
                look_at,
                vfov: 40.0,
                world
            }
        },
        6 => {
            let world = Arc::new(cornell_box_smoke_scene());

            // Camera
            let look_from = Point3::new(278.0, 278.0, -800.0);
            let look_at = Point3::new(278.0, 278.0, 0.0);

            Scene {
                aspect_ratio: 1.0,
                image_width: 600,
                samples_per_pixel: 40,
                background: Color::new(0.0, 0.0, 0.0),
                look_from,
                look_at,
                vfov: 40.0,
                world
            }
        },
        7 => {
            let world = Arc::new(final_scene());

            // Camera
            let look_from = Point3::new(478.0, 278.0, -600.0);
            let look_at = Point3::new(278.0, 278.0, 0.0);

            Scene {
                aspect_ratio: 1.0,
                image_width: 800,
                samples_per_pixel: 2000,
                background: Color::new(0.0, 0.0, 0.0),
                look_from,
                look_at,
                vfov: 40.0,
                world
            }
        },

        _ => {
            panic!("Unsupported scene selected")
        }
    };
    
    let image_width = scene.image_width;
    let image_height = (scene.image_width as f64 * scene.aspect_ratio) as usize;

    let camera = Arc::new(Camera::new(&scene.look_from, &scene.look_at, &vup, scene.vfov, scene.aspect_ratio, 0.1, dist_to_focus, 0.0, 1.0));

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    use std::{time, thread};
    use std::sync::{Arc, Mutex};

    let pixel_colors = Arc::new(Mutex::new(vec![vec![Color::new(0.0, 0.0, 0.0); image_height]; image_width]));
    let mut thread_handles = Vec::new();
    let mut thread_receivers = Vec::new();
    let pixels_to_process_count = image_width * image_height;

    eprintln!(
        "Rendering {}x{} ({} pixels) image with {} samples per pixel and a max depth of {}, using {} threads", 
        image_width,
        image_height,
        image_width * image_height,
        scene.samples_per_pixel,
        max_depth,
        thread_count
        );

    use std::time::Instant;
    use std::sync::mpsc;
    
    let now = Instant::now();

    for i in 0..thread_count {
        let pixel_colors = Arc::clone(&pixel_colors);
        let world = scene.world.clone();
        let camera = Arc::clone(&camera);
        let samples_per_pixel = scene.samples_per_pixel;
        let background = scene.background;

        let (tx, rx) = mpsc::channel();
        thread_receivers.push(rx);

        let handle = thread::spawn(move || {
            let mut local_pixel_colors = vec![vec![Color::new(0.0, 0.0, 0.0); image_height]; image_width];
            let mut pixels_left = pixels_to_process_count;
            let mut last_change = 0;

            for x in 0..image_width {
                for y in 0..image_height {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _s in 0..samples_per_pixel / thread_count {
                        let u = (x as f64 + random_double()) / (image_width as f64 - 1.0);
                        let v = (y as f64 + random_double()) / (image_height as f64 - 1.0);

                        let r = camera.get_ray(u, v);

                        pixel_color += ray_color(&r, &background, &world.hittables, max_depth, &world.materials);
                    }

                    local_pixel_colors[x][y] = pixel_color;
                    pixels_left -= 1;
                    last_change += 1;

                    if last_change == 50 {
                        match tx.send((i, pixels_left)) {
                            Ok(_) => {
                            },
                            Err(msg) => {
                                eprintln!("{:?}", msg);
                            }
                        }
                        last_change = 0;
                    }
                }
            }

            let mut pixels = pixel_colors.lock().unwrap();
            for x in 0..image_width {
                for y in 0..image_height {
                    pixels[x][y] += local_pixel_colors[x][y];
                }
            }
        });

        thread_handles.push(handle);
    }
        
    let one_second = time::Duration::from_secs(1);

    let mut thread_pixel_counts = vec![pixels_to_process_count; thread_count];

    let handle = thread::spawn(move || {
        loop {
            for r in &thread_receivers {
                if let Ok((t_index, count)) = r.try_recv() {
                    thread_pixel_counts[t_index] = count;
                }
            }
            
            eprint!("\rProgress: {:?}", &thread_pixel_counts);
            //eprint!("\rProgress: {:.2}%", 100.0 - (count as f64 / pixels_to_process_count as f64) * 100.0);
            
            let mut done = true;
            for counts in &thread_pixel_counts {
                if *counts > 0 {
                    done = false;
                    break;
                }
            }

            if !done {
                //thread::sleep(0.1); // Sleep one second
            } else {
                break;
            }
        }
    });

    //thread_handles.push(handle);


    for handle in thread_handles {
        handle.join().unwrap();
    }

    for j in (0..=image_height - 1).rev() {
        for i in 0..image_width {
            let colors = pixel_colors.lock().unwrap();
            colors[i][j].write_color(scene.samples_per_pixel as i32);
        }
    }

    eprintln!("Rendering finished in {} seconds", now.elapsed().as_secs());
}

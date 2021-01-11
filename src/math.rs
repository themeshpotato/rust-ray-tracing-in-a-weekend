use std::fmt;
use std::ops;
use rand::{thread_rng, Rng};

pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub type Point3 = Vector3;
pub type Color = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {
            x,
            y,
            z
        }
    }

    pub fn as_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn random() -> Vector3 {
        Vector3 {
            x: random_double(),
            y: random_double(),
            z: random_double()
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vector3 {
        Vector3 {
            x: random_double_range(min, max),
            y: random_double_range(min, max),
            z: random_double_range(min, max)
        }
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let p = Vector3::random_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vector3) -> Vector3 {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Vector3::dot(&in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vector3 {
        loop {
            let p = Vector3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vector3 {
        Self::normalize(&Self::random_in_unit_sphere())
    }

    pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z 
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
        Vector3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x
            )
    }

    pub fn normalize(v: &Vector3) -> Vector3 {
        *v / v.length()
    }

    pub fn reflect(v: &Vector3, n: &Vector3) -> Vector3 {
        *v - 2.0 * Vector3::dot(v, n) * n
    }

    pub fn refract(uv: &Vector3, n: &Vector3, etai_over_etat: f64) -> Vector3 {
        let cos_theta = Vector3::dot(&-uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * n);
        let r_out_perp_length = r_out_perp.length_squared();
        let r_out_parallel = -((1.0 - r_out_perp_length).abs()).sqrt() * n;

        r_out_perp + r_out_parallel
    }

    pub fn write_color(&self, samples_per_pixel: i32) { 
        let scale = 1.0 / samples_per_pixel as f64;

        // Divice the color by the number of samples and gamme-correct for gamme=2.0
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        let ir = (256.0 * clamp(r, 0.0, 0.999)) as i32;
        let ig = (256.0 * clamp(g, 0.0, 0.999)) as i32;
        let ib = (256.0 * clamp(b, 0.0, 0.999)) as i32;

        println!("{} {} {}", ir, ig, ib);
    }

    pub fn near_zero(&self) -> bool {
        const s: f64 = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

// This formats the vector as a color
impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vector3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vector3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector3::new(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

impl ops::Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(
            -self.x,
            -self.y,
            -self.z
        )
    }
}

impl ops::Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vector3::new(
            self.x * rhs.x,
            self.y * rhs.y,
            self.z * rhs.z
        )
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vector3::new(
            self.x * rhs,
            self.y * rhs,
            self.z * rhs
        )
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}

impl ops::Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3::new(
            rhs.x * self,
            rhs.y * self,
            rhs.z * self
        )
    }
}


impl ops::Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        (1.0 / rhs) * self
    }
}

pub fn random_double() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
   let mut rng = thread_rng();
   rng.gen_range(min..=max)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min }
    else if x > max { max }
    else { x }
}

use crate::math::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranvec: Vec<Vector3>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranvec: Vec<Vector3> = vec![Vector3::new(0.0, 0.0, 0.0); POINT_COUNT];
        
        for i in 0..POINT_COUNT {
            ranvec[i] = Vector3::normalize(&Vector3::random_range(-1.0, 1.0));
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Perlin {
            ranvec,
            perm_x,
            perm_y,
            perm_z
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let x = p.x.floor();
        let y = p.y.floor();
        let z = p.z.floor();

        let u = p.x - x;
        let v = p.y - y;
        let w = p.z - z;

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = x as i32; 
        let j = y as i32; 
        let k = z as i32;

        let mut c = [[[Vector3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = ((i + di) as i32 & 255) as usize;
                    let y = ((j + dj) as i32 & 255) as usize;
                    let z = ((k + dk) as i32 & 255) as usize;

                    c[di as usize][dj as usize][dk as usize] = self.ranvec[
                        (self.perm_x[x] ^
                        self.perm_y[y] ^
                        self.perm_z[z]) as usize
                    ];
                }
            }
        }
        
        Self::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &[[[Vector3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let val = c[i][j][k];
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let weight_v = Vector3::new(u - i, v - j, w - k);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu)) *
                             (j * vv + (1.0 - j) * (1.0 - vv)) *
                             (k * ww + (1.0 - k) * (1.0 - ww)) *
                             Vector3::dot(&val, &weight_v);
                }
            }
        }

        accum 
    }
    
    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
   
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = vec![0; POINT_COUNT]; 

        for i in 0..POINT_COUNT {
            p[i] = i as i32;
        }

        Self::permute(&mut p, POINT_COUNT);
        
        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n).rev() {
            let target = random_int_range(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = target as i32;
            p[target] = tmp;
        }
    }
}

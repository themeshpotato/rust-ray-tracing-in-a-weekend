use crate::math::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranfloat: Vec<f64>,
    pub perm_x: Vec<i32>,
    pub perm_y: Vec<i32>,
    pub perm_z: Vec<i32>
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat: Vec<f64> = vec![0.0; POINT_COUNT];
        
        for i in 0..POINT_COUNT {
            ranfloat[i] = random_double();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Perlin {
            ranfloat,
            perm_x,
            perm_y,
            perm_z
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        let index = (self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize;

        self.ranfloat[index]
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

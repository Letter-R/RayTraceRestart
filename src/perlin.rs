use nalgebra::Vector3;
use rand::Rng;

// point_count=256
pub struct Perlin {
    ranfloat: Vec<Vector3<f64>>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranfloat = Vec::with_capacity(256);
        for _ in 0..256 {
            ranfloat.push(
                Vector3::new(
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                    rng.gen_range(-1.0..1.0),
                )
                .normalize(),
            );
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vector3<f64>) -> f64 {
        let u = p.x - f64::floor(p.x);
        let v = p.y - f64::floor(p.y);
        let w = p.z - f64::floor(p.z);

        // 注意！这里不能直接转为usize，会导致负数范围都是0
        let i = f64::floor(p.x) as i64;
        let j = f64::floor(p.y) as i64;
        let k = f64::floor(p.z) as i64;

        let mut c: [[[Vector3<f64>; 2]; 2]; 2] = [[[Vector3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]]
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }

    // 生成一个Vec<usize>
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i);
        }
        Self::permute(&mut p, 256);
        p
    }

    // 交换，打乱P
    // 输入使用切片，可以防止改变大小
    fn permute(p: &mut [usize], n: usize) {
        let mut rng = rand::thread_rng();
        for i in (0..n).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }

    #[allow(dead_code)]
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }

    fn perlin_interp(c: [[[Vector3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Hermite cubic
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v: Vector3<f64> =
                        Vector3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }
        accum
    }

    pub fn turb(&self, p: &Vector3<f64>, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for i in 0..depth {
            accum += &weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }
}

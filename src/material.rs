use super::hitable::HitRecord;
use super::ray::Ray;
use nalgebra::Vector3;
use rand::Rng;

/// 生成一个长度小于1的随机向量
pub fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let unit = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

/// 反射
fn reflect(v: &Vector3<f64>, n: &Vector3<f64>) -> Vector3<f64> {
    v - 2.0 * v.dot(n) * n
}

/// 折射
fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_over_nt: f64) -> Option<Vector3<f64>> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        // 发生折射
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        // 全反射
        None
    }
}

pub trait Material: Sync + Send {
    /// 输入：入射光、命中信息、衰减率
    /// 输出：出射光、衰减率
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        //attenuation: Vector3<f64>,
    ) -> Option<(Ray, Vector3<f64>)>;
}

/// Lambertian材质
/// albedo：衰减率
pub struct Lambertian {
    albedo: Vector3<f64>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f64>) -> Lambertian {
        Lambertian { albedo }
    }
}

/// 对于Lambertian，发射光与输入无关，方向为法线附近
impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        hit_record: &HitRecord,
        //attenuation: Vector3<f64>,
    ) -> Option<(Ray, Vector3<f64>)> {
        let scatter_direction = hit_record.normal() + random_in_unit_sphere();
        let sactter = Ray::new(hit_record.point(), scatter_direction);
        Some((sactter, self.albedo))
    }
}

/// Metal材质
/// albedo：衰减率
pub struct Metal {
    albedo: Vector3<f64>,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        //attenuation: Vector3<f64>,
    ) -> Option<(Ray, Vector3<f64>)> {
        let reflected_direction = reflect(&r_in.direction().normalize(), &hit_record.normal())
            + self.fuzz * random_in_unit_sphere();
        if reflected_direction.dot(&hit_record.normal()) > 0.0 {
            //加了模糊反射后在表面外
            let sactter: Ray = Ray::new(hit_record.point(), reflected_direction);
            Some((sactter, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Vector3<f64>)> {
        let attenuation: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0); // 无损失
        let (outward_normal, ni_over_nt, cos_theta) =
            if r_in.direction().dot(&hit_record.normal()) > 0.0 {
                // 法线与入射光同侧，计算物体内表面的折射参数
                let outward_normal = -1.0 * hit_record.normal();
                let ni_over_nt = self.refraction_index;
                let cos_theta = ni_over_nt * r_in.direction().normalize().dot(&hit_record.normal());
                (outward_normal, ni_over_nt, cos_theta)
            } else {
                // 法线与入射光起点同侧，计算物体外表面的折射参数
                let outward_normal = hit_record.normal();
                let ni_over_nt = 1.0 / self.refraction_index;
                let cos_theta = -1.0 * r_in.direction().normalize().dot(&hit_record.normal());
                (outward_normal, ni_over_nt, cos_theta)
            };

        // 优先折射
        if let Some(refracted) = refract(&r_in.direction(), &outward_normal, ni_over_nt) {
            let reflectance_in = //正入射的反射率
                ((ni_over_nt - 1.0) / (ni_over_nt + 1.0)).powi(2);
            let reflectance_out = // 反射率
            reflectance_in + (1.0 - reflectance_in) * (1.0 - cos_theta).powi(5);

            if rand::thread_rng().gen::<f64>() > reflectance_out {
                return Some((Ray::new(hit_record.point(), refracted), attenuation));
            }
        }

        //不折射时，反射
        let scattered = Ray::new(
            hit_record.point(),
            reflect(&r_in.direction().normalize(), &hit_record.normal()),
        );

        Some((scattered, attenuation))
    }
}

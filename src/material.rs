use crate::{dot, util::random_double, Color, HitRecord, Ray, Vec3, COLOR_WHITE};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Color,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        // eprintln!("scatter Lambertian");
        let random_scatter_direction = rec.normal + Vec3::new_random_unit_vector();
        let scatter_direction = if random_scatter_direction.near_zero() {
            rec.normal
        } else {
            random_scatter_direction
        };

        Some(ScatterResult {
            scattered: Ray::new(rec.p, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        // eprintln!("scatter Metal");
        let reflected = reflect(r.dir.unit_vector(), rec.normal);

        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::new_random_in_unit_sphere() * self.fuzz,
        );
        if dot(scattered.dir, rec.normal) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (n * 2.0 * dot(v, n))
}

pub struct Dialectric {
    index_of_refraction: f64,
}

impl Dialectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dialectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = r_in.dir.unit_vector();

        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);
        Some(ScatterResult {
            scattered,
            attenuation: COLOR_WHITE,
        })
    }
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = ((-1.0) * uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length().powi(2)).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_reflect() {
        let v = Vec3::new(1.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(reflect(v, normal), Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_refract() {
        let v = Vec3::new(1.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(
            refract(v, normal, 1.5),
            Vec3::new(1.5, -1.118033988749895, 0.0)
        );
    }
}

use libc::rand;
use rand::Rng;

use super::{hittable::Hit, ray::Ray, utils::random_f32, vec3::Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }
        let scattered = Ray::new(hit.p, scatter_direction);
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
    }
}

#[derive(Clone)]
pub struct Dialectric {
    ir: f32,
}

impl Dialectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

fn schlick(cosine: f32, ir: f32) -> f32 {
    let r0 = ((1.0 - ir) / (1.0 + ir)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(ray.direction());

        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = random_f32() < schlick(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            Vec3::reflect(unit_direction, hit.normal)
        } else {
            Vec3::refract(unit_direction, hit.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit.p, direction);

        Some((Vec3::new(1.0, 1.0, 1.0), scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Self {
        Self {
            albedo,
            fuzz: if f < 1. { f } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let mut reflected = Vec3::reflect(ray.direction(), hit.normal);
        if self.fuzz > 0.0 {
            reflected += Vec3::random_in_unit_sphere() * self.fuzz
        };
        if Vec3::dot(reflected, hit.normal) > 0.0 {
            let scattered = Ray::new(hit.p, reflected);
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

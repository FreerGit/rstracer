use super::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

pub struct Hit<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'a dyn Material,
}

impl<'a> Hit<'a> {
    pub fn get_face_normal(r: Ray, outward_normal: Vec3) -> Vec3 {
        match Vec3::dot(r.direction(), outward_normal) < 0. {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}

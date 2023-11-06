use super::{interval::Interval, material::Material, ray::Ray, vec3::Vec3};

pub struct Hit<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub material: &'a dyn Material,
    pub front_face: bool,
}

impl<'a> Hit<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}

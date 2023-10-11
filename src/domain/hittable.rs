use super::{ray::Ray, vec3::Vec3};

#[derive(Default, Copy, Clone, Debug)]
pub struct Hit {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl Hit {
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) -> () {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.;
        match self.front_face {
            true => self.normal = outward_normal,
            false => self.normal = -outward_normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, tmin: f32, tmax: f32, hit_record: &mut Hit) -> bool;
}

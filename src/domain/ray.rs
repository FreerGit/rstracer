use super::vec3::Vec3;

pub struct Ray {
    u: Vec3,
    v: Vec3,
}

impl Ray {
    pub fn new(u: Vec3, v: Vec3) -> Ray {
        Ray { u, v }
    }

    pub fn origin(self) -> Vec3 {
        self.u
    }

    pub fn direction(self) -> Vec3 {
        self.v
    }

    pub fn at(self, t: f32) -> Vec3 {
        self.u + self.v * t
    }
}

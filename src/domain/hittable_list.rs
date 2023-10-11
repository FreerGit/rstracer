use super::{
    hittable::{Hit, Hittable},
    vec3::Vec3,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) -> () {
        Vec::clear(&mut self.objects)
    }

    pub fn push(&mut self, obj: Box<dyn Hittable>) -> () {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: super::ray::Ray, tmin: f32, tmax: f32, hit_record: &mut Hit) -> bool {
        let mut temp_hit = Hit::default();
        let mut hit_anything = false;
        let mut closest_so_far = tmax;
        for object in &self.objects {
            if object.hit(r, tmin, closest_so_far, &mut temp_hit) {
                hit_anything = true;
                closest_so_far = temp_hit.t;
                *hit_record = temp_hit;
            }
        }
        return hit_anything;
    }
}

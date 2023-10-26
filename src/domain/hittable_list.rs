use super::{
    hittable::{Hit, Hittable},
    interval::Interval,
    ray::Ray,
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
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
        let mut closest_so_far = interval.max;
        let mut hit_anything: Option<Hit> = None;
        for o in self.objects.iter() {
            if let Some(hit) = o.hit(ray, &Interval::new(interval.min, closest_so_far)) {
                closest_so_far = 0.;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}

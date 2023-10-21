use rstracer::domain::camera::Camera;
use rstracer::domain::hittable_list::HittableList;
use rstracer::domain::sphere::Sphere;
use rstracer::domain::vec3::Vec3;

fn main() {
    // World
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let cam = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);

    Camera::render(&cam, &world);
}

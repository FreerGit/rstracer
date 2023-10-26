use rstracer::domain::camera::Camera;
use rstracer::domain::hittable_list::HittableList;
use rstracer::domain::material::{Lambertian, Metal};
use rstracer::domain::sphere::Sphere;
use rstracer::domain::vec3::Vec3;

fn main() {
    // World
    let mut world = HittableList::new();

    let mat_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let mat_center = Lambertian::new(Vec3::new(0.7, 0.3, 0.3));
    let mat_left = Metal::new(Vec3::new(0.8, 0.8, 0.8));
    let mat_right = Metal::new(Vec3::new(0.8, 0.6, 0.2));

    world.push(Box::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        mat_center,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        mat_left,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        mat_right,
    )));
    world.push(Box::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        mat_ground,
    )));

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let sample_per_pixel = 100;
    let max_depth = 50;
    let cam = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);

    Camera::render(&cam, &world);
}

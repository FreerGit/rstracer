use rayon::vec;
use rstracer::domain::color::write_color;
use rstracer::domain::constants::INFINITY;
use rstracer::domain::hittable::{Hit, Hittable};
use rstracer::domain::hittable_list::HittableList;
use rstracer::domain::sphere::Sphere;
use rstracer::domain::{ray::Ray, vec3::Vec3};
use std::fs::File;
use std::io::Write;

fn ray_color(r: Ray, world: &dyn Hittable) -> Vec3 {
    let mut hit = Hit::default();
    if world.hit(r, 0., INFINITY, &mut hit) {
        return (hit.normal + Vec3::new(1., 1., 1.)) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1., 1., 1.) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.) * a
}

fn main() {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = (image_width as f32) / aspect_ratio;

    // Camera
    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * ((image_width as f32) / image_height);
    let camera_center = Vec3::new(0., 0., 0.);

    // World
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    // Calc the vectors across horizontal and vertical edges
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calc the delta vectors from u and v
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / image_height;

    // Calc the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // setup P3 file
    let mut ppm_file = String::new();
    let setup_ppm = format!("{}{} {}{}", "P3\n", image_width, image_height, "\n255\n");
    ppm_file.push_str(&setup_ppm.to_string());

    // Render
    for i in 0..(image_height as i32) {
        for j in 0..(image_width as i32) {
            let pixel_center = pixel00_loc + (pixel_delta_u * j) + (pixel_delta_v * i);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(ray, &world);
            ppm_file.push_str(&format!("{}", write_color(pixel_color)).to_string());
        }
    }

    let mut file = File::create("./images/test.ppm").expect("unable to read file");
    file.write(ppm_file.as_bytes())
        .expect("unable to write to file");
}

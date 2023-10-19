use std::{fs::File, io::Write};

use super::{
    color::write_color,
    constants::INFINITY,
    hittable::{Hit, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn render(image_width: i32, aspect_ratio: f32, world: &dyn Hittable) -> () {
        let init = Self::initialize(image_width, aspect_ratio);

        // setup P3 file
        let mut ppm_file = String::new();
        let setup_ppm = format!(
            "{}{} {}{}",
            "P3\n", init.image_width, init.image_height, "\n255\n"
        );
        ppm_file.push_str(&setup_ppm.to_string());

        // Render
        for i in 0..(init.image_height as i32) {
            for j in 0..(init.image_width as i32) {
                let pixel_center =
                    init.pixel00_loc + (init.pixel_delta_u * j) + (init.pixel_delta_v * i);
                let ray_direction = pixel_center - init.center;
                let ray = Ray::new(init.center, ray_direction);
                let pixel_color = Self::ray_color(ray, world);
                ppm_file.push_str(&format!("{}", write_color(pixel_color)).to_string());
            }
        }

        let mut file = File::create("./images/test.ppm").expect("unable to read file");
        file.write(ppm_file.as_bytes())
            .expect("unable to write to file");
    }

    fn initialize(width: i32, aspect_ratio: f32) -> Self {
        let image_width = width;
        let aspect_ratio = aspect_ratio;
        let image_height = image_width / aspect_ratio as i32;
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * ((image_width) / image_height) as f32;
        let camera_center = Vec3::new(0., 0., 0.);

        // Calc the vectors across horizontal and vertical edges
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calc the delta vectors from u and v
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calc the location of the upper left pixel
        let viewport_upper_left =
            camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        Self {
            image_width,
            aspect_ratio,
            image_height,
            center: camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }
    fn ray_color(ray: Ray, world: &dyn Hittable) -> Vec3 {
        let mut hit = Hit::default();
        if world.hit(ray, Interval::new(0., INFINITY), &mut hit) {
            return (hit.normal + Vec3::new(1., 1., 1.)) * 0.5;
        }

        let unit_direction = Vec3::unit_vector(ray.direction());
        let a = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1., 1., 1.) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.) * a
    }
}

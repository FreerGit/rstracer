use super::{interval::Interval, vec3::Vec3};

pub fn write_color(pixel_color: Vec3, samples_per_pixel: i32) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // divide by number of samples
    let scale = 1. / samples_per_pixel as f32;

    let r = r * scale;
    let g = g * scale;
    let b = b * scale;

    let intensity = Interval::new(0., 0.999);

    let mut str = String::from("");
    str.push_str(&(format!("{}", (256. * intensity.clamp(r)) as i32) + " "));
    str.push_str(&(format!("{}", (256. * intensity.clamp(g)) as i32) + " "));
    str.push_str(&(format!("{}", (256. * intensity.clamp(b)) as i32) + "\n"));
    str
}

use super::vec3::Vec3;

pub fn write_color(pixel_color: Vec3) -> String {
    let mut str = String::from("");
    str.push_str(&(format!("{}", (pixel_color.x() * 255.999) as i32) + " "));
    str.push_str(&(format!("{}", (pixel_color.y() * 255.999) as i32) + " "));
    str.push_str(&(format!("{}", (pixel_color.z() * 255.999) as i32) + "\n"));
    str
}

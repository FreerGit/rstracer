pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.
}

pub fn random_f32() -> f32 {
    (unsafe { libc::rand() as f32 } / (libc::RAND_MAX as f32))
}

pub fn random_f32_custom(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}

#[cfg(test)]
mod test {
    use crate::domain::utils::random_f32_custom;

    use super::random_f32;

    #[test]
    fn random() {
        for _ in 0..1000000 {
            let r = random_f32();
            assert!(r < 1.);
            assert!(r >= 0.);
        }
    }

    #[test]
    fn random_custom() {
        for max in 1..1000000 {
            let r = random_f32_custom(0., max as f32);
            assert!(r < max as f32);
            assert!(r >= 0.);
        }
    }
}

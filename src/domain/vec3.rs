use std::ops;

use super::utils::{random_f32, random_f32_custom};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(self) -> f32 {
        self.e[0]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }

    pub fn dot(self, v: Vec3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }

    pub fn unit_vector(u: Vec3) -> Vec3 {
        u / u.length()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_f32(), random_f32(), random_f32())
    }

    pub fn random_custom(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_f32_custom(min, max),
            random_f32_custom(min, max),
            random_f32_custom(min, max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_custom(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::unit_vector(Self::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Self::random_unit_vector();
        if Vec3::dot(on_unit_sphere, normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * 2 * Vec3::dot(v, n)
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(Vec3::dot(-uv, n), 1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -f32::abs(1.0 - r_out_perp.length_squared()).sqrt();
        r_out_perp + r_out_parallel
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, t: i32) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * (t as f32),
                self.e[1] * (t as f32),
                self.e[2] * (t as f32),
            ],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Vec3 {
        self * (1. / t)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        self * -1
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    fn inc() -> Vec3 {
        Vec3::new(1., 2., 3.)
    }

    fn vec_zero() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }

    fn ones() -> Vec3 {
        Vec3::new(1., 1., 1.)
    }

    #[test]
    fn add() {
        assert_eq!(vec_zero() + vec_zero(), vec_zero());
        assert_eq!(vec_zero() + inc(), inc().clone());
        assert_eq!(inc() + inc(), Vec3::new(2., 4., 6.,));
    }

    #[test]
    fn sub() {
        let ones = Vec3::new(-1., -1., -1.);
        assert_eq!(vec_zero() - vec_zero(), vec_zero());
        assert_eq!(vec_zero() - ones, Vec3::new(1., 1., 1.));
        assert_eq!(ones - ones, vec_zero());
    }

    #[test]
    fn mul() {
        assert_eq!(vec_zero() * vec_zero(), vec_zero());
        assert_eq!(vec_zero() * inc(), vec_zero());
        assert_eq!(inc() * inc(), Vec3::new(1., 4., 9.,));
        assert_eq!(inc() * 3., Vec3::new(3., 6., 9.,));
    }

    #[test]
    fn div() {
        assert_eq!(vec_zero() / 5., vec_zero());
        assert_eq!(vec_zero() / 2., vec_zero());
        assert_eq!(inc() / 2., Vec3::new(0.5, 1., 3. / 2.,));
        assert_eq!(inc() / 1., Vec3::new(1., 2., 3.,));
    }

    #[test]
    fn dot() {
        let a = inc();
        let b = Vec3::new(4., -5., 6.);
        assert_eq!(Vec3::dot(a, b), 12.);
        assert_eq!(Vec3::dot(vec_zero(), inc()), 0.);
    }

    #[test]
    fn cross() {
        let u = Vec3::new(3., -3., 1.);
        let v = Vec3::new(4., 9., 2.);
        let ans = Vec3::new(-15., -2., 39.);
        assert_eq!(Vec3::cross(u, v), ans);
        assert_eq!(Vec3::cross(vec_zero(), vec_zero()), vec_zero());
        assert_eq!(Vec3::cross(ones(), ones()), vec_zero());
    }

    #[test]
    fn unit_vector() {
        let x = 1.0 / (3 as f32).sqrt();
        let unit = Vec3::new(x, x, x);
        assert_eq!(Vec3::unit_vector(ones()), unit);
    }

    #[test]
    fn neg() {
        assert_eq!(-vec_zero(), vec_zero());
        assert_eq!(-ones(), Vec3::new(-1., -1., -1.));
        assert_eq!(-inc(), Vec3::new(-1., -2., -3.));
        assert_eq!(-inc(), -inc());
        assert_eq!(-Vec3::new(-1., -5., 4.), Vec3::new(1., 5., -4.));
    }

    #[test]
    fn add_assign() {
        let mut zeroes = Vec3::default();
        assert_eq!(vec_zero(), vec_zero());
        zeroes += ones();
        assert_eq!(vec_zero() + ones(), zeroes);
        zeroes += ones();
        assert_eq!(ones() + ones(), Vec3::new(2., 2., 2.));

        // assert_eq!(inc() + inc(), Vec3::new(2., 4., 6.,));
    }
}

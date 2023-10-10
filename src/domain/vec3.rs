use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn dot(u: Vec3, v: Vec3) -> f32 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
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

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Vec3 {
        self * (1. / t)
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
}

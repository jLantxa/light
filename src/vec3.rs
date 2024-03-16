#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn norm2(&self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }

    fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }

    fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.x + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.x - rhs.z,
        }
    }
}

/// Right side multiplication with f32
impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::Output {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

/// Left side multiplication with f32
impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn add() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(-1.0, 2.0, -3.0);
        assert_eq!(Vec3::new(0.0, 3.0, -2.0), v1 + v2);
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);
        let v2 = Vec3::new(-1.0, 2.0, -3.0);
        assert_eq!(Vec3::new(2.0, -1.0, 4.0), v1 - v2);
    }

    #[test]
    fn mul() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec3::new(0.5, 1.0, 1.5), 0.5 * v);
        assert_eq!(Vec3::new(0.5, 1.0, 1.5), v * 0.5);
    }

    #[test]
    fn negative() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(Vec3::new(-1.0, -1.0, -1.0), -v);
    }

    #[test]
    fn norm() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(3.0_f32.sqrt(), v.norm());
    }

    #[test]
    fn norm2() {
        let v = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(3.0_f32, v.norm2());
    }

    #[test]
    fn normalize() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v.normalize();

        assert_relative_eq!(1.0_f32, v.norm());
    }

    #[test]
    fn dot() {
        let vx = Vec3::new(1.0, 0.0, 0.0);
        let vy = Vec3::new(0.0, 1.0, 0.0);
        let vz = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(0.0, vx.dot(vy));
        assert_eq!(0.0, vy.dot(vz));
        assert_eq!(0.0, vx.dot(vz));
        assert_eq!(0.0, vz.dot(vx));
        assert_eq!(0.0, vy.dot(vz));
        assert_eq!(0.0, vz.dot(vy));

        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.5, 10.0, 10.0);

        assert_eq!(1.0, v1.dot(v1));
        assert_eq!(0.5, v1.dot(v2));
        assert_eq!(0.5, v2.dot(v1));
        assert_eq!(-1.0, v1.dot(-v1));
        assert_eq!(-1.0, (-v1).dot(v1));
    }

    #[test]
    fn cross() {
        let vx = Vec3::new(1.0, 0.0, 0.0);
        let vy = Vec3::new(0.0, 1.0, 0.0);
        let vz = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(vz, vx.cross(vy));
        assert_eq!(-vz, vy.cross(vx));
        assert_eq!(vx, vy.cross(vz));
        assert_eq!(-vx, vz.cross(vy));
        assert_eq!(vy, vz.cross(vx));
        assert_eq!(-vy, vx.cross(vz));
    }
}

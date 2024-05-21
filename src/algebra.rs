/*
 * light is a spectral path tracer written in Rust for educational purposes
 *
 * Copyright (C) 2024  Javier Lancha Vázquez
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use approx::{abs_diff_eq, relative_eq};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub const UNIT_X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
pub const UNIT_Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);
pub const UNIT_Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub const fn unit_x() -> Self {
        UNIT_X
    }

    pub const fn unit_y() -> Self {
        UNIT_Y
    }

    pub const fn unit_z() -> Self {
        UNIT_Z
    }

    pub fn norm2(&self) -> f32 {
        self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
    }

    pub fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }

    pub fn normal(&self) -> Self {
        let norm = self.norm();

        Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
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
            z: self.z + rhs.z,
        }
    }
}

impl<'a, 'b> std::ops::Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'b Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<'a, 'b> std::ops::Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'b Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

impl approx::AbsDiffEq for Vec3 {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon {
        std::f32::EPSILON
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        abs_diff_eq!(self.x, other.x, epsilon = epsilon)
            && abs_diff_eq!(self.y, other.y, epsilon = epsilon)
            && abs_diff_eq!(self.z, other.z, epsilon = epsilon)
    }
}

impl approx::RelativeEq for Vec3 {
    fn default_max_relative() -> f32 {
        std::f32::EPSILON
    }

    fn relative_eq(
        &self,
        other: &Self,
        _epsilon: Self::Epsilon,
        _max_relative: Self::Epsilon,
    ) -> bool {
        relative_eq!(self.x, other.x)
            && relative_eq!(self.y, other.y)
            && relative_eq!(self.z, other.z)
    }
}

pub fn solve_deg2_eq(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    if a != 0.0 {
        let discriminant: f32 = (b * b) - (4.0 * a * c);

        if discriminant > 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let x1 = (-b + sqrt_discriminant) / (2.0 * a);
            let x2 = (-b - sqrt_discriminant) / (2.0 * a);

            // Sort solutions
            if x1 <= x2 {
                return Some((x1, x2));
            } else {
                return Some((x2, x1));
            }
        } else if discriminant == 0.0 {
            let x = -b / (2.0 * a);
            return Some((x, x));
        } else {
            return None;
        }
    } else {
        if b != 0.0 {
            let x = -c / b;
            return Some((x, x));
        } else {
            return None;
        }
    }
}

/// Rotate a vector around an axis using Rodrigues' formula
/// https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula
pub fn rotate_vector(v: &Vec3, k: &Vec3, theta: f32) -> Vec3 {
    // Assuming k is a unit vector and theta is an angle in radians

    let cos_th = theta.cos();
    let sin_th = theta.sin();

    (*v * cos_th) + (k.cross(*v) * sin_th) + (*k * (k.dot(*v)) * (1.0_f32 - cos_th))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    mod vec3 {
        use super::*;

        #[test]
        fn add() {
            let v1 = Vec3::new(10.0, 15.0, 20.0);
            let v2 = Vec3::new(-1.0, 2.0, -2.0);
            assert_relative_eq!(Vec3::new(9.0, 17.0, 18.0), v1 + v2);
        }

        #[test]
        fn sub() {
            let v1 = Vec3::new(1.0, 33.0, 11.0);
            let v2 = Vec3::new(-1.0, 2.0, -3.0);
            assert_relative_eq!(Vec3::new(2.0, 31.0, 14.0), v1 - v2);
        }

        #[test]
        fn mul() {
            let v = Vec3::new(1.0, 2.0, 3.0);
            assert_relative_eq!(Vec3::new(0.5, 1.0, 1.5), 0.5 * v);
            assert_relative_eq!(Vec3::new(0.5, 1.0, 1.5), v * 0.5);
        }

        #[test]
        fn negative() {
            let v = Vec3::new(1.0, 1.0, 1.0);
            assert_relative_eq!(Vec3::new(-1.0, -1.0, -1.0), -v);
        }

        #[test]
        fn norm() {
            let v = Vec3::new(1.0, 1.0, 1.0);
            assert_relative_eq!(3.0_f32.sqrt(), v.norm());
        }

        #[test]
        fn norm2() {
            let v = Vec3::new(1.0, 1.0, 1.0);
            assert_relative_eq!(3.0_f32, v.norm2());
        }

        #[test]
        fn normalize() {
            let mut v = Vec3::new(1.0, 2.0, 3.0);
            v.normalize();

            assert_relative_eq!(1.0_f32, v.norm());
        }

        #[test]
        fn normal() {
            let v = Vec3::new(1.0, 1.0, 1.0);

            let component: f32 = 1.0 / (3.0_f32.sqrt());
            assert_relative_eq!(Vec3::new(component, component, component), v.normal());
        }

        #[test]
        fn dot() {
            let vx = Vec3::new(1.0, 0.0, 0.0);
            let vy = Vec3::new(0.0, 1.0, 0.0);
            let vz = Vec3::new(0.0, 0.0, 1.0);

            assert_relative_eq!(0.0, vx.dot(vy));
            assert_relative_eq!(0.0, vy.dot(vz));
            assert_relative_eq!(0.0, vx.dot(vz));
            assert_relative_eq!(0.0, vz.dot(vx));
            assert_relative_eq!(0.0, vy.dot(vz));
            assert_relative_eq!(0.0, vz.dot(vy));

            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.5, 10.0, 10.0);

            assert_relative_eq!(1.0, v1.dot(v1));
            assert_relative_eq!(0.5, v1.dot(v2));
            assert_relative_eq!(0.5, v2.dot(v1));
            assert_relative_eq!(-1.0, v1.dot(-v1));
            assert_relative_eq!(-1.0, (-v1).dot(v1));
        }

        #[test]
        fn cross() {
            let vx = Vec3::new(1.0, 0.0, 0.0);
            let vy = Vec3::new(0.0, 1.0, 0.0);
            let vz = Vec3::new(0.0, 0.0, 1.0);

            assert_relative_eq!(vz, vx.cross(vy));
            assert_relative_eq!(-vz, vy.cross(vx));
            assert_relative_eq!(vx, vy.cross(vz));
            assert_relative_eq!(-vx, vz.cross(vy));
            assert_relative_eq!(vy, vz.cross(vx));
            assert_relative_eq!(-vy, vx.cross(vz));
        }
    }

    #[test]
    fn deg2_eq_solutions() {
        let (a, b, c) = (-1.0, 2.0, 3.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 3.0)), solutions);

        let (a, b, c) = (1.0, 1.0, 0.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 0.0)), solutions);

        let (a, b, c) = (2.0, 2.0, 0.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 0.0)), solutions);

        let (a, b, c) = (0.0, 1.0, 2.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-2.0, -2.0)), solutions);

        let (a, b, c) = (0.0, 0.0, 2.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(None, solutions);

        let (a, b, c) = (1.0, 0.0, -1.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, 1.0)), solutions);

        let (a, b, c) = (1.0, 2.0, 1.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(Some((-1.0, -1.0)), solutions);

        let (a, b, c) = (1.0, 2.0, 3.0);
        let solutions = solve_deg2_eq(a, b, c);
        assert_eq!(None, solutions);
    }

    #[test]
    fn rotate_vectors() {
        let v = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let v_rot = rotate_vector(&v, &k, 45.0_f32.to_radians());
        assert_relative_eq!(
            Vec3::new(-2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0, 0.0),
            v_rot
        );

        let v = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let v_rot = rotate_vector(&v, &k, 90.0_f32.to_radians());
        assert_relative_eq!(Vec3::new(-1.0, 0.0, 0.0), v_rot);

        let v = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(1.0, 0.0, 0.0);
        let v_rot = rotate_vector(&v, &k, 45.0_f32.to_radians());
        assert_relative_eq!(
            Vec3::new(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0),
            v_rot
        );

        let v = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(1.0, 0.0, 0.0);
        let v_rot = rotate_vector(&v, &k, 90.0_f32.to_radians());
        assert_relative_eq!(Vec3::new(0.0, 0.0, 1.0_f32), v_rot);
    }
}

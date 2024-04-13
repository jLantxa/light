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

use crate::algebra::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    wavelength: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, wavelength: f32) -> Self {
        Self {
            origin: origin,
            direction: direction.normal(),
            wavelength: wavelength,
        }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }

    pub const fn origin(&self) -> Vec3 {
        self.origin
    }

    pub const fn direction(&self) -> Vec3 {
        self.direction
    }

    pub const fn wavelength(&self) -> f32 {
        self.wavelength
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn point_at() {
        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), 500e-9);

        let component: f32 = (3.0 + 3.0_f32.sqrt()) / 3.0;
        assert_relative_eq!(
            Vec3::new(component, component, component),
            ray.point_at(1.0)
        );
    }
}

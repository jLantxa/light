/*
 * light is a path tracer written in Rust for educational purposes
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

use glm;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: glm::DVec3,
    pub direction: glm::DVec3,
}

impl Ray {
    pub fn new(origin: glm::DVec3, direction: glm::DVec3) -> Self {
        Self {
            origin: origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at(&self, t: f64) -> glm::DVec3 {
        self.origin + (t * self.direction)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn point_at() {
        let ray = Ray::new(
            glm::DVec3::new(1.0, 1.0, 1.0),
            glm::DVec3::new(1.0, 1.0, 1.0),
        );

        let component: f64 = (3.0 + 3.0_f64.sqrt()) / 3.0;
        assert_relative_eq!(
            glm::DVec3::new(component, component, component),
            ray.point_at(1.0)
        );
    }
}

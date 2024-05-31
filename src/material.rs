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

use rand::rngs::StdRng;

use crate::color::Color;

#[derive(Debug)]
pub struct Material {
    pub color: Color,
}

impl Material {
    pub fn bsdf(normal: &glm::DVec3, vin: &glm::DVec3, vout: &glm::DVec3) -> Color {
        Color::zeros()
    }

    pub fn sample_bounce(normal: &glm::DVec3, vin: &glm::DVec3, rng: &mut StdRng) -> glm::DVec3 {
        normal.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

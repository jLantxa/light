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

use crate::spectrum::Spectrum;

#[derive(Debug, Default)]
pub struct MaterialProperties {
    emission: Option<Spectrum>,   // Emitted SPD
    absorption: Option<Spectrum>, // Absorbed SPD

    transmitance: f32, // Fraction of non-absorbed light that gets transmitted instead of reflected.
    roughness: f32,    // Fraction of diffuse reflection vs specular reflection.
    refraction_index: f32,
}

impl MaterialProperties {
    pub fn emission(&self) -> &Option<Spectrum> {
        &self.emission
    }

    pub fn absorption(&self) -> &Option<Spectrum> {
        &self.absorption
    }

    pub fn transmitance(&self) -> f32 {
        self.transmitance
    }

    pub fn roughness(&self) -> f32 {
        self.roughness
    }

    pub fn refraction_index(&self) -> f32 {
        self.refraction_index
    }
}

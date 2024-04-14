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

use crate::algebra;

#[derive(Debug)]
pub struct Spectrum {
    wavelengths: Vec<f32>,
    powers: Vec<f32>,
    size: usize,
}

impl Spectrum {
    pub fn new(wavelenths: Vec<f32>) -> Self {
        let size: usize = wavelenths.len();
        Self {
            wavelengths: wavelenths,
            powers: vec![0.0; size],
            size: size,
        }
    }

    fn find_wavelength_index(&self, wavelength: f32) -> Option<usize> {
        if self.wavelengths.len() < 2 {
            return None;
        }

        let first: f32 = *self.wavelengths.first().unwrap();
        let last: f32 = *self.wavelengths.last().unwrap();

        if (wavelength < first) || (wavelength > last) {
            return None;
        }

        for (index, w) in self.wavelengths.iter().enumerate() {
            if wavelength > *w {
                return Some(index);
            }
        }

        return None;
    }

    pub fn interpolate_at(&self, wavelength: f32) -> Option<f32> {
        if (self.size == 1) && (wavelength == self.wavelengths[0]) {
            Some(self.wavelengths[0])
        } else if self.size > 1 {
            let first = self.wavelengths.first().unwrap();
            let last = self.wavelengths.last().unwrap();
            let index = self.find_wavelength_index(wavelength);

            match index {
                None => None,
                Some(index) => {
                    let interpolated_value = algebra::linear_interpolation(
                        self.wavelengths[index],
                        self.wavelengths[index + 1],
                        self.powers[index],
                        self.powers[index + 1],
                        wavelength,
                    );

                    Some(interpolated_value)
                }
            }
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl std::ops::Index<usize> for Spectrum {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.powers[index]
    }
}

impl std::ops::IndexMut<usize> for Spectrum {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.powers[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

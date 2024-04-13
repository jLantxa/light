/*
 * light is a ray tracer written in Rust for educational purposes
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

use crate::geometry::Intersectable;

#[derive(Default)]
pub struct Scene {
    objects: Vec<Box<dyn Intersectable>>,
    // background_emission: Spectrum
}

impl Scene {
    pub fn add_object(&mut self, object: Box<dyn Intersectable>) {
        self.objects.push(object);
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn Intersectable>> {
        self.objects.as_ref()
    }
}

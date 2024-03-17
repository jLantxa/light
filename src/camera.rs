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
use crate::algebra::Vec3;
use crate::light::Ray;

pub struct Camera {
    position: Vec3,             // Position of the camera in 3D space
    facing: Vec3,               // Direction the camera is facing towards
    rotation: f32,              // Rotation, in the positive sense, around the facing axis
    resolution: (usize, usize), // Resolutions (width, height) in pixels
    fov: f32,                   // Field of view in radians
}

impl Camera {
    pub fn new(
        position: Vec3,
        facing: Vec3,
        rotation: f32,
        resolution: (usize, usize),
        fov: f32,
    ) -> Self {
        Self {
            position: position,
            facing: facing.normal(),
            rotation: rotation,
            resolution: resolution,
            fov: fov,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, new_position: Vec3) {
        self.position = new_position;
    }

    pub fn facing(&self) -> Vec3 {
        self.facing
    }

    pub fn set_facing(&mut self, new_facing: Vec3) {
        self.facing = new_facing;
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn set_rotation(&mut self, new_rotation: f32) {
        self.rotation = new_rotation;
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.resolution
    }

    pub fn set_resolution(&mut self, new_resolution: (usize, usize)) {
        self.resolution = new_resolution;
    }

    pub fn fov(&self) -> f32 {
        self.fov
    }

    pub fn set_fov(&mut self, new_fov: f32) {
        self.fov = new_fov;
    }

    /// Cast a Ray to pixel (i, j)
    fn cast_ray(&self, i: usize, j: usize) -> Option<Ray> {
        if (i >= self.resolution.0) || (j >= self.resolution.1) {
            return None;
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;

    #[test]
    fn cast_rays() {
        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0,
            (854, 480),
            FRAC_PI_2,
        );

        // Out of the sensor
        assert_eq!(None, camera.cast_ray(854, 0));
        assert_eq!(None, camera.cast_ray(0, 480));
        assert_eq!(None, camera.cast_ray(1000, 1000));
    }
}

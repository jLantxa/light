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

use std::f32::consts::PI;

use crate::algebra::{self, Vec3, UNIT_Y};
use crate::light::Ray;

#[derive(Debug, Clone, Copy)]
pub enum FieldOfView {
    Horizontal(f32),
    Vertical(f32),
}

impl Default for FieldOfView {
    fn default() -> Self {
        Self::Vertical(90.0_f32.to_radians())
    }
}

#[derive(Default, Debug)]
pub struct CameraConfig {
    pub position: Vec3,
    pub direction: Vec3,
    pub resolution: (usize, usize),
    pub rotation: f32,
    pub fov: FieldOfView,
}

#[derive(Default, Debug)]
struct CoordinateSystem {
    origin: Vec3,
    u: Vec3, // unit x =: y <cross> z
    v: Vec3, // unit y =: z <cross> x
    w: Vec3, // unit z =: x <cross> y
}

#[derive(Default, Debug)]
pub struct Camera {
    coordinate_system: CoordinateSystem, // Coordinate system (origin and base vectors)
    rotation: f32,                       // Rotation, in the positive sense, around the facing axis
    resolution: (usize, usize),          // Resolutions (width, height) in pixels
    fov: FieldOfView,                    // Field of view (Horizontal or Vertical) in radians

    distance_to_plane: f32,
    first_pixel_pos: Vec3,
    pixel_width: f32,
    pixel_height: f32,
}

impl CoordinateSystem {
    pub fn new(origin: Vec3, u: Vec3, v: Vec3, w: Vec3) -> Self {
        Self { origin, u, v, w }
    }
}

impl Camera {
    pub fn new(config: &CameraConfig) -> Self {
        let mut camera = Self::default();
        camera.config(config);
        return camera;
    }

    pub fn position(&self) -> Vec3 {
        self.coordinate_system.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.coordinate_system.w
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn fov(&self) -> FieldOfView {
        self.fov
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.resolution
    }

    pub fn config(&mut self, config: &CameraConfig) {
        const WORLD_UP: Vec3 = UNIT_Y;

        self.resolution = config.resolution;
        assert!(
            ((self.resolution.0 * self.resolution.1) > 0),
            "The camera resolution cannot be zero"
        );

        self.rotation = config.rotation;
        self.fov = config.fov;

        // Set coordinates
        self.coordinate_system.w = config.direction.normal();
        self.coordinate_system.u = self.coordinate_system.w.cross(WORLD_UP).normal();
        self.coordinate_system.v = self
            .coordinate_system
            .u
            .cross(self.coordinate_system.w)
            .normal();

        // Apply rotation
        if self.rotation != 0.0_f32 {
            self.coordinate_system.u = algebra::rotate_vector(
                &self.coordinate_system.u,
                &self.coordinate_system.w,
                self.rotation,
            );
            self.coordinate_system.u.normalize();
            self.coordinate_system.v = algebra::rotate_vector(
                &self.coordinate_system.v,
                &self.coordinate_system.w,
                self.rotation,
            );
            self.coordinate_system.v.normalize();
        }

        // Calculate distance to plane using fov
        let aspect_ratio: f32 = (self.resolution.0 as f32) / (self.resolution.1 as f32);
        let sensor_height: f32 = 1.0;
        let sensor_width: f32 = sensor_height * aspect_ratio;
        self.distance_to_plane = match self.fov {
            FieldOfView::Horizontal(mut alpha) => {
                alpha = alpha.abs();
                assert!(
                    alpha > 0.0,
                    "Horizontal field of view must be greater than 0 degrees"
                );
                assert!(
                    alpha < PI,
                    "Horizontal field of view must be smaller than 180 degrees"
                );
                sensor_width / (2.0 * (alpha / 2.0).tan())
            }
            FieldOfView::Vertical(mut alpha) => {
                alpha = alpha.abs();
                assert!(
                    alpha > 0.0,
                    "Vertical field of view must be greater than 0 degrees"
                );
                assert!(
                    alpha < PI,
                    "Vertical field of view must be smaller than 180 degrees"
                );
                sensor_height / (2.0 * (alpha / 2.0).tan())
            }
        };

        // Calculate pixel size
        self.pixel_width = sensor_width / (self.resolution.0 as f32);
        self.pixel_height = sensor_height / (self.resolution.1 as f32);

        // Calculate position for first pixel
        self.first_pixel_pos = self.coordinate_system.origin
            + (self.distance_to_plane * self.coordinate_system.w)
            - (self.coordinate_system.u * ((sensor_width / 2.0) - self.pixel_width / 2.0))
            + (self.coordinate_system.v * ((sensor_height / 2.0) - self.pixel_height / 2.0));
    }

    /// Cast a Ray to pixel (i, j)
    pub fn cast_ray(&self, i: usize, j: usize) -> Option<Ray> {
        if (i >= self.resolution.0) || (j >= self.resolution.1) {
            return None;
        }

        let pixel_position = self.first_pixel_pos
            + ((i as f32) * self.pixel_width * self.coordinate_system.u)
            - ((j as f32) * self.pixel_height * self.coordinate_system.v);
        let ray_direction = pixel_position - self.coordinate_system.origin;
        Some(Ray::new(self.coordinate_system.origin, ray_direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

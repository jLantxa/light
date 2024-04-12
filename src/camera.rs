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

use image::RgbImage;
use rand::{rngs::ThreadRng, Rng};

use crate::algebra::{self, Vec3, UNIT_Y};
use crate::light::{LightModel, Ray};
use crate::scene::Scene;

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

#[derive(Debug, Clone, Copy)]
pub enum FocusMode {
    FocalPlane { focal_distance: f32, aperture: f32 },
    PinHole,
}

impl Default for FocusMode {
    fn default() -> Self {
        Self::PinHole
    }
}

#[derive(Default, Debug)]
pub struct CameraConfig {
    pub position: Vec3,
    pub direction: Vec3,
    pub resolution: (u32, u32),
    pub rotation: f32,
    pub fov: FieldOfView,
    pub focus_mode: FocusMode,
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
    resolution: (u32, u32),              // Resolutions (width, height) in pixels
    fov: FieldOfView,                    // Field of view (Horizontal or Vertical) in radians
    focus_mode: FocusMode,

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

    pub fn resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn config(&mut self, config: &CameraConfig) -> Result<(), String> {
        const WORLD_UP: Vec3 = UNIT_Y;

        if (config.resolution.0 * config.resolution.1) == 0 {
            return Err("The camera resolution cannot be zero".to_string());
        }

        match config.fov {
            FieldOfView::Horizontal(mut alpha) => {
                alpha = alpha.abs();
                if alpha <= 0.0 || alpha > PI {
                    return Err(
                        "Horizontal field of view must be in the interval (0, 180)".to_string()
                    );
                }
                self.fov = FieldOfView::Horizontal(alpha);
            }
            FieldOfView::Vertical(mut alpha) => {
                alpha = alpha.abs();
                if alpha <= 0.0 || alpha > PI {
                    return Err(
                        "Vertical field of view must be in the interval (0, 180)".to_string()
                    );
                }
                self.fov = FieldOfView::Vertical(alpha);
            }
        }

        self.resolution = config.resolution;
        self.rotation = config.rotation;

        // Set coordinates
        self.coordinate_system.origin = config.position;
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

        // Calculate distance to plane using fov and focus parameters
        let aspect_ratio: f32 = (self.resolution.0 as f32) / (self.resolution.1 as f32);
        let (sensor_width, sensor_height): (f32, f32) = match config.focus_mode {
            FocusMode::FocalPlane {
                focal_distance,
                aperture,
            } => {
                if focal_distance < 0.0 {
                    return Err("Focal distance must be postive".to_string());
                } else if aperture < 0.0 {
                    return Err("Aperture must be postive".to_string());
                }

                self.distance_to_plane = focal_distance;
                match self.fov {
                    FieldOfView::Horizontal(alpha) => {
                        let sensor_width = 2.0 * self.distance_to_plane * ((alpha / 2.0).tan());
                        let sensor_height = sensor_width / aspect_ratio;
                        (sensor_width, sensor_height)
                    }
                    FieldOfView::Vertical(alpha) => {
                        let sensor_height = 2.0 * self.distance_to_plane * ((alpha / 2.0).tan());
                        let sensor_width = sensor_height * aspect_ratio;
                        (sensor_width, sensor_height)
                    }
                }
            }
            FocusMode::PinHole => {
                let sensor_height = 1.0;
                let sensor_width = sensor_height * aspect_ratio;
                self.distance_to_plane = match self.fov {
                    FieldOfView::Horizontal(alpha) => sensor_width / (2.0 * (alpha / 2.0).tan()),
                    FieldOfView::Vertical(alpha) => sensor_height / (2.0 * (alpha / 2.0).tan()),
                };
                (sensor_width, sensor_height)
            }
        };
        self.focus_mode = config.focus_mode;

        // Calculate pixel size
        self.pixel_width = sensor_width / (self.resolution.0 as f32);
        self.pixel_height = sensor_height / (self.resolution.1 as f32);

        // Calculate position for first pixel
        self.first_pixel_pos = self.coordinate_system.origin
            + (self.distance_to_plane * self.coordinate_system.w)
            - (self.coordinate_system.u * ((sensor_width / 2.0) - (self.pixel_width / 2.0)))
            + (self.coordinate_system.v * ((sensor_height / 2.0) - (self.pixel_height / 2.0)));

        return Ok(());
    }

    /// Cast a Ray to pixel (i, j)
    fn cast_ray(&self, i: u32, j: u32, wavelength: f32, rng: &mut ThreadRng) -> Option<Ray> {
        if (i >= self.resolution.0) || (j >= self.resolution.1) {
            return None;
        }

        let ray_origin = match self.focus_mode {
            FocusMode::PinHole => self.coordinate_system.origin,
            FocusMode::FocalPlane {
                focal_distance: _,
                aperture,
            } => {
                // Uniform sample of the aperture disc
                let r: f32 = (aperture / 2.0) * rng.gen::<f32>().sqrt();
                let phi: f32 = rng.gen_range(0.0..2.0 * PI);

                // Distances from origin (within the disc)
                let dx = r * phi.cos();
                let dy = r * phi.sin();

                self.coordinate_system.origin
                    + dx * self.coordinate_system.u
                    + dy * self.coordinate_system.v
            }
        };

        let pixel_position = self.first_pixel_pos
            + ((i as f32) * self.pixel_width * self.coordinate_system.u)
            - ((j as f32) * self.pixel_height * self.coordinate_system.v);
        let ray_direction = pixel_position - ray_origin;

        Some(Ray::new(ray_origin, ray_direction, wavelength))
    }

    pub fn capture(
        &self,
        scene: &Scene,
        model: &dyn LightModel,
        num_samples_per_pixel: u32,
    ) -> Result<RgbImage, String> {
        let mut image = RgbImage::new(self.resolution.0, self.resolution.1);
        let mut rng = rand::thread_rng();

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn sample_pinhole() {
        let config = CameraConfig {
            position: Vec3::zero(),
            direction: algebra::UNIT_Z,
            resolution: (800, 600),
            rotation: 0.0_f32,
            fov: FieldOfView::Horizontal(90f32.to_radians()),
            focus_mode: FocusMode::PinHole,
        };
        let camera = Camera::new(&config);

        let mut rng = rand::thread_rng();
        for i in 0..800 {
            for j in 0..600 {
                let ray = camera.cast_ray(i, j, 500e-9, &mut rng);
                assert_eq!(Vec3::zero(), ray.unwrap().origin());
            }
        }
    }

    #[test]
    fn sample_aperture() {
        let aperture: f32 = 0.1;
        let config = CameraConfig {
            position: Vec3::zero(),
            direction: algebra::UNIT_Z,
            resolution: (800, 600),
            rotation: 0.0_f32,
            fov: FieldOfView::Horizontal(90f32.to_radians()),
            focus_mode: FocusMode::FocalPlane {
                focal_distance: 1.0,
                aperture,
            },
        };
        let camera = Camera::new(&config);

        let mut rng = rand::thread_rng();
        for i in 0..800 {
            for j in 0..600 {
                let ray = camera.cast_ray(i, j, 500e-9, &mut rng);
                let ray_origin = ray.unwrap().origin();
                let x = ray_origin.x;
                let y = ray_origin.y;
                let z = ray_origin.z;
                assert!((x.powf(2.0) + y.powf(2.0)).sqrt() <= (aperture / 2.0));
                assert_relative_eq!(0.0, z);
            }
        }
    }
}

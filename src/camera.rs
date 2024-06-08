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

use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

use crate::light::Ray;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldOfView {
    Horizontal(f64),
    Vertical(f64),
}

impl Default for FieldOfView {
    fn default() -> Self {
        Self::Vertical(90.0_f64.to_radians())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusMode {
    FocalPlane {
        focal_distance: f64, // [m]
        aperture: f64,       // Aperture radius [m]
    },
    PinHole,
}

impl Default for FocusMode {
    fn default() -> Self {
        Self::PinHole
    }
}

#[derive(Debug)]
pub struct CameraConfig {
    pub position: glm::DVec3,
    pub direction: glm::DVec3,
    pub resolution: (u32, u32),
    pub rotation: f64,
    pub fov: FieldOfView,
    pub focus_mode: FocusMode,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: glm::DVec3::zeros(),
            direction: glm::DVec3::zeros(),
            resolution: (800, 600),
            rotation: 0.0_f64,
            fov: FieldOfView::default(),
            focus_mode: FocusMode::default(),
        }
    }
}

#[derive(Default, Debug)]
struct CoordinateSystem {
    origin: glm::DVec3,
    u: glm::DVec3, // unit x =: y <cross> z
    v: glm::DVec3, // unit y =: z <cross> x
    w: glm::DVec3, // unit z =: x <cross> y
}

#[derive(Default, Debug)]
pub struct Camera {
    coordinate_system: CoordinateSystem, // Coordinate system (origin and base vectors)
    rotation: f64, // Rotation [rad], in the positive sense, around the facing axis
    resolution: (u32, u32), // Resolutions (width, height) in pixels
    fov: FieldOfView, // Field of view (Horizontal or Vertical) in radians
    focus_mode: FocusMode,

    distance_to_plane: f64,
    first_pixel_pos: glm::DVec3,
    pixel_width: f64,
    pixel_height: f64,
}

impl CoordinateSystem {
    pub fn new(origin: glm::DVec3, u: glm::DVec3, v: glm::DVec3, w: glm::DVec3) -> Self {
        Self { origin, u, v, w }
    }
}

impl Camera {
    pub fn new(config: &CameraConfig) -> Self {
        let mut camera = Self::default();
        camera.config(config).expect("Couldn't configure camera");

        return camera;
    }

    pub fn position(&self) -> glm::DVec3 {
        self.coordinate_system.origin
    }

    pub fn direction(&self) -> glm::DVec3 {
        self.coordinate_system.w
    }

    pub fn rotation(&self) -> f64 {
        self.rotation
    }

    pub fn fov(&self) -> FieldOfView {
        self.fov
    }

    pub fn resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn config(&mut self, config: &CameraConfig) -> Result<(), &str> {
        const WORLD_UP: glm::DVec3 = glm::DVec3::new(0.0, 1.0, 0.0);

        if (config.resolution.0 * config.resolution.1) == 0 {
            return Err("The camera resolution cannot be zero");
        }

        match config.fov {
            FieldOfView::Horizontal(mut alpha) => {
                alpha = alpha.abs();
                if alpha <= 0.0 || alpha > PI {
                    return Err("Horizontal field of view must be in the interval (0, 180)");
                }
                self.fov = FieldOfView::Horizontal(alpha);
            }
            FieldOfView::Vertical(mut alpha) => {
                alpha = alpha.abs();
                if alpha <= 0.0 || alpha > PI {
                    return Err("Vertical field of view must be in the interval (0, 180)");
                }
                self.fov = FieldOfView::Vertical(alpha);
            }
        }

        self.resolution = config.resolution;
        self.rotation = config.rotation;

        // Set coordinates
        self.coordinate_system.origin = config.position;
        self.coordinate_system.w = config.direction.normalize();
        self.coordinate_system.u = self.coordinate_system.w.cross(&WORLD_UP).normalize();
        self.coordinate_system.v = self
            .coordinate_system
            .u
            .cross(&self.coordinate_system.w)
            .normalize();

        // Apply rotation
        if self.rotation != 0.0_f64 {
            self.coordinate_system.u = glm::rotate_vec3(
                &self.coordinate_system.u,
                self.rotation,
                &self.coordinate_system.w,
            );
            self.coordinate_system.u.normalize_mut();
            self.coordinate_system.v = glm::rotate_vec3(
                &self.coordinate_system.v,
                self.rotation,
                &self.coordinate_system.w,
            );
            self.coordinate_system.v.normalize_mut();
        }

        // Calculate distance to plane using fov and focus parameters
        let aspect_ratio: f64 = (self.resolution.0 as f64) / (self.resolution.1 as f64);
        let (sensor_width, sensor_height): (f64, f64) = match config.focus_mode {
            FocusMode::FocalPlane {
                focal_distance,
                aperture,
            } => {
                if focal_distance < 0.0 {
                    return Err("Focal distance must be postive");
                } else if aperture < 0.0 {
                    return Err("Aperture must be postive");
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
        self.pixel_width = sensor_width / (self.resolution.0 as f64);
        self.pixel_height = sensor_height / (self.resolution.1 as f64);

        // Calculate position for first pixel
        self.first_pixel_pos = self.coordinate_system.origin
            + (self.distance_to_plane * self.coordinate_system.w)
            - (self.coordinate_system.u * ((sensor_width / 2.0) - (self.pixel_width / 2.0)))
            + (self.coordinate_system.v * ((sensor_height / 2.0) - (self.pixel_height / 2.0)));

        return Ok(());
    }

    /// Cast a Ray to pixel (i, j)
    pub fn cast_ray(&self, i: u32, j: u32, rng: &mut ThreadRng) -> Option<Ray> {
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
                let [x, y]: [f64; 2] = rng.sample(rand_distr::UnitDisc);

                self.coordinate_system.origin
                    + aperture * (x * self.coordinate_system.u + y * self.coordinate_system.v)
            }
        };

        let pixel_position = self.first_pixel_pos
            + ((i as f64) * self.pixel_width * self.coordinate_system.u)
            - ((j as f64) * self.pixel_height * self.coordinate_system.v);
        let ray_direction = pixel_position - ray_origin;

        Some(Ray::new(ray_origin, ray_direction))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use rand::{rngs, SeedableRng};

    use super::*;

    #[test]
    fn sample_pinhole() {
        let config = CameraConfig {
            position: glm::DVec3::zeros(),
            direction: glm::DVec3::z(),
            resolution: (800, 600),
            rotation: 0.0_f64,
            fov: FieldOfView::Horizontal(90f64.to_radians()),
            focus_mode: FocusMode::PinHole,
        };
        let camera = Camera::new(&config);

        let mut rng = rand::thread_rng();
        for i in 0..800 {
            for j in 0..600 {
                let ray = camera.cast_ray(i, j, &mut rng);
                assert_eq!(glm::DVec3::zeros(), ray.unwrap().origin);
            }
        }
    }

    #[test]
    fn sample_aperture() {
        let aperture: f64 = 0.1;
        let config = CameraConfig {
            position: glm::DVec3::zeros(),
            direction: glm::DVec3::z(),
            resolution: (800, 600),
            rotation: 0.0_f64,
            fov: FieldOfView::Horizontal(90f64.to_radians()),
            focus_mode: FocusMode::FocalPlane {
                focal_distance: 1.0,
                aperture,
            },
        };
        let camera = Camera::new(&config);

        let mut rng = rand::thread_rng();
        for i in 0..800 {
            for j in 0..600 {
                let ray = camera.cast_ray(i, j, &mut rng);
                let ray_origin = ray.unwrap().origin;
                let x = ray_origin.x;
                let y = ray_origin.y;
                let z = ray_origin.z;
                assert!((x.powf(2.0) + y.powf(2.0)).sqrt() <= aperture);
                assert_relative_eq!(0.0, z);
            }
        }
    }

    #[test]
    fn default_camera_config() {
        let default_config = CameraConfig::default();
        assert_eq!(default_config.position, glm::DVec3::zeros());
        assert_eq!(default_config.direction, glm::DVec3::zeros());
        assert_eq!(default_config.resolution, (800, 600));
        assert_eq!(default_config.rotation, 0.0);
        assert_eq!(default_config.fov, FieldOfView::default());
        assert_eq!(default_config.focus_mode, FocusMode::default());
    }
}

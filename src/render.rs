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

use std::{f32::consts::PI, thread::Thread};

use image::{Rgb, RgbImage};
use rand::{rngs::ThreadRng, seq::IteratorRandom, Rng};

use crate::{
    algebra::Vec3, camera::Camera, light::Ray, object::MaterialObject, scene::Scene,
    spectrum::Spectrum,
};

const DEFAULT_SPP: u32 = 128;
const DEFAULT_HALF_LIFE: u32 = 5;

pub enum RayExtinction {
    Fix(u32),
    HalfLife(u32),
}

pub struct PathTracer {
    samples_per_pixel: u32,
    ray_extinction: RayExtinction,
    propagation_probability: f32,
}

impl Default for PathTracer {
    fn default() -> Self {
        let p: f32 = calculate_propagation_prob_for_half_life(DEFAULT_HALF_LIFE);

        Self {
            samples_per_pixel: DEFAULT_SPP,
            ray_extinction: RayExtinction::HalfLife(DEFAULT_HALF_LIFE),
            propagation_probability: p,
        }
    }
}

fn spectrum_to_rgb(spectrum: &Spectrum) -> Rgb<u8> {
    let mut rgb: Rgb<u8> = Rgb([0, 0, 0]);

    todo!()
}

impl PathTracer {
    pub fn render(&self, scene: &Scene, camera: &Camera, wavelengths: &Vec<f32>) -> RgbImage {
        let resolution = camera.resolution();
        let mut image = RgbImage::new(resolution.0, resolution.1);

        for (i, j, mut rgb) in image.enumerate_pixels_mut() {
            let spectrum = self.render_pixel(i, j, camera, scene, wavelengths);
            let pixel = spectrum_to_rgb(&spectrum);
            rgb[0] = pixel[0];
            rgb[1] = pixel[1];
            rgb[2] = pixel[2];
        }

        return image;
    }

    fn render_pixel(
        &self,
        i: u32,
        j: u32,
        camera: &Camera,
        scene: &Scene,
        wavelengths: &Vec<f32>,
    ) -> Spectrum {
        let mut spectrum = Spectrum::new(wavelengths.clone());
        let mut rng = rand::thread_rng();

        for _ in 0..self.samples_per_pixel {
            let (wavelenght_index, wavelength) = self
                .sample_wavelength(wavelengths, &mut rng)
                .expect("Expected a wavelength");
            let ray = camera.cast_ray(i, j, wavelength, &mut rng);

            let power: f32 = match ray {
                Some(ray) => self.propagate_ray(&ray, scene, 0, &mut rng),
                _ => 0.0,
            };

            // TODO: Average spectrum later
            spectrum[wavelenght_index] += power / (self.samples_per_pixel as f32);
        }

        return spectrum;
    }

    /// Propagates a Ray into the scene and returns the returned power for the propagated
    /// wavelength.
    fn propagate_ray(&self, ray: &Ray, scene: &Scene, counter: u32, rng: &mut ThreadRng) -> f32 {
        if self.should_extinguish(counter, rng) {
            return 0.0;
        }

        let intersection = self.intersect_ray(ray, scene);

        match intersection {
            None => 0.0,
            Some((point, object)) => {
                let material = object.get_material();
                let emission_spd = material.emission();
                let transmission_spd = material.absorption();

                let e = if let Some(emissive_spectrum) = emission_spd {
                    emissive_spectrum
                        .interpolate_at(ray.wavelength())
                        .unwrap_or(0.0)
                } else {
                    0.0
                };

                let t = if let Some(transmitted_spectrum) = transmission_spd {
                    transmitted_spectrum
                        .interpolate_at(ray.wavelength())
                        .unwrap_or(1.0)
                } else {
                    0.0
                };

                // All incident energy absorbed, so no secondary ray reflected
                if t <= 0.0 {
                    return e;
                }

                let next_ray = self.calculate_next_ray(&point, ray, object, rng);

                e + t * self.propagate_ray(&next_ray, scene, counter + 1, rng)
            }
        }
    }

    fn sample_hemisphere(&self, normal: &Vec3, rng: &mut ThreadRng) -> Vec3 {
        let phi = 2.0 * PI * rng.gen::<f32>();
        let r = rng.gen::<f32>();
        let sqrt_r = r.sqrt();

        let w = normal.normal();

        let rand_v = if (w.cross(Vec3::new(0.0, 0.0, 1.0))).norm() > 0.0 {
            w.cross(Vec3::new(0.0, 0.0, 1.0))
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let u = w.cross(rand_v);
        let v = w.cross(u);

        let sample_v = (sqrt_r * phi.cos()) * u + (sqrt_r * phi.sin()) * v + (1.0 - r).sqrt() * w;

        return sample_v;
    }

    fn calculate_next_ray(
        &self,
        intersection: &Vec3,
        incident_ray: &Ray,
        object: &Box<dyn MaterialObject>,
        rng: &mut ThreadRng,
    ) -> Ray {
        // TODO: Only diffuse component implement

        let material = object.get_material();
        let normal = object.hit_normal(intersection, &incident_ray.direction());
        let sample_direction = self.sample_hemisphere(&normal, rng);

        Ray::new(*intersection, sample_direction, incident_ray.wavelength())
    }

    /// Calculates the intersection of a Ray with the objects of the scene and returns the closest
    /// intersection.
    fn intersect_ray<'a>(
        &self,
        ray: &Ray,
        scene: &'a Scene,
    ) -> Option<(Vec3, &'a Box<dyn MaterialObject>)> {
        let objects = scene.get_objects();
        let mut intersections: Vec<(f32, &Box<dyn MaterialObject>)> = Vec::new();

        for obj in objects {
            let it = obj.intersect(&ray);
            if let Some(t) = it {
                intersections.push((t, obj));
            }
        }

        intersections.sort_unstable_by(|&(t0, _), &(t1, _)| t0.partial_cmp(&t1).unwrap());

        match intersections.len() {
            0 => None,
            _ => {
                let first = intersections.first().unwrap();
                let point = ray.point_at(first.0);
                Some((point, first.1))
            }
        }
    }

    fn sample_wavelength(
        &self,
        wavelenghts: &Vec<f32>,
        rng: &mut ThreadRng,
    ) -> Option<(usize, f32)> {
        let selection = wavelenghts.iter().enumerate().choose(rng);

        match selection {
            Some((i, w)) => Some((i, *w)),
            _ => None,
        }
    }

    fn should_extinguish(&self, counter: u32, rng: &mut ThreadRng) -> bool {
        match self.ray_extinction {
            RayExtinction::Fix(max_rays) => counter > max_rays,
            RayExtinction::HalfLife(_) => {
                let q = rng.gen::<f32>();
                self.propagation_probability < q
            }
        }
    }
}

fn calculate_propagation_prob_for_half_life(lambda: u32) -> f32 {
    std::f32::consts::E.powf(-std::f32::consts::LN_2 / (lambda as f32))
}

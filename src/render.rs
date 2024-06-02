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

use image::RgbImage;
use rand::rngs::ThreadRng;
use rand_distr::num_traits::AsPrimitive;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::color::Color;
use crate::light::Ray;
use crate::object::Object;
use crate::shape::HitRecord;
use crate::{camera::Camera, scene::Scene};

pub fn render_geometry(scene: &Scene, camera: &Camera) -> RgbImage {
    let (w, h) = camera.resolution();
    let mut image = image::RgbImage::new(w, h);

    image
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(i, j, rgb)| {
            let mut rng = rand::thread_rng();
            let ray = camera.cast_ray(i, j, &mut rng).expect("Expected a Ray");

            let closest_hit = get_closest_hit(&scene.objects, &ray);

            // Indirect
            let color = match closest_hit {
                None => scene.background_color,
                Some((record, object)) => object.material.color,
            };

            rgb[0] = color.x.as_();
            rgb[1] = color.y.as_();
            rgb[2] = color.z.as_();
        });

    image
}

fn get_closest_hit<'a>(objects: &'a Vec<Object>, ray: &Ray) -> Option<(HitRecord, &'a Object)> {
    let mut closest_hit = HitRecord::new();
    let mut obj = None;

    for object in objects {
        let hit = object.intersect(&ray);
        if hit.is_none() {
            continue;
        }

        let hit = hit.unwrap();
        if hit.ray_t < closest_hit.ray_t {
            let hit = hit;
            closest_hit = hit;
            obj = Some(object);
        }
    }

    Some((closest_hit, obj?))
}

pub struct PathTracer {
    spp: u32,
    max_depth: u32,
}

impl Default for PathTracer {
    fn default() -> Self {
        Self {
            spp: 16,
            max_depth: 5,
        }
    }
}

impl PathTracer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn samples_per_pixel(&mut self, spp: u32) -> &mut Self {
        self.spp = spp;
        self
    }

    pub fn max_depth(&mut self, depth: u32) -> &mut Self {
        self.max_depth = depth;
        self
    }

    pub fn render(&self, scene: &Scene, camera: &Camera) -> RgbImage {
        let (w, h) = camera.resolution();
        let mut image = image::RgbImage::new(w, h);

        image
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(i, j, rgb)| {
                let mut rng = rand::thread_rng();
                let mut color = Color::zeros();
                for n in 0..self.spp {
                    let ray = camera.cast_ray(i, j, &mut rng).expect("Expected a Ray");
                    color += self.trace_ray(&scene, &ray, 0, &mut rng);
                }

                rgb[0] += (color.x / self.spp as f64).min(255.0) as u8;
                rgb[1] += (color.y / self.spp as f64).min(255.0) as u8;
                rgb[2] += (color.z / self.spp as f64).min(255.0) as u8;
            });

        image
    }

    fn trace_ray(&self, scene: &Scene, ray: &Ray, counter: u32, rng: &mut ThreadRng) -> Color {
        let closest_hit = get_closest_hit(&scene.objects, &ray);

        // Indirect
        match closest_hit {
            None => scene.background_color,
            Some((record, object)) => {
                let material = &object.material;
                let vout = &-ray.direction;
                let vin = material
                    .sample_bounce(&record.normal, vout, rng)
                    .normalize();

                let mut color = material.emittance * material.color;

                if counter < self.max_depth {
                    let new_ray = Ray::new(record.point, vin);
                    color += material
                        .bsdf(&record.normal, &vin, vout)
                        .component_mul(&self.trace_ray(scene, &new_ray, counter + 1, rng));
                }

                color
            }
        }
    }
}

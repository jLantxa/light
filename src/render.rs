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

use std::time::SystemTime;

use image::RgbImage;
use rand::rngs::StdRng;
use rand::{rngs, SeedableRng};

use crate::color::Color;
use crate::light::Ray;
use crate::object::Object;
use crate::shape::HitRecord;
use crate::{camera::Camera, scene::Scene};

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
        let d = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Duration since UNIX_EPOCH failed");
        let mut rng = StdRng::seed_from_u64(d.as_secs());

        let (w, h) = camera.resolution();
        let mut image = image::RgbImage::new(w, h);

        for (i, j, rgb) in image.enumerate_pixels_mut() {
            let mut color = Color::zeros();
            for n in 0..self.spp {
                println!("{}, {}, {}", i, j, n);
                let ray = camera.cast_ray(i, j, &mut rng).expect("Expected a Ray");
                color += self.trace_ray(&scene, &ray, 0);
            }

            rgb[0] += (color.x / self.spp as f64).min(255.0) as u8;
            rgb[1] += (color.y / self.spp as f64).min(255.0) as u8;
            rgb[2] += (color.z / self.spp as f64).min(255.0) as u8;
        }

        image
    }

    fn trace_ray(&self, scene: &Scene, ray: &Ray, counter: u32) -> Color {
        let closest_hit = self.get_closest_hit(&scene.objects, &ray);

        match closest_hit {
            None => scene.background_color,
            Some((record, object)) => {
                println!("Hit");
                let material = &object.material;
                material.color
            }
        }
    }

    fn get_closest_hit<'a>(
        &'a self,
        objects: &'a Vec<Object>,
        ray: &Ray,
    ) -> Option<(HitRecord, &'_ Object)> {
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
}

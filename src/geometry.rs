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

use crate::algebra::{self, Vec3};
use crate::light::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn hit_normal(&self, intersection: &Vec3, direction: &Vec3) -> Vec3;
}

fn closest_sol((x1, x2): (f32, f32)) -> Option<f32> {
    // Assume x1 <= x2
    if x1 < 0.0 {
        if x2 < 0.0 {
            None
        } else {
            Some(x2)
        }
    } else {
        Some(x1)
    }
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let oc: Vec3 = ray.origin() - self.center;
        let d: Vec3 = ray.direction();

        let a: f32 = d.norm().powf(2.0);
        let b: f32 = 2.0 * oc.dot(d);
        let c: f32 = oc.norm().powf(2.0) - self.radius.powf(2.0);

        let solutions = algebra::solve_deg2_eq(a, b, c);

        match solutions {
            None => None,
            Some(sols) => closest_sol(sols),
        }
    }

    fn hit_normal(&self, intersection: &Vec3, direction: &Vec3) -> Vec3 {
        // -(d*n)n / |(d*n)n|
        let surf_normal = &self.center - intersection;
        (direction.dot(surf_normal) * surf_normal).normal()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 20.0), Vec3::new(0.0, 0.0, -1.0));

        let t = sphere.intersect(&ray).unwrap();
        let intersection = ray.point_at(t);
        let hit_normal = sphere.hit_normal(&intersection, &ray.direction());

        assert_eq!(Vec3::new(0.0, 0.0, 10.0), intersection);
        assert_eq!(Vec3::new(0.0, 0.0, -1.0), hit_normal);
    }

    #[test]
    fn intersect_from_inside_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        let t = sphere.intersect(&ray).unwrap();
        let intersection = ray.point_at(t);
        let hit_normal = sphere.hit_normal(&intersection, &ray.direction());

        assert_eq!(Vec3::new(0.0, 0.0, 10.0), intersection);
        assert_eq!(Vec3::new(0.0, 0.0, 1.0), hit_normal);
    }

    #[test]
    fn no_intersect_behind_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 20.0), Vec3::new(0.0, 0.0, 1.0));

        let t = sphere.intersect(&ray);
        assert_eq!(t, None);
    }
}

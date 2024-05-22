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

use crate::algebra::{self, Vec3};
use crate::light::Ray;

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub ray_t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord>;
}

/// Returns the closest positive distance (facing the direction of a Ray)
fn closest_facing_solution((t1, t2): (f32, f32)) -> Option<f32> {
    assert!(t1 <= t2);

    if t1 >= 0.0 {
        Some(t1)
    } else if t2 >= 0.0 {
        Some(t2)
    } else {
        None
    }
}

pub fn intersect_composite(objects: &Vec<Box<dyn Shape>>, ray: &Ray) -> Option<HitRecord> {
    let mut closest_hit: Option<HitRecord> = None;

    for object in objects {
        let hit = object.intersect(&ray);
        if hit.is_none() {
            continue;
        }

        let hit = hit.unwrap();
        if closest_hit.is_none()
            || ((hit.ray_t >= 0.0) && (hit.ray_t < closest_hit.as_ref().unwrap().ray_t))
        {
            let hit = hit;
            closest_hit.replace(hit);
        }
    }

    closest_hit
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

    pub fn normal(&self, intersection: &Vec3, direction: &Vec3) -> Vec3 {
        // -(d*n)n / |(d*n)n|
        let surf_normal = &self.center - intersection;
        (direction.dot(surf_normal) * surf_normal).normal()
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let d: Vec3 = ray.direction;

        let a: f32 = d.norm().powf(2.0);
        let b: f32 = 2.0 * oc.dot(d);
        let c: f32 = oc.norm().powf(2.0) - self.radius.powf(2.0);

        let solutions = algebra::solve_deg2_eq(a, b, c);

        match solutions {
            None => None,
            Some(sols) => {
                let t = closest_facing_solution(sols);
                if t.is_none() {
                    return None;
                }

                let t = t.unwrap();
                let point = ray.point_at(t);
                let normal = self.normal(&point, &ray.direction);

                Some(HitRecord {
                    ray_t: t,
                    point,
                    normal,
                })
            }
        }
    }
}

pub struct Composite {
    // bounding_box: Cube
    pub objects: Vec<Box<dyn Shape>>,
}

impl Shape for Composite {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        // If no interset bounding box, return None
        return intersect_composite(&self.objects, &ray);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn closest_sol() {
        assert_eq!(Some(1.0), closest_facing_solution((1.0, 2.0)));
        assert_eq!(Some(2.0), closest_facing_solution((-1.0, 2.0)));
        assert_eq!(Some(0.0), closest_facing_solution((-1.0, 0.0)));
        assert_eq!(Some(0.0), closest_facing_solution((0.0, 0.0)));
        assert_eq!(None, closest_facing_solution((-2.0, -1.0)));
    }

    #[test]
    fn intersect_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 20.0), Vec3::new(0.0, 0.0, -1.0));

        let hit = sphere.intersect(&ray).expect("Expected some HitRecord");

        assert_relative_eq!(Vec3::new(0.0, 0.0, 10.0), hit.point);
        assert_relative_eq!(Vec3::new(0.0, 0.0, -1.0), hit.normal);
    }

    #[test]
    fn intersect_from_inside_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));

        let hit_record = sphere.intersect(&ray).expect("Expected some HitRecord");

        assert_relative_eq!(Vec3::new(0.0, 0.0, 10.0), hit_record.point);
        assert_relative_eq!(Vec3::new(0.0, 0.0, 1.0), hit_record.normal);
    }

    #[test]
    fn no_intersect_behind_sphere() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 20.0), Vec3::new(0.0, 0.0, 1.0));

        let hit = sphere.intersect(&ray);
        assert_eq!(hit, None);
    }

    #[test]
    fn test_intersect_composite() {
        let objects: Vec<Box<dyn Shape>> = vec![
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, 20.0), 10.0)),
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, 20.0), 10.0)),
            Box::new(Sphere::new(Vec3::new(20.0, 0.0, 20.0), 10.0)),
        ];

        let hit = intersect_composite(
            &objects,
            &Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        );
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().ray_t, 10.0);

        let hit = intersect_composite(
            &objects,
            &Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        );
        assert!(hit.is_none());

        let hit = intersect_composite(
            &objects,
            &Ray::new(Vec3::new(0.0, 0.0, 50.0), Vec3::new(0.0, 0.0, -1.0)),
        );
        assert!(hit.is_some());
        assert_eq!(hit.unwrap().ray_t, 20.0);

        let hit = intersect_composite(
            &objects,
            &Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(20.0, 0.0, 20.0)),
        );
        assert!(hit.is_some());
        assert_relative_eq!(
            hit.unwrap().ray_t,
            (20.0_f32.powf(2.0) + 20.0_f32.powf(2.0)).sqrt() - 10.0 // t = sqrt(x² + y² + z²) - r = sqrt(20² + 0² + 20²) - 10
        );
    }
}

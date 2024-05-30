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

use crate::algebra;
use crate::light::Ray;

#[derive(Debug, PartialEq)]
pub struct HitRecord {
    pub ray_t: f64,
    pub point: glm::DVec3,
    pub normal: glm::DVec3,
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord>;
}

/// Returns the closest positive distance (facing the direction of a Ray)
fn closest_facing_solution((t1, t2): (f64, f64)) -> Option<f64> {
    assert!(t1 <= t2);

    if t1 >= 0.0 {
        Some(t1)
    } else if t2 >= 0.0 {
        Some(t2)
    } else {
        None
    }
}

#[derive(Debug)]
pub struct Sphere {
    center: glm::DVec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: glm::DVec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> glm::DVec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn normal(&self, intersection: &glm::DVec3, direction: &glm::DVec3) -> glm::DVec3 {
        // -(d*n)n / |(d*n)n|
        let surf_normal = &self.center - intersection;
        (direction.dot(&surf_normal) * surf_normal).normalize()
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let oc: glm::DVec3 = ray.origin - self.center;
        let d: glm::DVec3 = ray.direction;

        let a: f64 = d.norm().powf(2.0);
        let b: f64 = 2.0 * oc.dot(&d);
        let c: f64 = oc.norm().powf(2.0) - self.radius.powf(2.0);

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
        let sphere = Sphere::new(glm::DVec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(
            glm::DVec3::new(0.0, 0.0, 20.0),
            glm::DVec3::new(0.0, 0.0, -1.0),
        );

        let hit = sphere.intersect(&ray).expect("Expected some HitRecord");

        assert_relative_eq!(glm::DVec3::new(0.0, 0.0, 10.0), hit.point);
        assert_relative_eq!(glm::DVec3::new(0.0, 0.0, -1.0), hit.normal);
    }

    #[test]
    fn intersect_from_inside_sphere() {
        let sphere = Sphere::new(glm::DVec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(
            glm::DVec3::new(0.0, 0.0, 0.0),
            glm::DVec3::new(0.0, 0.0, 1.0),
        );

        let hit_record = sphere.intersect(&ray).expect("Expected some HitRecord");

        assert_relative_eq!(glm::DVec3::new(0.0, 0.0, 10.0), hit_record.point);
        assert_relative_eq!(glm::DVec3::new(0.0, 0.0, 1.0), hit_record.normal);
    }

    #[test]
    fn no_intersect_behind_sphere() {
        let sphere = Sphere::new(glm::DVec3::new(0.0, 0.0, 0.0), 10.0);
        let ray = Ray::new(
            glm::DVec3::new(0.0, 0.0, 20.0),
            glm::DVec3::new(0.0, 0.0, 1.0),
        );

        let hit = sphere.intersect(&ray);
        assert_eq!(hit, None);
    }
}

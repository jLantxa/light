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

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            ray_t: f64::INFINITY,
            point: glm::DVec3::zeros(),
            normal: glm::DVec3::zeros(),
        }
    }
}

impl HitRecord {
    pub fn new() -> Self {
        Self::default()
    }
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

#[derive(Debug, Default)]
pub struct Triangle {
    pub va: glm::DVec3,
    pub vb: glm::DVec3,
    pub vc: glm::DVec3,
    normal: glm::DVec3,
}

impl Triangle {
    pub fn new(a: glm::DVec3, b: glm::DVec3, c: glm::DVec3) -> Self {
        Self {
            va: a,
            vb: b,
            vc: c,
            normal: (c - a).cross(&(b - a)).normalize(),
        }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        let edge1 = self.vb - self.va;
        let edge2 = self.vc - self.va;

        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < f64::EPSILON {
            return None; // The ray is parallel to this triangle.
        }

        let f = 1.0 / a;
        let s = ray.origin - self.va;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > f64::EPSILON {
            let hit_point = ray.origin + t * ray.direction;
            return Some(HitRecord {
                ray_t: t,
                point: hit_point,
                normal: self.normal,
            });
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
pub struct Sphere {
    pub center: glm::DVec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: glm::DVec3, radius: f64) -> Self {
        Self { center, radius }
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

#[derive(Debug, Default)]
pub struct Plane {
    pub position: glm::DVec3,
    pub normal: glm::DVec3,
}

impl Shape for Plane {
    fn intersect(&self, ray: &Ray) -> Option<HitRecord> {
        // Plane equation: (p - p0) . n = 0
        // Ray equation: p = o + t * d
        // Substituting ray equation into plane equation:
        // (o + t * d - p0) . n = 0
        // Solving for t: t = (p0 - o) . n / d . n

        let denom = ray.direction.dot(&self.normal);
        if denom.abs() > f64::EPSILON {
            let p0_to_origin = self.position - ray.origin;
            let t = p0_to_origin.dot(&self.normal) / denom;
            if t >= 0.0 {
                let hit_point = ray.origin + t * ray.direction;
                return Some(HitRecord {
                    ray_t: t,
                    point: hit_point,
                    normal: self.normal,
                });
            }
        }
        None
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

    #[test]
    fn test_plane_intersection_hit() {
        let plane = Plane {
            position: glm::DVec3::new(0.0, 0.0, 0.0),
            normal: glm::DVec3::new(0.0, 1.0, 0.0),
        };

        let ray = Ray {
            origin: glm::DVec3::new(0.0, -1.0, 0.0),
            direction: glm::DVec3::new(0.0, 1.0, 0.0),
        };

        let hit_record = plane.intersect(&ray);

        assert!(hit_record.is_some());
        let hit_record = hit_record.unwrap();
        assert_eq!(hit_record.ray_t, 1.0);
        assert_eq!(hit_record.point, glm::DVec3::new(0.0, 0.0, 0.0));
        assert_eq!(hit_record.normal, glm::DVec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_plane_intersection_miss() {
        let plane = Plane {
            position: glm::DVec3::new(0.0, 0.0, 0.0),
            normal: glm::DVec3::new(0.0, 1.0, 0.0),
        };

        let ray = Ray {
            origin: glm::DVec3::new(0.0, 1.0, 0.0),
            direction: glm::DVec3::new(0.0, 1.0, 0.0),
        };

        let hit_record = plane.intersect(&ray);

        assert!(hit_record.is_none());
    }

    #[test]
    fn test_triangle_intersection_hit() {
        let triangle = Triangle::new(
            glm::DVec3::new(0.0, 0.0, 0.0),
            glm::DVec3::new(1.0, 0.0, 0.0),
            glm::DVec3::new(0.0, 1.0, 0.0),
        );

        let ray = Ray {
            origin: glm::DVec3::new(0.1, 0.1, -1.0),
            direction: glm::DVec3::new(0.0, 0.0, 1.0),
        };

        let hit_record = triangle.intersect(&ray);

        assert!(hit_record.is_some());
        let hit_record = hit_record.unwrap();
        assert!(hit_record.ray_t > 0.0);
        assert_eq!(hit_record.point, glm::DVec3::new(0.1, 0.1, 0.0));
        assert_eq!(hit_record.normal, triangle.normal);
    }

    #[test]
    fn test_triangle_intersection_miss() {
        let triangle = Triangle::new(
            glm::DVec3::new(0.0, 0.0, 0.0),
            glm::DVec3::new(1.0, 0.0, 0.0),
            glm::DVec3::new(0.0, 1.0, 0.0),
        );

        let ray = Ray {
            origin: glm::DVec3::new(1.0, 1.0, -1.0),
            direction: glm::DVec3::new(0.0, 0.0, 1.0),
        };

        let hit_record = triangle.intersect(&ray);

        assert!(hit_record.is_none());
    }
}

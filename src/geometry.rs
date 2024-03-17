use crate::algebra::Vec3;
use crate::light::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> f32;
    fn intersection_normal(&self, intesection_point: &Vec3) -> Vec3;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    fn center(&self) -> Vec3 {
        self.center
    }

    fn radius(&self) -> f32 {
        self.radius
    }
}

#[cfg(test)]
mod test {
    use super::*;
}

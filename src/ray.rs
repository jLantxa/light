use crate::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction.normal(),
        }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.origin
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_at() {
        let ray = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0));

        let component: f32 = (3.0 + 3.0_f32.sqrt()) / 3.0;
        assert_eq!(
            Vec3::new(component, component, component),
            ray.point_at(1.0)
        );
    }
}

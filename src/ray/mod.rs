pub use crate::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3::Point3,
    pub direction: Vec3::Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3::Point3 {
        return self.origin + t * self.direction;
    }

    pub fn ray_color(&self) -> Vec3::Color {
        let unit_direction = self.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t)
            * Vec3::Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * Vec3::Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };
    }
}

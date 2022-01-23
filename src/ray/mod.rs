pub use crate::vec3::{Color, Point3, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t * self.direction;
    }

    fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * oc.dot(self.direction);
        let c = oc.dot(oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return -1.0;
        } else {
            return (-b - discriminant.sqrt()) / (2.0 * a);
        }
    }

    pub fn ray_color(&self) -> Color {
        let mut t = self.hit_sphere(
            Point3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
        );

        if t > 0.0 {
            let N = (self.at(t)
                - Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                })
            .unit();
            return 0.5
                * Color {
                    x: N.x + 1.0,
                    y: N.y + 1.0,
                    z: N.z + 1.0,
                };
        }

        let unit_direction = self.direction.unit();
        t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t)
            * Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }
            + t * Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };
    }
}

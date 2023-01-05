use vec3::*;
use ray::*;

pub mod ray;
pub mod vec3;

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };

    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
    };

    //Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);

            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };

            let color = r.ray_color();

            write_as_color(color);
        }
    }
    eprintln!("\nDone.\n");
}

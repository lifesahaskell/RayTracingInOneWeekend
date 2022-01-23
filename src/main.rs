pub mod Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;
    /*
    println!("P3\n {} {}\n255\n", image_width, image_height);

    for y in (0..image_height - 1).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
    eprintln!("\nDone.\n"); */

    let mut vec3 = Vec3::Vec3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    vec3 *= 3.14;
    assert_eq!(
        Vec3::Vec3 {
            x: 3.14,
            y: 6.28,
            z: 9.42
        },
        vec3,
        "Checking that {} and {} are equal.",
        Vec3::Vec3 {
            x: 3.14,
            y: 6.28,
            z: 9.41
        },
        vec3,
    );
}

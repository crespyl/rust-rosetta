// Implements http://rosettacode.org/wiki/Mandelbrot_set
// Writes image to a PPM file: "mandelbrot.ppm"
use std::fs::File;
use std::io::Write;
use std::num::Float;

#[cfg(not(test))]
fn main() {
    // max screen coordinates
    let (ix_max, iy_max) = (800usize, 800usize);

    // world coordinates
    let (cx_min, cx_max) = (-2.5f32, 1.5f32);
    let (cy_min, cy_max) = (-2.0f32, 2.0f32);

    let pixel_width = (cx_max - cx_min) / ix_max as f32;
    let pixel_height = (cy_max - cy_min) / iy_max as f32;

    let iteration_max = 200usize;
    let escape_radius = 2.0 * 2.0;

    // create file, and write PPM header
    let mut f = File::create("mandelbrot.ppm").unwrap();
    f.write_all(format!("P6\n {}\n {}\n {}\n {}\n", "# ", ix_max, iy_max, 255).as_bytes());

    // compute and write image bytes
    for iy in 0..iy_max {
        let mut cy = cy_min + (iy as f32) * pixel_height;
        if cy.abs() < pixel_height / 2.0 { cy = 0.0; }

        for ix in 0..ix_max {
            let cx = cx_min + (ix as f32) * pixel_width;
            let mut zx = 0.0;
            let mut zy = 0.0;

            let mut iteration = 0;
            while iteration < iteration_max && (zx*zx + zy*zy) < escape_radius {
                let (zy_next, zx_next) = (2.0 * zx * zy + cy, (zx * zx) - (zy * zy) + cx);
                zy = zy_next;
                zx = zx_next;

                iteration += 1;
            }

            let color = if iteration == iteration_max {
                // inside set, black
                [0u8; 3]
            } else {
                // outside set, grayscale
                [iteration as u8; 3]
            };

            f.write(&color);
        }
    }
}

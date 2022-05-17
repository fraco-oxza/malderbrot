#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

use std::time::{Duration, Instant};

use indicatif::ProgressBar;
use malderbrot::*;

const MAX_ITERATION: u64 = 1000;
const RESOLUTION: u32 = 10000;

fn main() {
    // let bar = ProgressBar::new(RESOLUTION as u64 * RESOLUTION as u64);
    // bar.set_draw_rate(1);
    let res_float = RESOLUTION as f64;
    let va = 255.0 / MAX_ITERATION as f64;
    let mut img = Image::new(RESOLUTION, RESOLUTION);
    let mut r;
    let mut color;
    let start = Instant::now();
    for (x, y) in img.coordinates() {
        r = mandelbrot(
            ((2.5 * (x as f64)) / (res_float)) - 2.0,
            ((2.5 * (y as f64)) / (res_float)) - 1.25,
            MAX_ITERATION,
        );

        color = 255 - (va * r as f64) as u8;
        img.set_pixel(x, y, px!(color, color, color));
        // bar.inc(1);
    }
    let duration = start.elapsed().as_secs_f32();
    // bar.finish();
    println!("Ejecutado en {:.4}s", duration);
    img.save("img.bmp").unwrap();
}

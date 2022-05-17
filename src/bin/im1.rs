#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

use std::time::Instant;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use malderbrot::*;

const MAX_ITERATION: u64 = 100000;
const RESOLUTION: u32 = 27000;

fn main() {
    let max = RESOLUTION as u64 * RESOLUTION as u64;
    let bar = ProgressBar::new(max);
    // bar.set_draw_rate(1);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(&format!(
                "{{prefix:.bold}}▕{{bar:.{}}}▏{{percent}}% ({{eta}})",
                "yellow"
            ))
            .progress_chars("█▉▊▋▌▍▎▏  "),
    );
    bar.set_prefix("Calculating: ");
    bar.set_draw_rate(4);
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
        img.set_pixel(x, y, px!(color, color / 2, color / 3));
        bar.inc(1);
    }
    let duration = start.elapsed().as_secs();
    bar.finish();
    println!("Ejecutado en {}s", duration);
    println!("Saving...");
    img.save("img.bmp").unwrap();
}

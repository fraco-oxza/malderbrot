pub fn mandelbrot(x0: f64, y0: f64, max_iteration: u64) -> u64 {
    let mut x = 0.0;
    let mut y = 0.0;

    let mut i = 0;
    let mut xtemp;
    while x * x + y * y <= 4.0 && i < max_iteration {
        xtemp = x * x - y * y + x0;
        y = 2.0 * x * y + y0;
        x = xtemp;
        i += 1;
    }
    i
}

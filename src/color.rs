extern crate image;

use image::Rgba;

pub fn distance_srgb(a: Rgba<u8>, b: Rgba<u8>) -> f64 {
    let [a_r, a_g, a_b, _] = a.0;
    let [b_r, b_g, b_b, _] = b.0;

    let rbar = ((a_r - b_r) as f64) / 2.0;

    (
        (2.0 + rbar / 256.0) * ((a_r - b_r).pow(2) as f64) +
        4.0 * ((a_g - b_g).pow(2) as f64) +
        (2.0 + (255.0 - rbar) / 256.0) * ((a_b - b_b).pow(2) as f64)
    ).sqrt()
}

pub fn close_enough(want: Rgba<u8>, got: Rgba<u8>) -> bool {
    let tolerance: f64 = 50.0;
    distance_srgb(want, got) <= tolerance
}
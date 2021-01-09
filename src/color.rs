extern crate image;
extern crate regex;

use image::Rgba;
use regex::Regex;

fn unpack_rgba_floats(color: Rgba<u8>) -> (f64, f64, f64, f64) {
    let [r, g, b, a] = color.0;
    (r as f64, g as f64, b as f64, a as f64)
}

pub fn distance_srgb(a: Rgba<u8>, b: Rgba<u8>) -> f64 {
    let (a_r, a_g, a_b, _) = unpack_rgba_floats(a);
    let (b_r, b_g, b_b, _) = unpack_rgba_floats(b);

    let rbar = ((a_r - b_r) as f64) / 2.0;

    (
        (2.0 + rbar / 256.0) * (a_r - b_r).powi(2) +
        4.0 * (a_g - b_g).powi(2) +
        (2.0 + (255.0 - rbar) / 256.0) * (a_b - b_b).powi(2)
    ).sqrt()
}

pub fn close_enough(want: Rgba<u8>, got: Rgba<u8>) -> bool {
    let tolerance: f64 = 50.0;
    distance_srgb(want, got) <= tolerance
}

pub fn try_parse_rgba(color_hex: &str) -> Option<Rgba<u8>> {
    let re = Regex::new(r"^#?([A-Fa-f0-9]{2}[A-Fa-f0-9]{2}[A-Fa-f0-9]{2})$").unwrap();
    match re.captures(&color_hex) {
        Some(captures) => {
            // the following should just work, assuming the regex capture did its job
            let rgb = captures.get(1).unwrap().as_str();
            let bytes = hex::decode(rgb).unwrap();
            Some(Rgba([bytes[0], bytes[1], bytes[2], 255]))
        },
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_parse_hex_color_string() {
        assert_eq!(try_parse_rgba("#ff00ff").unwrap(), Rgba([255, 0, 255, 255]));
    }

    #[test]
    fn test_invalid_color_fails_parse() {
        assert!(try_parse_rgba("literal garbage").is_none());
    }
}
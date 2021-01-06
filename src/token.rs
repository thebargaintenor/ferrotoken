extern crate image;

use array2d::Array2D;
use image::{Rgba, RgbaImage};

use crate::color;

/// Defines X, Y, width, and height of a rectangle
type Rectangle = (usize, usize, usize, usize);

struct TransparencyMask {
    bounds: Rectangle,
    filter: Array2D<bool>,
}

fn get_transparency_mask(mask_color: Rgba<u8>, image: RgbaImage) -> Array2D<bool> {
    let (width, height) = image.dimensions();

    let mask = image.enumerate_pixels()
        .map(|(_x, _y, pixel)| {
            color::close_enough(mask_color, *pixel)
        });

    Array2D::from_iter_row_major(
        mask, 
        height as usize, 
        width as usize,
    )
}

fn find_mask_bounds(mask_color: Rgba<u8>, image: RgbaImage) -> Option<TransparencyMask> {
    let mask = get_transparency_mask(mask_color, image);

    let bounds = (
        // left
        (0..mask.num_columns()).position(|i| mask.column_iter(i).any(|&a| a)),
        // top
        (0..mask.num_rows()).position(|i| mask.row_iter(i).any(|&a| a)),
        // right
        (0..mask.num_columns()).rposition(|i| mask.column_iter(i).any(|&a| a)),
        // bottom
        (0..mask.num_rows()).rposition(|i| mask.row_iter(i).any(|&a| a)),
    );

    match bounds {
        (Some(left), Some(top), Some(right), Some(bottom)) => Some(TransparencyMask {
            bounds: (left, top, right - left, bottom - top),
            filter: mask,
        }),
        _ => None,
    }
}
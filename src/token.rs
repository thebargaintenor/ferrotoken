extern crate image;

use array2d::Array2D;
use image::{ImageBuffer, Rgba, RgbaImage, SubImage, imageops};
use image::math::Rect;

use crate::color;

struct TransparencyMask {
    bounds: Rect,
    filter: Array2D<bool>,
}

fn create_filter_from_color(mask_color: Rgba<u8>, image: &RgbaImage) -> Array2D<bool> {
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

fn find_mask_bounds(mask: &Array2D<bool>) -> Option<Rect> {
    let bounds = (
        // left
        (0..mask.num_columns())
            .position(|i| mask.column_iter(i).any(|&a| a)),
        // top
        (0..mask.num_rows())
            .position(|i| mask.row_iter(i).any(|&a| a)),
        // right
        (0..mask.num_columns())
            .rposition(|i| mask.column_iter(i).any(|&a| a)),
        // bottom
        (0..mask.num_rows())
            .rposition(|i| mask.row_iter(i).any(|&a| a)),
    );

    match bounds {
        (Some(left), Some(top), Some(right), Some(bottom)) => Some(
            Rect {
                x: left as u32,
                y: top as u32,
                width: (right - left + 1) as u32,
                height: (bottom - top + 1) as u32,
            },
        ),
        _ => None,
    }
}

fn get_transparency_mask(mask_color: Rgba<u8>, image: &RgbaImage) -> Option<TransparencyMask> {
    let mask = create_filter_from_color(mask_color, image);

    match find_mask_bounds(&mask) {
        Some(rect) => Some(TransparencyMask {bounds: rect, filter: mask}),
        _ => None,
    }
}

fn scale_to_fill_viewport(viewport: Rect, img: &mut RgbaImage) -> RgbaImage {
    let img_width = img.width() as f64;
    let img_height = img.height() as f64;
    let img_aspect = img_width / img_height;

    let viewport_aspect = viewport.width as f64 / viewport.height as f64;

    if viewport_aspect >= img_aspect {
        let scale_factor = viewport.width as f64 / img_width;
        let scaled_height: u32 = (img_width / img_aspect * scale_factor).ceil() as u32;
        imageops::resize(img, viewport.width, scaled_height, imageops::FilterType::CatmullRom)
    } else {
        let scale_factor = viewport.height as f64 / img_height;
        let scaled_width: u32 = (img_height * img_aspect * scale_factor).ceil() as u32;
        imageops::resize(img, scaled_width, viewport.height, imageops::FilterType::CatmullRom)
    }
}

fn crop_to_viewport(viewport: Rect, img: &mut RgbaImage) -> SubImage<&mut RgbaImage> {
    let x = ((img.width() as f64 - viewport.width as f64) / 2.0).max(0.0) as u32;
    let y = ((img.height() as f64 - viewport.height as f64) / 2.0).max(0.0) as u32;
    let width = viewport.width.min(img.width());
    let height = viewport.height.max(img.height());

    imageops::crop(img, x, y, width, height)
}

fn merge_images(mask: TransparencyMask, template: &RgbaImage, mut content: &mut RgbaImage) -> RgbaImage {
    let mut scaled = scale_to_fill_viewport(mask.bounds, &mut content);
    let cropped = crop_to_viewport(mask.bounds, &mut scaled).to_image();

    let mut token = ImageBuffer::from_pixel(template.width(), template.height(), Rgba([0, 0, 0, 0]));
    for (x, y, pixel) in template.enumerate_pixels() {
        token.put_pixel(x, y, pixel.clone())
    }

    for x in 0..mask.bounds.width {
        for y in 0..mask.bounds.height {
            let tx = x + mask.bounds.x;
            let ty = y + mask.bounds.y;
            // Array2D uses (row, column) for retrieval
            if mask.filter[(ty as usize, tx as usize)] {
                token.put_pixel(tx, ty, cropped.get_pixel(x, y).clone());
            }
        }
    }

    token
}

pub fn create(mask_color: Rgba<u8>, template: RgbaImage, content: &mut RgbaImage) -> Option<RgbaImage> {
    match get_transparency_mask(mask_color, &template) {
        Some(mask) => Some(merge_images(mask, &template, content)),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use image::ImageBuffer;

    use super::*;

    #[test]
    fn test_array_contains_mask() {
        let rows = vec![
            vec![false, false, false, true, false],
            vec![false, false, true, false, false],
            vec![false, true, false, true, false],
            vec![false, false, true, false, false],
            vec![false, false, false, false, false],
        ];

        let mask = Array2D::from_rows(&rows);
        let bounds = find_mask_bounds(&mask).unwrap();

        assert_eq!(bounds, Rect {x: 1, y: 0, width: 3, height: 4});
    }

    #[test]
    fn test_array_contains_no_mask() {
        let rows = vec![
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
            vec![false, false, false, false, false],
        ];

        let mask = Array2D::from_rows(&rows);
        let bounds = find_mask_bounds(&mask);

        assert!(bounds.is_none());
    }

    #[test]
    fn test_image_contains_transparency_mask() {
        let mask_pixels: Vec<(u32, u32)> = vec![
            (3, 0),
            (2, 1),
            (1, 2),
            (3, 2),
            (2, 3),
        ];

        let mut img: RgbaImage = ImageBuffer::new(5, 5);
        let mask_color: Rgba<u8> = Rgba([255, 0, 255, 255]);
        for (x, y) in mask_pixels {
            img.put_pixel(x, y, mask_color);
        }

        let transparency_mask = get_transparency_mask(mask_color, &img).unwrap();
        assert_eq!(transparency_mask.bounds, Rect {x: 1, y: 0, width: 3, height: 4});

        let filter_check = find_mask_bounds(&transparency_mask.filter).unwrap();
        assert_eq!(filter_check, Rect {x: 1, y: 0, width: 3, height: 4});
    }

    #[test]
    fn test_image_contains_no_mask() {
        let img: RgbaImage = ImageBuffer::new(5, 5);
        let mask_color: Rgba<u8> = Rgba([255, 0, 255, 255]);

        let transparency_mask = get_transparency_mask(mask_color, &img);
        assert!(transparency_mask.is_none());
    }
}
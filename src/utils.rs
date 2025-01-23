use image::{DynamicImage, GenericImageView, ImageError, Pixel};

pub fn save(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    img.save(path_out)?;
    Ok(())
}

pub fn get_pixel(img: &DynamicImage, x: u32, y: u32) -> image::Rgb<u8> {
    let pixel = img.get_pixel(x, y);
    let channels = pixel.channels();
    image::Rgb([channels[0], channels[1], channels[2]])
}

pub fn get_light(pixel: image::Rgb<u8>) -> f32 {
    let channels = pixel.channels();
    // !  d'après la formule de luminance
    let light =
        0.2126 * channels[0] as f32 + 0.7152 * channels[1] as f32 + 0.0722 * channels[2] as f32;
    light / 255.0
}

pub fn get_closest_color(pixel: image::Rgb<u8>, colors: &[image::Rgb<u8>]) -> image::Rgb<u8> {
    // Check if the pixel is already in the palette to early return
    if let Some(&color) = colors.iter().find(|&&c| c == pixel) {
        return color;
    }

    let mut min_distance = f32::MAX;
    let mut closest_color = colors[0];

    for color in colors.iter() {
        let distance = ((color[0] as f32 - pixel[0] as f32).powi(2)
            + (color[1] as f32 - pixel[1] as f32).powi(2)
            + (color[2] as f32 - pixel[2] as f32).powi(2))
        .sqrt();

        if distance < min_distance {
            min_distance = distance;
            closest_color = *color;
        }
    }

    closest_color
}

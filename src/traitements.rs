
use image::{DynamicImage, ImageBuffer, ImageError, RgbImage, GenericImageView, Pixel};
use crate::constants::{WHITE, GREY, BLACK, BLUE, RED, GREEN, YELLOW, MAGENTA, CYAN, COLORS};
use crate::utils::{get_light, get_closest_color, save};



pub fn traitement_split_white(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let (width, height) = img.dimensions(); 
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            if (x + y) % 2 == 0 {
                img_out.put_pixel(x, y, image::Rgb([255, 255, 255]));
            } else {
                img_out.put_pixel(x, y, img.get_pixel(x, y).to_rgb());
            }
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_monochrome(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let light = get_light(pixel);
            let new_pixel = if light > 127 {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            };
            img_out.put_pixel(x, y, new_pixel);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_paire_palette(
    img: &DynamicImage,
    path_out: String,
    couleur1: String,
    couleur2: String,
) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    let couleur1 = match couleur1.as_str() {
        "white" => WHITE,
        "grey" => GREY,
        "black" => BLACK,
        "blue" => BLUE,
        "red" => RED,
        "green" => GREEN,
        "yellow" => YELLOW,
        "magenta" => MAGENTA,
        "cyan" => CYAN,
        _ => WHITE,
    };

    let couleur2 = match couleur2.as_str() {
        "white" => WHITE,
        "grey" => GREY,
        "black" => BLACK,
        "blue" => BLUE,
        "red" => RED,
        "green" => GREEN,
        "yellow" => YELLOW,
        "magenta" => MAGENTA,
        "cyan" => CYAN,
        _ => WHITE,
    };

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let light = get_light(pixel);
            let new_pixel = if light > 127 { couleur1 } else { couleur2 };
            img_out.put_pixel(x, y, new_pixel);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_palette(
    img: &DynamicImage,
    path_out: String,
    _n_couleurs: usize,
) -> Result<(), ImageError> {
    // take the _n_couleurs first colors of the COLORS array and create a new array, and then replace all pixels by the closest color in the new array
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);
    let colors: Vec<image::Rgb<u8>> = COLORS.iter().take(_n_couleurs).cloned().collect();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let new_pixel = get_closest_color(pixel, &colors);
            img_out.put_pixel(x, y, new_pixel);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_dithering(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let seuil = rand::random::<f32>(); // Génération d'un nombre entre 0 et 1
            let light = get_light(pixel) as f32 / 255.0; // Normalisation pour comparaison avec le seuil
            let new_pixel = if light > seuil {
                WHITE // Appel à la constante WHITE
            } else {
                BLACK // Appel à la constante BLACK
            };
            img_out.put_pixel(x, y, new_pixel);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_ordered_dithering(img: &DynamicImage, path_out: String, bayer_matrix: [[u8; 4]; 4]) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let seuil = bayer_matrix[y as usize % 4][x as usize % 4] as f32 / 16.0; // Normalisation pour comparaison avec le seuil
            let light = get_light(pixel) as f32 / 255.0; // Normalisation pour comparaison avec le seuil
            let new_pixel = if light > seuil {
                WHITE // Appel à la constante WHITE
            } else {
                BLACK // Appel à la constante BLACK
            };
            img_out.put_pixel(x, y, new_pixel);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}
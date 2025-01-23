use crate::constantes::{BLACK, BLUE, COLORS, CYAN, GREEN, GREY, MAGENTA, RED, WHITE, YELLOW};
use crate::matrice_erreur::matrice_erreur::MatriceErreur;
use crate::utils::{get_closest_color, get_light, save, get_pixel};
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageError, RgbImage};

pub fn traitement_split_white(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let mut img_out: RgbImage = img.clone().to_rgb8();

    for (x, y, pixel) in img_out.enumerate_pixels_mut() {
        if (x + y) % 2 == 0 {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_monochrome(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let mut img_out: RgbImage = img.clone().to_rgb8();

    for (_, _, pixel) in img_out.enumerate_pixels_mut() {
        let light = get_light(*pixel);
        let new_pixel = if light > 0.5 {
            image::Rgb([255, 255, 255])
        } else {
            image::Rgb([0, 0, 0])
        };
        *pixel = new_pixel;
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
    let mut img_out: RgbImage = img.clone().to_rgb8();

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

    for (_, _, pixel) in img_out.enumerate_pixels_mut() {
        let light = get_light(*pixel);
        let new_pixel = if light > 0.5 { couleur1 } else { couleur2 };
        *pixel = new_pixel;
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
    let mut img_out: RgbImage = img.clone().to_rgb8();
    let colors: Vec<image::Rgb<u8>> = COLORS.iter().take(_n_couleurs).cloned().collect();

    for (_, _, pixel) in img_out.enumerate_pixels_mut() {
        let new_pixel = get_closest_color(*pixel, &colors);
        *pixel = new_pixel;
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_dithering(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let mut img_out: RgbImage = img.clone().to_rgb8();

    for (_, _, pixel) in img_out.enumerate_pixels_mut() {
        let light = get_light(*pixel); // Luminance du pixel
        let seuil = rand::random::<f32>(); // Génération d'un nombre entre 0 et 1
        let new_pixel = if light > seuil {
            WHITE // Appel à la constante WHITE
        } else {
            BLACK // Appel à la constante BLACK
        };
        *pixel = new_pixel;
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

pub fn traitement_ordered_dithering(
    img: &DynamicImage,
    path_out: String,
    bayer_matrix: [[u8; 4]; 4],
) -> Result<(), ImageError> {
    let mut img_out: RgbImage = img.clone().to_rgb8();

    for (x, y, pixel) in img_out.enumerate_pixels_mut() {
        let seuil = bayer_matrix[y as usize % 4][x as usize % 4] as f32 / 16.0; // Normalisation pour comparaison avec le seuil
        let light = get_light(*pixel); // Luminance du pixel
        let new_pixel = if light > seuil {
            WHITE // Appel à la constante WHITE
        } else {
            BLACK // Appel à la constante BLACK
        };
        *pixel = new_pixel;
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

// TODO: fix the error diffusion
/**
 * Applique le traitement de diffusion d'erreur sur l'image passée en paramètre.
 * Prend une matrice en entrée comme ça on définit celle qu'on veut
 * par exemple qst16 c'estune matrice avec des coeff 0.5 mais qst19 c'est matrice de Floyd-Steinberg
 */
pub fn traitement_error_diffusion(
    img: &DynamicImage,
    path_out: String,
    matrice_erreur: &MatriceErreur
) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    // Convertir l'image en niveaux de gris (noir et blanc)
    let mut luminances: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = get_pixel(img, x, y); // Utilisation de la méthode utilitaire
            luminances[y as usize][x as usize] = get_light(pixel); // Calcul de la luminance
        }
    }

    // Parcourir chaque pixel
    for y in 0..height as usize {
        for x in 0..width as usize {
            let old_luminance = luminances[y][x];
            let new_luminance = if old_luminance > 0.5 { 1.0 } else { 0.0 }; // Seuil binaire
            let error = old_luminance - new_luminance;

            // Définir la nouvelle couleur (noir ou blanc)
            let new_pixel = if new_luminance == 1.0 {
                WHITE
            } else {
                BLACK
            };
            img_out.put_pixel(x as u32, y as u32, new_pixel);

            // Diffuser l'erreur aux voisins en utilisant la matrice d'erreur
            for row in 0..matrice_erreur.matrix.len() {
                for col in 0..matrice_erreur.matrix[row].len() {
                    // Récupérer le coefficient de la matrice d'erreur
                    let coefficient = matrice_erreur.get_value(row, col).unwrap_or(0.0);
                    let nx = x as isize + col as isize - matrice_erreur.x_origin as isize;
                    let ny = y as isize + row as isize;

                    // Vérifier que les indices voisins sont dans les limites de l'image
                    if coefficient != 0.0
                        && nx >= 0
                        && nx < width as isize
                        && ny >= 0
                        && ny < height as isize
                    {
                        luminances[ny as usize][nx as usize] += error * coefficient as f32;
                    }
                }
            }
        }
    }

    // Convertir le buffer en DynamicImage et sauvegarder
    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out) // Utilisation de la méthode utilitaire pour sauvegarder
}

// TODO: fix the error diffusion
/**
 * Applique le traitement de diffusion d'erreur sur l'image passée en paramètre.
 * Prend une matrice en entrée comme ça on définit celle qu'on veut
 * par exemple qst16 c'estune matrice avec des coeff 0.5 mais qst19 c'est matrice de Floyd-Steinberg
 * Prend une paire de 2 couleurs en entrée pour définir les couleurs à utiliser
 */
pub fn traitement_error_diffusion_colors(
    img: &DynamicImage,
    path_out: String,
    matrice_erreur: &MatriceErreur,
    couleur1: String,
    couleur2: String
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

    // Convertir l'image en niveaux de gris (noir et blanc)
    let mut luminances: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = get_pixel(img, x, y); // Utilisation de la méthode utilitaire
            luminances[y as usize][x as usize] = get_light(pixel); // Calcul de la luminance
        }
    }

    // Parcourir chaque pixel
    for y in 0..height as usize {
        for x in 0..width as usize {
            let old_luminance = luminances[y][x];
            let new_luminance = if old_luminance > 0.5 { 1.0 } else { 0.0 }; // Seuil binaire
            let error = old_luminance - new_luminance;

            // Définir la nouvelle couleur (noir ou blanc)
            let new_pixel = if new_luminance == 1.0 {
                couleur1
            } else {
                couleur2
            };
            img_out.put_pixel(x as u32, y as u32, new_pixel);

            // Diffuser l'erreur aux voisins en utilisant la matrice d'erreur
            for row in 0..matrice_erreur.matrix.len() {
                for col in 0..matrice_erreur.matrix[row].len() {
                    // Récupérer le coefficient de la matrice d'erreur
                    let coefficient = matrice_erreur.get_value(row, col).unwrap_or(0.0);
                    let nx = x as isize + col as isize - matrice_erreur.x_origin as isize;
                    let ny = y as isize + row as isize;

                    // Vérifier que les indices voisins sont dans les limites de l'image
                    if coefficient != 0.0
                        && nx >= 0
                        && nx < width as isize
                        && ny >= 0
                        && ny < height as isize
                    {
                        luminances[ny as usize][nx as usize] += error * coefficient as f32;
                    }
                }
            }
        }
    }

    // Convertir le buffer en DynamicImage et sauvegarder
    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out) // Utilisation de la méthode utilitaire pour sauvegarder
}
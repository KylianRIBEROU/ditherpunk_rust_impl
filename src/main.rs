use argh::FromArgs;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageError, Pixel, RgbImage};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Pixel(OptsPixel),
    SplitWhite(OptsSplitWhite),
    Couleurs(OptsCouleurs),
    Tramage(OptsTramage),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "pixel")]
/// Affiche la couleur du pixel à la position (x, y)
struct OptsPixel {
    #[argh(option, description = "coordonnées x du pixel")]
    x: usize,
    #[argh(option, description = "coordonnées y du pixel")]
    y: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "split_white")]
/// Rendu de l'image en alternant les pixels en blanc
struct OptsSplitWhite {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "couleurs")]
/// Rendu de l’image par seuillage avec des couleurs personnalisées.
struct OptsCouleurs {
    /// la première couleur (format: 'red' / 'grey' / 'blue' / 'green' / 'yellow' / 'magenta' / 'cyan')
    #[argh(option)]
    couleur1: String,

    /// la seconde couleur (format: 'red' / 'grey' / 'blue' / 'green' / 'yellow' / 'magenta' / 'cyan')
    #[argh(option)]
    couleur2: String,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "tramage")]
/// Rendu de l’image par tramage aléatoire
struct OptsTramage {}

const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);

const COLORS: [image::Rgb<u8>; 8] = [BLACK, WHITE, RED, GREEN, BLUE, YELLOW, MAGENTA, CYAN];

// Utils

fn save(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    img.save(path_out)?;
    Ok(())
}

fn get_pixel(img: &DynamicImage, x: u32, y: u32) -> image::Rgb<u8> {
    let pixel = img.get_pixel(x, y);
    let channels = pixel.channels();
    image::Rgb([channels[0], channels[1], channels[2]])
}

fn get_light(pixel: image::Rgb<u8>) -> u8 {
    let channels = pixel.channels();
    // ! d'après la formule de luminance
    let light =
        0.2126 * channels[0] as f32 + 0.7152 * channels[1] as f32 + 0.0722 * channels[2] as f32;
    light as u8
}

fn get_closest_color(pixel: image::Rgb<u8>, colors: &[image::Rgb<u8>]) -> image::Rgb<u8> {
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

// Traitements

fn traitement_split_white(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
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

fn traitement_monochrome(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
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

fn traitement_paire_palette(
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

fn traitement_palette(
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

// Dithering (tramage aléatoire, pour chaque pixel, on genere une valeur entre 0 et 1, puis on le multiplie à chaque valeur RGB du pixel, puis si la valeur est supérieure à 127, on met le pixel en blanc, sinon en noir)
fn traitement_dithering(img: &DynamicImage, path_out: String) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let light = get_light(pixel);
            let random = rand::random::<u8>() as f32 / 255.0;
            let new_pixel = if light as f32 * random > 127.0 {
                WHITE
            } else {
                BLACK
            };
            img_out.put_pixel(x, y, new_pixel);
        }
    }

    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}

fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    let path_out = args.output.unwrap_or("./exports/default.png".to_string());
    let mode = args.mode;

    println!("path_in: {}", path_in);
    println!("path_out: {}", path_out);

    // Lire l'image
    let img: DynamicImage = ImageReader::open(path_in)?.decode()?;
    println!("Dimensions: {:?}", img.dimensions());

    match mode {
        Mode::Seuil(_) => {
            println!("Mode seuil");
            traitement_monochrome(&img, path_out)?;
        }
        Mode::Palette(opts) => {
            println!("Mode palette: {:?} couleurs", opts.n_couleurs);
            if opts.n_couleurs > 8 {
                println!("Le nombre de couleurs doit être inférieur ou égal à 8");
                return Ok(());
            } else if opts.n_couleurs < 2 {
                println!("Le nombre de couleurs doit être supérieur ou égal à 2");
                return Ok(());
            } else {
                traitement_palette(&img, path_out, opts.n_couleurs)?;
            }
        }
        Mode::Pixel(opts) => {
            println!("Mode pixel: ({}, {})", opts.x, opts.y);
            let pixel_color = get_pixel(&img, opts.x as u32, opts.y as u32);
            println!(
                "Couleur du pixel à la position ({}, {}): {:?}",
                opts.x, opts.y, pixel_color
            );
        }
        Mode::SplitWhite(_) => {
            println!("Mode split white");
            traitement_split_white(&img, path_out)?;
        }
        Mode::Couleurs(opts) => {
            println!("Mode couleurs: {}, {}", opts.couleur1, opts.couleur2);
            traitement_paire_palette(&img, path_out, opts.couleur1, opts.couleur2)?;
        }
        Mode::Tramage(_) => {
            println!("Mode tramage");
            traitement_dithering(&img, path_out)?;
        }
    }

    Ok(())
}

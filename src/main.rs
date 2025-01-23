mod cli_arguments;
mod constantes;
mod matrice_erreur;
mod traitements;
mod utils;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageError;

use matrice_erreur::matrice_erreur::MatriceErreur;
use utils::get_pixel;

use cli_arguments::{DitherArgs, Mode};
use constantes::BAYER_MATRIX;
use traitements::{
    traitement_diffusion_erreur, traitement_dithering, traitement_monochrome,
    traitement_ordered_dithering, traitement_paire_palette, traitement_palette,
    traitement_split_white,
};

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
        Mode::Dithering(_) => {
            println!("Mode tramage");
            traitement_dithering(&img, path_out)?;
        }
        Mode::OrderedDithering(_) => {
            println!("Mode tramage ordonné");
            traitement_ordered_dithering(&img, path_out, BAYER_MATRIX)?;
        }
        Mode::ErrorDiffusion(_) => {
            println!("Mode diffusion d'erreur");
            //todo matrice

            let floyd_stenbeirg_matrice = MatriceErreur::new(
                1,
                vec![
                    vec![0.0, 0.4375],    // Droite
                    vec![0.1875, 0.3125], // Gauche-bas, bas
                    vec![0.0625],         // Droite-bas
                ],
            );
            let qst16_matrice = MatriceErreur::new(
                0, // La position courante (le pixel à traiter) est dans la première colonne
                vec![
                    vec![0.5], // Erreur diffusée à droite
                    vec![0.5], // Erreur diffusée en bas
                ],
            );

            traitement_diffusion_erreur(&img, path_out, &qst16_matrice)?;
        }
    }

    Ok(())
}

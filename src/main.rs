use argh::FromArgs;
use image::io::Reader as ImageReader;
use image::{GenericImageView, ImageError};

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

// const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
// const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
// const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
// const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
// const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
// const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
// const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
// const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
// const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);

fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();
    let path_in = args.input;
    let path_out = args.output.unwrap_or("./exports/default.png".to_string());
    let mode = args.mode;

    println!("path_in: {}", path_in);
    println!("path_out: {}", path_out);

    match mode {
        Mode::Seuil(_) => {
            println!("Mode seuil");
        }
        Mode::Palette(opts) => {
            println!("Mode palette: {:?}", opts.n_couleurs);
        }
    }

    // Lire l'image
    let img = ImageReader::open(path_in)?.decode()?;
    println!("Image dimensions: {:?}", img.dimensions());
    Ok(())
}

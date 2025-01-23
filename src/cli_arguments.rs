use argh::FromArgs;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
pub struct DitherArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    pub input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    pub output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
pub enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Pixel(OptsPixel),
    SplitWhite(OptsSplitWhite),
    Couleurs(OptsCouleurs),
    Dithering(OptsDithering),
    OrderedDithering(OptsOrderedDithering),
    ErrorDiffusion(OptsErrorDiffusion),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuil")]
/// Rendu de l’image par seuillage monochrome.
pub struct OptsSeuil {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
pub struct OptsPalette {
    #[argh(option, description = "nombre de couleurs")]
    pub n_couleurs: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "pixel")]
/// Affiche la couleur du pixel à la position (x, y)
pub struct OptsPixel {
    #[argh(option, description = "position x")]
    pub x: usize,
    #[argh(option, description = "position y")]
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "split_white")]
/// Rendu de l'image en alternant les pixels en blanc
pub struct OptsSplitWhite {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "couleurs")]
/// Rendu de l’image par seuillage avec des couleurs personnalisées.
pub struct OptsCouleurs {
    #[argh(option, description = "couleur 1")]
    pub couleur1: String,
    #[argh(option, description = "couleur 2")]
    pub couleur2: String,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "dithering")]
/// Rendu de l’image par tramage aléatoire
pub struct OptsDithering {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "ordered_dithering")]
/// Rendu de l’image par tramage ordonné
pub struct OptsOrderedDithering {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "error_diffusion")]
/// Rendu de l’image par diffusion d’erreur
pub struct OptsErrorDiffusion {}

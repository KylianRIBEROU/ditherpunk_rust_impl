# Ditherpunk: retour au monochrome

LAVENANT Jordan - RIBEROU Kylian - BUT 3

Groupe 31A

# Pré-requis

> Traitement 1, 2, 3, 4 **OBLIGATOIRE**

> Faire les traitements 5, 6, 7 **pour améliorer la note**

> Librairie **ARGH**: les triples `/` permettent de guider l'utilisateur à utiliser l'application en CLI (grâce à la commande cargo run -- --help)

# Lancement

Build

```bash
cargo build
```

Importer une image

```bash
# Importer une image avec le mode seuil
cargo run -- ./imports/test.jpg seuil
```

```bash
# Importer une image avec le mode palette (fournir l'argument supplémentaire)
cargo run -- ./imports/test.jpg palette --n-couleurs 5
```

# Questions

## Partie 1 - manipulation bibliothèque Image

### Question 1

Création d'un nouveau projet Cargo :
```toml
[package]
name = "ditherpunk"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
image = "0.24.9"
argh = "0.1.13"
```

### Question 2

- Pour ouvrir une image depuis un fichier, on utilise :

```bash
cargo run -- ./imports/test.jpg
```

- On obtient un DynamicImage, à quoi correspond ce type?

DynamicImage est un type de la bibliothèque Rust image, conçu pour gérer des images de formats variés (RGB, RGBA, Luma, etc.) et de types de données différents (entiers, flottants, etc.). Il simplifie la manipulation d'images en abstrahant leur format interne.

- Comment obtenir une image en mode rbg8

Pour convertir une image en mode `Rgb8` (3 canaux R, G, B, chacun représenté par un `u8`), utilisez la méthode `to_rgb8()` de `DynamicImage`.

```rust
// Lire l'image
let img: DynamicImage = ImageReader::open(path_in)?.decode()?;
let img_rgb8 = img.to_rgb8();
```

Une image arbitraire peut avoir des pixels de nature différente:

- avec un nombre variables de canaux (couleurs ou non, transparence ou non)
- avec un type de donnée différent pour les canaux (entiers sur un octet, flottants ou autres)
  Passer l’image d’entrée en mode rgb8, c’est-à-dire avec 3 canaux R, G, B, représentés chacun
  par un u8.

### Question 3

_Sauver l’image obtenue au format png._

```rust
let args: DitherArgs = argh::from_env();
let path_in = args.input;
let path_out = args.output.unwrap_or("./exports/default.png".to_string());

// Lire l'image
let img: DynamicImage = ImageReader::open(path_in)?.decode()?;
let img_rgb8 = img.to_rgb8();

// Sauvegarder l'image en mode Rgb8
let rgb8_path = format!("{}_rgb8.png", path_out.trim_end_matches(".png"));
img_rgb8.save(&rgb8_path)?;
```

Exportation de l'image : 

!['question3_export'](assets/rgb8_export.png)

_Que se passe-t-il si l’image de départ avait un canal
alpha?_

Si l'image de départ avait un canal alpha, la méthode `to_rgb8()` de `DynamicImage` supprime le canal alpha et ne conserve que les canaux R, G, B.

### Question 4

 _Afficher dans le terminal la couleur du pixel (32,52) de l’image de votre choix._

Pour cette question, nous avons créer un nouveau mode de traitement `pixel`, prenant des sous-paramètres `x` et `y` pour déterminer les coordonnées du pixel à afficher.

```rust
#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    Pixel(OptsPixel),
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
```

Fonction de récupération de la couleur du pixel :

```rust
fn get_pixel(img: &DynamicImage, x: u32, y: u32) -> image::Rgb<u8> {
    let pixel = img.get_pixel(x, y);
    let channels = pixel.channels();
    image::Rgb([channels[0], channels[1], channels[2]])
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
        }
        Mode::Palette(opts) => {
            println!("Mode palette: {:?}", opts.n_couleurs);
        }
        Mode::Pixel(opts) => {
            println!("Mode pixel: ({}, {})", opts.x, opts.y);
            let pixel_color = get_pixel(&img, opts.x as u32, opts.y as u32);
            println!(
                "Couleur du pixel à la position ({}, {}): {:?}",
                opts.x, opts.y, pixel_color
            );
        }
    }

    Ok(())
}
```

Instructions pour afficher la couleur du pixel (32, 52) :

```bash
cargo run -- ./imports/test.jpg pixel --x 32 --y 52
```

Affichage de la couleur du pixel (32, 52) :

!['question4'](assets/question4.png)


### Question 5
&
_Passer un pixel sur deux d’une image en blanc. Est-ce que l’image obtenue est reconnaissable?_

Traitement : 

```rust
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
```

Instructions pour passer un pixel sur deux en blanc :

```bash
cargo run -- ./imports/test.jpg ./exports/split_white.png split_white
```

Résultat de l'image obtenue :

!['question5'](exports/split_white.png)

## Partie 2 - passage en monochrome par seuillage

### Question 6

_Comment récupérer la luminosité d’un pixel?_

D'après la formule de luminance, la luminosité d'un pixel peut être calculée en multipliant les valeurs des canaux R, G, B par des coefficients de pondération reflétant la sensibilité de l'œil humain à ces couleurs. La luminosité est ensuite obtenue en sommant ces valeurs pondérées.

Rouge : `0.2126`  
Vert : `0.7152`  
Bleu : `0.0722`  

```rust
pub fn get_light(pixel: image::Rgb<u8>) -> f32 {
    let channels = pixel.channels();
    let light = 0.2126 * channels[0] as f32 + 0.7152 * channels[1] as f32 + 0.0722 * channels[2] as f32;
    light / 255.0
}
```

Ainsi, la luminosité d'un pixel est une valeur entre 0 et 255, où 0 représente le noir et 255 le blanc.

### Question 7

_Implémenter le traitement_

Traitement : 

```rust
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
```
Instructions pour passer une image en monochrome selon la luminosité de ses pixels :

```bash
cargo run -- ./imports/test.jpg ./exports/monochrome.png seuil
```

Résultat de l'image obtenue :

!['question7'](exports/monochrome.png)

### Question 8

Traitement permettant de passer une paire (2) de couleur à une image, en fonction de sa luminosité :

```rust
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
```

```rust
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
```

Instructions pour passer une paire de palette de couleurs à une image, en fonction de sa luminosité : 

```bash
cargo run -- ./imports/test.jpg ./exports/red_blue.png couleurs --couleur1 red --couleur2 blue
```

Résultat de l'image obtenue :

> Red / Blue
!['question8.0'](exports/red_blue.png)

```bash
cargo run -- ./imports/test.jpg ./exports/magenta_yellow.png couleurs --couleur1 magenta --couleur2 yellow
```

> Magenta / Yellow
!['question8.1'](exports/magenta_yellow.png)

```bash
cargo run -- ./imports/test.jpg ./exports/cyan_green.png couleurs --couleur1 cyan --couleur2 green
```

> Cyan / Green
!['question8.2'](exports/cyan_green.png)

## Partie 3 - Passage à une palette

### Question 9

_Comment calculer la distance entre deux couleurs? Indiquer dans le README la méthode de
calcul choisie._

Pour calculer la distance entre deux couleurs, on peut utiliser la distance euclidienne. On peut donc sommer la distance entre chaque attributs R, G, et B, étant des nombres compris entre 0 et 255. On compare ensuite la distance entre chaque attribut de chaque couleur, puis nous conservons la distance la plus courte, et ainsi la couleur la plus proche.

```rust
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
```

> On peut ensuite optimiser cette fonction en vérifiant si le pixel passé en paramètre est directement dans la palette, pour éviter de parcourir la palette entière à chaque fois.

### Question 10

_Implémenter le traitement_

Traitement : 

```rust
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
```

Instructions pour passer une image en palette de couleurs :

```bash
cargo run -- ./imports/test.jpg ./exports/palette.png palette --n-couleurs 3
```

(2 couleurs)

!['question10.1'](exports/palette_1.png)

(3 couleurs)

!['question10.2'](exports/palette_2.png)

(8 couleurs)

!['question10.3'](exports/palette_3.png)

### Question 11

_Votre application doit se comporter correctement si on donne une palette vide. Vous
expliquerez dans votre README le choix que vous avez fait dans ce cas._

## Partie 4 - tramage aléatoire

### Question 12

_Implémenter le tramage aléatoire des images._

Traitement : 

```rust
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
```
Instructions pour passer une image en tramage aléatoire :

```bash
cargo run -- ./imports/test.jpg ./exports/dithering.png dithering
```

## Partie 5 - Utilisation de la matrice de Bayer

### Question 13

!['question13'](assets/question13_enonce.png)

_Déterminer 𝐵3._

$$
B3 = 
\frac{1}{64}.
\begin{bmatrix}
0 & 32 & 8 & 40 & 2 & 34 & 10 & 42 \\
48 & 16 & 56 & 24 & 50 & 18 & 58 & 26 \\
12 & 44 & 4 & 36 & 14 & 46 & 6 & 38 \\
60 & 28 & 52 & 20 & 62 & 30 & 54 & 22 \\
3 & 35 & 11 & 43 & 1 & 33 & 9 & 41 \\
51 & 19 & 59 & 27 & 49 & 17 & 57 & 25 \\
15 & 47 & 7 & 39 & 13 & 45 & 5 & 37 \\
63 & 31 & 55 & 23 & 61 & 29 & 53 & 21
\end{bmatrix}
$$

### Question 15

_Implémenter le tramage par matrice de Bayer._

Définition d'une **matrice de Bayer** : 

```rust
const BAYER_MATRIX: [[u8; 4]; 4] = [
    [0, 8, 2, 10],
    [12, 4, 14, 6],
    [3, 11, 1, 9],
    [15, 7, 13, 5],
];
```

Traitement :

```rust
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
```

Instructions pour passer une image en tramage ordonné :

```bash
cargo run -- ./imports/test.jpg ./exports/ordered_dithering.png ordered_dithering
```

## Partie 6 - Diffusion d'erreurs

_Pour une palette de couleurs comme dans la partie 3, expliquer dans votre README comment
vous représentez l’erreur commise à chaque pixel, comment vous la diffusez._

### Question 16

Nous développons tout d'abord une classe utilitaire `MatriceErreur` qui contiendra la matrice avec ses coefficients et sa taille.

Cette classe sera passée en entrée du traitement, pour pouvoir l'appliquer avec plusieurs matrices différentes.

Dans le cas présent, nous appliquons le traitement avec la matrice donnée : 

$$
\begin{bmatrix}
* & 0.5 \\
0.5 & 0
\end{bmatrix}
$$


Méthode de traitement : 

```rust
/**
 * Applique le traitement de diffusion d'erreur sur l'image passée en paramètre.
 * Prend une matrice en entrée comme ça on définit celle qu'on veut
 * par exemple qst16 c'estune matrice avec des coeff 0.5 mais qst19 c'est matrice de Floyd-Steinberg
 */
pub fn traitement_diffusion_erreur(
    img: &DynamicImage,
    path_out: String,
    matrice_erreur: &MatriceErreur,
) -> Result<(), ImageError> {
    let (width, height) = img.dimensions();
    let mut img_out: RgbImage = ImageBuffer::new(width, height);

    // Convertir l'image en niveaux de gris
    let mut luminances: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            luminances[y as usize][x as usize] = get_light(pixel) as f32 / 255.0;
        }
    }

    // Parcourir chaque pixel
    for y in 0..height as usize {
        for x in 0..width as usize {
            let old_luminance = luminances[y][x];
            let new_luminance = if old_luminance > 0.5 { 1.0 } else { 0.0 };
            let error = old_luminance - new_luminance;

            // Définir la nouvelle couleur (noir ou blanc)
            let new_pixel = if new_luminance == 1.0 {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            };
            img_out.put_pixel(x as u32, y as u32, new_pixel);

            // Diffuser l'erreur aux voisins en utilisant la matrice d'erreur
            for row in 0..matrice_erreur.matrix.len() {

                for col in 0..matrice_erreur.matrix[row].len() {
                    // par default le coefficient est la première valeur de la matrice
                    let coefficient = matrice_erreur.get_value(row, col).unwrap_or(0.0);
                    let nx = x as isize + col as isize - matrice_erreur.x_origin as isize;
                    let ny = y as isize + row as isize;
                    if coefficient != 0.0 && nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        luminances[ny as usize][nx as usize] += error * coefficient as f32;
                    }
                }
            }
        }
    }

    // Convertir le buffer en DynamicImage et sauvegarder
    let img_out = DynamicImage::ImageRgb8(img_out);
    save(&img_out, path_out)
}
```

Instruction :

```bash
cargo run -- ./imports/test.jpg ./exports/error_diffusion.png error_diffusion
```

On obtient ce résultat

![error_diffusion_image](./exports/error_diffusion.png)

### Question 17 

Appliquer la diffusion d'erreur dans le cadre d'une image transformée à l'aide d'une palette de couleurs donnée :

- pour chaque pixel de l'image, on détermine la couleur de la palette qui est la plus proche de la couleur réelle du pixel, avec la distance euclidienne .

- Une fois la couleur la plus proche identifiée, on récupère l'erreur qui correspond à la différence entre la couleur réelle du pixel et la couleur approximative proche de la palette. 

- propager l'erreur aux pixels pas traités avec une matrice de diffusion d'erreur. Par exemple, avec une matrice simple comme :

$$
\begin{bmatrix}
* & 0.5 \\
0.5 & 0
\end{bmatrix}
$$

> 50% de l'erreur est transmise au pixel à droite.

> 50% au pixel en dessous.

Instruction : 

```bash
cargo run -- ./imports/test.jpg ./exports/error_diffusion_colors.png error_diffusion_colors --couleur1 green --couleur2 blue
```

![error_diffusion_colors](./exports/error_diffusion_colors.png)

## Partie 7 - La bibliothèque ``argh``

### Question 21

_Donner une spécification de votre interface sous forme d’un projet d’écran d’aide, tel que celui qui sera obtenu par cargo run -- --help._


Instruction pour afficher l'écran d'aide :

```bash
cargo run -- --help
```


![question21](assets/question21.png)

### Question 22

_Déterminer le type Rust correspondant à une sélection d’options fournies par l’utilisateur._

Le type Rust correspondant à une sélection d'options fournies par l'utilisateur est un ``enum``, car il permet de représenter plusieurs choix distincts, chacun associé à des données spécifiques si nécessaire, comme :

- `seuil`
- `palette`
- `pixel`
- `split_white`
- `couleurs`
- `dithering`
- `ordered_dithering`
- `error_diffusion`
- `error_diffusion_colors`

### Question 23

_Implémenter votre interface en ligne de commande à l’aide de la directive
#[derive(FromArgs)] sur votre type, suivant la documentation à [la doc](https://docs.rs/argh/0.1.13/argh/)_

```rust
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
    ErrorDiffusionColors(OptsErrorDiffusionColors),
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

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "error_diffusion_colors")]
/// Rendu de l’image par diffusion d’erreur
pub struct OptsErrorDiffusionColors {
    #[argh(option, description = "couleur 1")]
    pub couleur1: String,
    #[argh(option, description = "couleur 2")]
    pub couleur2: String,
}
```
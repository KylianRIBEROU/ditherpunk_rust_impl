use image::Rgb;

pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const GREY: Rgb<u8> = Rgb([127, 127, 127]);
pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
pub const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const YELLOW: Rgb<u8> = Rgb([255, 255, 0]);
pub const MAGENTA: Rgb<u8> = Rgb([255, 0, 255]);
pub const CYAN: Rgb<u8> = Rgb([0, 255, 255]);

pub const BAYER_MATRIX: [[u8; 4]; 4] = [
    [0, 8, 2, 10],
    [12, 4, 14, 6],
    [3, 11, 1, 9],
    [15, 7, 13, 5],
];

pub const COLORS: [Rgb<u8>; 8] = [BLACK, WHITE, RED, GREEN, BLUE, YELLOW, MAGENTA, CYAN];

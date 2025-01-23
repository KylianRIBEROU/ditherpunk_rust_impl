pub mod matrice_erreur {
    type TailleMatrice = usize;
    type TauxPrecisionErreur = f32;

    pub struct MatriceErreur {
        pub x_origin: TailleMatrice,
        pub matrix: Vec<Vec<TauxPrecisionErreur>>,
    }

    impl MatriceErreur {
        pub fn new(x_origin: TailleMatrice, matrix: Vec<Vec<TauxPrecisionErreur>>) -> Self {
            MatriceErreur { x_origin, matrix }
        }

        pub fn get_value(&self, row: TailleMatrice, col: TailleMatrice) -> Option<TauxPrecisionErreur> {
            if row < self.matrix.len() && col < self.matrix[row].len() {
                Some(self.matrix[row][col])
            } else {
                None
            }
        }
    }
}
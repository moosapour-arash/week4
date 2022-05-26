use nalgebra::{DMatrix, DVector};
use ark_bls12_381::fq::Fq;
use ark_std;

#[derive(Debug)]
struct Freivald {
    x: Vec<DVector<Fq>>,
}

impl Freivald {
    fn new(array_size: usize) -> Self {
        let mut x : Vec<DVector<Fq>> = vec![];
        for _ in 0..5 {
            // Generate random number
            let r = ark_std::rand::random::<Fq>();
            // Populate vector with values r^i for i=0..matrix_size
            let mut xn: Vec<Fq> = vec![ark_bls12_381::fq::FQ_ONE, r];
            for n in 2..array_size {
                xn.push(r * &xn[n - 1]);
            }
            x.push(DVector::from_vec(xn));
        }

        // Return freivald value with this vector as its x value
        Self {
            x
        }
    }

    fn verify(&self, matrix_a: &DMatrix<Fq>, matrix_b: &DMatrix<Fq>, supposed_ab: &DMatrix<Fq>) -> bool {
        // changed assert! to if to prevent panic on last test !
        if !check_matrix_dimensions(&matrix_a, &matrix_b, &supposed_ab) {
            return false;
        }
        for i in 0..self.x.len() {
            let y = supposed_ab * &self.x[i];
            let mat_bx = matrix_b * &self.x[i];
            let z = matrix_a * mat_bx;
            if z != y {
                return false;
            }
        }
        return true;

    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    fn verify_once(matrix_a: &DMatrix<Fq>, matrix_b: &DMatrix<Fq>, supposed_ab: &DMatrix<Fq>) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}

pub fn check_matrix_dimensions(matrix_a: &DMatrix<Fq>, matrix_b: &DMatrix<Fq>, supposed_ab: &DMatrix<Fq>) -> bool {
    if matrix_a.is_square() && matrix_b.is_square() && supposed_ab.is_square() {
        if matrix_a.shape() == matrix_b.shape() && matrix_b.shape() == supposed_ab.shape() {
            return true;
        }
    }
    // If it doesn't you know its not the correct result independently of matrix contents
    false
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use nalgebra::{Dynamic, OMatrix};
    use rstest::rstest;

    use super::*;

    lazy_static! {
        static ref MATRIX_A: DMatrix<Fq> = make_square_mat(3);
        static ref MATRIX_A_DOT_A: DMatrix<Fq> = &*MATRIX_A * &*MATRIX_A;
        static ref MATRIX_B: DMatrix<Fq> = make_square_mat(3);
        static ref MATRIX_B_DOT_B: DMatrix<Fq> = &*MATRIX_B * &*MATRIX_B;
        static ref MATRIX_C: DMatrix<Fq> = make_square_mat(200);
        static ref MATRIX_C_DOT_C: DMatrix<Fq> = &*MATRIX_C * &*MATRIX_C;
    }

    #[rstest]
    #[case(& MATRIX_A, & MATRIX_A, & MATRIX_A_DOT_A)]
    #[case(& MATRIX_B, & MATRIX_B, & MATRIX_B_DOT_B)]
    #[case(& MATRIX_C, & MATRIX_C, & MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: &DMatrix<Fq>,
        #[case] matrix_b: &DMatrix<Fq>,
        #[case] supposed_ab: &DMatrix<Fq>,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(& MATRIX_A, & MATRIX_B, & MATRIX_A_DOT_A)]
    #[case(& MATRIX_B, & MATRIX_A, & MATRIX_B_DOT_B)]
    #[case(& MATRIX_C, & MATRIX_B, & MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: &DMatrix<Fq>,
        #[case] b: &DMatrix<Fq>,
        #[case] c: &DMatrix<Fq>,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
    fn make_square_mat(nrows: usize) -> OMatrix<Fq, Dynamic, Dynamic> {
        let mut v = Vec::new();
        for _ in 0..nrows.pow(2) {
            let r = ark_std::rand::random::<Fq>();
            v.push(r);
        }
        DMatrix::from_vec(nrows, nrows, v)
    }
}

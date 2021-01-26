mod iter;
mod ops;

use vectrix::{matrix, Matrix};

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_macro_const() {
    const MATRIX: Matrix<i64, 2, 2> = matrix![1, 3; 3, 7];
    assert_eq!(MATRIX, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_macro_row_vector() {
    let _matrix: Matrix<i64, 1, 4> = matrix![1, 3, 3, 7];
}

#[test]
fn matrix_macro_vector() {
    let _matrix: Matrix<i64, 4, 1> = matrix![1; 3; 3; 7];
}

#[test]
fn matrix_default() {
    let matrix = Matrix::default();
    assert_eq!(matrix, matrix![0, 0; 0, 0]);
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, M, N> methods
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_zero() {
    let matrix = Matrix::zero();
    assert_eq!(matrix, matrix![0, 0; 0, 0]);
}

#[test]
fn matrix_as_slice() {
    let matrix = matrix![1, 3, 3, 7];
    assert_eq!(matrix.as_slice(), vec![1, 3, 3, 7].as_slice());
}

#[test]
fn matrix_as_mut_slice() {
    let mut matrix = matrix![1, 2; 3, 4];
    let slice = matrix.as_mut_slice();
    slice[2] = 3;
    slice[3] = 7;
    assert_eq!(matrix, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_iter() {
    let matrix = matrix![1, 3; 2, 4];
    let values: Vec<_> = matrix.iter().collect();
    assert_eq!(values, vec![&1, &2, &3, &4]);
}

#[test]
fn matrix_iter_mut() {
    let mut matrix = matrix![0, 2; 2, 6];
    for n in matrix.iter_mut() {
        *n += 1;
    }
    assert_eq!(matrix, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_l1_norm() {
    let matrix = matrix![-1, 3; -3, 7];
    assert_eq!(matrix.l1_norm(), 10);
}

#[test]
fn matrix_l1_norm_vectors() {
    let matrix = matrix![-1, 3, -3, 7];
    assert_eq!(matrix.l1_norm(), 7);

    let matrix = matrix![-1; 3; -3; 7];
    assert_eq!(matrix.l1_norm(), 14);
}

mod matrix_multiplication {
    use super::*;
    #[test]
    fn dot_product() {
        let matrix = matrix![-1, 3, -3, 7];
        let other = matrix![-4; 4; 5; 2];

        let m = matrix.matmul(&other);
        assert_eq!(m, matrix![15]);
    }
    #[test]
    fn n_by_n() {
        let matrix = matrix![1, 2; 4, 5];
        let other = matrix![6, 7; 8, 9];

        let m = matrix.matmul(&other);
        assert_eq!(m, matrix![22, 25; 64, 73]);
    }
    #[test]
    fn n_by_m() {
        let matrix = matrix![1, 2, 3; 4, 5, 6];
        let other = matrix![6, 7, 8, 9; 10, 11, 12, 13; 14, 15, 16, 17];

        let m = matrix.matmul(&other);
        assert_eq!(m, matrix![68, 74, 80, 86; 158, 173, 188, 203]);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, N, N> methods
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_identity() {
    assert_eq!(
        Matrix::identity(),
        matrix![
            1, 0;
            0, 1;
        ]
    );

    assert_eq!(
        Matrix::identity(),
        matrix![
            1, 0, 0;
            0, 1, 0;
            0, 0, 1;
        ]
    );

    assert_eq!(
        Matrix::identity(),
        matrix![
            1, 0, 0, 0;
            0, 1, 0, 0;
            0, 0, 1, 0;
            0, 0, 0, 1;
        ]
    );
}

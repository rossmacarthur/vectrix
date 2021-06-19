use core::iter::FromIterator;

use vectrix::{matrix, Matrix};

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

#[test]
fn matrix_from_iter() {
    let matrix = Matrix::<i64, 2, 2>::from_iter(vec![1, 2, 3, 4]);
    assert_eq!(matrix, matrix![1, 3; 2, 4]);
}

#[test]
fn matrix_from_iter_not_copy_or_default() {
    #[derive(Debug, PartialEq)]
    struct Num(i64);
    let matrix = Matrix::<Num, 2, 2>::from_iter(vec![Num(1), Num(2), Num(3), Num(4)]);
    assert_eq!(matrix, matrix![Num(1), Num(3); Num(2), Num(4)]);
}

#[test]
fn matrix_from_iter_long() {
    let matrix = Matrix::<i64, 2, 2>::from_iter(vec![1, 2, 3, 4, 5]);
    assert_eq!(matrix, matrix![1, 3; 2, 4]);
}

#[test]
#[should_panic]
fn matrix_from_iter_short() {
    let _matrix = Matrix::<i64, 2, 2>::from_iter(vec![1, 2, 3]);
}

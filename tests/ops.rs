use vectrix::{matrix, Matrix};

macro_rules! for_each_op_assert_eq {
    ($a:expr, $op:tt, $b:expr, $expected:expr) => {
        let matrix = $a $op $b;
        assert_eq!(matrix, $expected);

        let matrix = $a $op &$b;
        assert_eq!(matrix, $expected);

        let matrix = &$a $op $b;
        assert_eq!(matrix, $expected);

        let matrix = &$a $op &$b;
        assert_eq!(matrix, $expected);
    };
}

macro_rules! for_each_op_assign_assert_eq {
    ($a:expr, $op:tt, $b:expr, $expected:expr) => {
        let mut matrix = $a.clone();
        matrix $op $b;
        assert_eq!(matrix, $expected);

        let mut matrix = $a.clone();
        matrix $op &$b;
        assert_eq!(matrix, $expected);
    };
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_index_usize() {
    let matrix: Matrix<_, 2, 3> = matrix![
        1, 3, 5;
        2, 4, 6;
    ];
    assert_eq!(matrix[0], 1);
    assert_eq!(matrix[1], 2);
    assert_eq!(matrix[2], 3);
    assert_eq!(matrix[3], 4);
    assert_eq!(matrix[4], 5);
    assert_eq!(matrix[5], 6);
}

#[test]
fn matrix_index_tuple() {
    let matrix: Matrix<_, 2, 3> = matrix![
        1, 3, 5;
        2, 4, 6;
    ];
    assert_eq!(matrix[(0, 0)], 1);
    assert_eq!(matrix[(0, 1)], 3);
    assert_eq!(matrix[(0, 2)], 5);
    assert_eq!(matrix[(1, 0)], 2);
    assert_eq!(matrix[(1, 1)], 4);
    assert_eq!(matrix[(1, 2)], 6);
}

////////////////////////////////////////////////////////////////////////////////
// Matrix + T
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_add_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, +, b, matrix![3, -1; 5, -5] }
}

#[test]
fn matrix_sub_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, -, b, matrix![-1, -5; 1, -9] }
}

#[test]
fn matrix_mul_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, *, b, matrix![2, -6; 6, -14] }
}

#[test]
fn matrix_div_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, /, b, matrix![0, -1; 1, -3] }
}

#[test]
fn matrix_rem_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, %, b, matrix![1, -1; 1, -1] }
}

#[test]
fn matrix_bit_and_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, &, b, matrix![0, 0; 2, 0] }
}

#[test]
fn matrix_bit_or_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, |, b, matrix![3, -1; 3, -5] }
}

#[test]
fn matrix_bit_xor_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, ^, b, matrix![3, -1; 1, -5] }
}

#[test]
fn matrix_shl_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, <<, b, matrix![4, -12; 12, -28] }
}

#[test]
fn matrix_shr_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assert_eq! { a, >>, b, matrix![0, -1; 0, -2] }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix += T
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_add_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, +=, b, matrix![3, -1; 5, -5] }
}

#[test]
fn matrix_sub_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, -=, b, matrix![-1, -5; 1, -9] }
}

#[test]
fn matrix_mul_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, *=, b, matrix![2, -6; 6, -14] }
}

#[test]
fn matrix_div_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, /=, b, matrix![0, -1; 1, -3] }
}

#[test]
fn matrix_rem_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, %=, b, matrix![1, -1; 1, -1] }
}

#[test]
fn matrix_bit_and_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, &=, b, matrix![0, 0; 2, 0] }
}

#[test]
fn matrix_bit_or_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, |=, b, matrix![3, -1; 3, -5] }
}

#[test]
fn matrix_bit_xor_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, ^=, b, matrix![3, -1; 1, -5] }
}

#[test]
fn matrix_bit_shl_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, <<=, b, matrix![4, -12; 12, -28] }
}

#[test]
fn matrix_bit_shr_assign_scalar() {
    let a = matrix![1, -3; 3, -7];
    let b = 2;
    for_each_op_assign_assert_eq! { a, >>=, b, matrix![0, -1; 0, -2] }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix + Matrix
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_add() {
    let a = matrix![1, -6; 3, -7];
    let b = matrix![0, 3; 0, 0];
    let c = matrix![1, -3; 3, -7];
    for_each_op_assert_eq! { a, +, b, c }
}

#[test]
fn matrix_sub() {
    let a = matrix![1, 0; 3, -7];
    let b = matrix![0, 3; 0, 0];
    let c = matrix![1, -3; 3, -7];
    for_each_op_assert_eq! { a, -, b, c }
}

////////////////////////////////////////////////////////////////////////////////
// Matrix += Matrix
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_add_assign() {
    let a = matrix![1, -6; 3, -7];
    let b = matrix![0, 3; 0, 0];
    let c = matrix![1, -3; 3, -7];
    for_each_op_assign_assert_eq! { a, +=, b, c }
}

#[test]
fn matrix_sub_assign() {
    let a = matrix![1, 0; 3, -7];
    let b = matrix![0, 3; 0, 0];
    let c = matrix![1, -3; 3, -7];
    for_each_op_assign_assert_eq! { a, -=, b, c }
}

////////////////////////////////////////////////////////////////////////////////
// -Matrix
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_neg() {
    let a = matrix![1, -3; 3, -7];
    assert_eq!(-a, matrix![-1, 3; -3, 7]);
    assert_eq!(-&a, matrix![-1, 3; -3, 7]);
}

#[test]
fn matrix_not() {
    let a = matrix![1, -3; 3, -7];
    assert_eq!(!a, matrix![-2, 2; -4, 6]);
    assert_eq!(!&a, matrix![-2, 2; -4, 6]);
}

mod matrix_multiplication {
    use super::*;
    #[test]
    fn dot_product() {
        let matrix = matrix![-1, 3, -3, 7];
        let other = matrix![-4; 4; 5; 2];

        let m = matrix * other;
        assert_eq!(m, matrix![15]);

        let m = (&matrix) * (&other);
        assert_eq!(m, matrix![15]);
    }
    #[test]
    fn n_by_n() {
        let matrix = matrix![1, 2; 4, 5];
        let other = matrix![6, 7; 8, 9];

        let m = matrix * other;
        assert_eq!(m, matrix![22, 25; 64, 73]);
    }
    #[test]
    fn n_by_m() {
        let matrix = matrix![1, 2, 3; 4, 5, 6];
        let other = matrix![6, 7, 8, 9; 10, 11, 12, 13; 14, 15, 16, 17];

        let m = matrix * other;
        assert_eq!(m, matrix![68, 74, 80, 86; 158, 173, 188, 203]);
    }
}

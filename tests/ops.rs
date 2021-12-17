use vectrix::{matrix, Matrix};

macro_rules! for_each_op_assert_eq {
    ($a:expr, $op:tt, $b:expr, $expected:expr) => {
        let m = $a $op $b;
        assert_eq!(m, $expected);

        let m = $a $op &$b;
        assert_eq!(m, $expected);

        let m = &$a $op $b;
        assert_eq!(m, $expected);

        let m = &$a $op &$b;
        assert_eq!(m, $expected);
    };
}

macro_rules! for_each_op_assign_assert_eq {
    ($a:expr, $op:tt, $b:expr, $expected:expr) => {
        let mut m = $a.clone();
        m $op $b;
        assert_eq!(m, $expected);

        let mut m = $a.clone();
        m $op &$b;
        assert_eq!(m, $expected);
    };
}

////////////////////////////////////////////////////////////////////////////////
// Indexing
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_index_usize() {
    let m: Matrix<_, 2, 3> = matrix![
        1, 3, 5;
        2, 4, 6;
    ];
    assert_eq!(m[0], 1);
    assert_eq!(m[1], 2);
    assert_eq!(m[2], 3);
    assert_eq!(m[3], 4);
    assert_eq!(m[4], 5);
    assert_eq!(m[5], 6);
}

#[test]
fn matrix_index_tuple() {
    let m: Matrix<_, 2, 3> = matrix![
        1, 3, 5;
        2, 4, 6;
    ];
    assert_eq!(m[(0, 0)], 1);
    assert_eq!(m[(0, 1)], 3);
    assert_eq!(m[(0, 2)], 5);
    assert_eq!(m[(1, 0)], 2);
    assert_eq!(m[(1, 1)], 4);
    assert_eq!(m[(1, 2)], 6);
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
// Matrix * Matrix
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_mul_dot_product() {
    let a = matrix![-1, 3, -3, 7];
    let b = matrix![-4; 4; 5; 2];
    let c = matrix![15];

    for_each_op_assert_eq! { a, *, b, c }
}

#[test]
fn matrix_mul_n_by_n() {
    let a = matrix![1, 2; 4, 5];
    let b = matrix![6, 7; 8, 9];
    let c = matrix![22, 25; 64, 73];

    for_each_op_assert_eq! { a, *, b, c }
}

#[test]
fn matrix_mul_n_by_m() {
    let a = matrix![1, 2, 3; 4, 5, 6];
    let b = matrix![6, 7, 8, 9; 10, 11, 12, 13; 14, 15, 16, 17];
    let c = matrix![68, 74, 80, 86; 158, 173, 188, 203];

    for_each_op_assert_eq! { a, *, b, c }
}

#[test]
fn matrix_mul_0_by_m() {
    let a = Matrix::<_, 0, 3>::zero();
    let b = matrix![6, 7, 8, 9; 10, 11, 12, 13; 14, 15, 16, 17];
    let c = Matrix::zero();

    for_each_op_assert_eq! { a, *, b, c }
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

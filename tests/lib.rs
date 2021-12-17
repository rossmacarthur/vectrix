use vectrix::{matrix, vector, Matrix};

////////////////////////////////////////////////////////////////////////////////
// Matrix<T, M, N> methods
////////////////////////////////////////////////////////////////////////////////

#[test]
fn matrix_zero() {
    let m = Matrix::zero();
    assert_eq!(m, matrix![0, 0; 0, 0]);
}

#[test]
fn matrix_repeat() {
    let m = Matrix::repeat(7);
    assert_eq!(m, matrix![7, 7; 7, 7]);
}

#[test]
fn matrix_repeat_with() {
    let mut state = 1;
    let m = Matrix::repeat_with(|| {
        state *= 2;
        state
    });
    assert_eq!(m, matrix![2, 8; 4, 16]);
}

#[test]
fn matrix_repeat_with_not_copy_or_default() {
    #[derive(Debug, PartialEq)]
    struct Num(i64);
    let mut state = 1;
    let m = Matrix::repeat_with(|| {
        state *= 2;
        Num(state)
    });
    assert_eq!(m, matrix![Num(2), Num(8); Num(4), Num(16)]);
}

#[test]
fn matrix_as_slice() {
    let m = matrix![1, 3, 3, 7];
    assert_eq!(m.as_slice(), vec![1, 3, 3, 7].as_slice());
}

#[test]
fn matrix_as_mut_slice() {
    let mut m = matrix![1, 2; 3, 4];
    let slice = m.as_mut_slice();
    slice[2] = 3;
    slice[3] = 7;
    assert_eq!(m, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_iter() {
    let m = matrix![1, 3; 2, 4];
    let values: Vec<_> = m.iter().collect();
    assert_eq!(values, vec![&1, &2, &3, &4]);
}

#[test]
fn matrix_iter_mut() {
    let mut m = matrix![0, 2; 2, 6];
    for n in m.iter_mut() {
        *n += 1;
    }
    assert_eq!(m, matrix![1, 3; 3, 7]);
}

#[test]
fn matrix_row() {
    let m = matrix![1, 3; -3, 7];
    let row = m.row(0);
    let vec: Vec<_> = row.iter().collect();
    assert_eq!(vec, &[&1, &3]);
    let row = m.row(1);
    let vec: Vec<_> = row.iter().collect();
    assert_eq!(vec, &[&-3, &7]);
}

#[test]
fn matrix_row_mut() {
    let mut m = matrix![2, 3; -3, 8];
    let row = m.row_mut(0);
    row[0] = 1;
    let row = m.row_mut(1);
    row[1] = 7;
    assert_eq!(m, matrix![1, 3; -3, 7]);
}

#[test]
fn matrix_column() {
    let m = matrix![1, 3; -3, 7];
    let col = m.column(0);
    let vec: Vec<_> = col.iter().collect();
    assert_eq!(vec, &[&1, &-3]);
    let column = m.column(1);
    let vec: Vec<_> = column.iter().collect();
    assert_eq!(vec, &[&3, &7]);
}

#[test]
fn matrix_column_mut() {
    let mut m = matrix![2, 3; -3, 8];
    let column = m.column_mut(0);
    column[0] = 1;
    let column = m.column_mut(1);
    column[1] = 7;
    assert_eq!(m, matrix![1, 3; -3, 7]);
}

#[test]
fn matrix_l1_norm() {
    let m = matrix![-1, 3; -3, 7];
    assert_eq!(m.l1_norm(), 10);
}

#[test]
fn matrix_l1_norm_vectors() {
    let m = matrix![-1, 3, -3, 7];
    assert_eq!(m.l1_norm(), 7);

    let m = matrix![-1; 3; -3; 7];
    assert_eq!(m.l1_norm(), 14);
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

#[test]
fn matrix_diagonal() {
    let m = matrix![
        1, 0, 0;
        0, 2, 0;
        0, 0, 3;
    ];
    assert_eq!(m.diagonal(), vector![1, 2, 3]);
}

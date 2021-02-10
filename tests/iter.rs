use vectrix::{matrix, Matrix};

#[test]
fn into_iter_debug() {
    let mut into_iter = matrix![1, 3; 3, 7].into_iter();
    assert_eq!(format!("{:?}", into_iter), "IntoIter([1, 3, 3, 7])");
    into_iter.next();
    assert_eq!(format!("{:?}", into_iter), "IntoIter([3, 3, 7])");
    into_iter.next_back();
    assert_eq!(format!("{:?}", into_iter), "IntoIter([3, 3])");
}

#[test]
fn matrix_into_iter() {
    let matrix = matrix![1, 3; 3, 7];
    let values: Vec<_> = matrix.into_iter().collect();
    assert_eq!(values, vec![1, 3, 3, 7]);
}

#[test]
fn matrix_into_iter_rev() {
    let matrix = matrix![1, 3; 3, 7];
    let values: Vec<_> = matrix.into_iter().rev().collect();
    assert_eq!(values, vec![7, 3, 3, 1]);
}

#[test]
fn matrix_into_iter_skip_rev() {
    let matrix = matrix![1, 3; 3, 7];
    let values: Vec<_> = matrix.into_iter().skip(1).rev().skip(1).collect();
    assert_eq!(values, vec![3, 3]);
}

#[test]
fn matrix_into_iter_count() {
    let matrix = matrix![1, 3; 3, 7];
    assert_eq!(matrix.into_iter().count(), 4);
    assert_eq!(matrix.into_iter().skip(1).count(), 3);
    assert_eq!(matrix.into_iter().skip(1).rev().skip(1).count(), 2);
}

#[test]
fn matrix_into_iter_fuse() {
    let matrix = matrix![1, 3, 3, 7];
    let mut iter = matrix.into_iter();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn matrix_sum() {
    let matrices = vec![
        matrix![1, -3; 3, -7],
        matrix![-1, 3; -3, 7],
        matrix![0, 0; 0, 0],
        matrix![1, 2; 3, 4],
    ];
    let matrix: Matrix<_, 2, 2> = matrices.into_iter().sum();
    assert_eq!(matrix, matrix![1, 2; 3, 4]);
}

#[test]
fn matrix_iter_rows() {
    let matrix = matrix![1, 3; 3, 7];
    let rows: Vec<_> = matrix.iter_rows().collect();
    assert_eq!(rows, vec![matrix.row(0), matrix.row(1)]);
}

#[test]
fn matrix_iter_columns() {
    let matrix = matrix![1, 3; 3, 7];
    let columns: Vec<_> = matrix.iter_columns().collect();
    assert_eq!(columns, vec![matrix.column(0), matrix.column(1)]);
}

#[test]
fn matrix_iter_rows_rev() {
    let matrix = matrix![1, 3; 3, 7];
    let rows: Vec<_> = matrix.iter_rows().rev().collect();
    assert_eq!(rows, vec![matrix.row(1), matrix.row(0)]);
}

#[test]
fn matrix_iter_columns_rev() {
    let matrix = matrix![1, 3; 3, 7];
    let columns: Vec<_> = matrix.iter_columns().rev().collect();
    assert_eq!(columns, vec![matrix.column(1), matrix.column(0)]);
}

#[test]
fn matrix_iter_rows_fuse() {
    let matrix = matrix![1; 3; 3; 7];
    let mut iter = matrix.iter_rows();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn matrix_iter_columns_fuse() {
    let matrix = matrix![1, 3, 3, 7];
    let mut iter = matrix.iter_columns();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn matrix_iter_rows_mut() {
    let mut matrix = matrix![1, 3; 3, 7];
    for row in matrix.iter_rows_mut() {
        row[0] *= 2;
    }
    assert_eq!(matrix, matrix![2, 3; 6, 7])
}

#[test]
fn matrix_iter_columns_mut() {
    let mut matrix = matrix![1, 3; 3, 7];
    for column in matrix.iter_columns_mut() {
        column[0] *= 2;
    }
    assert_eq!(matrix, matrix![2, 6; 3, 7])
}

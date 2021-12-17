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
fn matrix_from_iter_not_copy_or_default() {
    #[derive(Debug, PartialEq)]
    struct Num(i64);
    let _into_iter = matrix![Num(1), Num(2); Num(3), Num(4)].into_iter();
}

#[test]
fn matrix_into_iter() {
    let m = matrix![1, 3; 3, 7];
    let values: Vec<_> = m.into_iter().collect();
    assert_eq!(values, vec![1, 3, 3, 7]);
}

#[test]
fn matrix_into_iter_rev() {
    let m = matrix![1, 3; 3, 7];
    let values: Vec<_> = m.into_iter().rev().collect();
    assert_eq!(values, vec![7, 3, 3, 1]);
}

#[test]
fn matrix_into_iter_skip_rev() {
    let m = matrix![1, 3; 3, 7];
    let values: Vec<_> = m.into_iter().skip(1).rev().skip(1).collect();
    assert_eq!(values, vec![3, 3]);
}

#[test]
fn matrix_into_iter_count() {
    let m = matrix![1, 3; 3, 7];
    assert_eq!(m.into_iter().count(), 4);
    assert_eq!(m.into_iter().skip(1).count(), 3);
    assert_eq!(m.into_iter().skip(1).rev().skip(1).count(), 2);
}

#[test]
fn matrix_into_iter_fuse() {
    let m = matrix![1, 3, 3, 7];
    let mut iter = m.into_iter();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn matrix_sum() {
    let ms = vec![
        matrix![1, -3; 3, -7],
        matrix![-1, 3; -3, 7],
        matrix![0, 0; 0, 0],
        matrix![1, 2; 3, 4],
    ];
    let matrix: Matrix<_, 2, 2> = ms.into_iter().sum();
    assert_eq!(matrix, matrix![1, 2; 3, 4]);
}

#[test]
fn matrix_iter_rows() {
    let m = matrix![1, 3; 3, 7];
    let rows: Vec<_> = m.iter_rows().collect();
    assert_eq!(rows, vec![m.row(0), m.row(1)]);
}

#[test]
fn matrix_iter_columns() {
    let m = matrix![1, 3; 3, 7];
    let cols: Vec<_> = m.iter_columns().collect();
    assert_eq!(cols, vec![m.column(0), m.column(1)]);
}

#[test]
fn matrix_iter_rows_rev() {
    let m = matrix![1, 3; 3, 7];
    let rows: Vec<_> = m.iter_rows().rev().collect();
    assert_eq!(rows, vec![m.row(1), m.row(0)]);
}

#[test]
fn matrix_iter_columns_rev() {
    let m = matrix![1, 3; 3, 7];
    let cols: Vec<_> = m.iter_columns().rev().collect();
    assert_eq!(cols, vec![m.column(1), m.column(0)]);
}

#[test]
fn matrix_iter_rows_fuse() {
    let m = matrix![1; 3; 3; 7];
    let mut iter = m.iter_rows();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn matrix_iter_columns_fuse() {
    let m = matrix![1, 3, 3, 7];
    let mut iter = m.iter_columns();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn matrix_iter_rows_mut() {
    let mut m = matrix![1, 3; 3, 7];
    for row in m.iter_rows_mut() {
        row[0] *= 2;
    }
    assert_eq!(m, matrix![2, 3; 6, 7])
}

#[test]
fn matrix_iter_columns_mut() {
    let mut m = matrix![1, 3; 3, 7];
    for col in m.iter_columns_mut() {
        col[0] *= 2;
    }
    assert_eq!(m, matrix![2, 6; 3, 7])
}

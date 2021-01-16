use vectrix::{matrix, Matrix};

////////////////////////////////////////////////////////////////////////////////
/// Accessors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn row_vector_components() {
    let vector = matrix![1, 3, 3, 7, 0, 0];
    assert_eq!(vector.x, 1);
    assert_eq!(vector.y, 3);
    assert_eq!(vector.z, 3);
    assert_eq!(vector.w, 7);
    assert_eq!(vector.a, 0);
    assert_eq!(vector.b, 0);
}

#[test]
fn row_vector_components_mut() {
    let mut vector = matrix![1, 2, 3, 4, 5, 6];
    vector.x = 1;
    vector.y = 3;
    vector.z = 3;
    vector.w = 7;
    vector.a = 0;
    vector.b = 0;
    assert_eq!(vector[0], 1);
    assert_eq!(vector[1], 3);
    assert_eq!(vector[2], 3);
    assert_eq!(vector[3], 7);
    assert_eq!(vector[4], 0);
    assert_eq!(vector[5], 0);
}

#[test]
fn column_vector_components() {
    let vector = matrix![1; 3; 3; 7; 0; 0];
    assert_eq!(vector.x, 1);
    assert_eq!(vector.y, 3);
    assert_eq!(vector.z, 3);
    assert_eq!(vector.w, 7);
    assert_eq!(vector.a, 0);
    assert_eq!(vector.b, 0);
}

#[test]
fn column_vector_components_mut() {
    let mut vector = matrix![1; 2; 3; 4; 5; 6];
    vector.x = 1;
    vector.y = 3;
    vector.z = 3;
    vector.w = 7;
    vector.a = 0;
    vector.b = 0;
    assert_eq!(vector[0], 1);
    assert_eq!(vector[1], 3);
    assert_eq!(vector[2], 3);
    assert_eq!(vector[3], 7);
    assert_eq!(vector[4], 0);
    assert_eq!(vector[5], 0);
}

////////////////////////////////////////////////////////////////////////////////
/// Constructors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn row_vector_new() {
    type V<const N: usize> = Matrix<i64, 1, N>;
    assert_eq!(V::<1>::new(1), matrix![1]);
    assert_eq!(V::<2>::new(1, 2), matrix![1, 2]);
    assert_eq!(V::<3>::new(1, 2, 3), matrix![1, 2, 3]);
    assert_eq!(V::<4>::new(1, 2, 3, 4), matrix![1, 2, 3, 4]);
    assert_eq!(V::<5>::new(1, 2, 3, 4, 5), matrix![1, 2, 3, 4, 5]);
    assert_eq!(V::<6>::new(1, 2, 3, 4, 5, 6), matrix![1, 2, 3, 4, 5, 6]);
}

#[test]
fn row_vector_from_array() {
    type V<const N: usize> = Matrix<i64, 1, N>;
    assert_eq!(V::from([1]), matrix![1]);
    assert_eq!(V::from([1, 2]), matrix![1, 2]);
    assert_eq!(V::from([1, 2, 3]), matrix![1, 2, 3]);
    assert_eq!(V::from([1, 2, 3, 4]), matrix![1, 2, 3, 4]);
    assert_eq!(V::from([1, 2, 3, 4, 5]), matrix![1, 2, 3, 4, 5]);
    assert_eq!(V::from([1, 2, 3, 4, 5, 6]), matrix![1, 2, 3, 4, 5, 6]);
}

#[test]
fn column_vector_new() {
    type V<const M: usize> = Matrix<i64, M, 1>;
    assert_eq!(V::<1>::new(1), matrix![1]);
    assert_eq!(V::<2>::new(1, 2), matrix![1; 2]);
    assert_eq!(V::<3>::new(1, 2, 3), matrix![1; 2; 3]);
    assert_eq!(V::<4>::new(1, 2, 3, 4), matrix![1; 2; 3; 4]);
    assert_eq!(V::<5>::new(1, 2, 3, 4, 5), matrix![1; 2; 3; 4; 5]);
    assert_eq!(V::<6>::new(1, 2, 3, 4, 5, 6), matrix![1; 2; 3; 4; 5; 6]);
}

#[test]
fn column_vector_from_array() {
    type V<const M: usize> = Matrix<i64, M, 1>;
    assert_eq!(V::from([1]), matrix![1]);
    assert_eq!(V::from([1, 2]), matrix![1; 2]);
    assert_eq!(V::from([1, 2, 3]), matrix![1; 2; 3]);
    assert_eq!(V::from([1, 2, 3, 4]), matrix![1; 2; 3; 4]);
    assert_eq!(V::from([1, 2, 3, 4, 5]), matrix![1; 2; 3; 4; 5]);
    assert_eq!(V::from([1, 2, 3, 4, 5, 6]), matrix![1; 2; 3; 4; 5; 6]);
}

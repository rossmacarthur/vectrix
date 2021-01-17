use vectrix::{matrix, row_vector, vector, RowVector, Vector};

////////////////////////////////////////////////////////////////////////////////
// Accessors
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
fn vector_components() {
    let vector = matrix![1; 3; 3; 7; 0; 0];
    assert_eq!(vector.x, 1);
    assert_eq!(vector.y, 3);
    assert_eq!(vector.z, 3);
    assert_eq!(vector.w, 7);
    assert_eq!(vector.a, 0);
    assert_eq!(vector.b, 0);
}

#[test]
fn vector_components_mut() {
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
// Constructors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn row_vector_macro() {
    let vector = row_vector![1, 3, 3, 7];
    assert_eq!(vector, matrix![1, 3, 3, 7]);
}

#[test]
fn row_vector_macro_repeat() {
    let vector = row_vector![7; 4];
    assert_eq!(vector, matrix![7, 7, 7, 7]);
}

#[test]
fn row_vector_new() {
    type V<const N: usize> = RowVector<i64, N>;
    assert_eq!(V::<1>::new(1), matrix![1]);
    assert_eq!(V::<2>::new(1, 2), matrix![1, 2]);
    assert_eq!(V::<3>::new(1, 2, 3), matrix![1, 2, 3]);
    assert_eq!(V::<4>::new(1, 2, 3, 4), matrix![1, 2, 3, 4]);
    assert_eq!(V::<5>::new(1, 2, 3, 4, 5), matrix![1, 2, 3, 4, 5]);
    assert_eq!(V::<6>::new(1, 2, 3, 4, 5, 6), matrix![1, 2, 3, 4, 5, 6]);
}

#[test]
fn row_vector_from_array() {
    type V<const N: usize> = RowVector<i64, N>;
    assert_eq!(V::from([1]), matrix![1]);
    assert_eq!(V::from([1, 2]), matrix![1, 2]);
    assert_eq!(V::from([1, 2, 3]), matrix![1, 2, 3]);
    assert_eq!(V::from([1, 2, 3, 4]), matrix![1, 2, 3, 4]);
    assert_eq!(V::from([1, 2, 3, 4, 5]), matrix![1, 2, 3, 4, 5]);
    assert_eq!(V::from([1, 2, 3, 4, 5, 6]), matrix![1, 2, 3, 4, 5, 6]);
}

#[test]
fn vector_macro() {
    let vector = vector![1, 3, 3, 7];
    assert_eq!(vector, matrix![1; 3; 3; 7]);
}

#[test]
fn vector_macro_repeat() {
    let vector = vector![7; 4];
    assert_eq!(vector, matrix![7; 7; 7; 7]);
}

#[test]
fn vector_new() {
    type V<const M: usize> = Vector<i64, M>;
    assert_eq!(V::<1>::new(1), matrix![1]);
    assert_eq!(V::<2>::new(1, 2), matrix![1; 2]);
    assert_eq!(V::<3>::new(1, 2, 3), matrix![1; 2; 3]);
    assert_eq!(V::<4>::new(1, 2, 3, 4), matrix![1; 2; 3; 4]);
    assert_eq!(V::<5>::new(1, 2, 3, 4, 5), matrix![1; 2; 3; 4; 5]);
    assert_eq!(V::<6>::new(1, 2, 3, 4, 5, 6), matrix![1; 2; 3; 4; 5; 6]);
}

#[test]
fn vector_from_array() {
    type V<const M: usize> = Vector<i64, M>;
    assert_eq!(V::from([1]), matrix![1]);
    assert_eq!(V::from([1, 2]), matrix![1; 2]);
    assert_eq!(V::from([1, 2, 3]), matrix![1; 2; 3]);
    assert_eq!(V::from([1, 2, 3, 4]), matrix![1; 2; 3; 4]);
    assert_eq!(V::from([1, 2, 3, 4, 5]), matrix![1; 2; 3; 4; 5]);
    assert_eq!(V::from([1, 2, 3, 4, 5, 6]), matrix![1; 2; 3; 4; 5; 6]);
}

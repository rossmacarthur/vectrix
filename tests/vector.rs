use vectrix::{matrix, row_vector, vector, RowVector, Vector};

////////////////////////////////////////////////////////////////////////////////
// Accessors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn row_vector_components() {
    let v = matrix![1, 3, 3, 7, 0, 0];
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 3);
    assert_eq!(v.z, 3);
    assert_eq!(v.w, 7);
    assert_eq!(v.a, 0);
    assert_eq!(v.b, 0);
}

#[test]
fn row_vector_components_mut() {
    let mut v = matrix![1, 2, 3, 4, 5, 6];
    v.x = 1;
    v.y = 3;
    v.z = 3;
    v.w = 7;
    v.a = 0;
    v.b = 0;
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 3);
    assert_eq!(v[2], 3);
    assert_eq!(v[3], 7);
    assert_eq!(v[4], 0);
    assert_eq!(v[5], 0);
}

#[test]
fn vector_components() {
    let v = matrix![1; 3; 3; 7; 0; 0];
    assert_eq!(v.x, 1);
    assert_eq!(v.y, 3);
    assert_eq!(v.z, 3);
    assert_eq!(v.w, 7);
    assert_eq!(v.a, 0);
    assert_eq!(v.b, 0);
}

#[test]
fn vector_components_mut() {
    let mut v = matrix![1; 2; 3; 4; 5; 6];
    v.x = 1;
    v.y = 3;
    v.z = 3;
    v.w = 7;
    v.a = 0;
    v.b = 0;
    assert_eq!(v[0], 1);
    assert_eq!(v[1], 3);
    assert_eq!(v[2], 3);
    assert_eq!(v[3], 7);
    assert_eq!(v[4], 0);
    assert_eq!(v[5], 0);
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn row_vector_macro() {
    let v = row_vector![1, 3, 3, 7];
    assert_eq!(v, matrix![1, 3, 3, 7]);
}

#[test]
fn row_vector_macro_repeat() {
    let v = row_vector![7; 4];
    assert_eq!(v, matrix![7, 7, 7, 7]);
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
    let v = vector![1, 3, 3, 7];
    assert_eq!(v, matrix![1; 3; 3; 7]);
}

#[test]
fn vector_macro_repeat() {
    let v = vector![7; 4];
    assert_eq!(v, matrix![7; 7; 7; 7]);
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

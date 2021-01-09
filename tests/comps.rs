use vectrs::Vector;

#[test]
fn vector_components() {
    let vector = Vector::new([1, 3, 3, 7, 0, 0]);
    assert_eq!(vector.x, 1);
    assert_eq!(vector.y, 3);
    assert_eq!(vector.z, 3);
    assert_eq!(vector.w, 7);
    assert_eq!(vector.a, 0);
    assert_eq!(vector.b, 0);
}

#[test]
fn vector_components_mut() {
    let mut vector = Vector::new([1, 2, 3, 4, 5, 6]);
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

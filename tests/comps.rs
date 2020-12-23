use vectrs::Vector;

#[test]
fn vector_components() {
    let vector = Vector::new([1, 3, 3, 7]);
    assert_eq!(vector.x(), 1);
    assert_eq!(vector.y(), 3);
    assert_eq!(vector.z(), 3);
    assert_eq!(vector.w(), 7);
}

#[test]
fn vector_components_ref() {
    let vector = Vector::new([1, 3, 3, 7]);
    assert_eq!(vector.x_ref(), &1);
    assert_eq!(vector.y_ref(), &3);
    assert_eq!(vector.z_ref(), &3);
    assert_eq!(vector.w_ref(), &7);
}

#[test]
fn vector_components_mut() {
    let mut vector = Vector::new([1, 2, 3, 4]);
    *vector.x_mut() = 1;
    *vector.y_mut() = 3;
    *vector.z_mut() = 3;
    *vector.w_mut() = 7;
    assert_eq!(vector[0], 1);
    assert_eq!(vector[1], 3);
    assert_eq!(vector[2], 3);
    assert_eq!(vector[3], 7);
}

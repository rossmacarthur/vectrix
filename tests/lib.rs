use vectrs::Vector;

#[test]
fn vector_constructors() {
    assert_eq!(Vector::from([1, 2]), Vector::from((1, 2)));
    assert_eq!(Vector::from([1, 2, 3]), Vector::from((1, 2, 3)));
    assert_eq!(Vector::from([1, 2, 3, 4]), Vector::from((1, 2, 3, 4)));
}

#[test]
fn vector_components() {
    let vec = Vector::from((1, 2, 3, 4));
    assert_eq!(vec.x(), 1);
    assert_eq!(vec.y(), 2);
    assert_eq!(vec.z(), 3);
    assert_eq!(vec.w(), 4);
}

#[test]
fn vector_components_ref() {
    let vec = Vector::from((1, 2, 3, 4));
    assert_eq!(vec.x_ref(), &1);
    assert_eq!(vec.y_ref(), &2);
    assert_eq!(vec.z_ref(), &3);
    assert_eq!(vec.w_ref(), &4);
}

#[test]
fn vector_components_mut() {
    let mut vec = Vector::from((1, 2, 3, 4));
    *vec.x_mut() = 5;
    assert_eq!(vec.x(), 5);
}

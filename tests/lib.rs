use vectrs::Vector;

#[test]
fn vector_constructors() {
    assert_eq!(Vector::from([1, 2]), Vector::from((1, 2)));
    assert_eq!(Vector::from([1, 2, 3]), Vector::from((1, 2, 3)));
    assert_eq!(Vector::from([1, 2, 3, 4]), Vector::from((1, 2, 3, 4)));
}

#[test]
fn vector_components() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(vector.x(), 1);
    assert_eq!(vector.y(), 2);
    assert_eq!(vector.z(), 3);
    assert_eq!(vector.w(), 4);
}

#[test]
fn vector_debug() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(format!("{:?}", vector), "Vector([1, 2, 3, 4])");
}

#[test]
fn vector_components_ref() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(vector.x_ref(), &1);
    assert_eq!(vector.y_ref(), &2);
    assert_eq!(vector.z_ref(), &3);
    assert_eq!(vector.w_ref(), &4);
}

#[test]
fn vector_components_mut() {
    let mut vector = Vector::from((1, 2, 3, 4));
    *vector.x_mut() = 5;
    assert_eq!(vector.x(), 5);
}

#[test]
fn vector_add() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 3));
    let c = Vector::from((2, 0, 6));
    let vector = a + b;
    assert_eq!(vector, c);

    let vector = a + &b;
    assert_eq!(vector, c);

    let vector = &a + &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_sub() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 1));
    let c = Vector::from((0, 4, 2));

    let vector = a - b;
    assert_eq!(vector, c);

    let vector = a - &b;
    assert_eq!(vector, c);

    let vector = &a - &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_mul() {
    let a = Vector::from((1, -2, 3));
    let b = 2;
    let c = Vector::from((2, -4, 6));

    let vector = a * b;
    assert_eq!(vector, c);

    let vector = &a * b;
    assert_eq!(vector, c);

    let vector = a * &b;
    assert_eq!(vector, c);

    let vector = &a * &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_add_assign() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 3));
    let c = Vector::from((2, 0, 6));

    let mut vector = a.clone();
    vector += b;
    assert_eq!(vector, c);

    let mut vector = a.clone();
    vector += &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_sub_assign() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 1));
    let c = Vector::from((0, 4, 2));

    let mut vector = a.clone();
    vector -= b;
    assert_eq!(vector, c);

    let mut vector = a.clone();
    vector -= &b;
    assert_eq!(vector, c);
}

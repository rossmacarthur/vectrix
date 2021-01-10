use vectrs::Vector;

#[test]
fn into_iter_debug() {
    let mut into_iter = Vector::new([1, 3, 3, 7]).into_iter();
    assert_eq!(format!("{:?}", into_iter), "IntoIter([1, 3, 3, 7])");
    into_iter.next();
    assert_eq!(format!("{:?}", into_iter), "IntoIter([3, 3, 7])");
    into_iter.next_back();
    assert_eq!(format!("{:?}", into_iter), "IntoIter([3, 3])");
}

#[test]
fn vector_into_iter() {
    let vector = Vector::new([1, 3, 3, 7]);
    let values: Vec<_> = vector.into_iter().collect();
    assert_eq!(values, vec![1, 3, 3, 7]);
}

#[test]
fn vector_into_iter_rev() {
    let vector = Vector::new([1, 3, 3, 7]);
    let values: Vec<_> = vector.into_iter().rev().collect();
    assert_eq!(values, vec![7, 3, 3, 1]);
}

#[test]
fn vector_into_iter_skip_rev() {
    let vector = Vector::new([1, 3, 3, 7]);
    let values: Vec<_> = vector.into_iter().skip(1).rev().skip(1).collect();
    assert_eq!(values, vec![3, 3]);
}

#[test]
fn vector_into_iter_count() {
    let vector = Vector::new([1, 3, 3, 7]);
    assert_eq!(vector.into_iter().count(), 4);
    assert_eq!(vector.into_iter().skip(1).count(), 3);
    assert_eq!(vector.into_iter().skip(1).rev().skip(1).count(), 2);
}

#[test]
fn vector_into_iter_fuse() {
    let vector = Vector::new([1, 3, 3, 7]);
    let mut iter = vector.into_iter();
    for _ in 0..4 {
        assert!(matches!(iter.next(), Some(_)));
    }
    for _ in 0..10 {
        assert!(matches!(iter.next(), None));
    }
}

#[test]
fn vector_collect() {
    let vector: Vector<_, 4> = vec![1, 3, 3, 7].into_iter().collect();
    assert_eq!(vector, Vector::new([1, 3, 3, 7]));
}

#[test]
fn vector_collect_too_long() {
    let vector: Vector<_, 4> = vec![1, 3, 3, 7, 9].into_iter().collect();
    assert_eq!(vector, Vector::new([1, 3, 3, 7]));
}

#[test]
#[should_panic]
fn vector_collect_too_short() {
    let _vector: Vector<_, 4> = vec![1, 2].into_iter().collect();
}

#[test]
fn vector_sum() {
    let vector: Vector<_, 4> = vec![[1, -3, 3, -7], [-1, 3, -3, 7], [0, 0, 0, 0], [1, 2, 3, 4]]
        .into_iter()
        .map(Vector::new)
        .sum();
    assert_eq!(vector, Vector::new([1, 2, 3, 4]));
}

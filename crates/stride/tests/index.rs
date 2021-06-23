use stride::Stride;

#[test]
fn stride_get_index() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(0), Some(&1));
    assert_eq!(stride.get(1), Some(&3));
    assert_eq!(stride.get(2), Some(&5));
    assert_eq!(stride.get(3), None);
}

#[test]
fn stride_get_range() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(0..0), Some(Stride::<_, 2>::new(&[])));
    assert_eq!(stride.get(0..1), Some(Stride::<_, 2>::new(&[1, 2])));
    assert_eq!(stride.get(0..2), Some(Stride::<_, 2>::new(&[1, 2, 3, 4])));
    assert_eq!(stride.get(0..3), Some(stride));
    assert_eq!(stride.get(0..4), None);
}

#[test]
fn stride_get_range_from() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(0..), Some(stride));
    assert_eq!(stride.get(1..), Some(Stride::<_, 2>::new(&[3, 4, 5, 6])));
    assert_eq!(stride.get(2..), Some(Stride::<_, 2>::new(&[5, 6])));
    assert_eq!(stride.get(3..), Some(Stride::<_, 2>::new(&[])));
    assert_eq!(stride.get(4..), None);
}

#[test]
fn stride_get_range_full() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(..), Some(stride));
}

#[test]
fn stride_get_range_inclusive() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(0..=0), Some(Stride::<_, 2>::new(&[1, 2])));
    assert_eq!(stride.get(0..=1), Some(Stride::<_, 2>::new(&[1, 2, 3, 4])));
    assert_eq!(stride.get(0..=2), Some(stride));
    assert_eq!(stride.get(0..=3), None);
}

#[test]
fn stride_get_range_to() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(..0), Some(Stride::<_, 2>::new(&[])));
    assert_eq!(stride.get(..1), Some(Stride::<_, 2>::new(&[1, 2])));
    assert_eq!(stride.get(..2), Some(Stride::<_, 2>::new(&[1, 2, 3, 4])));
    assert_eq!(stride.get(..3), Some(stride));
    assert_eq!(stride.get(..4), None);
}

#[test]
fn stride_get_range_to_inclusive() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.get(..=0), Some(Stride::<_, 2>::new(&[1, 2])));
    assert_eq!(stride.get(..=1), Some(Stride::<_, 2>::new(&[1, 2, 3, 4])));
    assert_eq!(stride.get(..=2), Some(stride));
    assert_eq!(stride.get(..=3), None);
}

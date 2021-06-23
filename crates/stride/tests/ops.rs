use stride::Stride;

#[test]
fn stride_partial_eq() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = Stride::<_, 2>::new(&[1, 0, 4, 0]);
    assert_eq!(a, b);
}

#[test]
fn stride_partial_ne_len() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = Stride::<_, 2>::new(&[1, 0, 4, 0, 6]);
    assert_ne!(a, b);
}

#[test]
fn stride_partial_ne_values() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = Stride::<_, 2>::new(&[1, 0, 5, 0]);
    assert_ne!(a, b);
}

#[test]
fn stride_partial_eq_array() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = &[1, 4];
    assert_eq!(a, b);
}

#[test]
fn stride_partial_ne_array_len() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = &[1, 4, 6];
    assert_ne!(a, b);
}

#[test]
fn stride_partial_ne_array_values() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = &[1, 5];
    assert_ne!(a, &b[..]);
}

#[test]
fn stride_partial_eq_slice() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = &[1, 4];
    assert_eq!(a, b);
}

#[test]
fn stride_partial_ne_slice_len() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = &[1, 4, 6];
    assert_ne!(a, &b[..]);
}

#[test]
fn stride_partial_ne_slice_values() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = &[1, 5];
    assert_ne!(a, &b[..]);
}

#[test]
fn stride_hash() {
    let a = Stride::<_, 2>::new(&[1, 0, 4, 0]);
    let b = Stride::<_, 2>::new(&[1, -1, 4, -1]);
    let c = Stride::<_, 2>::new(&[1, -2, 4, -2]);
    let mut map = std::collections::HashSet::new();
    map.insert(a);
    map.insert(b);
    assert_eq!(map.len(), 1);
    map.remove(c);
    assert!(map.is_empty());
}

#[test]
fn stride_partial_ord() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 3, 0, 0]);
    let b = Stride::<_, 2>::new(&[1, 0, 4, 0]);
    assert!(a < b);
}

#[test]
fn stride_partial_ord_len() {
    let a = Stride::<_, 3>::new(&[1, 0, 0, 4, 0, 0]);
    let b = Stride::<_, 2>::new(&[1, 0, 4, 0, 6]);
    assert!(a < b);
}

#[test]
fn stride_index() {
    let stride = Stride::<_, 3>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride[0], 1);
    assert_eq!(stride[1], 4);
}

#[test]
fn stride_index_mut() {
    let mut data = vec![1, 2, 3, 4, 5, 6];
    let stride = Stride::<_, 3>::new_mut(data.as_mut_slice());
    stride[0] = 7;
    stride[1] = 8;
    assert_eq!(stride[0], 7);
    assert_eq!(stride[1], 8);
}

use stride::Stride;

#[test]
fn stride_iter() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    let vec: Vec<_> = stride.iter().collect();
    assert_eq!(vec, [&1, &3, &5]);
}

#[test]
fn stride_iter_mut() {
    let slice = &mut [1, 2, 3, 4, 5, 6];
    let stride = Stride::<_, 2>::new_mut(slice);
    for elem in stride.iter_mut() {
        *elem *= 2;
    }
    assert_eq!(slice, &[2, 2, 6, 4, 10, 6]);
}

#[test]
fn stride_iter_rev() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    let vec: Vec<_> = stride.iter().rev().collect();
    assert_eq!(vec, [&5, &3, &1]);
}

#[test]
fn stride_iter_skip_rev() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    let vec: Vec<_> = stride.iter().skip(1).rev().skip(1).collect();
    assert_eq!(vec, [&3]);
}

#[test]
fn stride_iter_len() {
    let stride = Stride::<_, 2>::new(&[1, 2, 3, 4, 5, 6]);
    assert_eq!(stride.iter().len(), 3);
}

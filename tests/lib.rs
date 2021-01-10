mod comps;
mod iter;
mod ops;

use vectrs::{vector, Vector};

#[test]
fn vector_debug_2() {
    let vector = Vector::new([1, 2]);
    assert_eq!(format!("{:?}", vector), "[1, 2]");
}

#[test]
fn vector_debug_4() {
    let vector = Vector::new([1, 3, 3, 7]);
    assert_eq!(format!("{:?}", vector), "[1, 3, 3, 7]");
}

#[test]
fn vector_debug_more() {
    let vector = Vector::new([1, 3, 3, 7, 0, 0, 0]);
    assert_eq!(format!("{:?}", vector), "[1, 3, 3, 7, 0, 0, 0]");
}

////////////////////////////////////////////////////////////////////////////////
// Constructors
////////////////////////////////////////////////////////////////////////////////

#[test]
fn vector_default() {
    let vector: Vector<i64, 3> = Vector::default();
    assert_eq!(vector, Vector::new([0, 0, 0]));
}

// `From` implementations

#[test]
fn vector_from_array() {
    let vector = Vector::from([1, 3, 3, 7]);
    assert_eq!(vector, Vector::new([1, 3, 3, 7]));
}

#[test]
fn vector_from_slice() {
    let vector: Vector<_, 4> = Vector::from(&[1, 3, 3, 7][..]);
    assert_eq!(vector, Vector::new([1, 3, 3, 7]));
}

#[cfg(feature = "std")]
#[test]
fn vector_from_vec() {
    let vector: Vector<_, 4> = Vector::from(vec![1, 3, 3, 7]);
    assert_eq!(vector, Vector::new([1, 3, 3, 7]));
}

#[test]
fn vector_from_tuple() {
    macro_rules! peel {
        ($n:literal, $($other:literal,)*) => { vector_from_tuples!($($other,)*) }
    }

    macro_rules! vector_from_tuples {
        () => ();
        ($($n:literal,)+) => {
            let vector = Vector::new([$($n,)+]);
            assert_eq!(vector, Vector::new([$($n,)+]));
            peel!($($n,)+);
        };
    }

    vector_from_tuples!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,);
}

// `FromPartial` implementations

#[test]
fn vector_from_partial_array() {
    let vector: Vector<_, 6> = Vector::from_partial([1, 3, 3, 7]);
    assert_eq!(vector, Vector::new([1, 3, 3, 7, 0, 0]));
}

#[test]
fn vector_from_partial_vector() {
    let vector: Vector<_, 6> = Vector::from_partial(Vector::new([1, 3, 3, 7]));
    assert_eq!(vector, Vector::new([1, 3, 3, 7, 0, 0]));
}

#[test]
fn vector_from_partial_slice() {
    let vector: Vector<_, 6> = Vector::from_partial(&[1, 3, 3, 7][..]);
    assert_eq!(vector, Vector::new([1, 3, 3, 7, 0, 0]));
}

#[cfg(feature = "std")]
#[test]
fn vector_from_partial_vec() {
    let vector: Vector<_, 6> = Vector::from_partial(vec![1, 3, 3, 7]);
    assert_eq!(vector, Vector::new([1, 3, 3, 7, 0, 0]));
}

#[test]
fn vector_from_partial_tuple() {
    macro_rules! peel {
        ($n:literal, $($other:literal,)*) => { vector_from_tuples!($($other,)*) }
    }

    macro_rules! vector_from_tuples {
        () => ();
        ($($n:literal,)+) => {
            let vector: Vector<_, 13> = Vector::from_partial(($($n,)+));
            assert_eq!(vector, Vector::from_partial([$($n,)+]));
            peel!($($n,)+);
        };
    }

    vector_from_tuples!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,);
}

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

#[test]
fn vector_macro() {
    const VECTOR: Vector<i64, 4> = vector![1, 3, 3, 7];
    assert_eq!(VECTOR, Vector::new([1, 3, 3, 7]));
    let vector = vector![-1, 0];
    assert_eq!(vector, Vector::new([-1, 0]));
    let vector = vector![1; 5];
    assert_eq!(vector, Vector::new([1, 1, 1, 1, 1]));
}

#[test]
fn vector_new() {
    const VECTOR: Vector<i64, 4> = Vector::new([1, 3, 3, 7]);
    assert_eq!(VECTOR[0], 1);
    assert_eq!(VECTOR[1], 3);
    assert_eq!(VECTOR[2], 3);
    assert_eq!(VECTOR[3], 7);
}

#[test]
fn vector_zero() {
    let vector: Vector<_, 3> = Vector::zero();
    assert_eq!(vector, Vector::new([0, 0, 0]));
}

#[test]
fn vector_as_slice() {
    let vector = Vector::new([1, 3, 3, 7]);
    assert_eq!(vector.as_slice(), vec![1, 3, 3, 7].as_slice());
}

#[test]
fn vector_as_mut_slice() {
    let mut vector = Vector::new([1, 2, 3, 4]);
    let slice = vector.as_mut_slice();
    slice[1] = 3;
    slice[3] = 7;
    assert_eq!(vector, Vector::new([1, 3, 3, 7]));
}

#[test]
fn vector_iter() {
    let vector = Vector::new([1, 3, 3, 7]);
    let values: Vec<_> = vector.iter().collect();
    assert_eq!(values, vec![&1, &3, &3, &7]);
}

#[test]
fn vector_into_array() {
    let vector = Vector::new([1, 3, 3, 7]);
    assert_eq!(vector.into_array(), [1, 3, 3, 7]);
}

#[test]
fn vector_unsigned_abs() {
    let vector: Vector<u64, 3> = Vector::new([1, 2, 3]);
    assert_eq!(vector.abs(), Vector::new([1, 2, 3]));
}

#[test]
fn vector_signed_abs() {
    let vector: Vector<i64, 3> = Vector::new([-1, 2, -3]);
    assert_eq!(vector.abs(), Vector::new([1, 2, 3]));
}

#[test]
fn vector_reduced() {
    assert_eq!(Vector::new([4, -8, 12]).reduced(), Vector::from([1, -2, 3]))
}

#[test]
fn vector_dot() {
    let a = Vector::new([1, 2, 3]);
    let b = Vector::new([4, 5, 6]);
    assert_eq!(a.dot(&b), 32);
    assert_eq!(b.dot(&a), 32);
}

#[test]
fn vector_unsigned_l1_norm() {
    let vector: Vector<u64, 4> = Vector::new([1, 3, 3, 7]);
    assert_eq!(vector.l1_norm(), 14);
}

#[test]
fn vector_signed_l1_norm() {
    let vector: Vector<i64, 4> = Vector::new([-1, 3, -3, 7]);
    assert_eq!(vector.l1_norm(), 14);
}

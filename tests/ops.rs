use vectrs::Vector;

macro_rules! for_each_op_assert_eq {
    ($a:expr, $op:tt, $b:expr, $expected:expr) => {
        let vector = $a $op $b;
        assert_eq!(vector, $expected);

        let vector = $a $op &$b;
        assert_eq!(vector, $expected);

        let vector = &$a $op $b;
        assert_eq!(vector, $expected);

        let vector = &$a $op &$b;
        assert_eq!(vector, $expected);
    };
}

macro_rules! for_each_op_assign_assert_eq {
    ($a:expr, $op:tt, $b:expr, $expected:expr) => {
        let mut vector = $a.clone();
        vector $op $b;
        assert_eq!(vector, $expected);

        let mut vector = $a.clone();
        vector $op &$b;
        assert_eq!(vector, $expected);
    };
}

////////////////////////////////////////////////////////////////////////////////
// Vector + T => Vector
////////////////////////////////////////////////////////////////////////////////

#[test]
fn vector_add_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, +, b, Vector::new([3, -1, 5, -5]) }
}

#[test]
fn vector_sub_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, -, b, Vector::new([-1, -5, 1, -9]) }
}

#[test]
fn vector_mul_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, *, b, Vector::new([2, -6, 6, -14]) }
}

#[test]
fn vector_div_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, /, b, Vector::new([0, -1, 1, -3]) }
}

#[test]
fn vector_rem_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, %, b, Vector::new([1, -1, 1, -1]) }
}

#[test]
fn vector_bit_and_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, &, b, Vector::new([0, 0, 2, 0]) }
}

#[test]
fn vector_bit_or_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, |, b, Vector::new([3, -1, 3, -5]) }
}

#[test]
fn vector_bit_xor_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, ^, b, Vector::new([3, -1, 1, -5]) }
}

#[test]
fn vector_shl_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, <<, b, Vector::new([4, -12, 12, -28]) }
}

#[test]
fn vector_shr_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assert_eq! { a, >>, b, Vector::new([0, -1, 0, -2]) }
}

////////////////////////////////////////////////////////////////////////////////
// Vector += T
////////////////////////////////////////////////////////////////////////////////

#[test]
fn vector_add_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, +=, b, Vector::new([3, -1, 5, -5]) }
}

#[test]
fn vector_sub_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, -=, b, Vector::new([-1, -5, 1, -9]) }
}

#[test]
fn vector_mul_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, *=, b, Vector::new([2, -6, 6, -14]) }
}

#[test]
fn vector_div_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, /=, b, Vector::new([0, -1, 1, -3]) }
}

#[test]
fn vector_rem_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, %=, b, Vector::new([1, -1, 1, -1]) }
}

#[test]
fn vector_bit_and_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, &=, b, Vector::new([0, 0, 2, 0]) }
}

#[test]
fn vector_bit_or_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, |=, b, Vector::new([3, -1, 3, -5]) }
}

#[test]
fn vector_bit_xor_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, ^=, b, Vector::new([3, -1, 1, -5]) }
}

#[test]
fn vector_bit_shl_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, <<=, b, Vector::new([4, -12, 12, -28]) }
}

#[test]
fn vector_bit_shr_assign_scalar() {
    let a = Vector::new([1, -3, 3, -7]);
    let b = 2;
    for_each_op_assign_assert_eq! { a, >>=, b, Vector::new([0, -1, 0, -2]) }
}

////////////////////////////////////////////////////////////////////////////////
// Vector + Vector => Vector
////////////////////////////////////////////////////////////////////////////////

#[test]
fn vector_add() {
    let a = Vector::new([1, 2, 3]);
    let b = Vector::new([1, -2, 3]);
    let c = Vector::new([2, 0, 6]);
    for_each_op_assert_eq! { a, +, b, c }
}

#[test]
fn vector_sub() {
    let a = Vector::new([1, 2, 3]);
    let b = Vector::new([1, -2, 1]);
    let c = Vector::new([0, 4, 2]);
    for_each_op_assert_eq! { a, -, b, c }
}

////////////////////////////////////////////////////////////////////////////////
// Vector += Vector
////////////////////////////////////////////////////////////////////////////////

#[test]
fn vector_add_assign() {
    let a = Vector::new([1, 2, 3]);
    let b = Vector::new([1, -2, 3]);
    let c = Vector::new([2, 0, 6]);
    for_each_op_assign_assert_eq! { a, +=, b, c }
}

#[test]
fn vector_sub_assign() {
    let a = Vector::new([1, 2, 3]);
    let b = Vector::new([1, -2, 1]);
    let c = Vector::new([0, 4, 2]);
    for_each_op_assign_assert_eq! { a, -=, b, c }
}

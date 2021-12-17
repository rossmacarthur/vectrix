use vectrix::{matrix, row_vector, vector};

#[test]
fn vector_debug() {
    assert_eq!(format!("{:?}", vector![24, 7]), "[24, 7]");
}

#[test]
fn row_vector_debug() {
    assert_eq!(format!("{:?}", row_vector![24, 7]), "[24, 7]");
}

#[test]
fn matrix_debug() {
    assert_eq!(
        format!("{:?}", matrix![-1, 3, 0; 0, 0, 0; -3, 24, 7]),
        "[[-1, 0, -3], [3, 0, 24], [0, 0, 7]]"
    );
}

#[test]
fn matrix_debug_precision() {
    assert_eq!(
        format!("{:.2?}", matrix![-1.0, 3.12, 0.0; -3.3, 24.7839, 7.1]),
        "[[-1.00, -3.30], [3.12, 24.78], [0.00, 7.10]]"
    );
}

#[test]
fn vector_display() {
    assert_eq!(
        format!("{}", vector![24, 7]),
        "
 ┌    ┐
 │ 24 │
 │  7 │
 └    ┘
"
    );
}

#[test]
fn vector_display_lower_exp() {
    assert_eq!(
        format!("{:e}", vector![24, 7]),
        "
 ┌       ┐
 │ 2.4e1 │
 │   7e0 │
 └       ┘
"
    );
}

#[test]
fn vector_display_upper_exp() {
    assert_eq!(
        format!("{:E}", vector![24, 7]),
        "
 ┌       ┐
 │ 2.4E1 │
 │   7E0 │
 └       ┘
"
    );
}

#[test]
fn vector_display_octal() {
    assert_eq!(
        format!("{:o}", vector![24, 7]),
        "
 ┌    ┐
 │ 30 │
 │  7 │
 └    ┘
"
    );
}

#[test]
fn vector_display_lower_hex() {
    assert_eq!(
        format!("{:x}", vector![31, 7]),
        "
 ┌    ┐
 │ 1f │
 │  7 │
 └    ┘
"
    );
}

#[test]
fn vector_display_upper_hex() {
    assert_eq!(
        format!("{:X}", vector![31, 7]),
        "
 ┌    ┐
 │ 1F │
 │  7 │
 └    ┘
"
    );
}

#[test]
fn vector_display_binary() {
    assert_eq!(
        format!("{:b}", vector![24, 7]),
        "
 ┌       ┐
 │ 11000 │
 │   111 │
 └       ┘
"
    );
}

#[test]
fn row_vector_display() {
    assert_eq!(
        format!("{}", row_vector![24, 7]),
        "
 ┌       ┐
 │ 24  7 │
 └       ┘
"
    );
}

#[test]
fn matrix_display() {
    assert_eq!(
        format!("{}", matrix![-1, 3, 0; -3, 24, 7]),
        "
 ┌           ┐
 │ -1   3  0 │
 │ -3  24  7 │
 └           ┘
"
    );
}

#[test]
fn matrix_display_precision() {
    assert_eq!(
        format!("{:.2}", matrix![-1.0, 3.12, 0.0; -3.3, 24.7839, 7.1]),
        "
 ┌                    ┐
 │ -1.00   3.12  0.00 │
 │ -3.30  24.78  7.10 │
 └                    ┘
"
    );
}

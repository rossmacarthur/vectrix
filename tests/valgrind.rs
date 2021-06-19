//! Valgrind should be run on these tests.

use std::sync::atomic::{AtomicUsize, Ordering};

use vectrix::matrix;

#[test]
#[should_panic]
fn into_iter_clone_no_leak() {
    // This test makes sure that the `Clone` implementation for `IntoIter` does
    // not cause any undefined behaviour if `T.clone()` panics.

    #[derive(Debug, PartialEq)]
    struct Num(Box<i64>);

    static CLONE_COUNT: AtomicUsize = AtomicUsize::new(0);

    impl Clone for Num {
        fn clone(&self) -> Self {
            // Make the implementation clone the two times, and panic for any
            // more clones.
            CLONE_COUNT.fetch_add(1, Ordering::SeqCst);
            if CLONE_COUNT.load(Ordering::SeqCst) > 2 {
                panic!();
            } else {
                Num(self.0.clone())
            }
        }
    }
    let into_iter = matrix![
        Num(Box::new(1)),
        Num(Box::new(3)),
        Num(Box::new(3)),
        Num(Box::new(7))
    ]
    .into_iter();
    CLONE_COUNT.store(0, Ordering::SeqCst);
    drop(into_iter.clone());
}

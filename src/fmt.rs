use core::fmt;
use core::fmt::Write;

use crate::new;
use crate::{Matrix, Vector};

////////////////////////////////////////////////////////////////////////////////
// Debug
////////////////////////////////////////////////////////////////////////////////

impl<T: fmt::Debug, const M: usize, const N: usize> fmt::Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if M == 1 || N == 1 {
            f.debug_list().entries(self.iter()).finish()
        } else {
            fmt::Debug::fmt(&self.data, f)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Display
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
struct CharCounter(usize);

impl fmt::Write for CharCounter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0 += s.chars().count();
        Ok(())
    }
}

macro_rules! count_chars {
    ($($arg:tt)*) => {{
        let mut counter = CharCounter::default();
        write!(counter, $($arg)*).unwrap();
        counter.0
    }};
}

fn fmt_matrix<T, F1, F2, const M: usize, const N: usize>(
    matrix: &Matrix<T, M, N>,
    f: &mut fmt::Formatter<'_>,
    width_fn: F1,
    mut fmt_fn: F2,
) -> fmt::Result
where
    F1: FnMut(&T) -> usize + Copy,
    F2: FnMut(&mut fmt::Formatter<'_>, &T, usize) -> fmt::Result + Copy,
{
    let widths = matrix
        .iter_columns()
        .map(|col| col.iter().map(width_fn).max().unwrap_or(0));
    let widths: Vector<usize, N> = unsafe { new::collect_unchecked(widths) };

    f.write_str("\n ┌")?;
    for w in widths.iter() {
        write!(f, " {:1$} ", "", w)?;
    }
    f.write_str("┐\n")?;

    for row in matrix.iter_rows() {
        f.write_str(" │")?;
        for (d, &width) in row.iter().zip(widths.iter()) {
            f.write_str(" ")?;
            fmt_fn(f, d, width)?;
            f.write_str(" ")?;
        }
        f.write_str("│\n")?;
    }

    f.write_str(" └")?;
    for w in widths.iter() {
        write!(f, " {:1$} ", "", w)?;
    }
    f.write_str("┘\n")?;

    Ok(())
}

macro_rules! impl_fmt {
    ($Trait:path, $count_precision:expr, $count:expr, $fmt_precision:expr, $fmt:expr) => {
        impl<T: $Trait, const M: usize, const N: usize> $Trait for Matrix<T, M, N> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let precision = f.precision();
                fmt_matrix(
                    self,
                    f,
                    |d| match precision {
                        Some(p) => count_chars!($count_precision, d, p),
                        None => count_chars!($count, d),
                    },
                    |f, d, width| match precision {
                        Some(p) => write!(f, $fmt_precision, d, width, p),
                        None => write!(f, $fmt, d, width),
                    },
                )
            }
        }
    };
}

impl_fmt! { fmt::Display, "{:.1$}", "{}", "{:1$.2$}", "{:1$}" }
impl_fmt! { fmt::LowerExp, "{:.1$e}", "{:e}", "{:1$.2$e}", "{:1$e}" }
impl_fmt! { fmt::UpperExp, "{:.1$E}", "{:E}", "{:1$.2$E}", "{:1$E}" }
impl_fmt! { fmt::Octal, "{:.1$o}", "{:o}", "{:1$.2$o}", "{:1$o}" }
impl_fmt! { fmt::LowerHex, "{:.1$x}", "{:x}", "{:1$.2$x}", "{:1$x}" }
impl_fmt! { fmt::UpperHex, "{:.1$X}", "{:X}", "{:1$.2$X}", "{:1$X}" }
impl_fmt! { fmt::Binary, "{:.1$b}", "{:b}", "{:1$.2$b}", "{:1$b}" }

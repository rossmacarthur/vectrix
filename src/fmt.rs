use core::fmt;
use core::fmt::Write;

use crate::{new, Matrix, Vector};

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
    if M == 1 || N == 1 {
        f.write_str("(")?;
        for (i, d) in matrix.iter().enumerate() {
            if i != 0 {
                f.write_str(", ")?;
            }
            fmt_fn(f, d, 0)?
        }
        f.write_str(")")?;
    } else {
        let widths = matrix
            .iter_columns()
            .map(|col| col.iter().map(width_fn).max().unwrap_or(0));
        let widths: Vector<usize, N> = unsafe { new::collect_unchecked(widths) };

        for (i, row) in matrix.iter_rows().enumerate() {
            let (left, right) = match i {
                0 => ("⎛ ", " ⎞\n"),
                i if i != M - 1 => ("⎜ ", " ⎟\n"),
                _ => ("⎝ ", " ⎠"),
            };

            f.write_str(left)?;
            for (i, (d, width)) in row.iter().zip(widths).enumerate() {
                if i != 0 {
                    f.write_str(", ")?;
                }
                fmt_fn(f, d, width)?;
            }
            f.write_str(right)?;
        }
    }
    Ok(())
}

impl<T: fmt::Debug, const M: usize, const N: usize> fmt::Debug for Matrix<T, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision();
        fmt_matrix(
            self,
            f,
            |d| match precision {
                Some(places) => count_chars!("{:.1$?}", d, places),
                None => count_chars!("{:?}", d),
            },
            |f, d, width| match precision {
                Some(places) => write!(f, "{:1$.2$?}", d, width, places),
                None => write!(f, "{:1$?}", d, width),
            },
        )
    }
}

impl<T: fmt::Display, const M: usize, const N: usize> fmt::Display for Matrix<T, M, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision();
        fmt_matrix(
            self,
            f,
            |d| match precision {
                Some(places) => count_chars!("{:.1$}", d, places),
                None => count_chars!("{}", d),
            },
            |f, d, width| match precision {
                Some(places) => write!(f, "{:1$.2$}", d, width, places),
                None => write!(f, "{:1$}", d, width),
            },
        )
    }
}

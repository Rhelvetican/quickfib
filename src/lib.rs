//! # Fast fibbonacci algorithm.
//!
//! This crate provides a fast implementation of the fibbonacci algorithm, `no_std` compatible.
//! The algorithm is based on the following formula:
//!
//! ```math
//! F(2n) = F(n) * (2 * F(n+1) - F(n))
//! F(2n+1) = F(n)^2 + F(n+1)^2
//! ```

use core::ops::{Add, Div, Mul, Rem, Sub};

/// Calculate the n-th fibbonacci number.
/// The function may panic if the type T is not large enough to hold the result.
///
/// # Examples
/// ```rust
/// let x = quickfib::fibbonacci(20);
/// assert_eq!(x, 6765);
/// ```

pub fn fibbonacci<T>(n: T) -> T
where
    T: From<u8>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + PartialEq
        + Copy,
{
    fn __fib<T>(n: T) -> (T, T)
    where
        T: From<u8>
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Rem<Output = T>
            + PartialEq
            + Copy,
    {
        if n == T::from(0) {
            (T::from(0), T::from(1))
        } else {
            let (a, b) = __fib(n / T::from(2));
            let c = a * ((b * T::from(2)) - a);
            let d = a * a + b * b;
            if n % T::from(2) == T::from(0) {
                (c, d)
            } else {
                (d, c + d)
            }
        }
    }

    __fib(n).0
}

/// Calculate the fibbonacci numbers for a range of numbers.
/// The function may panic if the type U is not large enough to hold the result.
///
/// # Examples
/// ```rust
///
/// let x = quickfib::fibbonacci_range(0..=9);
/// assert_eq!(x, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
/// ```

pub fn fibbonacci_range<T, U>(range: T) -> Vec<U>
where
    T: IntoIterator<Item = U>,
    U: From<u8>
        + Add<Output = U>
        + Sub<Output = U>
        + Mul<Output = U>
        + Div<Output = U>
        + Rem<Output = U>
        + PartialEq
        + Copy,
{
    let mut result = Vec::new();
    for i in range {
        result.push(fibbonacci(i));
    }
    result
}

#[cfg(test)]
mod tests {

    use super::{fibbonacci, fibbonacci_range};

    #[test]
    fn calc_1() {
        let result = fibbonacci(2);
        assert_eq!(result, 1);
    }

    #[test]
    fn calc_2() {
        let result = fibbonacci(20);
        assert_eq!(result, 6765);
    }

    #[test]
    fn calc_3() {
        let result = fibbonacci(30);
        assert_eq!(result, 832040);
    }

    #[test]
    fn calc_4() {
        let result = fibbonacci::<u128>(100);
        assert_eq!(result, 354224848179261915075);
    }

    #[test]
    fn calc_range() {
        let result = fibbonacci_range(0..=9);
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}

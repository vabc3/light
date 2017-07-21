//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//! 
//! ```
//! use w::add_two;
//! assert_eq!(4, w::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use w::add_two;
///
/// assert_eq!(23, add_two(21));
/// ```
/// ```
/// use w::add_two;
///
/// assert_eq!(23, add_two(21));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }
}

mod lineno;
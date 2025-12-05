#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleDefault` trait.

#[tuplities_derive::impl_tuple_default]
/// A trait for creating default instances of tuples.
pub trait TupleDefault {
    /// Returns the default value of the tuple.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_default::TupleDefault;
    ///
    /// let default_tuple: (i32, String, f64) = <(i32, String, f64)>::tuple_default();
    ///
    /// assert_eq!(default_tuple, (0, String::new(), 0.0));
    /// ```
    fn tuple_default() -> Self;
}

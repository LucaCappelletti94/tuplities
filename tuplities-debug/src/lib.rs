//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleDebug` trait.
#![no_std]

extern crate alloc;

#[tuplities_derive::impl_tuple_debug]
/// A trait for debugging tuples.
pub trait TupleDebug {
    /// Returns a string representation of the tuple for debugging.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_debug::TupleDebug;
    ///
    /// let tuple = (1, "hello", 3.14);
    /// let debug_str = tuple.tuple_debug();
    ///
    /// assert_eq!(debug_str, "(1, \"hello\", 3.14)");
    /// ```
    fn tuple_debug(&self) -> alloc::string::String;
}

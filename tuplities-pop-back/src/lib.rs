#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TuplePopBack` trait.

#[tuplities_derive::impl_pop_back]
/// A trait for tuples that defines the `Init` and `Back` types, and provides a method to pop the back element.
pub trait TuplePopBack {
    /// The type of the tuple without the last element.
    type Init;

    /// The type of the last element.
    type Back;

    /// Consumes the tuple and returns the initial part and the last element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_pop_back::TuplePopBack;
    ///
    /// let tuple = (1, 2, 3);
    /// let (front, back) = tuple.pop_back();
    /// assert_eq!(front, (1, 2));
    /// assert_eq!(back, 3);
    /// ```
    fn pop_back(self) -> (Self::Init, Self::Back);
}

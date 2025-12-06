//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleFrom` and `TupleInto` traits.

#[tuplities_derive::impl_tuple_from]
/// A trait for infallibly converting from one tuple type to another.
///
/// This trait allows converting between tuples where each element can be converted
/// using the standard `From` trait. Unlike `TupleTryFrom`, this trait guarantees
/// that the conversion will succeed.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleFrom<T> {
    /// Converts from `T` into `Self`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_from::TupleFrom;
    ///
    /// let source = (1u8, 2u8);
    /// let target: (u32, u32) = TupleFrom::tuple_from(source);
    /// assert_eq!(target, (1, 2));
    /// ```
    fn tuple_from(value: T) -> Self;
}

/// A trait for infallibly converting a tuple into another tuple type.
///
/// This trait allows converting tuples into other tuple types where each element can be converted
/// using the standard `From` trait. Unlike `TupleTryInto`, this trait guarantees
/// that the conversion will succeed.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleInto<T> {
    /// Converts `self` into `T`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_from::TupleInto;
    ///
    /// let source = (1u8, 2u8);
    /// let target: (u32, u32) = TupleInto::tuple_into(source);
    /// assert_eq!(target, (1, 2));
    /// ```
    fn tuple_into(self) -> T;
}

// Blanket implementation of TupleInto based on TupleFrom
impl<T, U> TupleInto<T> for U
where
    T: TupleFrom<U>,
{
    fn tuple_into(self) -> T {
        T::tuple_from(self)
    }
}

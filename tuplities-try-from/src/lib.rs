//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleTryFrom` and `TupleTryInto` traits.

#[tuplities_derive::impl_tuple_try_from]
/// A trait for fallibly converting from one tuple type to another.
///
/// This trait allows converting between tuples where each element can be converted
/// using the standard `TryFrom` trait. The error type `E` must implement `From` for
/// each individual `TryFrom` error in the tuple, allowing error accumulation.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleTryFrom<T, E> {
    /// Attempts to convert from `T` into `Self`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if any element conversion fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_try_from::TupleTryFrom;
    ///
    /// let source = (1u32, 2u32);
    /// let target: Result<(u8, u8), _> = TupleTryFrom::<_, std::num::TryFromIntError>::tuple_try_from(source);
    /// assert_eq!(target, Ok((1, 2)));
    ///
    /// let invalid = (300u32, 2u32); // 300 is too big for u8
    /// let result: Result<(u8, u8), _> = TupleTryFrom::<_, std::num::TryFromIntError>::tuple_try_from(invalid);
    /// assert!(result.is_err());
    /// ```
    fn tuple_try_from(value: T) -> Result<Self, E>
    where
        Self: Sized;
}

/// A trait for fallibly converting a tuple into another tuple type.
///
/// This trait allows converting tuples into other tuple types where each element can be converted
/// using the standard `TryFrom` trait. The error type `E` must implement `From` for
/// each individual `TryFrom` error in the tuple, allowing error accumulation.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleTryInto<T, E> {
    /// Attempts to convert `self` into `T`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if any element conversion fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_try_from::TupleTryInto;
    ///
    /// let source = (1u32, 2u32);
    /// let target: Result<(u8, u8), _> = TupleTryInto::<(u8, u8), std::num::TryFromIntError>::tuple_try_into(source);
    /// assert_eq!(target, Ok((1, 2)));
    ///
    /// let invalid = (300u32, 2u32); // 300 is too big for u8
    /// let result: Result<(u8, u8), _> = TupleTryInto::<(u8, u8), std::num::TryFromIntError>::tuple_try_into(invalid);
    /// assert!(result.is_err());
    /// ```
    fn tuple_try_into(self) -> Result<T, E>;
}

// Blanket implementation of TupleTryInto based on TupleTryFrom
impl<T, U, E> TupleTryInto<T, E> for U
where
    T: TupleTryFrom<U, E>,
{
    fn tuple_try_into(self) -> Result<T, E> {
        T::tuple_try_from(self)
    }
}

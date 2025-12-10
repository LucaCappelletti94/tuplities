//! Module providing the `NestedTupleTryFrom` and `NestedTupleTryInto` traits.

use core::convert::TryFrom;

/// A trait for fallibly converting from one nested tuple type to another.
///
/// This trait allows converting between nested tuples where each element can be converted
/// using the standard `TryFrom` trait. The error type `E` must implement `From` for
/// each individual `TryFrom` error in the tuple, allowing error accumulation.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleTryFrom<T, E>: Sized {
    /// Attempts to convert from `T` into `Self`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if any element conversion fails.
    fn nested_tuple_try_from(other: T) -> Result<Self, E>;
}

/// A trait for fallibly converting into another nested tuple type.
///
/// This trait is automatically implemented for any type that implements `NestedTupleTryFrom`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleTryInto<T, E>: Sized {
    /// Attempts to convert `Self` into `T`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if any element conversion fails.
    fn nested_tuple_try_into(self) -> Result<T, E>;
}

impl<T, U, E> NestedTupleTryInto<U, E> for T
where
    U: NestedTupleTryFrom<T, E>,
{
    #[inline]
    fn nested_tuple_try_into(self) -> Result<U, E> {
        U::nested_tuple_try_from(self)
    }
}

impl<E> NestedTupleTryFrom<(), E> for () {
    #[inline]
    fn nested_tuple_try_from(_other: ()) -> Result<Self, E> {
        Ok(())
    }
}

impl<Head, OtherHead, E> NestedTupleTryFrom<(OtherHead,), E> for (Head,)
where
    Head: TryFrom<OtherHead>,
    E: From<Head::Error>,
{
    #[inline]
    fn nested_tuple_try_from(other: (OtherHead,)) -> Result<Self, E> {
        let (other_head,) = other;
        match Head::try_from(other_head) {
            Ok(head) => Ok((head,)),
            Err(e) => Err(E::from(e)),
        }
    }
}

impl<Head, Tail, OtherHead, OtherTail, E> NestedTupleTryFrom<(OtherHead, OtherTail), E>
    for (Head, Tail)
where
    Head: TryFrom<OtherHead>,
    E: From<Head::Error>,
    Tail: NestedTupleTryFrom<OtherTail, E>,
{
    #[inline]
    fn nested_tuple_try_from(other: (OtherHead, OtherTail)) -> Result<Self, E> {
        let (other_head, other_tail) = other;
        let head = Head::try_from(other_head).map_err(E::from)?;
        let tail = Tail::nested_tuple_try_from(other_tail)?;
        Ok((head, tail))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::Infallible;

    #[derive(Debug, PartialEq)]
    struct MyError;

    impl From<Infallible> for MyError {
        fn from(_: Infallible) -> Self {
            MyError
        }
    }

    #[test]
    fn test_nested_tuple_try_from_single() {
        let source = (1u8,);
        let target: Result<(u16,), MyError> = NestedTupleTryFrom::nested_tuple_try_from(source);
        assert_eq!(target, Ok((1u16,)));
    }

    #[test]
    fn test_nested_tuple_try_from_nested() {
        let source = (1u8, (2u8, (3u8,)));
        let target: Result<(u16, (u16, (u16,))), MyError> =
            NestedTupleTryFrom::nested_tuple_try_from(source);
        assert_eq!(target, Ok((1u16, (2u16, (3u16,)))));
    }

    #[test]
    fn test_nested_tuple_try_into() {
        let source = (1u8, (2u8,));
        let target: Result<(u16, (u16,)), MyError> = source.nested_tuple_try_into();
        assert_eq!(target, Ok((1u16, (2u16,))));
    }

    // Note: We avoid using `TryFromIntError` in these tests to improve portability
    // and to focus on custom error types that do not implement `From<Infallible>`.

    #[derive(Debug, PartialEq)]
    struct WrapErr;

    struct WrapI16(i16);

    impl TryFrom<WrapI16> for u8 {
        type Error = WrapErr;

        fn try_from(val: WrapI16) -> Result<u8, WrapErr> {
            if let Ok(val) = u8::try_from(val.0) {
                Ok(val)
            } else {
                Err(WrapErr)
            }
        }
    }

    #[test]
    fn test_unit_tuple_try_from() {
        let source = ();
        let target: Result<(), WrapErr> = NestedTupleTryFrom::nested_tuple_try_from(source);
        assert_eq!(target, Ok(()));
    }

    #[test]
    fn test_nested_tuple_try_from_custom_wrapper_error_single() {
        let source = (WrapI16(-1),);
        let target: Result<(u8,), WrapErr> = NestedTupleTryFrom::nested_tuple_try_from(source);
        assert!(target.is_err());
    }

    #[test]
    fn test_nested_tuple_try_from_custom_wrapper_error_nested() {
        let source = (WrapI16(256), (WrapI16(1), (WrapI16(2),)));
        let target: Result<(u8, (u8, (u8,))), WrapErr> =
            NestedTupleTryFrom::nested_tuple_try_from(source);
        assert!(target.is_err());
    }
}

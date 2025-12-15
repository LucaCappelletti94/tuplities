//! Module providing the `NestedTupleOptionTryFrom` and `NestedTupleOptionTryInto` traits.

use core::convert::TryFrom;

/// A trait for fallibly converting from one nested tuple type to another.
///
/// This trait allows converting between nested tuples where each element can be converted
/// using the standard `TryFrom` trait. The error type `E` must implement `From` for
/// each individual `TryFrom` error in the tuple, allowing error accumulation.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleOptionTryFrom<T, E>: Sized {
    /// Attempts to convert from `T` into `Self`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if any element conversion fails.
    fn nested_tuple_option_try_from(other: T) -> Result<Self, E>;
}

/// A trait for fallibly converting into another nested tuple type.
///
/// This mirrors `TryInto` but works element-wise across nested tuple structures.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleOptionTryInto<T, E>: Sized {
    /// Attempts to convert `Self` into `T`.
    ///
    /// # Errors
    ///
    /// Returns an error of type `E` if any element conversion fails.
    fn nested_tuple_option_try_into(self) -> Result<T, E>;
}

impl<E> NestedTupleOptionTryInto<(), E> for () {
    #[inline]
    fn nested_tuple_option_try_into(self) -> Result<(), E> {
        Ok(())
    }
}

impl<Head, OtherHead, E> NestedTupleOptionTryInto<(Option<OtherHead>,), E> for (Option<Head>,)
where
    Head: TryInto<OtherHead>,
    E: From<Head::Error>,
{
    #[inline]
    fn nested_tuple_option_try_into(self) -> Result<(Option<OtherHead>,), E> {
        let (head,) = self;
        match head {
            Some(val) => match val.try_into() {
                Ok(other_head) => Ok((Some(other_head),)),
                Err(e) => Err(E::from(e)),
            },
            None => Ok((None,)),
        }
    }
}

impl<Head, Tail, OtherHead, OtherTail, E>
    NestedTupleOptionTryInto<(Option<OtherHead>, OtherTail), E> for (Option<Head>, Tail)
where
    Head: TryInto<OtherHead>,
    E: From<Head::Error>,
    Tail: NestedTupleOptionTryInto<OtherTail, E>,
{
    #[inline]
    fn nested_tuple_option_try_into(self) -> Result<(Option<OtherHead>, OtherTail), E> {
        let (head, tail) = self;
        let other_head = match head {
            Some(val) => Some(val.try_into().map_err(E::from)?),
            None => None,
        };
        let other_tail = tail.nested_tuple_option_try_into()?;
        Ok((other_head, other_tail))
    }
}

/// A trait for infallibly converting from one nested tuple type to another.
///
/// This mirrors `From` but works element-wise across nested tuple structures.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleOptionFrom<T>: Sized {
    /// Converts `other` into `Self` by applying `From` element-wise.
    fn nested_tuple_option_from(other: T) -> Self;
}

/// A trait for infallibly converting into another nested tuple type.
///
/// This mirrors `Into` but works element-wise across nested tuple structures.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleOptionInto<T>: Sized {
    /// Converts `self` into `T` by applying `Into` element-wise.
    fn nested_tuple_option_into(self) -> T;
}

impl NestedTupleOptionInto<()> for () {
    #[inline]
    fn nested_tuple_option_into(self) -> () {}
}

impl<Head, OtherHead> NestedTupleOptionInto<(Option<OtherHead>,)> for (Option<Head>,)
where
    Head: Into<OtherHead>,
{
    #[inline]
    fn nested_tuple_option_into(self) -> (Option<OtherHead>,) {
        let (head,) = self;
        (head.map(Into::into),)
    }
}

impl<Head, Tail, OtherHead, OtherTail> NestedTupleOptionInto<(Option<OtherHead>, OtherTail)>
    for (Option<Head>, Tail)
where
    Head: Into<OtherHead>,
    Tail: NestedTupleOptionInto<OtherTail>,
{
    #[inline]
    fn nested_tuple_option_into(self) -> (Option<OtherHead>, OtherTail) {
        let (head, tail) = self;
        (head.map(Into::into), tail.nested_tuple_option_into())
    }
}

impl NestedTupleOptionFrom<()> for () {
    #[inline]
    fn nested_tuple_option_from(_other: ()) -> Self {}
}

impl<Head, OtherHead> NestedTupleOptionFrom<(Option<OtherHead>,)> for (Option<Head>,)
where
    Head: From<OtherHead>,
{
    #[inline]
    fn nested_tuple_option_from((other_head,): (Option<OtherHead>,)) -> Self {
        (other_head.map(Head::from),)
    }
}

impl<Head, Tail, OtherHead, OtherTail> NestedTupleOptionFrom<(Option<OtherHead>, OtherTail)>
    for (Option<Head>, Tail)
where
    Head: From<OtherHead>,
    Tail: NestedTupleOptionFrom<OtherTail>,
{
    #[inline]
    fn nested_tuple_option_from(other: (Option<OtherHead>, OtherTail)) -> Self {
        let (other_head, other_tail) = other;
        let head = other_head.map(Head::from);
        let tail = Tail::nested_tuple_option_from(other_tail);
        (head, tail)
    }
}

impl<E> NestedTupleOptionTryFrom<(), E> for () {
    #[inline]
    fn nested_tuple_option_try_from(_other: ()) -> Result<Self, E> {
        Ok(())
    }
}

impl<Head, OtherHead, E> NestedTupleOptionTryFrom<(Option<OtherHead>,), E> for (Option<Head>,)
where
    Head: TryFrom<OtherHead>,
    E: From<Head::Error>,
{
    #[inline]
    fn nested_tuple_option_try_from(other: (Option<OtherHead>,)) -> Result<Self, E> {
        let (other_head,) = other;
        match other_head {
            Some(val) => match Head::try_from(val) {
                Ok(head) => Ok((Some(head),)),
                Err(e) => Err(E::from(e)),
            },
            None => Ok((None,)),
        }
    }
}

impl<Head, Tail, OtherHead, OtherTail, E>
    NestedTupleOptionTryFrom<(Option<OtherHead>, OtherTail), E> for (Option<Head>, Tail)
where
    Head: TryFrom<OtherHead>,
    E: From<Head::Error>,
    Tail: NestedTupleOptionTryFrom<OtherTail, E>,
{
    #[inline]
    fn nested_tuple_option_try_from(other: (Option<OtherHead>, OtherTail)) -> Result<Self, E> {
        let (other_head, other_tail) = other;
        let head = match other_head {
            Some(val) => Some(Head::try_from(val).map_err(E::from)?),
            None => None,
        };
        let tail = Tail::nested_tuple_option_try_from(other_tail)?;
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
    fn test_nested_tuple_option_try_from_single() {
        let source = (Some(1u8),);
        let target: Result<(Option<u16>,), MyError> =
            NestedTupleOptionTryFrom::nested_tuple_option_try_from(source);
        assert_eq!(target, Ok((Some(1u16),)));
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_tuple_option_try_from_nested() {
        let source = (Some(1u8), (Some(2u8), (Some(3u8),)));
        let target: Result<(Option<u16>, (Option<u16>, (Option<u16>,))), MyError> =
            NestedTupleOptionTryFrom::nested_tuple_option_try_from(source);
        assert_eq!(target, Ok((Some(1u16), (Some(2u16), (Some(3u16),)))));
    }

    #[test]
    fn test_nested_tuple_option_try_into() {
        let source = (Some(1u8), (Some(2u8),));
        let target: Result<(Option<u16>, (Option<u16>,)), MyError> =
            source.nested_tuple_option_try_into();
        assert_eq!(target, Ok((Some(1u16), (Some(2u16),))));
    }

    #[test]
    fn test_nested_tuple_option_from_single() {
        let source = (Some(1u8),);
        let target: (Option<u16>,) = NestedTupleOptionFrom::nested_tuple_option_from(source);
        assert_eq!(target, (Some(1u16),));
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_tuple_option_from_nested() {
        let source = (Some(1u8), (Some(2u8), (Some(3u8),)));
        let target: (Option<u16>, (Option<u16>, (Option<u16>,))) =
            NestedTupleOptionFrom::nested_tuple_option_from(source);
        assert_eq!(target, (Some(1u16), (Some(2u16), (Some(3u16),))));
    }

    #[test]
    fn test_nested_tuple_option_into() {
        let source = (Some(1u8), (Some(2u8),));
        let target: (Option<u16>, (Option<u16>,)) = source.nested_tuple_option_into();
        assert_eq!(target, (Some(1u16), (Some(2u16),)));
    }

    #[test]
    fn test_nested_tuple_option_from_empty() {
        let source = ();
        let target: () = NestedTupleOptionFrom::nested_tuple_option_from(source);
        assert_eq!(target, ());
    }

    #[test]
    fn test_nested_tuple_option_into_empty() {
        let source = ();
        let target: () = source.nested_tuple_option_into();
        assert_eq!(target, ());
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
        let target: Result<(), WrapErr> =
            NestedTupleOptionTryFrom::nested_tuple_option_try_from(source);
        assert_eq!(target, Ok(()));
    }

    #[test]
    fn test_nested_tuple_option_try_from_custom_wrapper_error_single() {
        let source = (Some(WrapI16(-1)),);
        let target: Result<(Option<u8>,), WrapErr> =
            NestedTupleOptionTryFrom::nested_tuple_option_try_from(source);
        assert!(target.is_err());
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_tuple_option_try_from_custom_wrapper_error_nested() {
        let source = (Some(WrapI16(256)), (Some(WrapI16(1)), (Some(WrapI16(2)),)));
        let target: Result<(Option<u8>, (Option<u8>, (Option<u8>,))), WrapErr> =
            NestedTupleOptionTryFrom::nested_tuple_option_try_from(source);
        assert!(target.is_err());
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_tuple_option_from_with_none() {
        let source = (None::<u8>, (Some(2u8), (None::<u8>,)));
        let target: (Option<u16>, (Option<u16>, (Option<u16>,))) =
            NestedTupleOptionFrom::nested_tuple_option_from(source);
        assert_eq!(target, (None, (Some(2u16), (None,))));
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_tuple_option_try_from_with_none() {
        let source = (None::<u8>, (Some(2u8), (None::<u8>,)));
        let target: Result<(Option<u16>, (Option<u16>, (Option<u16>,))), MyError> =
            NestedTupleOptionTryFrom::nested_tuple_option_try_from(source);
        assert_eq!(target, Ok((None, (Some(2u16), (None,)))));
    }
}

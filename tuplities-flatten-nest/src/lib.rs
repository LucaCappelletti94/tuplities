#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `FlattenNestedTuple` and `NestTuple` traits.

use tuplities_push_front::TuplePushFront;
use typenum;

/// A trait for flattening nested tuples into flat tuples.
///
/// This trait takes a nested tuple structure like `(A, (B, (C,)))` and converts it
/// to a flat tuple like `(A, B, C)`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait FlattenNestedTuple {
    /// The flattened tuple type.
    type Flattened;

    /// Flattens the nested tuple into a flat tuple.
    fn flatten(self) -> Self::Flattened;
}

impl FlattenNestedTuple for () {
    type Flattened = ();

    #[inline]
    fn flatten(self) -> Self::Flattened {
        ()
    }
}

impl<N1> FlattenNestedTuple for (N1,) {
    type Flattened = (N1,);

    #[inline]
    fn flatten(self) -> Self::Flattened {
        let (n1,) = self;
        (n1,)
    }
}

impl<Head, Tail> FlattenNestedTuple for (Head, Tail)
where
    Tail: FlattenNestedTuple<Flattened: TuplePushFront<Head>>,
{
    type Flattened = <Tail::Flattened as TuplePushFront<Head>>::Output;

    #[inline]
    fn flatten(self) -> Self::Flattened {
        let (head, tail) = self;
        tail.flatten().push_front(head)
    }
}

#[tuplities_derive::impl_nest_tuple]
/// A trait for nesting flat tuples into nested tuples.
///
/// This trait takes a flat tuple like `(A, B, C)` and converts it
/// to a nested tuple structure like `(A, (B, (C,)))`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestTuple {
    /// The nested tuple type.
    type Nested;

    /// Nests the flat tuple into a nested tuple.
    fn nest(self) -> Self::Nested;
}

/// A trait for indexing into nested tuples at compile-time known positions.
///
/// This trait allows accessing elements at specific flat indices `Idx`
/// from nested tuples, where `Idx` is a compile-time constant from `typenum`.
/// The index corresponds to the position in the flattened tuple.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::NestedTupleIndex;
/// use typenum::U1;
///
/// let nested = (1, (2, (3,)));
/// let element = NestedTupleIndex::<U1>::nested_index(&nested);
/// assert_eq!(*element, 2);
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIndex<Idx>: FlattenNestedTuple {
    /// The type of the element at flat index `Idx`.
    type Element;

    /// Returns a reference to the element at flat index `Idx`.
    fn nested_index(&self) -> &Self::Element;
}

/// A trait for mutable indexing into nested tuples at compile-time known positions.
///
/// This trait allows mutable access to elements at specific flat indices `Idx`
/// from nested tuples, where `Idx` is a compile-time constant from `typenum`.
/// The index corresponds to the position in the flattened tuple.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::NestedTupleIndexMut;
/// use typenum::U1;
///
/// let mut nested = (1, (2, (3,)));
/// let element = NestedTupleIndexMut::<U1>::nested_index_mut(&mut nested);
/// *element = 20;
/// assert_eq!(nested, (1, (20, (3,))));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIndexMut<Idx>: FlattenNestedTuple {
    /// The type of the element at flat index `Idx`.
    type Element;

    /// Returns a mutable reference to the element at flat index `Idx`.
    fn nested_index_mut(&mut self) -> &mut Self::Element;
}

impl<Head> NestedTupleIndex<typenum::U0> for (Head,) {
    type Element = Head;

    fn nested_index(&self) -> &Self::Element {
        &self.0
    }
}

impl<Head, Tail> NestedTupleIndex<typenum::U0> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
{
    type Element = Head;

    fn nested_index(&self) -> &Self::Element {
        &self.0
    }
}

impl<Head, Tail, U, B> NestedTupleIndex<typenum::UInt<U, B>> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleIndex<typenum::Sub1<typenum::UInt<U, B>>>,
{
    type Element = <Tail as NestedTupleIndex<typenum::Sub1<typenum::UInt<U, B>>>>::Element;

    fn nested_index(&self) -> &Self::Element {
        NestedTupleIndex::<typenum::Sub1<typenum::UInt<U, B>>>::nested_index(&self.1)
    }
}

impl<Head> NestedTupleIndexMut<typenum::U0> for (Head,) {
    type Element = Head;

    fn nested_index_mut(&mut self) -> &mut Self::Element {
        &mut self.0
    }
}

impl<Head, Tail> NestedTupleIndexMut<typenum::U0> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
{
    type Element = Head;

    fn nested_index_mut(&mut self) -> &mut Self::Element {
        &mut self.0
    }
}

impl<Head, Tail, U, B> NestedTupleIndexMut<typenum::UInt<U, B>> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleIndexMut<typenum::Sub1<typenum::UInt<U, B>>>,
{
    type Element = <Tail as NestedTupleIndexMut<typenum::Sub1<typenum::UInt<U, B>>>>::Element;

    fn nested_index_mut(&mut self) -> &mut Self::Element {
        NestedTupleIndexMut::<typenum::Sub1<typenum::UInt<U, B>>>::nested_index_mut(&mut self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nest_tuple_3() {
        let flat = (1, 2, 3);
        let nested = flat.nest();
        assert_eq!(nested, (1, (2, (3,))));
    }

    #[test]
    fn test_flatten_nested_tuple_3() {
        let nested = (1, (2, (3,)));
        let flat = nested.flatten();
        assert_eq!(flat, (1, 2, 3));
    }

    #[test]
    fn test_round_trip_4() {
        let original = (1, 2, 3, 4);
        let nested = original.nest();
        let flattened = nested.flatten();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_round_trip_2() {
        let original = (42, "hello");
        let nested = original.nest();
        let flattened = nested.flatten();
        assert_eq!(original, flattened);
    }
}

#[cfg(test)]
mod nested_index_tests {
    use super::*;

    #[test]
    fn test_nested_index_3_u0() {
        let nested = (1, (2, (3,)));
        let element = NestedTupleIndex::<typenum::U0>::nested_index(&nested);
        assert_eq!(*element, 1);
    }

    #[test]
    fn test_nested_index_3_u1() {
        let nested = (1, (2, (3,)));
        let element = NestedTupleIndex::<typenum::U1>::nested_index(&nested);
        assert_eq!(*element, 2);
    }

    #[test]
    fn test_nested_index_3_u2() {
        let nested = (1, (2, (3,)));
        let element = NestedTupleIndex::<typenum::U2>::nested_index(&nested);
        assert_eq!(*element, 3);
    }

    #[test]
    fn test_nested_index_mut_3_u0() {
        let mut nested = (1, (2, (3,)));
        let element = NestedTupleIndexMut::<typenum::U0>::nested_index_mut(&mut nested);
        *element = 10;
        assert_eq!(nested, (10, (2, (3,))));
    }

    #[test]
    fn test_nested_index_mut_3_u1() {
        let mut nested = (1, (2, (3,)));
        let element = NestedTupleIndexMut::<typenum::U1>::nested_index_mut(&mut nested);
        *element = 20;
        assert_eq!(nested, (1, (20, (3,))));
    }

    #[test]
    fn test_nested_index_mut_3_u2() {
        let mut nested = (1, (2, (3,)));
        let element = NestedTupleIndexMut::<typenum::U2>::nested_index_mut(&mut nested);
        *element = 30;
        assert_eq!(nested, (1, (2, (30,))));
    }
}

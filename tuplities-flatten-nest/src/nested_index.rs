//! Module providing indexing traits for nested tuples.

use crate::flatten_nested::FlattenNestedTuple;

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
pub trait NestedTupleIndexMut<Idx>: NestedTupleIndex<Idx> {
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
    fn nested_index_mut(&mut self) -> &mut Self::Element {
        &mut self.0
    }
}

impl<Head, Tail> NestedTupleIndexMut<typenum::U0> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
{
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
    fn nested_index_mut(&mut self) -> &mut Self::Element {
        NestedTupleIndexMut::<typenum::Sub1<typenum::UInt<U, B>>>::nested_index_mut(&mut self.1)
    }
}

#[cfg(test)]
mod tests {
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

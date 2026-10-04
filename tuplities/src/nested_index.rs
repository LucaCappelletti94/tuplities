//! Module providing indexing traits for nested tuples.

use crate::NestedTuple;

/// A trait for indexing into nested tuples at compile-time known positions.
///
/// The `typenum` index counts heads from zero. A tuple-valued head occupies one position.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIndex<Idx>: NestedTuple {
    /// The element type selected by `Idx`.
    type Element;

    /// Returns a reference to the element at flat index `Idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestedTupleIndex;
    /// use typenum::U1;
    ///
    /// let nested = (1, (2, (3,)));
    /// let element = NestedTupleIndex::<U1>::nested_index(&nested);
    /// assert_eq!(*element, 2);
    /// ```
    ///
    /// Indices outside the list have no implementation.
    ///
    /// ```compile_fail
    /// use tuplities::NestedTupleIndex;
    /// use typenum::U2;
    ///
    /// let values = (1_u8, (2_u16, ()));
    /// let _ = NestedTupleIndex::<U2>::nested_index(&values);
    /// ```
    fn nested_index(&self) -> &Self::Element;
}

/// A trait for mutable indexing into nested tuples at compile-time known positions.
///
/// The `typenum` index counts heads from zero. A tuple-valued head occupies one position.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIndexMut<Idx>: NestedTupleIndex<Idx> {
    /// Returns a mutable reference to the element at flat index `Idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestedTupleIndexMut;
    /// use typenum::U1;
    ///
    /// let mut nested = (1, (2, (3,)));
    /// let element = NestedTupleIndexMut::<U1>::nested_index_mut(&mut nested);
    /// *element = 20;
    /// assert_eq!(nested, (1, (20, (3,))));
    /// ```
    ///
    /// Shared tails cannot provide mutable access.
    ///
    /// ```compile_fail
    /// use tuplities::NestedTupleIndexMut;
    /// use typenum::U1;
    ///
    /// let tail = (1_u8,);
    /// let mut values = (0_u8, &tail);
    /// let _ = NestedTupleIndexMut::<U1>::nested_index_mut(&mut values);
    /// ```
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
    Tail: NestedTuple,
{
    type Element = Head;

    fn nested_index(&self) -> &Self::Element {
        &self.0
    }
}

impl<Head, Tail, U, B> NestedTupleIndex<typenum::UInt<U, B>> for (Head, Tail)
where
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
    Tail: NestedTuple,
{
    fn nested_index_mut(&mut self) -> &mut Self::Element {
        &mut self.0
    }
}

impl<Head, Tail, U, B> NestedTupleIndexMut<typenum::UInt<U, B>> for (Head, Tail)
where
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleIndexMut<typenum::Sub1<typenum::UInt<U, B>>>,
{
    fn nested_index_mut(&mut self) -> &mut Self::Element {
        NestedTupleIndexMut::<typenum::Sub1<typenum::UInt<U, B>>>::nested_index_mut(&mut self.1)
    }
}

impl<Idx, T: NestedTupleIndex<Idx> + ?Sized> NestedTupleIndex<Idx> for &T {
    type Element = T::Element;

    fn nested_index(&self) -> &Self::Element {
        T::nested_index(*self)
    }
}

impl<Idx, T: NestedTupleIndex<Idx> + ?Sized> NestedTupleIndex<Idx> for &mut T {
    type Element = T::Element;

    fn nested_index(&self) -> &Self::Element {
        T::nested_index(*self)
    }
}

impl<Idx, T: NestedTupleIndexMut<Idx> + ?Sized> NestedTupleIndexMut<Idx> for &mut T {
    fn nested_index_mut(&mut self) -> &mut Self::Element {
        T::nested_index_mut(*self)
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

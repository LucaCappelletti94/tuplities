//! Module providing the `NestTuple` trait for nesting flat tuples.

use crate::flatten_nested::FlattenNestedTuple;

/// A trait for nesting flat tuples into nested tuples.
///
/// This trait takes a flat tuple like `(A, B, C)` and converts it
/// to a nested tuple structure like `(A, (B, (C,)))`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestTuple {
    /// The nested tuple type.
    type Nested: FlattenNestedTuple;

    /// Nests the flat tuple into a nested tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestTuple;
    ///
    /// assert_eq!((7_u8,).nest(), (7_u8,));
    ///
    /// # #[cfg(feature = "flatten-nest")] {
    /// let flat = (1, 2, 3);
    /// assert_eq!(flat.nest(), (1, (2, (3,))));
    /// # }
    /// ```
    fn nest(self) -> Self::Nested;
}

/// A trait for nesting flat tuples into nested tuples of references.
///
/// This trait takes a flat tuple like `(A, B, C)` and converts it
/// to a nested tuple of references like `(&A, (&B, (&C,)))`.
pub trait NestTupleRef {
    /// The nested tuple type containing references.
    type NestedRef<'a>: FlattenNestedTuple
    where
        Self: 'a;

    /// Nests the flat tuple into a nested tuple of references.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestTupleRef;
    ///
    /// let flat = (7,);
    /// assert_eq!(flat.nest_ref(), (&7,));
    ///
    /// # #[cfg(feature = "flatten-nest")] {
    /// let flat = (1, 2, 3);
    /// assert_eq!(flat.nest_ref(), (&1, (&2, (&3,))));
    /// # }
    /// ```
    fn nest_ref(&self) -> Self::NestedRef<'_>;
}

/// A trait for nesting flat tuples into nested tuples of mutable references.
///
/// This trait takes a flat tuple like `(A, B, C)` and converts it
/// to a nested tuple of mutable references like `(&mut A, (&mut B, (&mut C,)))`.
pub trait NestTupleMut {
    /// The nested tuple type containing mutable references.
    type NestedMut<'a>: FlattenNestedTuple
    where
        Self: 'a;

    /// Nests the flat tuple into a nested tuple of mutable references.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestTupleMut;
    ///
    /// let mut single = (7,);
    /// *single.nest_mut().0 = 9;
    /// assert_eq!(single, (9,));
    ///
    /// # #[cfg(feature = "flatten-nest")] {
    /// let mut flat = (1, 2);
    /// let nested = flat.nest_mut();
    /// *nested.1.0 = 20;
    /// assert_eq!(flat, (1, 20));
    /// # }
    /// ```
    fn nest_mut(&mut self) -> Self::NestedMut<'_>;
}

impl<'a, T> NestTuple for &'a T
where
    T: NestTupleRef,
{
    type Nested = T::NestedRef<'a>;

    #[inline]
    fn nest(self) -> Self::Nested {
        (*self).nest_ref()
    }
}

impl<'a, T> NestTuple for &'a mut T
where
    T: NestTupleMut,
{
    type Nested = T::NestedMut<'a>;

    #[inline]
    fn nest(self) -> Self::Nested {
        (*self).nest_mut()
    }
}

impl NestTuple for () {
    type Nested = ();

    #[inline]
    fn nest(self) -> Self::Nested {}
}

impl NestTupleRef for () {
    type NestedRef<'a>
        = ()
    where
        Self: 'a;

    #[inline]
    fn nest_ref(&self) -> Self::NestedRef<'_> {}
}

impl NestTupleMut for () {
    type NestedMut<'a>
        = ()
    where
        Self: 'a;

    #[inline]
    fn nest_mut(&mut self) -> Self::NestedMut<'_> {}
}

impl<N1> NestTuple for (N1,) {
    type Nested = (N1,);

    #[inline]
    fn nest(self) -> Self::Nested {
        self
    }
}

impl<N1> NestTupleRef for (N1,) {
    type NestedRef<'a>
        = (&'a N1,)
    where
        Self: 'a;

    #[inline]
    fn nest_ref(&self) -> Self::NestedRef<'_> {
        (&self.0,)
    }
}

impl<N1> NestTupleMut for (N1,) {
    type NestedMut<'a>
        = (&'a mut N1,)
    where
        Self: 'a;

    #[inline]
    fn nest_mut(&mut self) -> Self::NestedMut<'_> {
        (&mut self.0,)
    }
}

#[cfg(all(test, feature = "flatten-nest"))]
mod tests {
    use super::*;

    #[test]
    fn test_nest_tuple_3() {
        let flat = (1, 2, 3);
        let nested = flat.nest();
        assert_eq!(nested, (1, (2, (3,))));
    }

    #[test]
    fn test_round_trip_4() {
        use crate::flatten_nested::FlattenNestedTuple;
        let original = (1, 2, 3, 4);
        let nested = original.nest();
        let flattened = nested.flatten();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_round_trip_2() {
        use crate::flatten_nested::FlattenNestedTuple;
        let original = (42, "hello");
        let nested = original.nest();
        let flattened = nested.flatten();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_nest_single() {
        let flat = (99,);
        let nested = flat.nest();
        assert_eq!(nested, (99,));
    }

    #[test]
    fn test_nest_ref() {
        let flat = (1, 2, 3);
        let nested_ref = flat.nest_ref();
        assert_eq!(nested_ref, (&1, (&2, (&3,))));
    }

    #[test]
    fn test_nest_mut() {
        let mut flat = (1, 2, 3);
        let nested_mut = flat.nest_mut();
        assert_eq!(nested_mut, (&mut 1, (&mut 2, (&mut 3,))));
    }
}

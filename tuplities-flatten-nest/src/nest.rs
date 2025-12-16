//! Module providing the `NestTuple` trait for nesting flat tuples.

use crate::FlattenNestedTuple;

#[tuplities_derive::impl_nest_tuple]
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
    fn nest(self) -> Self::Nested;
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
    fn nest_mut(&mut self) -> Self::NestedMut<'_>;
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
    fn test_nest_empty() {
        let flat: () = ();
        let _nested: () = flat.nest();
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

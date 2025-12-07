#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `FlattenNestedTuple` and `NestTuple` traits.

use tuplities_mut::TupleMut;
use tuplities_ref::TupleRef;

#[tuplities_derive::impl_flatten_nested_tuple]
#[tuplities_derive::impl_nest_tuple]
/// A trait for flattening nested tuples into flat tuples.
///
/// This trait takes a nested tuple structure like `(A, (B, (C,)))` and converts it
/// to a flat tuple like `(A, B, C)`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait FlattenNestedTuple {
    /// The flattened tuple type.
    type Flattened: NestTuple<Nested = Self> + TupleRef + TupleMut;

    /// Flattens the nested tuple into a flat tuple.
    fn flatten(self) -> Self::Flattened;

    /// Flattens the nested tuple into a flat tuple of references.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_flatten_nest::FlattenNestedTuple;
    ///
    /// let nested = (1, (2, (3,)));
    /// let refs = nested.flatten_ref();
    /// assert_eq!(refs, (&1, &2, &3));
    /// ```
    fn flatten_ref(&self) -> <Self::Flattened as TupleRef>::Ref<'_>;

    /// Flattens the nested tuple into a flat tuple of mutable references.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_flatten_nest::FlattenNestedTuple;
    ///
    /// let mut nested = (1, (2, (3,)));
    /// let mut_refs = nested.flatten_mut();
    /// *mut_refs.0 = 10;
    /// *mut_refs.1 = 20;
    /// *mut_refs.2 = 30;
    /// assert_eq!(nested, (10, (20, (30,))));
    /// ```
    fn flatten_mut(&mut self) -> <Self::Flattened as TupleMut>::Mut<'_>;
}

/// A trait for nesting flat tuples into nested tuples.
///
/// This trait takes a flat tuple like `(A, B, C)` and converts it
/// to a nested tuple structure like `(A, (B, (C,)))`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestTuple {
    /// The nested tuple type.
    type Nested: FlattenNestedTuple<Flattened = Self>;

    /// Nests the flat tuple into a nested tuple.
    fn nest(self) -> Self::Nested;
}

#[tuplities_derive::impl_nested_tuple_index]
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

#[tuplities_derive::impl_nested_tuple_index_mut]
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

    #[test]
    fn test_flatten_ref_3() {
        let nested = (1, (2, (3,)));
        let refs = nested.flatten_ref();
        assert_eq!(refs, (&1, &2, &3));
    }

    #[test]
    fn test_flatten_mut_3() {
        let mut nested = (1, (2, (3,)));
        let mut_refs = nested.flatten_mut();
        *mut_refs.0 = 10;
        *mut_refs.1 = 20;
        *mut_refs.2 = 30;
        assert_eq!(nested, (10, (20, (30,))));
    }

    #[test]
    fn test_flatten_ref_2() {
        let nested = (42, ("hello",));
        let refs = nested.flatten_ref();
        assert_eq!(refs, (&42, &"hello"));
    }

    #[test]
    fn test_flatten_mut_2() {
        let mut nested = (42, ([1, 2, 3],));
        let mut_refs = nested.flatten_mut();
        *mut_refs.0 = 24;
        mut_refs.1[0] = 10;
        mut_refs.1[1] = 20;
        mut_refs.1[2] = 30;
        assert_eq!(nested, (24, ([10, 20, 30],)));
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

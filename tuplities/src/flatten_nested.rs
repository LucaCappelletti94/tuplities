//! Module providing the `FlattenNestedTuple` trait for flattening nested tuples.

use crate::nest::NestTuple;

/// A trait for flattening nested tuples into flat tuples.
///
/// This trait takes a nested tuple structure like `(A, (B, (C,)))` and converts it
/// to a flat tuple like `(A, B, C)`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait FlattenNestedTuple {
    /// The flattened tuple type.
    type Flattened: NestTuple<Nested = Self>;

    /// Flattens the nested tuple into a flat tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::FlattenNestedTuple;
    ///
    /// assert_eq!((7_u8,).flatten(), (7_u8,));
    ///
    /// # #[cfg(feature = "flatten-nest")] {
    /// let nested = (1, (2, (3,)));
    /// assert_eq!(nested.flatten(), (1, 2, 3));
    /// # }
    /// ```
    fn flatten(self) -> Self::Flattened;
}

impl FlattenNestedTuple for () {
    type Flattened = ();

    #[inline]
    fn flatten(self) -> Self::Flattened {}
}

impl<N1> FlattenNestedTuple for (N1,) {
    type Flattened = (N1,);

    #[inline]
    fn flatten(self) -> Self::Flattened {
        let (n1,) = self;
        (n1,)
    }
}

#[cfg(all(test, feature = "flatten-nest"))]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_nested_tuple_3() {
        let nested = (1, (2, (3,)));
        let flat = nested.flatten();
        assert_eq!(flat, (1, 2, 3));
    }
}

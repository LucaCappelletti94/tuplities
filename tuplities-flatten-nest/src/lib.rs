#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `FlattenNestedTuple` and `NestTuple` traits.

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
    type Flattened: NestTuple;

    /// Flattens the nested tuple into a flat tuple.
    fn flatten(self) -> Self::Flattened;
}

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

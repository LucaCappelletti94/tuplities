//! Module providing the `FlattenNestedTuple` trait for flattening nested tuples.

use tuplities_push_front::TuplePushFront;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_nested_tuple_3() {
        let nested = (1, (2, (3,)));
        let flat = nested.flatten();
        assert_eq!(flat, (1, 2, 3));
    }
}

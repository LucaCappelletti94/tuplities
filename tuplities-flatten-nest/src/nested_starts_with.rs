//! Module providing the `NestedTupleStartsWith` trait.

/// A trait for checking if a nested tuple starts with the same types as another nested tuple.
///
/// This trait is implemented for nested tuples where the beginning elements match
/// the types of another nested tuple. For example, `(A, (B, (C,)))` implements
/// `NestedTupleStartsWith<(A,)>` and `NestedTupleStartsWith<(A, (B,))>`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleStartsWith<Other> {}

// Base case: every tuple starts with the empty tuple
impl<T> NestedTupleStartsWith<()> for T {}

// Single element tuple starts with itself
impl<Head> NestedTupleStartsWith<(Head,)> for (Head,) {}

// Nested tuple starts with a single element if the head matches
impl<Head, Tail> NestedTupleStartsWith<(Head,)> for (Head, Tail) where
    Tail: NestedTupleStartsWith<()>
{
}

// Nested tuple starts with another nested tuple if heads match and tail starts with other tail
impl<Head, Tail, OtherTail> NestedTupleStartsWith<(Head, OtherTail)> for (Head, Tail) where
    Tail: NestedTupleStartsWith<OtherTail>
{
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to test that a type implements the trait
    fn _assert_starts_with<T, U>()
    where
        T: NestedTupleStartsWith<U>,
    {
    }

    #[test]
    fn test_empty_starts_with_empty() {
        _assert_starts_with::<(), ()>();
    }

    #[test]
    fn test_single_starts_with_empty() {
        _assert_starts_with::<(i32,), ()>();
    }

    #[test]
    fn test_single_starts_with_itself() {
        _assert_starts_with::<(i32,), (i32,)>();
    }

    #[test]
    fn test_nested_starts_with_empty() {
        _assert_starts_with::<(i32, (i32, (i32,))), ()>();
    }

    #[test]
    fn test_nested_starts_with_single() {
        _assert_starts_with::<(i32, (i32, (i32,))), (i32,)>();
    }

    #[test]
    fn test_nested_starts_with_prefix() {
        _assert_starts_with::<(i32, (i32, (i32,))), (i32, (i32,))>();
    }

    #[test]
    fn test_nested_starts_with_itself() {
        _assert_starts_with::<(i32, (i32, (i32,))), (i32, (i32, (i32,)))>();
    }

    #[test]
    fn test_different_types() {
        _assert_starts_with::<(i32, (u64, (bool,))), (i32,)>();
        _assert_starts_with::<(i32, (u64, (bool,))), (i32, (u64,))>();
    }
}

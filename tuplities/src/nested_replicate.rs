//! Module providing replication trait for nested tuples.

/// A trait for replicating a value into a nested tuple.
///
/// The empty and single-element forms take no `Clone` bound; longer forms clone for all but the
/// last position and move the value there.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleReplicate<T> {
    /// Creates a nested tuple where all elements are the provided value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities::NestedTupleReplicate;
    ///
    /// let nested: (i32, (i32, (i32,))) = NestedTupleReplicate::nested_tuple_replicate(42);
    /// assert_eq!(nested, (42, (42, (42,))));
    ///
    /// let nested_single: (i32,) = NestedTupleReplicate::nested_tuple_replicate(42);
    /// assert_eq!(nested_single, (42,));
    ///
    /// let _nested_empty: () = NestedTupleReplicate::nested_tuple_replicate(42);
    /// ```
    fn nested_tuple_replicate(value: T) -> Self;
}

impl<T> NestedTupleReplicate<T> for () {
    #[inline]
    fn nested_tuple_replicate(_value: T) -> Self {}
}

impl<T> NestedTupleReplicate<T> for (T,) {
    #[inline]
    fn nested_tuple_replicate(value: T) -> Self {
        (value,)
    }
}

impl<T, Tail> NestedTupleReplicate<T> for (T, Tail)
where
    Tail: NestedTupleReplicate<T>,
    T: Clone,
{
    #[inline]
    fn nested_tuple_replicate(value: T) -> Self {
        (value.clone(), Tail::nested_tuple_replicate(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replicate_empty() {
        let nested: () = NestedTupleReplicate::nested_tuple_replicate(42);
        assert_eq!(nested, ());
    }

    #[test]
    fn test_replicate_single() {
        let nested: (i32,) = NestedTupleReplicate::nested_tuple_replicate(42);
        assert_eq!(nested, (42,));
    }

    #[test]
    fn test_replicate_three() {
        let nested: (i32, (i32, (i32,))) = NestedTupleReplicate::nested_tuple_replicate(42);
        assert_eq!(nested, (42, (42, (42,))));
    }

    #[test]
    fn test_replicate_with_refs() {
        let value = &42;
        let nested: (&i32, (&i32, (&i32,))) = NestedTupleReplicate::nested_tuple_replicate(value);
        assert_eq!(nested, (&42, (&42, (&42,))));
    }
}

//! Module providing the `NestedTupleChain` trait for chaining nested tuples.

/// A trait for chaining two nested tuples.
///
/// This trait allows joining two nested tuples into a single nested tuple.
/// It effectively appends the `other` nested tuple to `self`.
///
/// # Examples
///
/// ```rust
/// use tuplities_flatten_nest::NestedTupleChain;
///
/// // Chaining (1, (2,)) and (3, (4,)) results in (1, (2, (3, (4,))))
/// let t1 = (1, (2,));
/// let t2 = (3, (4,));
/// let chained = t1.chain(t2);
/// assert_eq!(chained, (1, (2, (3, (4,)))));
///
/// // Chaining (1, (2, ())) and (3, (4, ())) results in (1, (2, (3, (4, ()))))
/// let t3 = (1, (2, ()));
/// let t4 = (3, (4, ()));
/// let chained2 = t3.chain(t4);
/// assert_eq!(chained2, (1, (2, (3, (4, ())))));
/// ```
pub trait NestedTupleChain<Other> {
    /// The resulting chained nested tuple type.
    type Chained;

    /// Chains `other` to the end of `self`.
    fn chain(self, other: Other) -> Self::Chained;
}

impl<Other> NestedTupleChain<Other> for () {
    type Chained = Other;

    #[inline]
    fn chain(self, other: Other) -> Self::Chained {
        other
    }
}

impl<T, Other> NestedTupleChain<Other> for (T,) {
    type Chained = (T, Other);

    #[inline]
    fn chain(self, other: Other) -> Self::Chained {
        let (t,) = self;
        (t, other)
    }
}

impl<Head, Tail, Other> NestedTupleChain<Other> for (Head, Tail)
where
    Tail: NestedTupleChain<Other>,
{
    type Chained = (Head, Tail::Chained);

    #[inline]
    fn chain(self, other: Other) -> Self::Chained {
        let (head, tail) = self;
        (head, tail.chain(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_unit_base() {
        // (1, (2, ())) chain (3, (4, ()))
        let t1 = (1, (2, ()));
        let t2 = (3, (4, ()));
        let chained = t1.chain(t2);
        assert_eq!(chained, (1, (2, (3, (4, ())))));
    }

    #[test]
    fn test_chain_tuple1_base() {
        // (1, (2,)) chain (3, (4,))
        let t1 = (1, (2,));
        let t2 = (3, (4,));
        let chained = t1.chain(t2);
        assert_eq!(chained, (1, (2, (3, (4,)))));
    }

    #[test]
    fn test_chain_mixed_bases() {
        // (1, (2,)) chain (3, (4, ()))
        let t1 = (1, (2,));
        let t2 = (3, (4, ()));
        let chained = t1.chain(t2);
        assert_eq!(chained, (1, (2, (3, (4, ())))));

        // (1, (2, ())) chain (3, (4,))
        let t3 = (1, (2, ()));
        let t4 = (3, (4,));
        let chained2 = t3.chain(t4);
        assert_eq!(chained2, (1, (2, (3, (4,)))));
    }

    #[test]
    fn test_chain_empty_lhs() {
        let t1 = ();
        let t2 = (1, (2,));
        let chained = t1.chain(t2);
        assert_eq!(chained, (1, (2,)));
    }

    #[test]
    fn test_chain_empty_rhs() {
        let t1 = (1, (2,));
        let t2 = ();
        let chained = t1.chain(t2);
        assert_eq!(chained, (1, (2, ())));
    }

    #[test]
    fn test_chain_single_lhs() {
        let t1 = (1,);
        let t2 = (2, (3,));
        let chained = t1.chain(t2);
        assert_eq!(chained, (1, (2, (3,))));
    }
}

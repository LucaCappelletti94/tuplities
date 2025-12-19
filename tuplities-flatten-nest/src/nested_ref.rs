//! Module providing reference accessors for nested tuples (nested tuples -> nested tuples of references).

use crate::IntoNestedTupleOption;

/// Trait to obtain a nested tuple of references from a nested tuple of values.
pub trait NestedTupleRef {
    /// The nested tuple type containing references to each leaf value.
    type Ref<'a>: Copy + IntoNestedTupleOption
    where
        Self: 'a;

    /// Returns a nested tuple containing references to each leaf value.
    fn nested_tuple_ref(&self) -> Self::Ref<'_>;
}

/// Trait to obtain a nested tuple of mutable references from a nested tuple of values.
pub trait NestedTupleMut {
    /// The nested tuple type containing mutable references to each leaf value.
    type Mut<'a>
    where
        Self: 'a;

    /// Returns a nested tuple containing mutable references to each leaf value.
    fn nested_tuple_mut(&mut self) -> Self::Mut<'_>;
}

// Implementations for `NestedTupleRef` and `NestedTupleMut` follow a simple recursive pattern
// analogous to other nested traits: `()`, single-element tuple `(T,)`, and recursive `(Head, Tail)`.

impl NestedTupleRef for () {
    type Ref<'a> = ();

    #[inline]
    fn nested_tuple_ref(&self) -> Self::Ref<'_> {}
}

impl NestedTupleMut for () {
    type Mut<'a> = ();

    #[inline]
    fn nested_tuple_mut(&mut self) -> Self::Mut<'_> {}
}

impl<T> NestedTupleRef for (T,) {
    type Ref<'a>
        = (&'a T,)
    where
        T: 'a;

    #[inline]
    fn nested_tuple_ref(&self) -> Self::Ref<'_> {
        (&self.0,)
    }
}

impl<T> NestedTupleMut for (T,) {
    type Mut<'a>
        = (&'a mut T,)
    where
        T: 'a;

    #[inline]
    fn nested_tuple_mut(&mut self) -> Self::Mut<'_> {
        (&mut self.0,)
    }
}

impl<Head, Tail> NestedTupleRef for (Head, Tail)
where
    Tail: NestedTupleRef,
{
    type Ref<'a>
        = (&'a Head, <Tail as NestedTupleRef>::Ref<'a>)
    where
        Head: 'a,
        Tail: 'a;

    #[inline]
    fn nested_tuple_ref(&self) -> Self::Ref<'_> {
        (&self.0, <Tail as NestedTupleRef>::nested_tuple_ref(&self.1))
    }
}

impl<Head, Tail> NestedTupleMut for (Head, Tail)
where
    Tail: NestedTupleMut,
{
    type Mut<'a>
        = (&'a mut Head, <Tail as NestedTupleMut>::Mut<'a>)
    where
        Head: 'a,
        Tail: 'a;

    #[inline]
    fn nested_tuple_mut(&mut self) -> Self::Mut<'_> {
        (
            &mut self.0,
            <Tail as NestedTupleMut>::nested_tuple_mut(&mut self.1),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_tuple_ref_empty() {
        let t: () = ();
        let r: () = t.nested_tuple_ref();
        assert_eq!(r, ());
    }

    #[test]
    fn test_nested_tuple_ref_single() {
        let t: (i32,) = (42,);
        let r: (&i32,) = t.nested_tuple_ref();
        assert_eq!(r, (&42,));
    }

    #[test]
    fn test_nested_tuple_ref_nested() {
        let t: (i32, (i32,)) = (1, (2,));
        let r: (&i32, (&i32,)) = t.nested_tuple_ref();
        assert_eq!(r, (&1, (&2,)));
    }

    #[test]
    fn test_nested_tuple_mut_modify() {
        let mut t: (i32, (i32,)) = (1, (2,));
        let r: (&mut i32, (&mut i32,)) = t.nested_tuple_mut();
        *r.0 = 10;
        *r.1.0 = 20;
        assert_eq!(t, (10, (20,)));
    }
}

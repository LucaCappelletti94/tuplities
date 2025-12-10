//! Module providing nested row accessors for nested tuple-of-tuples.

use crate::{NestedTupleIndex, NestedTupleIndexMut, NestedTupleMut, NestedTupleRef};

/// Skeleton trait for nested row retrieval.
pub trait NestedTupleRow<Idx> {
    /// The nested tuple type representing a row across the matrix.
    type RowType: NestedTupleRef;

    /// Return the nested row at the given index.
    fn nested_tuple_row(&self) -> <Self::RowType as NestedTupleRef>::Ref<'_>;
}

impl<Idx> NestedTupleRow<Idx> for () {
    type RowType = ();

    #[inline]
    fn nested_tuple_row(&self) -> <Self::RowType as NestedTupleRef>::Ref<'_> {}
}

impl<T1, Idx> NestedTupleRow<Idx> for (T1,)
where
    T1: NestedTupleIndex<Idx>,
{
    type RowType = (T1::Element,);

    #[inline]
    fn nested_tuple_row(&self) -> <Self::RowType as NestedTupleRef>::Ref<'_> {
        let (t1,) = self;
        (t1.nested_index(),)
    }
}

impl<Idx, Head, Tail> NestedTupleRow<Idx> for (Head, Tail)
where
    Head: NestedTupleIndex<Idx>,
    Tail: NestedTupleRow<Idx>,
    Tail::RowType: NestedTupleRef,
{
    type RowType = (Head::Element, Tail::RowType);

    #[inline]
    fn nested_tuple_row(&self) -> <Self::RowType as NestedTupleRef>::Ref<'_> {
        let (head, tail) = self;
        (head.nested_index(), tail.nested_tuple_row())
    }
}

/// Skeleton trait for mutable nested row retrieval.
pub trait NestedTupleRowMut<Idx>: NestedTupleRow<Idx, RowType: NestedTupleMut> {
    /// Returns a nested tuple containing mutable references to the row elements.
    fn nested_tuple_row_mut(&mut self) -> <Self::RowType as NestedTupleMut>::Mut<'_>;
}

impl<Idx> NestedTupleRowMut<Idx> for ()
where
    Self: NestedTupleRow<Idx, RowType = ()>,
{
    #[inline]
    fn nested_tuple_row_mut(&mut self) -> <Self::RowType as NestedTupleMut>::Mut<'_> {}
}

impl<T1, Idx> NestedTupleRowMut<Idx> for (T1,)
where
    T1: NestedTupleIndexMut<Idx> + NestedTupleMut,
{
    #[inline]
    fn nested_tuple_row_mut(&mut self) -> <Self::RowType as NestedTupleMut>::Mut<'_> {
        let (t1,) = self;
        (t1.nested_index_mut(),)
    }
}

impl<Idx, Head, Tail> NestedTupleRowMut<Idx> for (Head, Tail)
where
    Head: NestedTupleIndexMut<Idx> + NestedTupleMut,
    Tail: NestedTupleRowMut<Idx>,
    Tail::RowType: NestedTupleMut,
{
    #[inline]
    fn nested_tuple_row_mut(&mut self) -> <Self::RowType as NestedTupleMut>::Mut<'_> {
        let (head, tail) = self;
        (head.nested_index_mut(), tail.nested_tuple_row_mut())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::{U0, U1, U2};

    #[test]
    fn test_nested_row_empty() {
        let matrix: () = ();
        let _row: () = NestedTupleRow::<U0>::nested_tuple_row(&matrix);
    }

    #[test]
    fn test_nested_row_single_element() {
        let matrix: ((i32, (i32, (i32,))),) = ((10, (20, (30,))),);
        let row0 = NestedTupleRow::<U0>::nested_tuple_row(&matrix);
        assert_eq!(*row0.0, 10);
        let row1 = NestedTupleRow::<U1>::nested_tuple_row(&matrix);
        assert_eq!(*row1.0, 20);
        let row2 = NestedTupleRow::<U2>::nested_tuple_row(&matrix);
        assert_eq!(*row2.0, 30);
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_row_two_outer_elements() {
        // outer nested pair with two elements: (Head, (Second,))
        let matrix: ((i32, (i32,)), ((i32, (i32,)),)) = ((1, (2,)), ((3, (4,)),));
        let row0 = NestedTupleRow::<U0>::nested_tuple_row(&matrix);
        assert_eq!(*row0.0, 1);
        assert_eq!(*row0.1.0, 3);
        let row1 = NestedTupleRow::<U1>::nested_tuple_row(&matrix);
        assert_eq!(*row1.0, 2);
        assert_eq!(*row1.1.0, 4);
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_row_three_outer_elements() {
        // outer nested pair with three elements: (A, (B, (C,)))
        let matrix: (
            (i32, (i32, (i32,))),
            ((i32, (i32, (i32,))), ((i32, (i32, (i32,))),)),
        ) = ((1, (2, (3,))), ((4, (5, (6,))), ((7, (8, (9,))),)));

        let row0 = NestedTupleRow::<U0>::nested_tuple_row(&matrix);
        assert_eq!(*row0.0, 1);
        assert_eq!(*row0.1.0, 4);
        assert_eq!(*row0.1.1.0, 7);

        let row1 = NestedTupleRow::<U1>::nested_tuple_row(&matrix);
        assert_eq!(*row1.0, 2);
        assert_eq!(*row1.1.0, 5);
        assert_eq!(*row1.1.1.0, 8);

        let row2 = NestedTupleRow::<U2>::nested_tuple_row(&matrix);
        assert_eq!(*row2.0, 3);
        assert_eq!(*row2.1.0, 6);
        assert_eq!(*row2.1.1.0, 9);
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_row_mut_modify_two_elements() {
        let mut matrix: ((i32, (i32,)), ((i32, (i32,)),)) = ((1, (2,)), ((3, (4,)),));
        let row0 = NestedTupleRowMut::<U0>::nested_tuple_row_mut(&mut matrix);
        *row0.0 = 10;
        *row0.1.0 = 30;
        assert_eq!(matrix, ((10, (2,)), ((30, (4,)),)));

        let row1 = NestedTupleRowMut::<U1>::nested_tuple_row_mut(&mut matrix);
        *row1.0 = 20;
        *row1.1.0 = 40;
        assert_eq!(matrix, ((10, (20,)), ((30, (40,)),)));
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_row_mut_modify_three_elements() {
        let mut matrix: (
            (i32, (i32, (i32,))),
            ((i32, (i32, (i32,))), ((i32, (i32, (i32,))),)),
        ) = ((1, (2, (3,))), ((4, (5, (6,))), ((7, (8, (9,))),)));

        let row2 = NestedTupleRowMut::<U2>::nested_tuple_row_mut(&mut matrix);
        *row2.0 = 33;
        *row2.1.0 = 66;
        *row2.1.1.0 = 99;
        assert_eq!(
            matrix,
            ((1, (2, (33,))), ((4, (5, (66,))), ((7, (8, (99,))),)))
        );
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_row_mixed_types_ref() {
        let matrix: ((i32, (&str,)), ((i32, (&str,)),)) = ((1, ("a",)), ((2, ("b",)),));
        let row0 = NestedTupleRow::<U0>::nested_tuple_row(&matrix);
        assert_eq!(*row0.0, 1);
        assert_eq!(*row0.1.0, 2);
        let row1 = NestedTupleRow::<U1>::nested_tuple_row(&matrix);
        assert_eq!(row1.0, &"a");
        assert_eq!(row1.1.0, &"b");
    }
}

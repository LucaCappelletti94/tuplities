#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleRow` and `TupleRowMut` traits.

use tuplities_len::TupleLen;
use tuplities_mut::TupleMut;
use tuplities_ref::TupleRef;

/// A trait for indexing rows in tuples of tuples.
///
/// This trait allows accessing elements at a specific index across all tuples in a tuple of tuples.
/// For a tuple of tuples like `((A, B), (C, D))`, `TupleRow<U0>` would return `(&A, &C)` and `TupleRow<U1>` would return `(&B, &D)`.
///
/// Each inner tuple must implement `TupleIndex<Idx>`, and the returned row tuple implements `TupleRef`.
///
/// # Examples
///
/// ```
/// use tuplities_row::TupleRow;
/// use typenum::U0;
///
/// let matrix = ((1, 2), (3, 4), (5, 6));
/// let first_row = TupleRow::<U0>::tuple_row(&matrix);
/// assert_eq!(first_row, (&1, &3, &5));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_row]
pub trait TupleRow<Idx: typenum::Unsigned>: TupleLen {
    /// The type of the row tuple containing elements at index `Idx`.
    type RowType: TupleRef + TupleLen<Len = <Self as TupleLen>::Len>;

    /// Returns a tuple of references to the elements at index `Idx` in each inner tuple.
    fn tuple_row(&self) -> <Self::RowType as TupleRef>::Ref<'_>;
}

/// A trait for mutable indexing rows in tuples of tuples.
///
/// This trait allows mutable access to elements at a specific index across all tuples in a tuple of tuples.
/// For a tuple of tuples like `((A, B), (C, D))`, `TupleRowMut<U0>` would return `(&mut A, &mut C)` and `TupleRowMut<U1>` would return `(&mut B, &mut D)`.
///
/// Each inner tuple must implement `TupleIndexMut<Idx>`, and the returned row tuple implements `TupleMut`.
///
/// # Examples
///
/// ```
/// use tuplities_row::TupleRowMut;
/// use typenum::U0;
///
/// let mut matrix = ((1, 2), (3, 4), (5, 6));
/// let first_row = TupleRowMut::<U0>::tuple_row_mut(&mut matrix);
/// *first_row.0 = 10;
/// *first_row.1 = 30;
/// *first_row.2 = 50;
/// assert_eq!(matrix, ((10, 2), (30, 4), (50, 6)));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_row_mut]
pub trait TupleRowMut<Idx: typenum::Unsigned>: TupleRow<Idx, RowType: TupleMut> {
    /// Returns a tuple of mutable references to the elements at index `Idx` in each inner tuple.
    fn tuple_row_mut(&mut self) -> <Self::RowType as TupleMut>::Mut<'_>;
}

/// A convenience trait for accessing the first row (index 0) in tuples of tuples.
///
/// This trait is automatically implemented for any tuple of tuples where each inner tuple implements `TupleRefFront`.
///
/// # Examples
///
/// ```
/// use tuplities_row::FirstTupleRow;
///
/// let matrix = ((1, 2), (3, 4), (5, 6));
/// let first_row = matrix.first_tuple_row();
/// assert_eq!(first_row, (&1, &3, &5));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_first_tuple_row]
pub trait FirstTupleRow: TupleLen {
    /// The type of the row tuple containing references to the first elements.
    type RowType: TupleRef + TupleLen<Len = <Self as TupleLen>::Len>;

    /// Returns a tuple of references to the first element in each inner tuple.
    fn first_tuple_row(&self) -> <Self::RowType as TupleRef>::Ref<'_>;
}

/// A convenience trait for accessing the last row in tuples of tuples.
///
/// This trait is automatically implemented for tuples of tuples where the implementation
/// depends on the length of the inner tuples. It provides access to the last element
/// of each inner tuple.
///
/// # Examples
///
/// ```
/// use tuplities_row::LastTupleRow;
///
/// let matrix = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
/// let last_row = matrix.last_tuple_row();
/// assert_eq!(last_row, (&3, &6, &9));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_last_tuple_row]
pub trait LastTupleRow: TupleLen {
    /// The type of the row tuple containing references to the last elements.
    type RowType: TupleRef + TupleLen<Len = <Self as TupleLen>::Len>;

    /// Returns a tuple of references to the last element in each inner tuple.
    fn last_tuple_row(&self) -> <Self::RowType as TupleRef>::Ref<'_>;
}

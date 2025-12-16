//! Module providing matrix nesting and flattening traits for tuples of tuples.

use crate::{flatten_nested::FlattenNestedTuple, nest::NestTuple};

/// A trait for nesting tuples of flat tuples into nested tuples of nested tuples.
///
/// This trait takes a flat tuple where each element is also a flat tuple (like `((A, B), (C, D))`)
/// and converts it into a nested tuple where each element is also nested
/// (like `((A, (B,)), ((C, (D,)),))`). Both the outer and inner tuples are nested.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::NestTupleMatrix;
///
/// let flat_matrix = ((1, 2), (3, 4));
/// let nested_matrix = flat_matrix.nest_matrix();
/// assert_eq!(nested_matrix, ((1, (2,)), ((3, (4,)),)));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestTupleMatrix {
    /// The nested matrix type.
    type NestedMatrix;

    /// Converts a flat tuple of flat tuples into a nested tuple of nested tuples.
    fn nest_matrix(self) -> Self::NestedMatrix;
}

/// A trait for flattening nested tuples of nested tuples into flat tuples of flat tuples.
///
/// This trait takes a nested tuple where each element is also a nested tuple
/// (like `((A, (B,)), ((C, (D,)),))`) and converts it into a flat tuple of flat tuples
/// (like `((A, B), (C, D))`). Both the outer and inner tuples are flattened.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::FlattenNestedTupleMatrix;
///
/// let nested_matrix = ((1, (2,)), ((3, (4,)),));
/// let flat_matrix = nested_matrix.flatten_matrix();
/// assert_eq!(flat_matrix, ((1, 2), (3, 4)));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait FlattenNestedTupleMatrix {
    /// The flattened matrix type.
    type FlattenedMatrix;

    /// Converts a nested tuple of nested tuples into a flat tuple of flat tuples.
    fn flatten_matrix(self) -> Self::FlattenedMatrix;
}

// For flat tuples that can be nested, first nest the outer tuple structure,
// then recursively nest each element in the resulting nested structure.
// This requires a helper trait to handle the recursion over nested structures.

/// Helper trait to recursively nest elements in an already-nested outer structure
pub trait NestMatrixElements {
    /// The type after nesting all elements
    type Output: FlattenMatrixElements;

    /// Nest all elements in the structure
    fn nest_elements(self) -> Self::Output;
}

impl NestMatrixElements for () {
    type Output = ();

    #[inline]
    fn nest_elements(self) -> Self::Output {}
}

impl<T> NestMatrixElements for (T,)
where
    T: NestTuple,
{
    type Output = (T::Nested,);

    #[inline]
    fn nest_elements(self) -> Self::Output {
        let (t,) = self;
        (t.nest(),)
    }
}

impl<Head, Tail> NestMatrixElements for (Head, Tail)
where
    Head: NestTuple,
    Tail: NestMatrixElements,
    Head::Nested: FlattenNestedTuple<Flattened = Head>,
    Tail::Output: FlattenMatrixElements<Output = Tail>,
{
    type Output = (Head::Nested, Tail::Output);

    #[inline]
    fn nest_elements(self) -> Self::Output {
        let (head, tail) = self;
        (head.nest(), tail.nest_elements())
    }
}

// For flat tuples, nest the outer structure then nest each element
impl<T> NestTupleMatrix for T
where
    T: NestTuple,
    T::Nested: NestMatrixElements,
    <T::Nested as NestMatrixElements>::Output: FlattenNestedTuple,
{
    type NestedMatrix = <T::Nested as NestMatrixElements>::Output;

    #[inline]
    fn nest_matrix(self) -> Self::NestedMatrix {
        self.nest().nest_elements()
    }
}

/// Helper trait to recursively flatten elements in a nested matrix structure
pub trait FlattenMatrixElements {
    /// The type after flattening all nested elements
    type Output;

    /// Flatten all elements in the structure
    fn flatten_elements(self) -> Self::Output;
}

impl FlattenMatrixElements for () {
    type Output = ();

    #[inline]
    fn flatten_elements(self) -> Self::Output {}
}

impl<T> FlattenMatrixElements for (T,)
where
    T: FlattenNestedTuple,
{
    type Output = (T::Flattened,);

    #[inline]
    fn flatten_elements(self) -> Self::Output {
        let (t,) = self;
        (t.flatten(),)
    }
}

impl<Head, Tail> FlattenMatrixElements for (Head, Tail)
where
    Head: FlattenNestedTuple,
    Tail: FlattenMatrixElements,
    Head::Flattened: NestTuple<Nested = Head>,
    Tail::Output: NestMatrixElements<Output = Tail>,
{
    type Output = (Head::Flattened, Tail::Output);

    #[inline]
    fn flatten_elements(self) -> Self::Output {
        let (head, tail) = self;
        (head.flatten(), tail.flatten_elements())
    }
}

// For nested tuples, flatten each element then flatten the outer structure
impl<T> FlattenNestedTupleMatrix for T
where
    T: FlattenMatrixElements + FlattenNestedTuple,
    T::Output: FlattenNestedTuple,
    <T::Output as FlattenNestedTuple>::Flattened: NestTuple,
{
    type FlattenedMatrix = <T::Output as FlattenNestedTuple>::Flattened;

    #[inline]
    fn flatten_matrix(self) -> Self::FlattenedMatrix {
        self.flatten_elements().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nest_matrix_empty() {
        let flat_matrix = ();
        let _nested_matrix: () = flat_matrix.nest_matrix();
    }

    #[test]
    fn test_flatten_matrix_empty() {
        let nested_matrix = ();
        let _flat_matrix: () = nested_matrix.flatten_matrix();
    }

    #[test]
    fn test_nest_matrix_single() {
        let flat_matrix = ((1, 2, 3),);
        let nested_matrix = flat_matrix.nest_matrix();
        assert_eq!(nested_matrix, ((1, (2, (3,))),));
    }

    #[test]
    fn test_flatten_matrix_single() {
        let nested_matrix = ((1, (2, (3,))),);
        let flat_matrix = nested_matrix.flatten_matrix();
        assert_eq!(flat_matrix, ((1, 2, 3),));
    }

    #[test]
    fn test_nest_matrix_2x2() {
        let flat_matrix = ((1, 2), (3, 4));
        let nested_matrix = flat_matrix.nest_matrix();
        assert_eq!(nested_matrix, ((1, (2,)), ((3, (4,)),)));
    }

    #[test]
    fn test_flatten_matrix_2x2() {
        let nested_matrix = ((1, (2,)), ((3, (4,)),));
        let flat_matrix = nested_matrix.flatten_matrix();
        assert_eq!(flat_matrix, ((1, 2), (3, 4)));
    }

    #[test]
    fn test_nest_matrix_2x3() {
        let flat_matrix = ((1, 2, 3), (4, 5, 6));
        let nested_matrix = flat_matrix.nest_matrix();
        assert_eq!(nested_matrix, ((1, (2, (3,))), ((4, (5, (6,))),)));
    }

    #[test]
    fn test_flatten_matrix_2x3() {
        let nested_matrix = ((1, (2, (3,))), ((4, (5, (6,))),));
        let flat_matrix = nested_matrix.flatten_matrix();
        assert_eq!(flat_matrix, ((1, 2, 3), (4, 5, 6)));
    }

    #[test]
    fn test_nest_matrix_3x2() {
        let flat_matrix = ((1, 2), (3, 4), (5, 6));
        let nested_matrix = flat_matrix.nest_matrix();
        assert_eq!(nested_matrix, ((1, (2,)), ((3, (4,)), ((5, (6,)),))));
    }

    #[test]
    fn test_flatten_matrix_3x2() {
        let nested_matrix = ((1, (2,)), ((3, (4,)), ((5, (6,)),)));
        let flat_matrix = nested_matrix.flatten_matrix();
        assert_eq!(flat_matrix, ((1, 2), (3, 4), (5, 6)));
    }

    #[test]
    fn test_round_trip_2x2() {
        let original = ((1, 2), (3, 4));
        let nested = original.nest_matrix();
        let flattened = nested.flatten_matrix();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_round_trip_3x3() {
        let original = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
        let nested = original.nest_matrix();
        let flattened = nested.flatten_matrix();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_round_trip_mixed_types() {
        let original = (("a", "b"), ("c", "d"));
        let nested = original.nest_matrix();
        let flattened = nested.flatten_matrix();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_round_trip_single_element_tuples() {
        let original = ((1,), (2,), (3,));
        let nested = original.nest_matrix();
        let flattened = nested.flatten_matrix();
        assert_eq!(original, flattened);
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_nested_matrix_structure() {
        let flat_matrix = ((1, 2), (3, 4));
        let nested_matrix: ((i32, (i32,)), ((i32, (i32,)),)) = flat_matrix.nest_matrix();
        // Verify the nested structure
        let ((first_row_0, (first_row_1,)), ((second_row_0, (second_row_1,)),)) = nested_matrix;
        assert_eq!(first_row_0, 1);
        assert_eq!(first_row_1, 2);
        assert_eq!(second_row_0, 3);
        assert_eq!(second_row_1, 4);
    }
}

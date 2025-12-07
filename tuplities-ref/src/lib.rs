#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleRef` trait.

#[tuplities_derive::impl_tuple_ref]
/// A trait for tuples that provides a method to get a tuple of references.
///
/// This trait provides both an associated type `Ref<'a>` that represents a tuple
/// of references to the elements, and a method `tuple_ref` that returns such a tuple.
///
/// # Examples
///
/// ```rust
/// use tuplities_ref::TupleRef;
///
/// let tuple = (1, "hello".to_string(), vec![1, 2, 3]);
/// let refs = tuple.tuple_ref();
/// assert_eq!(refs, (&1, &"hello".to_string(), &vec![1, 2, 3]));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleRef {
    /// The type of a tuple containing references to each element.
    type Ref<'a>
    where
        Self: 'a;

    /// Returns a tuple of references to each element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_ref::TupleRef;
    ///
    /// let tuple = (42, "world");
    /// let refs = tuple.tuple_ref();
    /// assert_eq!(refs, (&42, &"world"));
    /// ```
    fn tuple_ref(&self) -> Self::Ref<'_>;
}

#[tuplities_derive::impl_tuple_ref_map]

/// A trait for applying `TupleRef` to each element of a tuple.
///
/// This trait takes a tuple where each element implements `TupleRef` and returns
/// a tuple where each element is the result of calling `tuple_ref()` on the original elements.
///
/// # Examples
///
/// ```rust
/// use tuplities_ref::TupleRefMap;
///
/// let matrix = ((1, 2), (3, 4), (5, 6));
/// let ref_matrix = matrix.tuple_ref_map();
/// assert_eq!(ref_matrix, ((&1, &2), (&3, &4), (&5, &6)));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleRefMap {
    /// The type of a tuple containing tuples of references to each inner element.
    type RefMap<'a>
    where
        Self: 'a;

    /// Returns a tuple where each element is a tuple of references to the inner elements.
    fn tuple_ref_map(&self) -> Self::RefMap<'_>;
}

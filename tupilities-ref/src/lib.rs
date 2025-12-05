#![no_std]

//! Tupilities suite crate providing the `TupleRef` trait.

#[tupilities_derive::impl_tuple_ref]
/// A trait for tuples that provides a method to get a tuple of references.
///
/// This trait provides both an associated type `Ref<'a>` that represents a tuple
/// of references to the elements, and a method `tuple_ref` that returns such a tuple.
///
/// # Examples
///
/// ```rust
/// use tupilities_ref::TupleRef;
///
/// let tuple = (1, "hello".to_string(), vec![1, 2, 3]);
/// let refs = tuple.tuple_ref();
/// assert_eq!(refs, (&1, &"hello".to_string(), &vec![1, 2, 3]));
/// ```
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
    /// use tupilities_ref::TupleRef;
    ///
    /// let tuple = (42, "world");
    /// let refs = tuple.tuple_ref();
    /// assert_eq!(refs, (&42, &"world"));
    /// ```
    fn tuple_ref(&self) -> Self::Ref<'_>;
}

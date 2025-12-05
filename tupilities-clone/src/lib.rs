//! Tupilities suite crate providing the `TupleClone` trait.

#[tupilities_derive::impl_tuple_clone]
/// A trait for cloning tuples.
pub trait TupleClone {
    #[must_use]
    /// Clones `self` into a new instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_clone::TupleClone;
    ///
    /// let tuple = (1, "hello", vec![1, 2, 3]);
    /// let cloned_tuple = tuple.tuple_clone();
    ///
    /// assert_eq!(tuple, cloned_tuple);
    /// ```
    fn tuple_clone(&self) -> Self;
}

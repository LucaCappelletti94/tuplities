#![no_std]

//! Tupilities suite crate providing the `TupleMut` trait.

#[tupilities_derive::impl_tuple_mut]
/// A trait for tuples that provides a method to get a tuple of mutable references.
///
/// This trait provides both an associated type `Mut<'a>` that represents a tuple
/// of mutable references to the elements, and a method `tuple_mut` that returns such a tuple.
///
/// # Examples
///
/// ```rust
/// use tupilities_mut::TupleMut;
///
/// let mut tuple = (1, "hello".to_string(), vec![1, 2, 3]);
/// let mut_refs = tuple.tuple_mut();
/// *mut_refs.0 = 42;
/// mut_refs.1.push_str(" world");
/// mut_refs.2.push(4);
/// assert_eq!(tuple, (42, "hello world".to_string(), vec![1, 2, 3, 4]));
/// ```
pub trait TupleMut {
    /// The type of a tuple containing mutable references to each element.
    type Mut<'a>
    where
        Self: 'a;

    /// Returns a tuple of mutable references to each element.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_mut::TupleMut;
    ///
    /// let mut tuple = (42, "world".to_string());
    /// let mut_refs = tuple.tuple_mut();
    /// *mut_refs.0 = 24;
    /// mut_refs.1.make_ascii_uppercase();
    /// assert_eq!(tuple, (24, "WORLD".to_string()));
    /// ```
    fn tuple_mut(&mut self) -> Self::Mut<'_>;
}

#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleMut` trait.

#[tuplities_derive::impl_tuple_mut]
/// A trait for tuples that provides a method to get a tuple of mutable references.
///
/// This trait provides both an associated type `Mut<'a>` that represents a tuple
/// of mutable references to the elements, and a method `tuple_mut` that returns such a tuple.
///
/// # Examples
///
/// ```rust
/// use tuplities_mut::TupleMut;
///
/// let mut tuple = (1, "hello".to_string(), vec![1, 2, 3]);
/// let mut_refs = tuple.tuple_mut();
/// *mut_refs.0 = 42;
/// mut_refs.1.push_str(" world");
/// mut_refs.2.push(4);
/// assert_eq!(tuple, (42, "hello world".to_string(), vec![1, 2, 3, 4]));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
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
    /// use tuplities_mut::TupleMut;
    ///
    /// let mut tuple = (42, "world".to_string());
    /// let mut_refs = tuple.tuple_mut();
    /// *mut_refs.0 = 24;
    /// mut_refs.1.make_ascii_uppercase();
    /// assert_eq!(tuple, (24, "WORLD".to_string()));
    /// ```
    fn tuple_mut(&mut self) -> Self::Mut<'_>;
}

#[tuplities_derive::impl_tuple_mut_map]
/// A trait for applying `TupleMut` to each element of a tuple.
///
/// This trait takes a mutable reference to a tuple where each element implements `TupleMut` and returns
/// a tuple where each element is the result of calling `tuple_mut()` on the original elements.
///
/// # Examples
///
/// ```rust
/// use tuplities_mut::TupleMutMap;
///
/// let mut matrix = ((1, 2), (3, 4), (5, 6));
/// let mut_ref_matrix = matrix.tuple_mut_map();
/// assert_eq!(mut_ref_matrix, ((&mut 1, &mut 2), (&mut 3, &mut 4), (&mut 5, &mut 6)));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait TupleMutMap {
    /// The type of a tuple containing tuples of mutable references to each inner element.
    type MutMap<'a>
    where
        Self: 'a;

    /// Returns a tuple where each element is a tuple of mutable references to the inner elements.
    fn tuple_mut_map(&mut self) -> Self::MutMap<'_>;
}

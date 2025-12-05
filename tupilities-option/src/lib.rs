#![no_std]

//! Tupilities suite crate providing the `TupleOption` and `IntoTupleOption` traits.

#[tupilities_derive::impl_tuple_option]
/// A trait for transposing a tuple of options into an option of a tuple.
pub trait TupleOption {
    /// The transposed type: an option of the tuple of the inner types.
    type Transposed: IntoTupleOption<AsOptions = Self>;

    /// Transposes the tuple of options into an option of the tuple.
    ///
    /// Returns `Some((a, b, ...))` if all elements are `Some`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_option::TupleOption;
    ///
    /// let tuple = (Some(1), Some(2));
    /// let transposed: Option<(i32, i32)> = tuple.transpose();
    /// assert_eq!(transposed, Some((1, 2)));
    ///
    /// let tuple = (Some(1), None);
    /// let transposed: Option<(i32, i32)> = tuple.transpose();
    /// assert_eq!(transposed, None);
    /// ```
    fn transpose(self) -> Option<Self::Transposed>;
}

/// A trait for converting a tuple into a tuple of options.
pub trait IntoTupleOption {
    /// The tuple of options type.
    type AsOptions: TupleOption<Transposed = Self>;

    /// Converts the tuple into a tuple of `Some` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tupilities_option::IntoTupleOption;
    ///
    /// let tuple = (1, 2);
    /// let as_options: (Option<i32>, Option<i32>) = tuple.as_options();
    /// assert_eq!(as_options, (Some(1), Some(2)));
    /// ```
    fn as_options(self) -> Self::AsOptions;
}
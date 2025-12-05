#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleOption` and `IntoTupleOption` traits.

use tuplities_default::TupleDefault;

#[tuplities_derive::impl_tuple_option]
/// A trait for transposing a tuple of options into an option of a tuple.
pub trait TupleOption: TupleDefault {
    /// The transposed type: an option of the tuple of the inner types.
    type Transposed: IntoTupleOption<IntoOptions = Self>;

    /// Transposes the tuple of options into an option of the tuple.
    ///
    /// Returns `Some((a, b, ...))` if all elements are `Some`, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_option::TupleOption;
    ///
    /// let tuple = (Some(1), Some(2));
    /// let transposed: Option<(i32, i32)> = tuple.transpose();
    /// assert_eq!(transposed, Some((1, 2)));
    ///
    /// let tuple = (Some(1), None);
    /// let transposed: Option<(i32, i32)> = tuple.transpose();
    /// assert_eq!(transposed, None);
    /// ```
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn transpose(self) -> Option<Self::Transposed>;
}

/// A trait for converting a tuple into a tuple of options.
pub trait IntoTupleOption {
    /// The tuple of options type.
    type IntoOptions: TupleOption<Transposed = Self> + TupleDefault;

    /// Converts the tuple into a tuple of `Some` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_option::IntoTupleOption;
    ///
    /// let tuple = (1, 2);
    /// let into_options: (Option<i32>, Option<i32>) = tuple.into_options();
    /// assert_eq!(into_options, (Some(1), Some(2)));
    /// ```
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn into_options(self) -> Self::IntoOptions;
}

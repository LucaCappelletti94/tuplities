//! Module providing option transposition traits for nested tuples.

/// A trait for transposing nested tuples of options into options of nested tuples.
///
/// This trait takes a nested tuple where each element is an `Option<T>` and transposes it
/// into an `Option` of the nested tuple with the inner types. Returns `Some(nested_tuple)`
/// if all elements are `Some`, otherwise `None`.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::NestedTupleOption;
///
/// let nested_options = (Some(1), (Some(2), (Some(3),)));
/// let transposed: Option<(i32, (i32, (i32,)))> = nested_options.transpose();
/// assert_eq!(transposed, Some((1, (2, (3,)))));
///
/// let nested_options = (Some(1), (None, (Some(3),)));
/// let transposed: Option<(i32, (i32, (i32,)))> = nested_options.transpose();
/// assert_eq!(transposed, None);
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleOption {
    /// The transposed type: an option of the nested tuple of the inner types.
    type Transposed: IntoNestedTupleOption<IntoOptions = Self>;

    /// Transposes the nested tuple of options into an option of the nested tuple.
    ///
    /// Returns `Some(nested_tuple)` if all elements are `Some`, otherwise `None`.
    fn transpose(self) -> Option<Self::Transposed>;
}

/// A trait for converting nested tuples into nested tuples of options.
pub trait IntoNestedTupleOption {
    /// The nested tuple of options type.
    type IntoOptions: NestedTupleOption<Transposed = Self>;

    /// Converts the nested tuple into a nested tuple of `Some` values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_flatten_nest::IntoNestedTupleOption;
    ///
    /// let nested = (1, (2, (3,)));
    /// let into_options: (Option<i32>, (Option<i32>, (Option<i32>,))) = nested.into_options();
    /// assert_eq!(into_options, (Some(1), (Some(2), (Some(3),))));
    /// ```
    ///
    /// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
    fn into_options(self) -> Self::IntoOptions;
}

// Implementations for NestedTupleOption

impl NestedTupleOption for () {
    type Transposed = ();

    #[inline]
    fn transpose(self) -> Option<Self::Transposed> {
        Some(())
    }
}

impl<T> NestedTupleOption for (Option<T>,) {
    type Transposed = (T,);

    #[inline]
    fn transpose(self) -> Option<Self::Transposed> {
        let (opt,) = self;
        opt.map(|t| (t,))
    }
}

impl<Head, Tail> NestedTupleOption for (Option<Head>, Tail)
where
    Tail: NestedTupleOption,
{
    type Transposed = (Head, Tail::Transposed);

    #[inline]
    fn transpose(self) -> Option<Self::Transposed> {
        let (head_opt, tail) = self;
        match (head_opt, tail.transpose()) {
            (Some(head), Some(tail_transposed)) => Some((head, tail_transposed)),
            _ => None,
        }
    }
}

// Implementations for IntoNestedTupleOption

impl IntoNestedTupleOption for () {
    type IntoOptions = ();

    #[inline]
    fn into_options(self) -> Self::IntoOptions {}
}

impl<T> IntoNestedTupleOption for (T,) {
    type IntoOptions = (Option<T>,);

    #[inline]
    fn into_options(self) -> Self::IntoOptions {
        let (t,) = self;
        (Some(t),)
    }
}

impl<Head, Tail> IntoNestedTupleOption for (Head, Tail)
where
    Tail: IntoNestedTupleOption,
{
    type IntoOptions = (Option<Head>, Tail::IntoOptions);

    #[inline]
    fn into_options(self) -> Self::IntoOptions {
        let (head, tail) = self;
        (Some(head), tail.into_options())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_empty() {
        let nested_options = ();
        let transposed: Option<()> = nested_options.transpose();
        assert_eq!(transposed, Some(()));
    }

    #[test]
    fn test_transpose_single_some() {
        let nested_options = (Some(42),);
        let transposed: Option<(i32,)> = nested_options.transpose();
        assert_eq!(transposed, Some((42,)));
    }

    #[test]
    fn test_transpose_single_none() {
        let nested_options = (None::<i32>,);
        let transposed: Option<(i32,)> = nested_options.transpose();
        assert_eq!(transposed, None);
    }

    #[test]
    fn test_transpose_three_all_some() {
        let nested_options = (Some(1), (Some(2), (Some(3),)));
        let transposed: Option<(i32, (i32, (i32,)))> = nested_options.transpose();
        assert_eq!(transposed, Some((1, (2, (3,)))));
    }

    #[test]
    fn test_transpose_three_with_none() {
        let nested_options = (Some(1), (None, (Some(3),)));
        let transposed: Option<(i32, (i32, (i32,)))> = nested_options.transpose();
        assert_eq!(transposed, None);
    }

    #[test]
    fn test_into_options_empty() {
        let nested = ();
        let into_options: () = nested.into_options();
        assert_eq!(into_options, ());
    }

    #[test]
    fn test_into_options_single() {
        let nested = (42,);
        let into_options: (Option<i32>,) = nested.into_options();
        assert_eq!(into_options, (Some(42),));
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn test_into_options_three() {
        let nested = (1, (2, (3,)));
        let into_options: (Option<i32>, (Option<i32>, (Option<i32>,))) = nested.into_options();
        assert_eq!(into_options, (Some(1), (Some(2), (Some(3),))));
    }

    #[test]
    fn test_round_trip_options() {
        let original = (1, (2, (3,)));
        let options = original.into_options();
        let back = options.transpose().unwrap();
        assert_eq!(original, back);
    }
}

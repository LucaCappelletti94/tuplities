//! Module providing option transposition traits for nested tuples.

use crate::NestedTupleRef;

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

/// Helper trait for operating on nested `Option` structures with a parallel homogeneous
/// nested tuple type `SameDepth` carrying elements of type `H`.
pub trait NestedTupleOptionWith<H>: NestedTupleOption {
    /// The parallel homogeneous nested tuple type for `H` which has the same shape as `Self`.
    type SameDepth;

    /// Returns the first `H` corresponding to the first `None` in `self`.
    fn first_none_with(self, xs: Self::SameDepth) -> Option<H>;

    /// Returns the first `H` corresponding to the first `Some` in `self`.
    fn first_some_with(self, xs: Self::SameDepth) -> Option<H>;

    /// Like `transpose`, but returns a `Result` with `Ok(Transposed)` when all elements
    /// are `Some`, or `Err(H)` with the first `H` corresponding to the first `None`.
    ///
    /// # Errors
    ///
    /// Returns `Err(h)` when any element in `self` is `None`. The `h` returned is the
    /// corresponding element of the parallel homogeneous nested tuple `xs` for the first
    /// `None` encountered (traversed left-to-right, depth-first in the nested pair
    /// representation).
    fn transpose_or(self, xs: Self::SameDepth) -> Result<Self::Transposed, H>;
}

/// A trait for converting nested tuples into nested tuples of options.
pub trait IntoNestedTupleOption {
    /// The nested tuple of options type.
    type IntoOptions: NestedTupleOption<Transposed = Self> + NestedTupleRef;

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

impl<H> NestedTupleOptionWith<H> for () {
    type SameDepth = ();

    #[inline]
    fn first_none_with(self, _xs: Self::SameDepth) -> Option<H> {
        None
    }

    #[inline]
    fn first_some_with(self, _xs: Self::SameDepth) -> Option<H> {
        None
    }

    #[inline]
    fn transpose_or(self, _xs: Self::SameDepth) -> Result<Self::Transposed, H> {
        Ok(())
    }
}

impl<T, H> NestedTupleOptionWith<H> for (Option<T>,) {
    type SameDepth = (H,);

    #[inline]
    fn first_none_with(self, xs: Self::SameDepth) -> Option<H> {
        let (opt,) = self;
        let (h,) = xs;
        match opt {
            None => Some(h),
            Some(_) => None,
        }
    }

    #[inline]
    fn first_some_with(self, xs: Self::SameDepth) -> Option<H> {
        let (opt,) = self;
        let (h,) = xs;
        opt.map(|_| h)
    }

    #[inline]
    fn transpose_or(self, xs: Self::SameDepth) -> Result<Self::Transposed, H> {
        let (opt,) = self;
        let (h,) = xs;
        match opt {
            Some(t) => Ok((t,)),
            None => Err(h),
        }
    }
}

impl<Head, Tail, H> NestedTupleOptionWith<H> for (Option<Head>, Tail)
where
    Tail: NestedTupleOptionWith<H>,
{
    type SameDepth = (H, Tail::SameDepth);

    #[inline]
    fn first_none_with(self, xs: Self::SameDepth) -> Option<H> {
        let (head_opt, tail) = self;
        let (h_head, h_tail) = xs;
        match head_opt {
            None => Some(h_head),
            Some(_) => tail.first_none_with(h_tail),
        }
    }

    #[inline]
    fn first_some_with(self, xs: Self::SameDepth) -> Option<H> {
        let (head_opt, tail) = self;
        let (h_head, h_tail) = xs;
        match head_opt {
            Some(_) => Some(h_head),
            None => tail.first_some_with(h_tail),
        }
    }

    #[inline]
    fn transpose_or(self, xs: Self::SameDepth) -> Result<Self::Transposed, H> {
        let (head_opt, tail) = self;
        let (h_head, h_tail) = xs;
        match head_opt {
            None => Err(h_head),
            Some(head) => match tail.transpose_or(h_tail) {
                Ok(tail_transposed) => Ok((head, tail_transposed)),
                Err(h) => Err(h),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq, Eq)]
    struct NoCopy(i32);

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

    #[test]
    fn test_first_none_and_first_some_with_homogeneous() {
        let options = (Some(1), (None::<i32>, (Some(3),)));
        let hom = (10, (20, (30,)));
        assert_eq!(options.first_none_with(hom), Some(20));

        let hom2 = (11, (12, (13,)));
        assert_eq!(options.first_some_with(hom2), Some(11));
    }

    #[test]
    fn test_all_some_first_none_returns_none() {
        let options = (Some(1), (Some(2), (Some(3),)));
        let hom = (10, (20, (30,)));
        assert_eq!(options.first_none_with(hom), None);
    }

    #[test]
    fn test_all_none_first_some_returns_none() {
        let options = (None::<i32>, (None::<i32>, (None::<i32>,)));
        let hom = (10, (20, (30,)));
        assert_eq!(options.first_some_with(hom), None);
    }

    #[test]
    fn test_deeply_nested_first_none_prefers_leftmost() {
        let options = (Some(1), (None::<i32>, (None::<i32>,)));
        let hom = (10, (20, (30,)));
        assert_eq!(options.first_none_with(hom), Some(20));
    }

    #[test]
    fn test_deeply_nested_first_some_returns_deeper_value_when_head_none() {
        let options = (None::<i32>, (Some(2), (Some(3),)));
        let hom = (10, (20, (30,)));
        assert_eq!(options.first_some_with(hom), Some(20));
    }

    #[test]
    fn test_first_none_with_noncopy_hom_movable() {
        let options = (Some(1), (None::<i32>, (Some(3),)));
        let hom = (NoCopy(10), (NoCopy(20), (NoCopy(30),)));
        assert_eq!(options.first_none_with(hom), Some(NoCopy(20)));
    }

    #[test]
    fn test_first_some_with_noncopy_hom_movable() {
        let options = (None::<i32>, (Some(2), (None::<i32>,)));
        let hom = (NoCopy(11), (NoCopy(22), (NoCopy(33),)));
        assert_eq!(options.first_some_with(hom), Some(NoCopy(22)));
    }

    #[test]
    fn test_all_none_first_none_returns_head() {
        let options = (None::<i32>, (None::<i32>, (None::<i32>,)));
        let hom = (10, (20, (30,)));
        assert_eq!(options.first_none_with(hom), Some(10));
    }

    #[test]
    fn test_transpose_or_ok_and_err() {
        let options = (Some(1), (None::<i32>, (Some(3),)));
        let hom = (10, (20, (30,)));
        // Since the second element is None, we expect Err(20)
        assert_eq!(options.transpose_or(hom), Err(20));

        let options2 = (Some(1), (Some(2), (Some(3),)));
        let hom2 = (10, (20, (30,)));
        assert_eq!(options2.transpose_or(hom2), Ok((1, (2, (3,)))));
    }

    #[test]
    fn test_transpose_or_single_ok_and_err() {
        let options = (None::<i32>,);
        let hom = (99,);
        assert_eq!(options.transpose_or(hom), Err(99));

        let some = (Some(5),);
        let hom2 = (77,);
        assert_eq!(some.transpose_or(hom2), Ok((5,)));
    }
}

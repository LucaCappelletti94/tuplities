#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleSplit` trait.

/// A trait for splitting a tuple at a compile-time known index.
///
/// This trait allows splitting a tuple at a specific compile-time known index `Idx`,
/// returning two tuples: the elements before the index and the elements at and after the index.
///
/// # Examples
///
/// ```rust
/// use tuplities_split::TupleSplit;
/// use typenum::U2;
///
/// let tuple = (1, 2, 3, 4);
/// let (left, right) = TupleSplit::<U2>::split(tuple);
/// assert_eq!(left, (1, 2));
/// assert_eq!(right, (3, 4));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
#[tuplities_derive::impl_split]
pub trait TupleSplit<Idx: typenum::Unsigned> {
    /// The type of the tuple containing elements before the split index.
    type Left;

    /// The type of the tuple containing elements at and after the split index.
    type Right;

    /// Splits the tuple at index `Idx`, returning two tuples.
    fn split(self) -> (Self::Left, Self::Right);
}

#[cfg(test)]
mod tests {
    use super::TupleSplit;
    use typenum::{U0, U1, U2, U3};

    #[test]
    fn test_split_at_zero() {
        let tuple = (1, 2, 3);
        let (left, right) = TupleSplit::<U0>::split(tuple);
        assert_eq!(left, ());
        assert_eq!(right, (1, 2, 3));
    }

    #[test]
    fn test_split_at_middle() {
        let tuple = (1, 2, 3, 4);
        let (left, right) = TupleSplit::<U2>::split(tuple);
        assert_eq!(left, (1, 2));
        assert_eq!(right, (3, 4));
    }

    #[test]
    fn test_split_at_end() {
        let tuple = (1, 2, 3);
        let (left, right) = TupleSplit::<U3>::split(tuple);
        assert_eq!(left, (1, 2, 3));
        assert_eq!(right, ());
    }

    #[test]
    fn test_split_empty_tuple() {
        let tuple = ();
        let (left, right) = TupleSplit::<U0>::split(tuple);
        assert_eq!(left, ());
        assert_eq!(right, ());
    }

    #[test]
    fn test_split_single_element_at_zero() {
        let tuple = (42,);
        let (left, right) = TupleSplit::<U0>::split(tuple);
        assert_eq!(left, ());
        assert_eq!(right, (42,));
    }

    #[test]
    fn test_split_single_element_at_one() {
        let tuple = (42,);
        let (left, right) = TupleSplit::<U1>::split(tuple);
        assert_eq!(left, (42,));
        assert_eq!(right, ());
    }

    #[test]
    fn test_split_two_elements_at_zero() {
        let tuple = (1, 2);
        let (left, right) = TupleSplit::<U0>::split(tuple);
        assert_eq!(left, ());
        assert_eq!(right, (1, 2));
    }

    #[test]
    fn test_split_two_elements_at_one() {
        let tuple = (1, 2);
        let (left, right) = TupleSplit::<U1>::split(tuple);
        assert_eq!(left, (1,));
        assert_eq!(right, (2,));
    }

    #[test]
    fn test_split_two_elements_at_two() {
        let tuple = (1, 2);
        let (left, right) = TupleSplit::<U2>::split(tuple);
        assert_eq!(left, (1, 2));
        assert_eq!(right, ());
    }
}
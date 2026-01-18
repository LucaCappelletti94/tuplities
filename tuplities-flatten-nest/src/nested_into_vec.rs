extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

/// A trait for converting a nested homogeneous tuple into a vector.
///
/// This trait handles nested tuples structured as cons-lists (e.g. `(head, (head2, (..., tail)))` or `(head, tail)`).
/// All elements must be of the same type `T`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIntoVec<T> {
    /// Converts the nested tuple into a `Vec<T>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_flatten_nest::NestedTupleIntoVec;
    ///
    /// let nested = (1, (2, (3,)));
    /// let vec: Vec<i32> = nested.into_vec();
    /// assert_eq!(vec, vec![1, 2, 3]);
    /// ```
    fn into_vec(self) -> Vec<T>;

    /// Appends the elements of the nested tuple to an existing vector.
    ///
    /// This is used internally by `into_vec` for efficiency but can also be used directly.
    fn append_to_vec(self, buf: &mut Vec<T>);
}

impl<T> NestedTupleIntoVec<T> for () {
    #[inline]
    fn into_vec(self) -> Vec<T> {
        Vec::new()
    }

    #[inline]
    fn append_to_vec(self, _buf: &mut Vec<T>) {}
}

impl<T> NestedTupleIntoVec<T> for (T,) {
    #[inline]
    fn into_vec(self) -> Vec<T> {
        let (t,) = self;
        vec![t]
    }

    #[inline]
    fn append_to_vec(self, buf: &mut Vec<T>) {
        let (t,) = self;
        buf.push(t);
    }
}

impl<T, Tail> NestedTupleIntoVec<T> for (T, Tail)
where
    Tail: NestedTupleIntoVec<T>,
{
    #[inline]
    fn into_vec(self) -> Vec<T> {
        let mut v = Vec::new();
        self.append_to_vec(&mut v);
        v
    }

    #[inline]
    fn append_to_vec(self, buf: &mut Vec<T>) {
        let (head, tail) = self;
        buf.push(head);
        tail.append_to_vec(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_vec_empty() {
        let nested: () = ();
        let v: Vec<i32> = nested.into_vec();
        assert_eq!(v, Vec::<i32>::new());
    }

    #[test]
    fn test_into_vec_single() {
        let nested = (1,);
        let v = nested.into_vec();
        assert_eq!(v, vec![1]);
    }

    #[test]
    fn test_into_vec_nested() {
        let nested = (1, (2, (3,)));
        let v = nested.into_vec();
        assert_eq!(v, vec![1, 2, 3]);
    }
}

//! Structural membership, length, and construction for recursive tuples.

/// A recursive list with unit or singleton termination and an element count.
///
/// ```
/// use tuplities::NestedTuple;
///
/// assert_eq!(<()>::LEN, 0);
/// assert_eq!(<(u8, (u16,))>::LEN, 2);
/// assert_eq!(<(u8, (u16, ()))>::LEN, 2);
/// ```
///
/// Scalar tails do not form recursive lists.
///
/// ```compile_fail
/// use tuplities::NestedTuple;
///
/// fn length<T: NestedTuple>(_: T) -> usize { T::LEN }
/// let _ = length((1_u8, 2_u8));
/// ```
pub trait NestedTuple {
    /// The number of heads in the recursive list.
    const LEN: usize;
}

impl NestedTuple for () {
    const LEN: usize = 0;
}

impl<Head> NestedTuple for (Head,) {
    const LEN: usize = 1;
}

impl<Head, Tail: NestedTuple> NestedTuple for (Head, Tail) {
    const LEN: usize = 1 + Tail::LEN;
}

impl<T: NestedTuple + ?Sized> NestedTuple for &T {
    const LEN: usize = T::LEN;
}

impl<T: NestedTuple + ?Sized> NestedTuple for &mut T {
    const LEN: usize = T::LEN;
}

/// Constructs a singleton-terminated recursive tuple, evaluating each expression once from left to right.
///
/// ```
/// use tuplities::neple;
///
/// let values = neple!(1_u8, "two", (true, 3_u16),);
/// assert_eq!(values, (1, ("two", ((true, 3),))));
/// ```
#[macro_export]
macro_rules! neple {
    () => {
        ()
    };
    ($head:expr $(,)?) => {
        ($head,)
    };
    ($head:expr, $($tail:expr),+ $(,)?) => {
        ($head, $crate::neple!($($tail),+))
    };
}

/// Names the singleton-terminated recursive tuple type for the supplied element types.
///
/// ```
/// use tuplities::{neple, neplety, NestedTuple};
///
/// type Values = neplety!(u8, &'static str, (bool, u16),);
/// let values: Values = neple!(1, "two", (true, 3));
/// assert_eq!(Values::LEN, 3);
/// assert_eq!(values, (1, ("two", ((true, 3),))));
/// ```
#[macro_export]
macro_rules! neplety {
    () => {
        ()
    };
    ($head:ty $(,)?) => {
        ($head,)
    };
    ($head:ty, $($tail:ty),+ $(,)?) => {
        ($head, $crate::neplety!($($tail),+))
    };
}

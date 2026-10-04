//! Module providing the `NestedTupleReverse` trait for reversing nested tuples.

use crate::NestedTuple;

/// Consumes owned tuple nodes into singleton-terminated reverse order.
pub trait NestedTupleReverse: NestedTuple {
    /// The reversed nested tuple type.
    type Output: NestedTuple;

    /// Consumes the list and returns its elements in reverse order.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::{neple, neplety, NestedTupleReverse};
    ///
    /// let () = ().nested_reverse();
    /// assert_eq!(neple!(7_u8).nested_reverse(), (7,));
    /// assert_eq!((7_u8, ()).nested_reverse(), (7,));
    ///
    /// let original = ((1_u8, 2_u16), (3_u32, (false, ())));
    /// let reversed: neplety!(bool, u32, (u8, u16)) = original.nested_reverse();
    /// assert_eq!(reversed, neple!(false, 3, (1, 2)));
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::{neple, NestedTupleReverse};
    ///
    /// let tail = neple!(2_u16, 3_u32);
    /// let _ = NestedTupleReverse::nested_reverse((1_u8, &tail));
    /// ```
    fn nested_reverse(self) -> Self::Output;
}

/// Reverses an owned tail into a nonempty singleton-terminated accumulator.
#[doc(hidden)]
pub trait NestedReverseAccumulator<Acc: NestedTuple>: NestedTuple {
    /// The fully reversed nested tuple type.
    type Output: NestedTuple;

    /// Consumes the tail and returns its elements reversed ahead of `acc`.
    fn nested_reverse_into(self, acc: Acc) -> Self::Output;
}

impl NestedTupleReverse for () {
    type Output = ();

    #[inline]
    fn nested_reverse(self) -> Self::Output {}
}

impl<Head> NestedTupleReverse for (Head,) {
    type Output = (Head,);

    #[inline]
    fn nested_reverse(self) -> Self::Output {
        self
    }
}

impl<Head, Tail> NestedTupleReverse for (Head, Tail)
where
    Tail: NestedReverseAccumulator<(Head,)>,
{
    type Output = <Tail as NestedReverseAccumulator<(Head,)>>::Output;

    #[inline]
    fn nested_reverse(self) -> Self::Output {
        let (head, tail) = self;
        tail.nested_reverse_into((head,))
    }
}

impl<Acc> NestedReverseAccumulator<Acc> for ()
where
    Acc: NestedTuple,
{
    type Output = Acc;

    #[inline]
    fn nested_reverse_into(self, acc: Acc) -> Self::Output {
        acc
    }
}

impl<Head, Acc> NestedReverseAccumulator<Acc> for (Head,)
where
    Acc: NestedTuple,
{
    type Output = (Head, Acc);

    #[inline]
    fn nested_reverse_into(self, acc: Acc) -> Self::Output {
        (self.0, acc)
    }
}

impl<Head, Tail, Acc> NestedReverseAccumulator<Acc> for (Head, Tail)
where
    Acc: NestedTuple,
    Tail: NestedReverseAccumulator<(Head, Acc)>,
{
    type Output = <Tail as NestedReverseAccumulator<(Head, Acc)>>::Output;

    #[inline]
    fn nested_reverse_into(self, acc: Acc) -> Self::Output {
        let (head, tail) = self;
        tail.nested_reverse_into((head, acc))
    }
}

impl<Acc> NestedReverseAccumulator<Acc> for &()
where
    Acc: NestedTuple,
{
    type Output = Acc;

    #[inline]
    fn nested_reverse_into(self, acc: Acc) -> Self::Output {
        acc
    }
}

impl<Acc> NestedReverseAccumulator<Acc> for &mut ()
where
    Acc: NestedTuple,
{
    type Output = Acc;

    #[inline]
    fn nested_reverse_into(self, acc: Acc) -> Self::Output {
        acc
    }
}

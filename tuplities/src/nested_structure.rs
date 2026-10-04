//! Module providing the consuming structural split, insert, and remove traits.

use crate::{NestedTuple, NestedTuplePushFront};

/// Splits at `typenum` indices through `NestedTuple::LEN`, singleton-terminating the prefix and preserving the suffix.
pub trait NestedTupleSplit<Idx>: NestedTuple {
    /// The rebuilt prefix holding the first `Idx` heads, singleton-terminated.
    type Prefix: NestedTuple;

    /// The untouched suffix starting at `Idx`, preserving the input representation.
    type Suffix: NestedTuple;

    /// Consumes the list, returning the prefix and the suffix.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::{neple, NestedTupleSplit};
    /// use typenum::{U0, U2};
    ///
    /// assert_eq!(NestedTupleSplit::<U0>::nested_split(()), ((), ()));
    ///
    /// let (prefix, suffix) = NestedTupleSplit::<U2>::nested_split(neple!(1_u8, 2_u16, 3_u32));
    /// assert_eq!((prefix, suffix), (neple!(1_u8, 2_u16), neple!(3_u32)));
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::NestedTupleSplit;
    /// use typenum::U3;
    ///
    /// let _ = NestedTupleSplit::<U3>::nested_split((1_u8, (2_u16, ())));
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::{neple, NestedTupleSplit};
    /// use typenum::U2;
    ///
    /// let tail = neple!(1_u8, 2_u16);
    /// let _ = NestedTupleSplit::<U2>::nested_split((0_u8, &tail));
    /// ```
    fn nested_split(self) -> (Self::Prefix, Self::Suffix);
}

/// Inserts at `typenum` indices through `NestedTuple::LEN`, using singleton termination at an empty tail.
pub trait NestedTupleInsert<Idx, Item>: NestedTuple {
    /// The list type with `Item` inserted at `Idx`.
    type Output: NestedTuple;

    /// Consumes the list, returning the list with `Item` inserted at `Idx`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::{neple, NestedTupleInsert};
    /// use typenum::{U0, U1};
    ///
    /// assert_eq!(NestedTupleInsert::<U0, u8>::nested_insert((), 7_u8), (7_u8,));
    ///
    /// let inserted = NestedTupleInsert::<U1, u32>::nested_insert(neple!(1_u8, 2_u16), 3_u32);
    /// assert_eq!(inserted, neple!(1_u8, 3_u32, 2_u16));
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::NestedTupleInsert;
    /// use typenum::U3;
    ///
    /// let _ = NestedTupleInsert::<U3, u32>::nested_insert((1_u8, (2_u16, ())), 3_u32);
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::{neple, NestedTupleInsert};
    /// use typenum::U2;
    ///
    /// let tail = neple!(1_u8, 2_u16);
    /// let _ = NestedTupleInsert::<U2, u16>::nested_insert((0_u8, &tail), 9_u16);
    /// ```
    fn nested_insert(self, item: Item) -> Self::Output;
}

/// Removes at `typenum` indices below `NestedTuple::LEN`, singleton-terminating rebuilt prefixes when the remainder becomes empty.
pub trait NestedTupleRemove<Idx>: NestedTuple {
    /// The type of the removed head.
    type Removed;

    /// The rebuilt list holding the remaining heads.
    type Remainder: NestedTuple;

    /// Consumes the list, returning the removed head and the remainder.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::{neple, NestedTupleRemove};
    /// use typenum::{U0, U2};
    ///
    /// assert_eq!(NestedTupleRemove::<U0>::nested_remove(neple!(7_u8)), (7_u8, ()));
    ///
    /// let (removed, remainder) =
    ///     NestedTupleRemove::<U2>::nested_remove(neple!(1_u8, 2_u16, 3_u32));
    /// assert_eq!((removed, remainder), (3_u32, neple!(1_u8, 2_u16)));
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::NestedTupleRemove;
    /// use typenum::U2;
    ///
    /// let _ = NestedTupleRemove::<U2>::nested_remove((1_u8, (2_u16, ())));
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::NestedTupleRemove;
    /// use typenum::U0;
    ///
    /// let _ = NestedTupleRemove::<U0>::nested_remove(());
    /// ```
    ///
    /// ```compile_fail
    /// use tuplities::{neple, NestedTupleRemove};
    /// use typenum::U1;
    ///
    /// let tail = neple!(1_u8, 2_u16);
    /// let _ = NestedTupleRemove::<U1>::nested_remove((0_u8, &tail));
    /// ```
    fn nested_remove(self) -> (Self::Removed, Self::Remainder);
}

impl<T: NestedTuple> NestedTupleSplit<typenum::U0> for T {
    type Prefix = ();
    type Suffix = Self;

    #[inline]
    fn nested_split(self) -> (Self::Prefix, Self::Suffix) {
        ((), self)
    }
}

impl<Head> NestedTupleSplit<typenum::U1> for (Head,) {
    type Prefix = (Head,);
    type Suffix = ();

    #[inline]
    fn nested_split(self) -> (Self::Prefix, Self::Suffix) {
        ((self.0,), ())
    }
}

impl<Head, Tail, U, B> NestedTupleSplit<typenum::UInt<U, B>> for (Head, Tail)
where
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleSplit<typenum::Sub1<typenum::UInt<U, B>>>,
    <Tail as NestedTupleSplit<typenum::Sub1<typenum::UInt<U, B>>>>::Prefix:
        NestedTuplePushFront<Head, Output: NestedTuple>,
{
    type Prefix = <<Tail as NestedTupleSplit<typenum::Sub1<typenum::UInt<U, B>>>>::Prefix
        as NestedTuplePushFront<Head>>::Output;
    type Suffix = <Tail as NestedTupleSplit<typenum::Sub1<typenum::UInt<U, B>>>>::Suffix;

    #[inline]
    fn nested_split(self) -> (Self::Prefix, Self::Suffix) {
        let (head, tail) = self;
        let (prefix, suffix) =
            <Tail as NestedTupleSplit<typenum::Sub1<typenum::UInt<U, B>>>>::nested_split(tail);
        (prefix.nested_push_front(head), suffix)
    }
}

impl<T, Item> NestedTupleInsert<typenum::U0, Item> for T
where
    T: NestedTuple + NestedTuplePushFront<Item, Output: NestedTuple>,
{
    type Output = <Self as NestedTuplePushFront<Item>>::Output;

    #[inline]
    fn nested_insert(self, item: Item) -> Self::Output {
        self.nested_push_front(item)
    }
}

impl<Head, Item> NestedTupleInsert<typenum::U1, Item> for (Head,) {
    type Output = (Head, (Item,));

    #[inline]
    fn nested_insert(self, item: Item) -> Self::Output {
        (self.0, (item,))
    }
}

impl<Head, Item, Tail, U, B> NestedTupleInsert<typenum::UInt<U, B>, Item> for (Head, Tail)
where
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleInsert<typenum::Sub1<typenum::UInt<U, B>>, Item>,
{
    type Output = (
        Head,
        <Tail as NestedTupleInsert<typenum::Sub1<typenum::UInt<U, B>>, Item>>::Output,
    );

    #[inline]
    fn nested_insert(self, item: Item) -> Self::Output {
        let (head, tail) = self;
        let inserted =
            <Tail as NestedTupleInsert<typenum::Sub1<typenum::UInt<U, B>>, Item>>::nested_insert(
                tail, item,
            );
        (head, inserted)
    }
}

impl<Head> NestedTupleRemove<typenum::U0> for (Head,) {
    type Removed = Head;
    type Remainder = ();

    #[inline]
    fn nested_remove(self) -> (Self::Removed, Self::Remainder) {
        (self.0, ())
    }
}

impl<Head, Tail: NestedTuple> NestedTupleRemove<typenum::U0> for (Head, Tail) {
    type Removed = Head;
    type Remainder = Tail;

    #[inline]
    fn nested_remove(self) -> (Self::Removed, Self::Remainder) {
        (self.0, self.1)
    }
}

impl<Head, Tail, U, B> NestedTupleRemove<typenum::UInt<U, B>> for (Head, Tail)
where
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleRemove<typenum::Sub1<typenum::UInt<U, B>>>,
    <Tail as NestedTupleRemove<typenum::Sub1<typenum::UInt<U, B>>>>::Remainder:
        NestedTuplePushFront<Head, Output: NestedTuple>,
{
    type Removed = <Tail as NestedTupleRemove<typenum::Sub1<typenum::UInt<U, B>>>>::Removed;
    type Remainder = <<Tail as NestedTupleRemove<typenum::Sub1<typenum::UInt<U, B>>>>::Remainder
        as NestedTuplePushFront<Head>>::Output;

    #[inline]
    fn nested_remove(self) -> (Self::Removed, Self::Remainder) {
        let (head, tail) = self;
        let (removed, remainder) =
            <Tail as NestedTupleRemove<typenum::Sub1<typenum::UInt<U, B>>>>::nested_remove(tail);
        (removed, remainder.nested_push_front(head))
    }
}

#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `FlattenNestedTuple` and `NestTuple` traits.

use tuplities_push_front::TuplePushFront;

/// A trait for flattening nested tuples into flat tuples.
///
/// This trait takes a nested tuple structure like `(A, (B, (C,)))` and converts it
/// to a flat tuple like `(A, B, C)`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait FlattenNestedTuple {
    /// The flattened tuple type.
    type Flattened;

    /// Flattens the nested tuple into a flat tuple.
    fn flatten(self) -> Self::Flattened;
}

impl FlattenNestedTuple for () {
    type Flattened = ();

    #[inline]
    fn flatten(self) -> Self::Flattened {}
}

impl<N1> FlattenNestedTuple for (N1,) {
    type Flattened = (N1,);

    #[inline]
    fn flatten(self) -> Self::Flattened {
        let (n1,) = self;
        (n1,)
    }
}

impl<Head, Tail> FlattenNestedTuple for (Head, Tail)
where
    Tail: FlattenNestedTuple<Flattened: TuplePushFront<Head>>,
{
    type Flattened = <Tail::Flattened as TuplePushFront<Head>>::Output;

    #[inline]
    fn flatten(self) -> Self::Flattened {
        let (head, tail) = self;
        tail.flatten().push_front(head)
    }
}

#[tuplities_derive::impl_nest_tuple]
/// A trait for nesting flat tuples into nested tuples.
///
/// This trait takes a flat tuple like `(A, B, C)` and converts it
/// to a nested tuple structure like `(A, (B, (C,)))`.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestTuple {
    /// The nested tuple type.
    type Nested;

    /// Nests the flat tuple into a nested tuple.
    fn nest(self) -> Self::Nested;
}

/// A trait for indexing into nested tuples at compile-time known positions.
///
/// This trait allows accessing elements at specific flat indices `Idx`
/// from nested tuples, where `Idx` is a compile-time constant from `typenum`.
/// The index corresponds to the position in the flattened tuple.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::NestedTupleIndex;
/// use typenum::U1;
///
/// let nested = (1, (2, (3,)));
/// let element = NestedTupleIndex::<U1>::nested_index(&nested);
/// assert_eq!(*element, 2);
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIndex<Idx>: FlattenNestedTuple {
    /// The type of the element at flat index `Idx`.
    type Element;

    /// Returns a reference to the element at flat index `Idx`.
    fn nested_index(&self) -> &Self::Element;
}

/// A trait for mutable indexing into nested tuples at compile-time known positions.
///
/// This trait allows mutable access to elements at specific flat indices `Idx`
/// from nested tuples, where `Idx` is a compile-time constant from `typenum`.
/// The index corresponds to the position in the flattened tuple.
///
/// # Examples
///
/// ```
/// use tuplities_flatten_nest::NestedTupleIndexMut;
/// use typenum::U1;
///
/// let mut nested = (1, (2, (3,)));
/// let element = NestedTupleIndexMut::<U1>::nested_index_mut(&mut nested);
/// *element = 20;
/// assert_eq!(nested, (1, (20, (3,))));
/// ```
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleIndexMut<Idx>: FlattenNestedTuple {
    /// The type of the element at flat index `Idx`.
    type Element;

    /// Returns a mutable reference to the element at flat index `Idx`.
    fn nested_index_mut(&mut self) -> &mut Self::Element;
}

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

/// A trait for replicating a value into a nested tuple.
///
/// This trait allows creating a nested tuple where all elements are the same value.
/// The implementation follows the same optimization as `TupleReplicate`: for empty tuples
/// and single-element tuples, no `Clone` bound is required. For nested tuples with 2+
/// elements, `Clone` is required but the original value is moved to the last
/// position to minimize clones.
///
/// Part of the [`tuplities`](https://docs.rs/tuplities/latest/tuplities/) crate.
pub trait NestedTupleReplicate<T> {
    /// Creates a nested tuple where all elements are the provided value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_flatten_nest::NestedTupleReplicate;
    ///
    /// let nested: (i32, (i32, (i32,))) = NestedTupleReplicate::nested_tuple_replicate(42);
    /// assert_eq!(nested, (42, (42, (42,))));
    ///
    /// let nested_single: (i32,) = NestedTupleReplicate::nested_tuple_replicate(42);
    /// assert_eq!(nested_single, (42,));
    ///
    /// let _nested_empty: () = NestedTupleReplicate::nested_tuple_replicate(42);
    /// ```
    fn nested_tuple_replicate(value: T) -> Self;
}

impl<Head> NestedTupleIndex<typenum::U0> for (Head,) {
    type Element = Head;

    fn nested_index(&self) -> &Self::Element {
        &self.0
    }
}

impl<Head, Tail> NestedTupleIndex<typenum::U0> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
{
    type Element = Head;

    fn nested_index(&self) -> &Self::Element {
        &self.0
    }
}

impl<Head, Tail, U, B> NestedTupleIndex<typenum::UInt<U, B>> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleIndex<typenum::Sub1<typenum::UInt<U, B>>>,
{
    type Element = <Tail as NestedTupleIndex<typenum::Sub1<typenum::UInt<U, B>>>>::Element;

    fn nested_index(&self) -> &Self::Element {
        NestedTupleIndex::<typenum::Sub1<typenum::UInt<U, B>>>::nested_index(&self.1)
    }
}

impl<Head> NestedTupleIndexMut<typenum::U0> for (Head,) {
    type Element = Head;

    fn nested_index_mut(&mut self) -> &mut Self::Element {
        &mut self.0
    }
}

impl<Head, Tail> NestedTupleIndexMut<typenum::U0> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
{
    type Element = Head;

    fn nested_index_mut(&mut self) -> &mut Self::Element {
        &mut self.0
    }
}

impl<Head, Tail, U, B> NestedTupleIndexMut<typenum::UInt<U, B>> for (Head, Tail)
where
    (Head, Tail): FlattenNestedTuple,
    typenum::UInt<U, B>: core::ops::Sub<typenum::B1>,
    Tail: NestedTupleIndexMut<typenum::Sub1<typenum::UInt<U, B>>>,
{
    type Element = <Tail as NestedTupleIndexMut<typenum::Sub1<typenum::UInt<U, B>>>>::Element;

    fn nested_index_mut(&mut self) -> &mut Self::Element {
        NestedTupleIndexMut::<typenum::Sub1<typenum::UInt<U, B>>>::nested_index_mut(&mut self.1)
    }
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

// Implementations for NestedTupleReplicate

impl<T> NestedTupleReplicate<T> for () {
    #[inline]
    fn nested_tuple_replicate(_value: T) -> Self {}
}

impl<T> NestedTupleReplicate<T> for (T,) {
    #[inline]
    fn nested_tuple_replicate(value: T) -> Self {
        (value,)
    }
}

impl<T, Tail> NestedTupleReplicate<T> for (T, Tail)
where
    Tail: NestedTupleReplicate<T>,
    T: Clone,
{
    #[inline]
    fn nested_tuple_replicate(value: T) -> Self {
        (value.clone(), Tail::nested_tuple_replicate(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nest_tuple_3() {
        let flat = (1, 2, 3);
        let nested = flat.nest();
        assert_eq!(nested, (1, (2, (3,))));
    }

    #[test]
    fn test_flatten_nested_tuple_3() {
        let nested = (1, (2, (3,)));
        let flat = nested.flatten();
        assert_eq!(flat, (1, 2, 3));
    }

    #[test]
    fn test_round_trip_4() {
        let original = (1, 2, 3, 4);
        let nested = original.nest();
        let flattened = nested.flatten();
        assert_eq!(original, flattened);
    }

    #[test]
    fn test_round_trip_2() {
        let original = (42, "hello");
        let nested = original.nest();
        let flattened = nested.flatten();
        assert_eq!(original, flattened);
    }
}

#[cfg(test)]
mod nested_index_tests {
    use super::*;

    #[test]
    fn test_nested_index_3_u0() {
        let nested = (1, (2, (3,)));
        let element = NestedTupleIndex::<typenum::U0>::nested_index(&nested);
        assert_eq!(*element, 1);
    }

    #[test]
    fn test_nested_index_3_u1() {
        let nested = (1, (2, (3,)));
        let element = NestedTupleIndex::<typenum::U1>::nested_index(&nested);
        assert_eq!(*element, 2);
    }

    #[test]
    fn test_nested_index_3_u2() {
        let nested = (1, (2, (3,)));
        let element = NestedTupleIndex::<typenum::U2>::nested_index(&nested);
        assert_eq!(*element, 3);
    }

    #[test]
    fn test_nested_index_mut_3_u0() {
        let mut nested = (1, (2, (3,)));
        let element = NestedTupleIndexMut::<typenum::U0>::nested_index_mut(&mut nested);
        *element = 10;
        assert_eq!(nested, (10, (2, (3,))));
    }

    #[test]
    fn test_nested_index_mut_3_u1() {
        let mut nested = (1, (2, (3,)));
        let element = NestedTupleIndexMut::<typenum::U1>::nested_index_mut(&mut nested);
        *element = 20;
        assert_eq!(nested, (1, (20, (3,))));
    }

    #[test]
    fn test_nested_index_mut_3_u2() {
        let mut nested = (1, (2, (3,)));
        let element = NestedTupleIndexMut::<typenum::U2>::nested_index_mut(&mut nested);
        *element = 30;
        assert_eq!(nested, (1, (2, (30,))));
    }
}

#[cfg(test)]
mod nested_option_tests {
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

#[cfg(test)]
mod nested_replicate_tests {
    use super::*;

    #[test]
    fn test_replicate_empty() {
        let nested: () = NestedTupleReplicate::nested_tuple_replicate(42);
        assert_eq!(nested, ());
    }

    #[test]
    fn test_replicate_single() {
        let nested: (i32,) = NestedTupleReplicate::nested_tuple_replicate(42);
        assert_eq!(nested, (42,));
    }

    #[test]
    fn test_replicate_three() {
        let nested: (i32, (i32, (i32,))) = NestedTupleReplicate::nested_tuple_replicate(42);
        assert_eq!(nested, (42, (42, (42,))));
    }

    #[test]
    fn test_replicate_with_refs() {
        let value = &42;
        let nested: (&i32, (&i32, (&i32,))) = NestedTupleReplicate::nested_tuple_replicate(value);
        assert_eq!(nested, (&42, (&42, (&42,))));
    }
}

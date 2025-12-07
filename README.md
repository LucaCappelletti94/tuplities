# tuplities

[![Documentation](https://docs.rs/tuplities/badge.svg)](https://docs.rs/tuplities)
[![CI](https://github.com/LucaCappelletti94/tuplities/workflows/Rust%20CI/badge.svg)](https://github.com/LucaCappelletti94/tuplities/actions)
[![Security Audit](https://github.com/LucaCappelletti94/tuplities/workflows/Security%20Audit/badge.svg)](https://github.com/LucaCappelletti94/tuplities/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/tuplities.svg)](https://crates.io/crates/tuplities)

Tuple utilities in Rust, fractioned across several crates to improve compile times. There is a main crate `tuplities` that re-exports traits from subcrates in its `prelude` module, so that the subcrates may be compiled in parallel cutting down on overall build times.

This library is `#[no_std]` compatible, making it suitable for embedded systems and other environments without the standard library.

```toml
[dependencies]
tuplities = "0.1.4"
```

The library provides several traits for working with tuples:

- [`TupleClone`](https://docs.rs/tuplities-clone/latest/tuplities_clone/trait.TupleClone.html): Provides a [`tuple_clone()`](https://docs.rs/tuplities-clone/latest/tuplities_clone/trait.TupleClone.html#tymethod.tuple_clone) method to clone tuples. All elements must implement `Clone`.
- [`TupleCopy`](https://docs.rs/tuplities-copy/latest/tuplities_copy/trait.TupleCopy.html): Provides a [`tuple_copy()`](https://docs.rs/tuplities-copy/latest/tuplities_copy/trait.TupleCopy.html#tymethod.tuple_copy) method to copy tuples. All elements must implement `Copy`.
- [`TupleDebug`](https://docs.rs/tuplities-debug/latest/tuplities_debug/trait.TupleDebug.html): Provides a [`tuple_debug()`](https://docs.rs/tuplities-debug/latest/tuplities_debug/trait.TupleDebug.html#tymethod.tuple_debug) method that returns a debug string representation of the tuple. All elements must implement `Debug`.
- [`TupleDefault`](https://docs.rs/tuplities-default/latest/tuplities_default/trait.TupleDefault.html): Provides a [`tuple_default()`](https://docs.rs/tuplities-default/latest/tuplities_default/trait.TupleDefault.html#tymethod.tuple_default) method to create default instances of tuples. All elements must implement `Default`.
- [`TupleReplicate<T>`](https://docs.rs/tuplities-replicate/latest/tuplities_replicate/trait.TupleReplicate.html): Provides a [`tuple_replicate(value)`](https://docs.rs/tuplities-replicate/latest/tuplities_replicate/trait.TupleReplicate.html#tymethod.tuple_replicate) method to create tuples by replicating a single value across all positions. The value must implement `Clone` for tuples with 2+ elements, but not for empty tuples or single-element tuples.
- [`TupleHash`](https://docs.rs/tuplities-hash/latest/tuplities_hash/trait.TupleHash.html): Provides a [`tuple_hash<H: Hasher>()`](https://docs.rs/tuplities-hash/latest/tuplities_hash/trait.TupleHash.html#tymethod.tuple_hash) method to hash tuples with any hasher. All elements must implement `Hash`.
- [`TuplePartialEq`](https://docs.rs/tuplities-partial-eq/latest/tuplities_partial_eq/trait.TuplePartialEq.html): Provides a [`tuple_eq()`](https://docs.rs/tuplities-partial-eq/latest/tuplities_partial_eq/trait.TuplePartialEq.html#tymethod.tuple_eq) method to compare tuples for partial equality. All elements must implement `PartialEq`.
- [`TupleEq`](https://docs.rs/tuplities-eq/latest/tuplities_eq/trait.TupleEq.html): Provides a [`tuple_eq()`](https://docs.rs/tuplities-eq/latest/tuplities_eq/trait.TupleEq.html#tymethod.tuple_eq) method to compare tuples for total equality. All elements must implement `Eq`.
- [`TuplePartialOrd`](https://docs.rs/tuplities-partial-ord/latest/tuplities_partial_ord/trait.TuplePartialOrd.html): Provides a [`tuple_partial_cmp()`](https://docs.rs/tuplities-partial-ord/latest/tuplities_partial_ord/trait.TuplePartialOrd.html#tymethod.tuple_partial_cmp) method to compare tuples for partial ordering. All elements must implement `PartialOrd`.
- [`TupleOrd`](https://docs.rs/tuplities-ord/latest/tuplities_ord/trait.TupleOrd.html): Provides a [`tuple_cmp()`](https://docs.rs/tuplities-ord/latest/tuplities_ord/trait.TupleOrd.html#tymethod.tuple_cmp) method to compare tuples for total ordering. All elements must implement `Ord`.
- [`TupleOption`](https://docs.rs/tuplities-option/latest/tuplities_option/trait.TupleOption.html): Provides a [`transpose()`](https://docs.rs/tuplities-option/latest/tuplities_option/trait.TupleOption.html#tymethod.transpose) method to transpose a tuple of options into an option of a tuple. All elements must be `Option<T>`.
- [`IntoTupleOption`](https://docs.rs/tuplities-option/latest/tuplities_option/trait.IntoTupleOption.html): Provides an [`into_options()`](https://docs.rs/tuplities-option/latest/tuplities_option/trait.IntoTupleOption.html#tymethod.into_options) method to convert a tuple into a tuple of options.
- [`TupleRef`](https://docs.rs/tuplities-ref/latest/tuplities_ref/trait.TupleRef.html): Provides a [`tuple_ref()`](https://docs.rs/tuplities-ref/latest/tuplities_ref/trait.TupleRef.html#tymethod.tuple_ref) method to get references to each element in the tuple.
- [`TupleMut`](https://docs.rs/tuplities-mut/latest/tuplities_mut/trait.TupleMut.html): Provides a [`tuple_mut()`](https://docs.rs/tuplities-mut/latest/tuplities_mut/trait.TupleMut.html#tymethod.tuple_mut) method to get mutable references to each element in the tuple.
- [`TupleRefMap`](https://docs.rs/tuplities-ref/latest/tuplities_ref/trait.TupleRefMap.html): Provides a [`tuple_ref_map()`](https://docs.rs/tuplities-ref/latest/tuplities_ref/trait.TupleRefMap.html#tymethod.tuple_ref_map) method to apply `TupleRef` to each element of a tuple of tuples, returning a tuple of tuples of references.
- [`TupleMutMap`](https://docs.rs/tuplities-mut/latest/tuplities_mut/trait.TupleMutMap.html): Provides a [`tuple_mut_map()`](https://docs.rs/tuplities-mut/latest/tuplities_mut/trait.TupleMutMap.html#tymethod.tuple_mut_map) method to apply `TupleMut` to each element of a tuple of tuples, returning a tuple of tuples of mutable references.
- [`TuplePopFront`](https://docs.rs/tuplities-pop-front/latest/tuplities_pop_front/trait.TuplePopFront.html): Provides a [`pop_front()`](https://docs.rs/tuplities-pop-front/latest/tuplities_pop_front/trait.TuplePopFront.html#tymethod.pop_front) method to remove and return the first element of the tuple along with the remaining elements as a new tuple.
- [`TuplePopBack`](https://docs.rs/tuplities-pop-back/latest/tuplities_pop_back/trait.TuplePopBack.html): Provides a [`pop_back()`](https://docs.rs/tuplities-pop-back/latest/tuplities_pop_back/trait.TuplePopBack.html#tymethod.pop_back) method to remove and return the last element of the tuple along with the remaining elements as a new tuple.
- [`TuplePushFront<T>`](https://docs.rs/tuplities-push-front/latest/tuplities_push_front/trait.TuplePushFront.html): Provides a [`push_front(element)`](https://docs.rs/tuplities-push-front/latest/tuplities_push_front/trait.TuplePushFront.html#tymethod.push_front) method to add an element to the front of the tuple, returning a new tuple.
- [`TuplePushBack<T>`](https://docs.rs/tuplities-push-back/latest/tuplities_push_back/trait.TuplePushBack.html): Provides a [`push_back(element)`](https://docs.rs/tuplities-push-back/latest/tuplities_push_back/trait.TuplePushBack.html#tymethod.push_back) method to add an element to the back of the tuple, returning a new tuple.
- [`TupleRemove<Idx>`](https://docs.rs/tuplities-remove/latest/tuplities_remove/trait.TupleRemove.html): Provides a [`remove()`](https://docs.rs/tuplities-remove/latest/tuplities_remove/trait.TupleRemove.html#tymethod.remove) method to remove and return the element at the specified index [`typenum`](https://docs.rs/typenum/latest/typenum/)'s `Idx` of the tuple along with the remaining elements as a new tuple.
- [`TupleInsert<Idx, T>`](https://docs.rs/tuplities-insert/latest/tuplities_insert/trait.TupleInsert.html): Provides an [`insert()`](https://docs.rs/tuplities-insert/latest/tuplities_insert/trait.TupleInsert.html#tymethod.insert) method to insert an element at the specified index [`typenum`](https://docs.rs/typenum/latest/typenum/)'s `Idx` into the tuple, returning the tuple with the element inserted.
- [`TupleSplit<Idx>`](https://docs.rs/tuplities-split/latest/tuplities_split/trait.TupleSplit.html): Provides a [`split()`](https://docs.rs/tuplities-split/latest/tuplities_split/trait.TupleSplit.html#tymethod.split) method to split a tuple at the specified compile-time index [`typenum`](https://docs.rs/typenum/latest/typenum/)'s `Idx`, returning two tuples containing the elements before and at/after the index.
- [`TupleReverse`](https://docs.rs/tuplities-reverse/latest/tuplities_reverse/trait.TupleReverse.html): Provides a [`reverse()`](https://docs.rs/tuplities-reverse/latest/tuplities_reverse/trait.TupleReverse.html#tymethod.reverse) method to reverse the order of elements in a tuple.
- [`TupleTryFrom<T>`](https://docs.rs/tuplities-try-from/latest/tuplities_try_from/trait.TupleTryFrom.html): Provides a [`tuple_try_from()`](https://docs.rs/tuplities-try-from/latest/tuplities_try_from/trait.TupleTryFrom.html#tymethod.tuple_try_from) method to fallibly convert from other types into tuples.
- [`TupleTryInto<T>`](https://docs.rs/tuplities-try-from/latest/tuplities_try_from/trait.TupleTryInto.html): Provides a [`tuple_try_into()`](https://docs.rs/tuplities-try-from/latest/tuplities_try_from/trait.TupleTryInto.html#tymethod.tuple_try_into) method to fallibly convert tuples into other types.
- [`TupleFrom<T>`](https://docs.rs/tuplities-from/latest/tuplities_from/trait.TupleFrom.html): Provides a [`tuple_from()`](https://docs.rs/tuplities-from/latest/tuplities_from/trait.TupleFrom.html#tymethod.tuple_from) method to infallibly convert from other types into tuples.
- [`TupleInto<T>`](https://docs.rs/tuplities-from/latest/tuplities_from/trait.TupleInto.html): Provides a [`tuple_into()`](https://docs.rs/tuplities-from/latest/tuplities_from/trait.TupleInto.html#tymethod.tuple_into) method to infallibly convert tuples into other types.
- [`FlattenNestedTuple`](https://docs.rs/tuplities-flatten-nest/latest/tuplities_flatten_nest/trait.FlattenNestedTuple.html): Provides a [`flatten()`](https://docs.rs/tuplities-flatten-nest/latest/tuplities_flatten_nest/trait.FlattenNestedTuple.html#tymethod.flatten) method to convert nested tuples like `(A, (B, (C,)))` into flat tuples like `(A, B, C)`.
- [`NestTuple`](https://docs.rs/tuplities-flatten-nest/latest/tuplities_flatten_nest/trait.NestTuple.html): Provides a [`nest()`](https://docs.rs/tuplities-flatten-nest/latest/tuplities_flatten_nest/trait.NestTuple.html#tymethod.nest) method to convert flat tuples like `(A, B, C)` into nested tuples like `(A, (B, (C,)))`.
- [`TupleLen`](https://docs.rs/tuplities-len/latest/tuplities_len/trait.TupleLen.html): Provides the length of the tuple as a compile-time [`typenum::Unsigned`](https://docs.rs/typenum/latest/typenum/marker_traits/trait.Unsigned.html) type.
- [`UnitTuple`](https://docs.rs/tuplities-len/latest/tuplities_len/trait.UnitTuple.html): A marker trait implemented for empty tuples `()` with `TupleLen<Len = U0>`.
- [`SingletonTuple`](https://docs.rs/tuplities-len/latest/tuplities_len/trait.SingletonTuple.html): A marker trait implemented for single-element tuples `(T,)` with `TupleLen<Len = U1>`.
- [`PairTuple`](https://docs.rs/tuplities-len/latest/tuplities_len/trait.PairTuple.html): A marker trait implemented for two-element tuples `(T1, T2)` with `TupleLen<Len = U2>`.
- [`TupleIndex<Idx>`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.TupleIndex.html): Provides an [`index()`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.TupleIndex.html#tymethod.index) method to access the element at the specified index [`typenum`](https://docs.rs/typenum/latest/typenum/)'s `Idx` of the tuple.
- [`TupleIndexMut<Idx>`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.TupleIndexMut.html): Provides an [`index_mut()`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.TupleIndexMut.html#tymethod.index_mut) method to access a mutable reference to the element at the specified index [`typenum`](https://docs.rs/typenum/latest/typenum/)'s `Idx` of the tuple.
- [`FirstTupleIndex`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.FirstTupleIndex.html): A convenience trait providing a [`first_tuple_index()`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.FirstTupleIndex.html#tymethod.first_tuple_index) method to access the first element of a tuple.
- [`LastTupleIndex`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.LastTupleIndex.html): A convenience trait providing a [`last_tuple_index()`](https://docs.rs/tuplities-index/latest/tuplities_index/trait.LastTupleIndex.html#tymethod.last_tuple_index) method to access the last element of a tuple.
- [`TupleRow<Idx>`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.TupleRow.html): Provides a [`tuple_row()`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.TupleRow.html#tymethod.tuple_row) method to access elements at the specified index across all tuples in a tuple of tuples (row-wise indexing).
- [`TupleRowMut<Idx>`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.TupleRowMut.html): Provides a [`tuple_row_mut()`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.TupleRowMut.html#tymethod.tuple_row_mut) method to access mutable elements at the specified index across all tuples in a tuple of tuples (mutable row-wise indexing).
- [`FirstTupleRow`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.FirstTupleRow.html): A convenience trait providing a [`first_tuple_row()`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.FirstTupleRow.html#tymethod.first_tuple_row) method to access the first element of each tuple in a tuple of tuples.
- [`LastTupleRow`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.LastTupleRow.html): A convenience trait providing a [`last_tuple_row()`](https://docs.rs/tuplities-row/latest/tuplities_row/trait.LastTupleRow.html#tymethod.last_tuple_row) method to access the last element of each tuple in a tuple of tuples.

## Features

The crate provides features to generate trait implementations for tuples up to different sizes: 8 (default), 16, 32, 48, 64, 96, or 128 elements. Use the `size-XX` features to enable larger tuple support.

```toml
[dependencies]
tuplities = { version = "0.1.4", features = ["size-32"] }
```

## Performance

Compile times scale with tuple size due to code generation. Here are measured build times for different maximum tuple sizes (on a typical development machine):

| Max Tuple Size | Compile Time |
|----------------|--------------|
| 8 (default)    | 3.4s         |
| 16             | 2.6s         |
| 32             | 3.1s         |
| 48             | 4.1s         |
| 64             | 6.1s         |
| 96             | 14.0s        |
| 128            | 28.7s        |

## Architecture

The project is split into multiple crates for improved compile times:

- `tuplities/`: Main crate that re-exports traits from subcrates
- `tuplities-{trait_name}/`: Individual crates providing specific traits
- `tuplities-derive/`: Procedural macro crate that generates trait implementations

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contribution

Contributions are welcome! Please open issues or pull requests on the [GitHub repository](https://github.com/lucacappelletti94/tuplities).

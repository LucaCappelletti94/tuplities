# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added in Unreleased

- `TupleReplicate<T>` trait for creating tuples by replicating a single value across all positions
- Optimized implementation that avoids unnecessary cloning: no `Clone` bound for empty/single-element tuples, moves value to last position for multi-element tuples
- `TupleSplit<Idx>` trait for splitting tuples at compile-time indices using `typenum`
- `TupleReverse` trait for reversing tuple element order
- `UnitTuple` marker trait for empty tuples `()` with `TupleLen<Len = U0>`
- `SingletonTuple` marker trait for single-element tuples `(T,)` with `TupleLen<Len = U1>`
- `PairTuple` marker trait for two-element tuples `(T1, T2)` with `TupleLen<Len = U2>`
- `TupleTryFrom<T, E>` and `TupleTryInto<U, E>` traits for fallible conversions between tuples where elements implement `TryFrom`/`TryInto`
- `TupleFrom<T>` and `TupleInto<U>` traits for infallible conversions between tuples where elements implement `From`/`Into`
- Support for tuple conversions with proper error handling and type safety
- `TupleRow<Idx>` and `TupleRowMut<Idx>` traits for indexing rows in tuples of tuples
- `FirstTupleRow` and `LastTupleRow` convenience traits for accessing the first and last rows in tuples of tuples
- `FirstTupleIndex` and `LastTupleIndex` convenience traits for accessing the first and last elements in tuples
- `TupleRefMap` trait for applying `TupleRef` to each element of a tuple of tuples
- `TupleMutMap` trait for applying `TupleMut` to each element of a tuple of tuples
- `FlattenNestedTuple` trait for converting nested tuples like `(A, (B, (C,)))` to flat tuples like `(A, B, C)`
- `NestTuple` trait for converting flat tuples like `(A, B, C)` to nested tuples like `(A, (B, (C,)))`
- Added `flatten_ref` and `flatten_mut` methods to `FlattenNestedTuple` trait for accessing references to flattened elements without moving
- Added `NestedTupleIndex<Idx>` and `NestedTupleIndexMut<Idx>` traits for compile-time indexing into nested tuples using flat indices

## [0.1.0] - 2025-12-05

### Added in 0.1.0

- Initial release of tuplities, a collection of utilities for working with tuples in Rust.
- Support for various tuple operations: clone, copy, debug, default, eq, hash, index, insert, len, mut, option, ord, partial_eq, partial_ord, pop_back, pop_front, push_back, push_front, ref, remove.
- Procedural macro for generating trait implementations for tuples up to size 128.
- Feature flags for controlling maximum supported tuple size (default: 8, up to 128).
- No-std compatible.

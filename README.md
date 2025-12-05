# Tupilities

[![Documentation](https://docs.rs/tupilities/badge.svg)](https://docs.rs/tupilities)
[![CI](https://github.com/LucaCappelletti94/tupilities/workflows/Rust%20CI/badge.svg)](https://github.com/LucaCappelletti94/tupilities/actions)
[![Security Audit](https://github.com/LucaCappelletti94/tupilities/workflows/Security%20Audit/badge.svg)](https://github.com/LucaCappelletti94/tupilities/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/tupilities.svg)](https://crates.io/crates/tupilities)

Tuple utilities in Rust, fractioned across several crates to improve compile times. There is a main crate `tupilities` that re-exports traits from subcrates, which can be compiled in parallel cutting down on overall build times.

This library is `#[no_std]` compatible, making it suitable for embedded systems and other environments without the standard library.

```toml
[dependencies]
tupilities = "0.1"
```

## Traits

The library provides several traits for working with tuples:

- `TupleClone`: Provides a `tuple_clone()` method to clone tuples. All elements must implement `Clone`.
- `TupleCopy`: Provides a `tuple_copy()` method to copy tuples. All elements must implement `Copy`.
- `TupleDebug`: Provides a `tuple_debug()` method that returns a debug string representation of the tuple. All elements must implement `Debug`.
- `TupleDefault`: Provides a `tuple_default()` method to create default instances of tuples. All elements must implement `Default`.
- `TupleHash`: Provides a `tuple_hash<H: Hasher>()` method to hash tuples with any hasher. All elements must implement `Hash`.
- `TupleSipHasher24`: Provides a `tuple_sip_hash()` method that returns a hash value using SipHasher24. All elements must implement `Hash`.
- `TuplePartialEq`: Provides a `tuple_eq()` method to compare tuples for partial equality. All elements must implement `PartialEq`.
- `TupleEq`: Provides a `tuple_eq()` method to compare tuples for total equality. All elements must implement `Eq`.
- `TuplePartialOrd`: Provides a `tuple_partial_cmp()` method to compare tuples for partial ordering. All elements must implement `PartialOrd`.
- `TupleOrd`: Provides a `tuple_cmp()` method to compare tuples for total ordering. All elements must implement `Ord`.
- `TupleOption`: Provides a `transpose()` method to transpose a tuple of options into an option of a tuple. All elements must be `Option<T>`.
- `IntoTupleOption`: Provides an `into_options()` method to convert a tuple into a tuple of options.
- `TupleRef`: Provides a `tuple_ref()` method to get references to each element in the tuple.
- `TupleMut`: Provides a `tuple_mut()` method to get mutable references to each element in the tuple.

## Features

The crate provides features to generate trait implementations for tuples up to different sizes: 8 (default), 16, 32, 48, 64, 96, or 128 elements. Use the `size-XX` features to enable larger tuple support.

```toml
[dependencies]
tupilities = { version = "0.1", features = ["size-32"] }
```

## Architecture

The project is split into multiple crates for improved compile times:

- `tupilities/`: Main crate that re-exports traits from subcrates
- `tupilities-{trait_name}/`: Individual crates providing specific traits
- `tupilities-derive/`: Procedural macro crate that generates trait implementations

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contribution

Contributions are welcome! Please open issues or pull requests on the [GitHub repository](https://github.com/lucac/tupilities).

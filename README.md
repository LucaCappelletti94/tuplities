# tuplities

[![Crates.io](https://img.shields.io/crates/v/tuplities.svg)](https://crates.io/crates/tuplities)
[![Documentation](https://docs.rs/tuplities/badge.svg)](https://docs.rs/tuplities)
[![CI](https://github.com/LucaCappelletti94/tuplities/workflows/Rust%20CI/badge.svg)](https://github.com/LucaCappelletti94/tuplities/actions)
[![Security Audit](https://github.com/LucaCappelletti94/tuplities/workflows/Security%20Audit/badge.svg)](https://github.com/LucaCappelletti94/tuplities/actions)
[![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/LucaCappelletti94/tuplities/branch/main/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/tuplities)

Recursive heterogeneous tuple utilities for `no_std` Rust. Use `neple!` to construct lists and `neplety!` to name their types. Traits and macros are available at the crate root and through `tuplities::prelude`.

```rust
use tuplities::prelude::*;

type Values = neplety!(u8, &'static str, bool);
let values: Values = neple!(1_u8, "two").chain(neple!(true));
assert_eq!(values, (1, ("two", (true,))));
assert_eq!(values.into_options().transpose(), Some(values));
assert!(*NestedTupleIndex::<typenum::U2>::nested_index(&values));
assert_eq!(Values::LEN, 3);

let (prefix, suffix) = NestedTupleSplit::<typenum::U1>::nested_split(values);
let inserted = NestedTupleInsert::<typenum::U1, _>::nested_insert(prefix.chain(suffix), 42_u16);
let (removed, remainder) = NestedTupleRemove::<typenum::U1>::nested_remove(inserted);
assert_eq!(removed, 42);
assert_eq!(remainder.nested_reverse(), neple!(true, "two", 1_u8));
```

Split preserves its suffix and singleton-terminates its prefix. Insertion and removal preserve untouched suffixes and singleton-terminate reconstructed empty tails, while reversal produces a singleton-terminated list. Consuming operations accept reference-valued elements and leave borrowed recursive tails untouched, with traversal into borrowed tails rejected.

The default `flatten-nest` feature provides flat/nested tuple and matrix conversions and enables `alloc`. Flat conversions support widths through 8, with `size-16`, `size-32`, `size-48`, `size-64`, `size-96`, and `size-128` selecting larger limits. Enabling multiple size features selects the largest limit.

```rust
# #[cfg(feature = "flatten-nest")] {
use tuplities::{FlattenNestedTuple, NestTuple};

let flat = (String::from("one"), 2_u8, true);
let nested = flat.nest();
assert_eq!(nested, (String::from("one"), (2, (true,))));
assert_eq!(nested.flatten(), (String::from("one"), 2, true));
# }
```

Disable default features for the allocator-free recursive core and enable `alloc` for `NestedTupleIntoVec`. Indexing, row access, and `NestedTuple::LEN` depend on recursive structure without flat arity limits. Both singleton-terminated and unit-terminated lists support these operations, including borrowed tails.

Rust `1.85` or later is required. Standard traits such as `Clone`, `Copy`, `Debug`, equality, ordering, and hashing compose through nested pairs without additional tuple traits.

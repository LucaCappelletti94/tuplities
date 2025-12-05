# Tupilities Copilot Instructions

## Architecture Overview
This is a Rust workspace providing tuple utilities. The project is split into three crates for improved compile times:

- `tupilities/`: Main crate that re-exports traits from subcrates
- `tupilities-{snake_trait_name}/`: One of several crates of the suite which provides the `Tuple{camel_trait_name}` trait with derive macro implementation
- `tupilities-derive/`: Procedural macro crate that generates all of the trait implementations for tuples of various sizes

The derive macro generates implementations for tuples from size 0 (unit `()`) up to `MAX_TUPLE_SIZE`, controlled by Cargo features to balance functionality against compile time.

## Key Patterns

- **Feature-gated sizes**: Use features like `size-16`, `size-32` to control maximum supported tuple size (default: 8). See `tupilities-derive/src/tuple_size.rs` for size configuration logic.
- **Procedural macro generation**: Implementations are generated using `generate_all_sizes()` function that iterates over tuple sizes. See `tupilities-derive/src/tupilities_clone.rs` for the pattern.
- **Type parameter generation**: Use `type_params()` and `indices()` helpers in `tuple_size.rs` for generating generic type lists and field access indices.

## Developer Workflows
- Build all crates: `cargo build --workspace`
- Run tests: `cargo test --workspace`
- Enable larger tuple support: `cargo build --workspace --features size-32` (propagates to all crates)
- Debug macro expansion: `cargo expand --package tupilities-derive`

## Code Examples
```rust
// Basic usage
use tupilities::prelude::TupleClone;

let original = (1, "hello".to_string(), vec![1, 2, 3]);
let cloned = original.tuple_clone();
assert_eq!(original, cloned);
```

## Conventions

- Strict linting: Workspace enforces `missing_docs = "forbid"` and extensive Clippy rules. Always document public APIs.
- Edition 2024: Use modern Rust features available in the 2024 edition.
- Workspace dependencies: Define shared deps in root `Cargo.toml` under `[workspace.dependencies]`.

## Integration Points
- External deps: `syn`, `quote`, `proc-macro2` for macro generation
- Cross-crate communication: Derive crate generates code used by clone crate, re-exported by main crate
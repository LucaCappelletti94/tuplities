# tuplities Copilot Instructions

## Architecture

The workspace contains one crate, `tuplities/`. Recursive utilities operate on `()`, singleton tuples `(T,)`, and nested pairs `(Head, Tail)`.

`tuplities/src/flat_bridge.rs` generates flat/nested boundary conversions through internal `macro_rules!` macros. The default flat limit is 8. Features `size-16`, `size-32`, `size-48`, `size-64`, `size-96`, and `size-128` select larger limits, with the largest enabled size taking precedence.

The crate is `no_std`. The `alloc` feature enables homogeneous vector conversion. The default `flatten-nest` feature enables flat tuple and matrix conversions and includes `alloc`.

## Conventions

- Use recursive trait implementations for nested tuple operations.
- Preserve public associated-type equalities used by downstream generic code.
- Keep shared dependencies under `[workspace.dependencies]`.
- Document public APIs and preserve the workspace's Rust and Clippy lints.
- The crate README provides crate-level documentation and executable doctests.
- Use Rust edition `2024` and support the declared minimum compiler.

## Verification

Scope checks to `-p tuplities`. Verify default features, `--no-default-features`, `alloc`, and supported flat width configurations. Exercise flat/nested round trips, mutable borrowing, and downstream behavior when changing the bridges.

# Tupilities

[![Documentation](https://docs.rs/tupilities/badge.svg)](https://docs.rs/tupilities)
[![CI](https://github.com/LucaCappelletti94/tupilities/workflows/Rust%20CI/badge.svg)](https://github.com/LucaCappelletti94/tupilities/actions)
[![Security Audit](https://github.com/LucaCappelletti94/tupilities/workflows/Security%20Audit/badge.svg)](https://github.com/LucaCappelletti94/tupilities/actions)
[![Codecov](https://codecov.io/gh/LucaCappelletti94/tupilities/branch/main/graph/badge.svg)](https://codecov.io/gh/LucaCappelletti94/tupilities)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/crates/d/tupilities.svg)](https://crates.io/crates/tupilities)

Tuple utilities in Rust, fractioned across several crates to improve compile times. There is a main crate `tupilities` that re-exports traits from subcrates, which can be compiled in parallel cutting down on overall build times.

```toml
[dependencies]
tupilities = "0.1"
```

## Traits

- `TupleClone`: A trait that provides a method to clone tuples of various sizes. All elements of the tuple must implement the `Clone` trait.

## Features

At this time, the crate provides feature to generate the trait implementations for tuples up until the size of: 8 (default), 16, 32, 48, 64, 96, or 128 elements with the `size-XX` features.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contribution

Contributions are welcome! Please open issues or pull requests on the [GitHub repository](https://github.com/lucac/tupilities).

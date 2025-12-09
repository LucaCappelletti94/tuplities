#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing traits for flattening and nesting tuples.

mod flatten_nested;
mod matrix;
mod nest;
mod nested_index;
mod nested_option;
mod nested_replicate;

// Re-export all public traits and types
pub use flatten_nested::FlattenNestedTuple;
pub use matrix::{
    FlattenMatrixElements, FlattenNestedTupleMatrix, NestMatrixElements, NestTupleMatrix,
};
pub use nest::NestTuple;
pub use nested_index::{NestedTupleIndex, NestedTupleIndexMut};
pub use nested_option::{IntoNestedTupleOption, NestedTupleOption};
pub use nested_replicate::NestedTupleReplicate;

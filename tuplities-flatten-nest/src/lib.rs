#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing traits for flattening and nesting tuples.

mod flatten_nested;
mod matrix;
mod nest;
mod nested_index;
mod nested_option;
mod nested_push_pop;
mod nested_ref;
mod nested_replicate;
mod nested_row;
mod nested_try_from;

// Re-export all public traits and types
pub use flatten_nested::FlattenNestedTuple;
pub use matrix::{
    FlattenMatrixElements, FlattenNestedTupleMatrix, NestMatrixElements, NestTupleMatrix,
};
pub use nest::NestTuple;
pub use nested_index::{NestedTupleIndex, NestedTupleIndexMut};
pub use nested_option::{IntoNestedTupleOption, NestedTupleOption, NestedTupleOptionWith};
pub use nested_push_pop::{
    NestedTuplePopBack, NestedTuplePopFront, NestedTuplePushBack, NestedTuplePushFront,
};
pub use nested_ref::{NestedTupleMut, NestedTupleRef};
pub use nested_replicate::NestedTupleReplicate;
pub use nested_row::{NestedTupleRow, NestedTupleRowMut};
pub use nested_try_from::{NestedTupleTryFrom, NestedTupleTryInto};

#![no_std]
#![recursion_limit = "256"]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "flatten-nest")]
mod flat_bridge;
mod flatten_nested;
#[cfg(feature = "flatten-nest")]
mod matrix;
mod nest;
mod nested_chain;
mod nested_index;
#[cfg(feature = "alloc")]
mod nested_into_vec;
mod nested_option;
mod nested_option_try_from;
mod nested_push_pop;
mod nested_ref;
mod nested_replicate;
mod nested_reverse;
mod nested_row;
mod nested_starts_with;
mod nested_structure;
mod nested_try_from;
mod nested_tuple;

pub use flatten_nested::FlattenNestedTuple;
#[cfg(feature = "flatten-nest")]
pub use matrix::{
    FlattenMatrixElements, FlattenNestedTupleMatrix, NestMatrixElements, NestTupleMatrix,
};
pub use nest::{NestTuple, NestTupleMut, NestTupleRef};
pub use nested_chain::NestedTupleChain;
pub use nested_index::{NestedTupleIndex, NestedTupleIndexMut};
#[cfg(feature = "alloc")]
pub use nested_into_vec::NestedTupleIntoVec;
pub use nested_option::{
    IntoNestedTupleOption, NestedTupleFlattenOption, NestedTupleOption, NestedTupleOptionWith,
};
pub use nested_option_try_from::{
    NestedTupleOptionFrom, NestedTupleOptionInto, NestedTupleOptionTryFrom,
    NestedTupleOptionTryInto,
};
pub use nested_push_pop::{
    NestedTuplePopBack, NestedTuplePopFront, NestedTuplePushBack, NestedTuplePushFront,
};
pub use nested_ref::{NestedTupleMut, NestedTupleRef};
pub use nested_replicate::NestedTupleReplicate;
pub use nested_reverse::NestedTupleReverse;
pub use nested_row::{NestedTupleRow, NestedTupleRowMut};
pub use nested_starts_with::NestedTupleStartsWith;
pub use nested_structure::{NestedTupleInsert, NestedTupleRemove, NestedTupleSplit};
pub use nested_try_from::{
    NestedTupleFrom, NestedTupleInto, NestedTupleTryFrom, NestedTupleTryInto,
};
pub use nested_tuple::NestedTuple;

/// Traits and construction macros for recursive tuples and optional flat-tuple conversions.
pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use crate::NestedTupleIntoVec;
    #[cfg(feature = "flatten-nest")]
    pub use crate::{
        FlattenMatrixElements, FlattenNestedTupleMatrix, NestMatrixElements, NestTupleMatrix,
    };
    pub use crate::{
        FlattenNestedTuple, IntoNestedTupleOption, NestTuple, NestTupleMut, NestTupleRef,
        NestedTuple, NestedTupleChain, NestedTupleFlattenOption, NestedTupleFrom, NestedTupleIndex,
        NestedTupleIndexMut, NestedTupleInsert, NestedTupleInto, NestedTupleMut, NestedTupleOption,
        NestedTupleOptionFrom, NestedTupleOptionInto, NestedTupleOptionTryFrom,
        NestedTupleOptionTryInto, NestedTupleOptionWith, NestedTuplePopBack, NestedTuplePopFront,
        NestedTuplePushBack, NestedTuplePushFront, NestedTupleRef, NestedTupleRemove,
        NestedTupleReplicate, NestedTupleReverse, NestedTupleRow, NestedTupleRowMut,
        NestedTupleSplit, NestedTupleStartsWith, NestedTupleTryFrom, NestedTupleTryInto, neple,
        neplety,
    };
}

#![no_std]
#![doc = include_str!("../README.md")]

/// Re-exports of commonly used traits and types.
pub mod prelude {
    #[cfg(feature = "clone")]
    pub use tuplities_clone::TupleClone;
    #[cfg(feature = "copy")]
    pub use tuplities_copy::TupleCopy;
    #[cfg(feature = "debug")]
    pub use tuplities_debug::TupleDebug;
    #[cfg(feature = "tuple-default")]
    pub use tuplities_default::TupleDefault;
    #[cfg(feature = "eq")]
    pub use tuplities_eq::TupleEq;
    #[cfg(feature = "flatten-nest")]
    pub use tuplities_flatten_nest::{
        FlattenMatrixElements, FlattenNestedTuple, FlattenNestedTupleMatrix, IntoNestedTupleOption,
        NestMatrixElements, NestTuple, NestTupleMatrix, NestedTupleIndex, NestedTupleIndexMut,
        NestedTupleOption, NestedTuplePopBack, NestedTuplePopFront, NestedTuplePushBack,
        NestedTuplePushFront, NestedTupleReplicate, NestedTupleTryFrom, NestedTupleTryInto,
    };
    #[cfg(feature = "from")]
    pub use tuplities_from::{TupleFrom, TupleInto};
    #[cfg(feature = "hash")]
    pub use tuplities_hash::TupleHash;
    #[cfg(feature = "index")]
    pub use tuplities_index::{FirstTupleIndex, LastTupleIndex, TupleIndex, TupleIndexMut};
    #[cfg(feature = "insert")]
    pub use tuplities_insert::TupleInsert;
    #[cfg(feature = "len")]
    pub use tuplities_len::{PairTuple, SingletonTuple, TupleLen, UnitTuple};
    #[cfg(feature = "mut")]
    pub use tuplities_mut::TupleMut;
    #[cfg(feature = "mut")]
    pub use tuplities_mut::TupleMutMap;
    #[cfg(feature = "option")]
    pub use tuplities_option::{IntoTupleOption, TupleOption};
    #[cfg(feature = "ord")]
    pub use tuplities_ord::TupleOrd;
    #[cfg(feature = "partial-eq")]
    pub use tuplities_partial_eq::TuplePartialEq;
    #[cfg(feature = "partial-ord")]
    pub use tuplities_partial_ord::TuplePartialOrd;
    #[cfg(feature = "pop-back")]
    pub use tuplities_pop_back::{TupleMutBack, TuplePopBack, TupleRefBack};
    #[cfg(feature = "pop-front")]
    pub use tuplities_pop_front::{TupleMutFront, TuplePopFront, TupleRefFront};
    #[cfg(feature = "push-back")]
    pub use tuplities_push_back::TuplePushBack;
    #[cfg(feature = "push-front")]
    pub use tuplities_push_front::TuplePushFront;
    #[cfg(feature = "ref")]
    pub use tuplities_ref::TupleRef;
    #[cfg(feature = "ref")]
    pub use tuplities_ref::TupleRefMap;
    #[cfg(feature = "remove")]
    pub use tuplities_remove::TupleRemove;
    #[cfg(feature = "replicate")]
    pub use tuplities_replicate::TupleReplicate;
    #[cfg(feature = "reverse")]
    pub use tuplities_reverse::TupleReverse;
    #[cfg(feature = "row")]
    pub use tuplities_row::{FirstTupleRow, LastTupleRow, TupleRow, TupleRowMut};
    #[cfg(feature = "split")]
    pub use tuplities_split::TupleSplit;
    #[cfg(feature = "try-from")]
    pub use tuplities_try_from::{TupleTryFrom, TupleTryInto};
}

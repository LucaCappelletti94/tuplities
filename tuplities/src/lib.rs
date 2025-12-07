#![no_std]
#![doc = include_str!("../README.md")]

/// Re-exports of commonly used traits and types.
pub mod prelude {
    pub use tuplities_clone::TupleClone;
    pub use tuplities_copy::TupleCopy;
    pub use tuplities_debug::TupleDebug;
    pub use tuplities_default::TupleDefault;
    pub use tuplities_eq::TupleEq;
    pub use tuplities_from::{TupleFrom, TupleInto};
    pub use tuplities_hash::TupleHash;
    pub use tuplities_index::{FirstTupleIndex, LastTupleIndex, TupleIndex, TupleIndexMut};
    pub use tuplities_insert::TupleInsert;
    pub use tuplities_len::{PairTuple, SingletonTuple, TupleLen, UnitTuple};
    pub use tuplities_mut::TupleMut;
    pub use tuplities_option::{IntoTupleOption, TupleOption};
    pub use tuplities_ord::TupleOrd;
    pub use tuplities_partial_eq::TuplePartialEq;
    pub use tuplities_partial_ord::TuplePartialOrd;
    pub use tuplities_pop_back::{TupleMutBack, TuplePopBack, TupleRefBack};
    pub use tuplities_pop_front::{TupleMutFront, TuplePopFront, TupleRefFront};
    pub use tuplities_push_back::TuplePushBack;
    pub use tuplities_push_front::TuplePushFront;
    pub use tuplities_ref::TupleRef;
    pub use tuplities_remove::TupleRemove;
    pub use tuplities_replicate::TupleReplicate;
    pub use tuplities_reverse::TupleReverse;
    pub use tuplities_row::{FirstTupleRow, LastTupleRow, TupleRow, TupleRowMut};
    pub use tuplities_split::TupleSplit;
    pub use tuplities_try_from::{TupleTryFrom, TupleTryInto};
}

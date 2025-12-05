#![no_std]
#![doc = "../README.md"]

/// Re-exports of commonly used traits and types.
pub mod prelude {
    pub use tupilities_as_ref::TupleAsRef;
    pub use tupilities_clone::TupleClone;
    pub use tupilities_copy::TupleCopy;
    pub use tupilities_debug::TupleDebug;
    pub use tupilities_default::TupleDefault;
    pub use tupilities_eq::TupleEq;
    pub use tupilities_hash::{TupleHash, TupleSipHasher24};
    pub use tupilities_partial_eq::TuplePartialEq;
}

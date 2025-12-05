#![no_std]
#![doc = "../README.md"]

/// Re-exports of commonly used traits and types.
pub mod prelude {
    pub use tupilities_clone::TupleClone;
    pub use tupilities_copy::TupleCopy;
    pub use tupilities_debug::TupleDebug;
    pub use tupilities_default::TupleDefault;
    pub use tupilities_eq::TupleEq;
    pub use tupilities_hash::{TupleHash, TupleSipHasher24};
    pub use tupilities_mut::TupleMut;
    pub use tupilities_option::{IntoTupleOption, TupleOption};
    pub use tupilities_ord::TupleOrd;
    pub use tupilities_partial_eq::TuplePartialEq;
    pub use tupilities_partial_ord::TuplePartialOrd;
    pub use tupilities_pop_back::PopBack;
    pub use tupilities_pop_front::PopFront;
    pub use tupilities_push_back::PushBack;
    pub use tupilities_push_front::PushFront;
    pub use tupilities_ref::TupleRef;
}

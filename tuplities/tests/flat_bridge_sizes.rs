//! Flat bridge boundaries preserve element order and borrowing.
#![recursion_limit = "256"]

use tuplities::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

macro_rules! nested_values {
    ($head:expr) => { ($head,) };
    ($head:expr, $($tail:expr),+) => { ($head, nested_values!($($tail),+)) };
}

macro_rules! boundary_contract {
    ($name:ident; $($index:tt),+) => {
        #[test]
        fn $name() {
            let mut flat = ($($index + 1_000_usize,)+);
            {
                let references = flat.nest_mut().flatten();
                $(*references.$index += 1_000;)+
            }
            let nested = flat.nest();
            assert_eq!(nested, nested_values!($($index + 2_000_usize),+));
            let flat = nested.flatten();
            $(assert_eq!(flat.$index, $index + 2_000);)+
            let references = flat.nest_ref().flatten();
            $(assert!(core::ptr::eq(references.$index, &flat.$index));)+
        }
    };
}

boundary_contract!(width_8; 0, 1, 2, 3, 4, 5, 6, 7);

#[cfg(any(
    feature = "size-16",
    feature = "size-32",
    feature = "size-48",
    feature = "size-64",
    feature = "size-96",
    feature = "size-128"
))]
boundary_contract!(width_16; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

#[cfg(any(
    feature = "size-32",
    feature = "size-48",
    feature = "size-64",
    feature = "size-96",
    feature = "size-128"
))]
boundary_contract!(width_32; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);

#[cfg(any(
    feature = "size-48",
    feature = "size-64",
    feature = "size-96",
    feature = "size-128"
))]
boundary_contract!(width_48; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47);

#[cfg(any(feature = "size-64", feature = "size-96", feature = "size-128"))]
boundary_contract!(width_64; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

#[cfg(any(feature = "size-96", feature = "size-128"))]
boundary_contract!(width_96; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95);

#[cfg(feature = "size-128")]
boundary_contract!(width_128; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127);

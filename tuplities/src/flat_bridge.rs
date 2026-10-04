//! Flat tuple conversions up to the selected arity.

use crate::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

macro_rules! nested_type {
    ($head:ty) => { ($head,) };
    ($head:ty, $($tail:ty),+) => { ($head, nested_type!($($tail),+)) };
}

macro_rules! nested_value {
    ($head:expr) => { ($head,) };
    ($head:expr, $($tail:expr),+) => { ($head, nested_value!($($tail),+)) };
}

macro_rules! nested_pattern {
    ($head:ident) => { ($head,) };
    ($head:ident, $($tail:ident),+) => { ($head, nested_pattern!($($tail),+)) };
}

macro_rules! flat_bridge {
    ($($ty:ident $value:ident $index:tt),+) => {
        impl<$($ty),+> NestTuple for ($($ty,)+) {
            type Nested = nested_type!($($ty),+);

            #[inline]
            fn nest(self) -> Self::Nested {
                nested_value!($(self.$index),+)
            }
        }

        impl<$($ty),+> NestTupleRef for ($($ty,)+) {
            type NestedRef<'a> = nested_type!($(&'a $ty),+) where Self: 'a;

            #[inline]
            fn nest_ref(&self) -> Self::NestedRef<'_> {
                nested_value!($(&self.$index),+)
            }
        }

        impl<$($ty),+> NestTupleMut for ($($ty,)+) {
            type NestedMut<'a> = nested_type!($(&'a mut $ty),+) where Self: 'a;

            #[inline]
            fn nest_mut(&mut self) -> Self::NestedMut<'_> {
                nested_value!($(&mut self.$index),+)
            }
        }

        impl<$($ty),+> FlattenNestedTuple for nested_type!($($ty),+) {
            type Flattened = ($($ty,)+);

            #[inline]
            fn flatten(self) -> Self::Flattened {
                let nested_pattern!($($value),+) = self;
                ($($value,)+)
            }
        }
    };
}

macro_rules! bridge_prefixes {
    (($($ty:ident $value:ident $index:tt),*);) => {};
    (($($ty:ident $value:ident $index:tt),*);
        $next_ty:ident $next_value:ident $next_index:tt
        $(, $rest_ty:ident $rest_value:ident $rest_index:tt)*) => {
        flat_bridge!($($ty $value $index,)* $next_ty $next_value $next_index);
        bridge_prefixes!(
            ($($ty $value $index,)* $next_ty $next_value $next_index);
            $($rest_ty $rest_value $rest_index),*
        );
    };
}

bridge_prefixes!((T0 v0 0); T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7);

#[cfg(any(
    feature = "size-16",
    feature = "size-32",
    feature = "size-48",
    feature = "size-64",
    feature = "size-96",
    feature = "size-128"
))]
mod width_16 {
    use super::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

    bridge_prefixes!((T0 v0 0, T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7); T8 v8 8, T9 v9 9, T10 v10 10, T11 v11 11, T12 v12 12, T13 v13 13, T14 v14 14, T15 v15 15);
}

#[cfg(any(
    feature = "size-32",
    feature = "size-48",
    feature = "size-64",
    feature = "size-96",
    feature = "size-128"
))]
mod width_32 {
    use super::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

    bridge_prefixes!((T0 v0 0, T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7, T8 v8 8, T9 v9 9, T10 v10 10, T11 v11 11, T12 v12 12, T13 v13 13, T14 v14 14, T15 v15 15); T16 v16 16, T17 v17 17, T18 v18 18, T19 v19 19, T20 v20 20, T21 v21 21, T22 v22 22, T23 v23 23, T24 v24 24, T25 v25 25, T26 v26 26, T27 v27 27, T28 v28 28, T29 v29 29, T30 v30 30, T31 v31 31);
}

#[cfg(any(
    feature = "size-48",
    feature = "size-64",
    feature = "size-96",
    feature = "size-128"
))]
mod width_48 {
    use super::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

    bridge_prefixes!((T0 v0 0, T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7, T8 v8 8, T9 v9 9, T10 v10 10, T11 v11 11, T12 v12 12, T13 v13 13, T14 v14 14, T15 v15 15, T16 v16 16, T17 v17 17, T18 v18 18, T19 v19 19, T20 v20 20, T21 v21 21, T22 v22 22, T23 v23 23, T24 v24 24, T25 v25 25, T26 v26 26, T27 v27 27, T28 v28 28, T29 v29 29, T30 v30 30, T31 v31 31); T32 v32 32, T33 v33 33, T34 v34 34, T35 v35 35, T36 v36 36, T37 v37 37, T38 v38 38, T39 v39 39, T40 v40 40, T41 v41 41, T42 v42 42, T43 v43 43, T44 v44 44, T45 v45 45, T46 v46 46, T47 v47 47);
}

#[cfg(any(feature = "size-64", feature = "size-96", feature = "size-128"))]
mod width_64 {
    use super::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

    bridge_prefixes!((T0 v0 0, T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7, T8 v8 8, T9 v9 9, T10 v10 10, T11 v11 11, T12 v12 12, T13 v13 13, T14 v14 14, T15 v15 15, T16 v16 16, T17 v17 17, T18 v18 18, T19 v19 19, T20 v20 20, T21 v21 21, T22 v22 22, T23 v23 23, T24 v24 24, T25 v25 25, T26 v26 26, T27 v27 27, T28 v28 28, T29 v29 29, T30 v30 30, T31 v31 31, T32 v32 32, T33 v33 33, T34 v34 34, T35 v35 35, T36 v36 36, T37 v37 37, T38 v38 38, T39 v39 39, T40 v40 40, T41 v41 41, T42 v42 42, T43 v43 43, T44 v44 44, T45 v45 45, T46 v46 46, T47 v47 47); T48 v48 48, T49 v49 49, T50 v50 50, T51 v51 51, T52 v52 52, T53 v53 53, T54 v54 54, T55 v55 55, T56 v56 56, T57 v57 57, T58 v58 58, T59 v59 59, T60 v60 60, T61 v61 61, T62 v62 62, T63 v63 63);
}

#[cfg(any(feature = "size-96", feature = "size-128"))]
mod width_96 {
    use super::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

    bridge_prefixes!((T0 v0 0, T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7, T8 v8 8, T9 v9 9, T10 v10 10, T11 v11 11, T12 v12 12, T13 v13 13, T14 v14 14, T15 v15 15, T16 v16 16, T17 v17 17, T18 v18 18, T19 v19 19, T20 v20 20, T21 v21 21, T22 v22 22, T23 v23 23, T24 v24 24, T25 v25 25, T26 v26 26, T27 v27 27, T28 v28 28, T29 v29 29, T30 v30 30, T31 v31 31, T32 v32 32, T33 v33 33, T34 v34 34, T35 v35 35, T36 v36 36, T37 v37 37, T38 v38 38, T39 v39 39, T40 v40 40, T41 v41 41, T42 v42 42, T43 v43 43, T44 v44 44, T45 v45 45, T46 v46 46, T47 v47 47, T48 v48 48, T49 v49 49, T50 v50 50, T51 v51 51, T52 v52 52, T53 v53 53, T54 v54 54, T55 v55 55, T56 v56 56, T57 v57 57, T58 v58 58, T59 v59 59, T60 v60 60, T61 v61 61, T62 v62 62, T63 v63 63); T64 v64 64, T65 v65 65, T66 v66 66, T67 v67 67, T68 v68 68, T69 v69 69, T70 v70 70, T71 v71 71, T72 v72 72, T73 v73 73, T74 v74 74, T75 v75 75, T76 v76 76, T77 v77 77, T78 v78 78, T79 v79 79, T80 v80 80, T81 v81 81, T82 v82 82, T83 v83 83, T84 v84 84, T85 v85 85, T86 v86 86, T87 v87 87, T88 v88 88, T89 v89 89, T90 v90 90, T91 v91 91, T92 v92 92, T93 v93 93, T94 v94 94, T95 v95 95);
}

#[cfg(feature = "size-128")]
mod width_128 {
    use super::{FlattenNestedTuple, NestTuple, NestTupleMut, NestTupleRef};

    bridge_prefixes!((T0 v0 0, T1 v1 1, T2 v2 2, T3 v3 3, T4 v4 4, T5 v5 5, T6 v6 6, T7 v7 7, T8 v8 8, T9 v9 9, T10 v10 10, T11 v11 11, T12 v12 12, T13 v13 13, T14 v14 14, T15 v15 15, T16 v16 16, T17 v17 17, T18 v18 18, T19 v19 19, T20 v20 20, T21 v21 21, T22 v22 22, T23 v23 23, T24 v24 24, T25 v25 25, T26 v26 26, T27 v27 27, T28 v28 28, T29 v29 29, T30 v30 30, T31 v31 31, T32 v32 32, T33 v33 33, T34 v34 34, T35 v35 35, T36 v36 36, T37 v37 37, T38 v38 38, T39 v39 39, T40 v40 40, T41 v41 41, T42 v42 42, T43 v43 43, T44 v44 44, T45 v45 45, T46 v46 46, T47 v47 47, T48 v48 48, T49 v49 49, T50 v50 50, T51 v51 51, T52 v52 52, T53 v53 53, T54 v54 54, T55 v55 55, T56 v56 56, T57 v57 57, T58 v58 58, T59 v59 59, T60 v60 60, T61 v61 61, T62 v62 62, T63 v63 63, T64 v64 64, T65 v65 65, T66 v66 66, T67 v67 67, T68 v68 68, T69 v69 69, T70 v70 70, T71 v71 71, T72 v72 72, T73 v73 73, T74 v74 74, T75 v75 75, T76 v76 76, T77 v77 77, T78 v78 78, T79 v79 79, T80 v80 80, T81 v81 81, T82 v82 82, T83 v83 83, T84 v84 84, T85 v85 85, T86 v86 86, T87 v87 87, T88 v88 88, T89 v89 89, T90 v90 90, T91 v91 91, T92 v92 92, T93 v93 93, T94 v94 94, T95 v95 95); T96 v96 96, T97 v97 97, T98 v98 98, T99 v99 99, T100 v100 100, T101 v101 101, T102 v102 102, T103 v103 103, T104 v104 104, T105 v105 105, T106 v106 106, T107 v107 107, T108 v108 108, T109 v109 109, T110 v110 110, T111 v111 111, T112 v112 112, T113 v113 113, T114 v114 114, T115 v115 115, T116 v116 116, T117 v117 117, T118 v118 118, T119 v119 119, T120 v120 120, T121 v121 121, T122 v122 122, T123 v123 123, T124 v124 124, T125 v125 125, T126 v126 126, T127 v127 127);
}

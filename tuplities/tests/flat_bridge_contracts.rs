//! Flat and nested conversions preserve values and borrows.

use tuplities::{
    FlattenNestedTuple, FlattenNestedTupleMatrix, NestTuple, NestTupleMatrix, NestTupleMut,
    NestTupleRef,
};

#[test]
fn owned_round_trip_preserves_tuple_valued_heads() {
    let values = (String::from("head"), (3_u8, 5_u16), vec![7_u32, 11]);
    let nested = values.nest();
    assert_eq!(nested.0, "head");
    assert_eq!(nested.1.0, (3, 5));
    assert_eq!(nested.1.1.0, vec![7, 11]);
    let flat = nested.flatten();
    assert_eq!(flat, (String::from("head"), (3, 5), vec![7, 11]));
}

#[test]
fn borrowed_bridges_keep_the_original_storage() {
    let mut flat = (String::from("head"), vec![2_u8, 3], 5_u16);
    let references = flat.nest_ref().flatten();
    assert!(core::ptr::eq(references.0, &raw const flat.0));
    assert!(core::ptr::eq(references.1, &raw const flat.1));
    {
        let nested = flat.nest_mut();
        nested.0.push('!');
        nested.1.0.push(7);
        *nested.1.1.0 = 11;
    }
    assert_eq!(flat, (String::from("head!"), vec![2, 3, 7], 11));
}

#[test]
fn matrix_round_trip_preserves_rows_and_columns() {
    let matrix = ((String::from("a"), 2_u8), (String::from("b"), 3_u8));
    let nested = matrix.nest_matrix();
    assert_eq!(nested.0.0, "a");
    assert_eq!(nested.0.1.0, 2);
    assert_eq!(nested.1.0.0, "b");
    assert_eq!(nested.1.0.1.0, 3);
    let flat = nested.flatten_matrix();
    assert_eq!(flat, ((String::from("a"), 2), (String::from("b"), 3)));
}

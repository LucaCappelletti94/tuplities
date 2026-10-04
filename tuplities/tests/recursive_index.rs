//! Recursive indexing is independent of flat tuple conversion limits.

use tuplities::{
    NestedTuple, NestedTupleIndex, NestedTupleIndexMut, NestedTupleRow, NestedTupleRowMut,
};
use typenum::{U0, U1, U2, U4, U8, U9};

#[test]
fn recursive_index_exceeds_flat_limit() {
    let mut values = (1_u8, (2, (3, (4, (5, (6, (7, (8, (9, (10,))))))))));
    assert_eq!(*NestedTupleIndex::<U8>::nested_index(&values), 9);
    *NestedTupleIndexMut::<U0>::nested_index_mut(&mut values) = 11;
    *NestedTupleIndexMut::<U4>::nested_index_mut(&mut values) = 55;
    *NestedTupleIndexMut::<U9>::nested_index_mut(&mut values) = 100;
    assert_eq!(
        values,
        (11, (2, (3, (4, (55, (6, (7, (8, (9, (100,))))))))))
    );
}

#[test]
fn length_counts_heads_and_both_termination_forms() {
    fn length<T: NestedTuple>(_: &T) -> usize {
        T::LEN
    }

    assert_eq!(length(&()), 0);
    assert_eq!(length(&(String::from("singleton"),)), 1);
    let mut singleton_terminated = ((1_u8, 2_u16), (String::from("head"), (true,)));
    assert_eq!(length(&singleton_terminated), 3);
    assert_eq!(length(&&singleton_terminated), 3);
    assert_eq!(length(&&mut singleton_terminated), 3);
    assert_eq!(
        length(&((1_u8, 2_u16), (String::from("head"), (true, ())))),
        3
    );
}

#[test]
fn unit_terminated_indices_preserve_unselected_elements() {
    let mut values = (String::from("head"), (2_u16, (String::from("tail"), ())));
    assert_eq!(NestedTupleIndex::<U0>::nested_index(&values), "head");
    assert_eq!(*NestedTupleIndex::<U1>::nested_index(&values), 2);
    assert_eq!(NestedTupleIndex::<U2>::nested_index(&values), "tail");
    NestedTupleIndexMut::<U2>::nested_index_mut(&mut values).push('!');
    assert_eq!(
        values,
        (String::from("head"), (2, (String::from("tail!"), ())))
    );
}

#[test]
fn indexing_borrowed_tails_mutates_original_storage() {
    let mut tail = (String::from("middle"), (String::from("tail"), ()));
    {
        let mut values = (1_u8, &tail);
        assert!(core::ptr::eq(
            NestedTupleIndex::<U1>::nested_index(&values),
            &raw const tail.0,
        ));
        assert_eq!(NestedTupleIndex::<U2>::nested_index(&values), "tail");
        *NestedTupleIndexMut::<U0>::nested_index_mut(&mut values) = 2;
        assert_eq!(values.0, 2);
    }
    {
        let mut values = (1_u8, &mut tail);
        NestedTupleIndexMut::<U1>::nested_index_mut(&mut values).push('!');
        NestedTupleIndexMut::<U2>::nested_index_mut(&mut values).push('?');
        assert_eq!(*NestedTupleIndex::<U0>::nested_index(&values), 1);
    }
    assert_eq!(tail, (String::from("middle!"), (String::from("tail?"), ())));
}

#[test]
fn row_access_exceeds_flat_limit_for_both_inner_terminations() {
    let first = (1_u8, (2, (3, (4, (5, (6, (7, (8, (9, (10,))))))))));
    let second = (
        11_u16,
        (12, (13, (14, (15, (16, (17, (18, (19, (20, ()))))))))),
    );
    let mut matrix = (first, (second, ()));
    let row = NestedTupleRow::<U8>::nested_tuple_row(&matrix);
    assert_eq!(row, (&9, (&19, ())));
    {
        let row = NestedTupleRowMut::<U8>::nested_tuple_row_mut(&mut matrix);
        *row.0 = 99;
        *row.1.0 = 199;
    }
    assert_eq!(
        matrix,
        (
            (1, (2, (3, (4, (5, (6, (7, (8, (99, (10,)))))))))),
            (
                (
                    11,
                    (12, (13, (14, (15, (16, (17, (18, (199, (20, ())))))))))
                ),
                ()
            )
        ),
    );
}

#[test]
fn row_access_through_borrowed_outer_tail_updates_original_matrix() {
    let mut tail = ((30_u16, (40_u16, ())), ());
    {
        let mut matrix = ((10_u8, (20_u8, ())), &mut tail);
        let row = NestedTupleRow::<U1>::nested_tuple_row(&matrix);
        assert_eq!(row, (&20, (&40, ())));
        let row = NestedTupleRowMut::<U1>::nested_tuple_row_mut(&mut matrix);
        *row.0 = 21;
        *row.1.0 = 41;
        assert_eq!(matrix.0, (10, (21, ())));
    }
    assert_eq!(tail, ((30, (41, ())), ()));
    let matrix = ((10_u8, (20_u8, ())), &tail);
    assert_eq!(
        NestedTupleRow::<U1>::nested_tuple_row(&matrix),
        (&20, (&41, ()))
    );
}

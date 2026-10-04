//! Consuming operations preserve elements, ownership, and termination contracts.

use core::cell::Cell;
use tuplities::prelude::*;
use typenum::{U0, U1, U2, U3};

#[test]
fn split_preserves_suffixes_and_normalizes_prefixes() {
    assert_eq!(NestedTupleSplit::<U0>::nested_split(()), ((), ()));
    let result: ((), neplety!(u8)) = NestedTupleSplit::<U0>::nested_split(neple!(7_u8));
    assert_eq!(result, ((), (7,)));
    let result: (neplety!(u8), ()) = NestedTupleSplit::<U1>::nested_split(neple!(7_u8));
    assert_eq!(result, ((7,), ()));
    let original = (1_u8, (2_u16, (3_u32, ())));
    let (prefix, suffix): (neplety!(u8), (u16, (u32, ()))) =
        NestedTupleSplit::<U1>::nested_split(original);
    assert_eq!((prefix, suffix), ((1,), (2, (3, ()))));
    assert_eq!(prefix.chain(suffix), original);
    let result: (neplety!(u8, u16), neplety!(u32)) =
        NestedTupleSplit::<U2>::nested_split(neple!(1_u8, 2_u16, 3_u32));
    assert_eq!(result, ((1, (2,)), (3,)));
    let result: (neplety!(u8, u16, u32), ()) = NestedTupleSplit::<U3>::nested_split(original);
    assert_eq!(result, (neple!(1, 2, 3), ()));
}

#[test]
fn insert_accepts_every_boundary_and_preserves_untouched_termination() {
    assert_eq!(NestedTupleInsert::<U0, _>::nested_insert((), 7_u8), (7,));
    assert_eq!(
        NestedTupleInsert::<U0, _>::nested_insert(neple!(2_u16), 1_u8),
        neple!(1, 2)
    );
    assert_eq!(
        NestedTupleInsert::<U1, _>::nested_insert(neple!(1_u8), 2_u16),
        neple!(1, 2)
    );
    let original = (1_u8, (2_u16, (3_u32, ())));
    assert_eq!(
        NestedTupleInsert::<U0, _>::nested_insert(original, false),
        (false, original)
    );
    assert_eq!(
        NestedTupleInsert::<U1, _>::nested_insert(original, (4_u8, 5_u16)),
        (1, ((4, 5), (2, (3, ()))))
    );
    assert_eq!(
        NestedTupleInsert::<U2, _>::nested_insert(original, false),
        (1, (2, (false, (3, ()))))
    );
    let appended: neplety!(u8, u16, u32, bool) =
        NestedTupleInsert::<U3, _>::nested_insert(original, true);
    assert_eq!(appended, neple!(1, 2, 3, true));
}

#[test]
fn remove_preserves_suffixes_and_normalizes_empty_remainders() {
    assert_eq!(
        NestedTupleRemove::<U0>::nested_remove(neple!(7_u8)),
        (7, ())
    );
    assert_eq!(NestedTupleRemove::<U0>::nested_remove((7_u8, ())), (7, ()));
    let original = (1_u8, (2_u16, (3_u32, ())));
    assert_eq!(
        NestedTupleRemove::<U0>::nested_remove(original),
        (1, (2, (3, ())))
    );
    assert_eq!(
        NestedTupleRemove::<U1>::nested_remove(original),
        (2, (1, (3, ())))
    );
    let result: (u32, neplety!(u8, u16)) = NestedTupleRemove::<U2>::nested_remove(original);
    assert_eq!(result, (3, neple!(1, 2)));
    assert_eq!(
        NestedTupleRemove::<U0>::nested_remove(neple!((1_u8, 2_u16), true)),
        ((1, 2), (true,))
    );
}

#[test]
fn reverse_normalizes_both_terminations_without_flattening_heads() {
    assert_eq!(NestedTupleReverse::nested_reverse(neple!(7_u8)), (7,));
    assert_eq!(NestedTupleReverse::nested_reverse((7_u8, ())), (7,));
    let original = ((1_u8, 2_u16), (3_u32, (false, ())));
    let reversed: neplety!(bool, u32, (u8, u16)) = NestedTupleReverse::nested_reverse(original);
    assert_eq!(reversed, neple!(false, 3, (1, 2)));
    assert_eq!(reversed.nested_reverse(), neple!((1, 2), 3, false));
    assert_eq!(
        NestedTupleReverse::nested_reverse(neple!(1_u8, 2_u16, 3_u32)),
        neple!(3, 2, 1)
    );
}

#[test]
fn borrowed_suffixes_remain_untouched_at_owned_boundaries() {
    let tail = neple!(String::from("tail"), false);
    let (prefix, suffix) = NestedTupleSplit::<U1>::nested_split((1_u8, &tail));
    assert_eq!(prefix, (1,));
    assert!(core::ptr::eq(suffix, &raw const tail));
    let inserted = NestedTupleInsert::<U1, _>::nested_insert((1_u8, &tail), 2_u16);
    assert_eq!((inserted.0, inserted.1.0), (1, 2));
    assert!(core::ptr::eq(inserted.1.1, &raw const tail));
    let (removed, remainder) = NestedTupleRemove::<U0>::nested_remove((1_u8, &tail));
    assert_eq!(removed, 1);
    assert!(core::ptr::eq(remainder, &raw const tail));
    let empty = ();
    let result: (_, (u8,)) = NestedTupleRemove::<U1>::nested_remove((1_u8, (2_u16, &empty)));
    assert_eq!(result, (2, (1,)));
    assert_eq!(NestedTupleReverse::nested_reverse((1_u8, &empty)), (1,));
}

#[test]
fn reference_elements_use_the_same_consuming_operations() {
    let mut values = neple!(String::from("a"), String::from("b"), String::from("c"));
    let mut extra = String::from("new");
    {
        let (prefix, suffix) = NestedTupleSplit::<U1>::nested_split(values.nested_tuple_mut());
        let joined = prefix.chain(suffix);
        let inserted = NestedTupleInsert::<U1, _>::nested_insert(joined, &mut extra);
        let (removed, remainder) = NestedTupleRemove::<U2>::nested_remove(inserted);
        removed.push('?');
        let reversed = remainder.nested_reverse();
        reversed.0.push('#');
        reversed.1.0.push('+');
        reversed.1.1.0.push('!');
    }
    assert_eq!(
        (
            values.0.as_str(),
            values.1.0.as_str(),
            values.1.1.0.as_str(),
            extra.as_str()
        ),
        ("a!", "b?", "c#", "new+")
    );
    let shared = values.nested_tuple_ref().nested_reverse();
    assert_eq!(
        (
            shared.0.as_str(),
            shared.1.0.as_str(),
            shared.1.1.0.as_str()
        ),
        ("c#", "b?", "a!")
    );
}

#[derive(Debug)]
struct Tracked<'a> {
    id: u8,
    drops: &'a Cell<u8>,
}

impl Drop for Tracked<'_> {
    fn drop(&mut self) {
        self.drops.set(self.drops.get().saturating_add(1));
    }
}

#[test]
fn all_operations_transfer_noncopy_elements_and_drop_each_once() {
    let drops: [Cell<u8>; 4] = core::array::from_fn(|_| Cell::new(0));
    {
        let source = neple!(
            Tracked {
                id: 0,
                drops: &drops[0]
            },
            Tracked {
                id: 1,
                drops: &drops[1]
            },
            Tracked {
                id: 2,
                drops: &drops[2]
            }
        );
        let (prefix, suffix) = NestedTupleSplit::<U1>::nested_split(source);
        let inserted = NestedTupleInsert::<U2, _>::nested_insert(
            prefix.chain(suffix),
            Tracked {
                id: 3,
                drops: &drops[3],
            },
        );
        let (removed, remainder) = NestedTupleRemove::<U1>::nested_remove(inserted);
        let reversed = remainder.nested_reverse();
        assert_eq!(removed.id, 1);
        assert_eq!(
            (reversed.0.id, reversed.1.0.id, reversed.1.1.0.id),
            (2, 3, 0)
        );
        assert_eq!(drops.each_ref().map(Cell::get), [0, 0, 0, 0]);
        drop(removed);
        assert_eq!(drops.each_ref().map(Cell::get), [0, 1, 0, 0]);
    }
    assert_eq!(drops.each_ref().map(Cell::get), [1, 1, 1, 1]);
}

#[test]
fn mutable_borrowed_suffixes_preserve_storage_without_traversal() {
    let mut tail = neple!(String::from("head"), String::from("tail"));
    {
        let (prefix, suffix) = NestedTupleSplit::<U1>::nested_split((1_u8, &mut tail));
        assert_eq!(prefix, (1,));
        suffix.0.push('!');
        let inserted = NestedTupleInsert::<U1, _>::nested_insert((prefix.0, suffix), 2_u16);
        inserted.1.1.1.0.push('?');
        let (removed, remainder) = NestedTupleRemove::<U1>::nested_remove(inserted);
        assert_eq!(removed, 2);
        assert_eq!(remainder.0, 1);
    }
    assert_eq!((tail.0.as_str(), tail.1.0.as_str()), ("head!", "tail?"));
    let mut empty = ();
    let remainder: (u8,) = NestedTupleRemove::<U1>::nested_remove((1_u8, (2_u16, &mut empty))).1;
    assert_eq!(remainder, (1,));
    assert_eq!(NestedTupleReverse::nested_reverse((3_u8, &mut empty)), (3,));
}

//! Construction preserves element boundaries, ownership, and evaluation order.

use tuplities::{NestedTuple, NestedTupleChain, neple as list, neplety as List};

#[derive(Debug, PartialEq)]
struct Owned(u8);

#[test]
fn empty_and_singleton_inputs_have_canonical_termination() {
    type Empty = List!();
    type Singleton = List!(Owned,);
    assert_eq!(Empty::LEN, 0);
    assert_eq!(Singleton::LEN, 1);
    let original = Owned(7);
    let values: Singleton = list!().chain(list!(original,));
    assert_eq!(values, (Owned(7),));
    let singleton: List!(Owned) = list!(Owned(8));
    assert_eq!(singleton, (Owned(8),));
}

#[test]
fn expressions_are_moved_once_in_left_to_right_order() {
    let mut calls = 0;
    let mut next = || {
        calls += 1;
        Owned(calls)
    };
    let values = list!(next(), next(), next(),);
    assert_eq!(values, (Owned(1), (Owned(2), (Owned(3),))));
    assert_eq!(calls, 3);
}

#[test]
fn complex_types_and_tuple_heads_keep_element_boundaries() {
    type Values = List!((u8, u16), Option<Result<u8, u16>>, [u8; 2],);
    let values: Values = list!((1, 2), Some(Ok(3)), [4, 5]);
    assert_eq!(values, ((1, 2), (Some(Ok(3)), ([4, 5],))));
    assert_eq!(Values::LEN, 3);
}

#[test]
fn borrowed_heads_preserve_lifetimes_and_mutate_original_storage() {
    fn borrow(bytes: &mut [u8; 2]) -> List!(&mut [u8; 2], fn(u8, u8) -> u8) {
        list!(bytes, u8::wrapping_add as fn(u8, u8) -> u8)
    }

    let mut bytes = [250, 10];
    {
        let values = borrow(&mut bytes);
        values.0[0] = values.1.0(values.0[0], values.0[1]);
    }
    assert_eq!(bytes, [4, 10]);
}

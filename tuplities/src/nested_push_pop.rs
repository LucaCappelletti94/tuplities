//! Module providing pop/push traits for nested tuples.

use crate::NestedTuple;

/// A trait for popping the front element of a nested tuple.
pub trait NestedTuplePopFront {
    /// The type of the first element.
    type Front;

    /// The type of the nested tuple after removing the first element.
    type Tail;

    /// Consume the nested tuple and return the first element and the remaining nested tuple.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestedTuplePopFront;
    ///
    /// let (front, tail) = (1, (2, (3,))).nested_pop_front();
    /// assert_eq!(front, 1);
    /// assert_eq!(tail, (2, (3,)));
    ///
    /// let shared = (1, (2, (3,)));
    /// let (shared_front, shared_tail) = (&shared).nested_pop_front();
    /// assert_eq!(*shared_front, 1);
    /// assert_eq!(*shared_tail, (2, (3,)));
    /// assert_eq!(shared, (1, (2, (3,))));
    /// ```
    fn nested_pop_front(self) -> (Self::Front, Self::Tail);
}

/// A trait for popping the back element of a nested tuple.
pub trait NestedTuplePopBack {
    /// The type of the tuple without the last element.
    type Init;

    /// The type of the last element.
    type Back;

    /// Consume the nested tuple and return the remaining nested tuple and the last element.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestedTuplePopBack;
    ///
    /// let (init, back) = (1, (2, (3,))).nested_pop_back();
    /// assert_eq!(init, (1, (2,)));
    /// assert_eq!(back, 3);
    /// ```
    fn nested_pop_back(self) -> (Self::Init, Self::Back);
}

/// A trait for pushing an element to the front of a nested tuple.
pub trait NestedTuplePushFront<Item> {
    /// The result type after pushing the item to the front.
    type Output;

    /// Push an element to the front.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestedTuplePushFront;
    ///
    /// let pushed = ().nested_push_front(1);
    /// assert_eq!(pushed, (1,));
    ///
    /// let pushed = (2, (3,)).nested_push_front(1);
    /// assert_eq!(pushed, (1, (2, (3,))));
    /// ```
    fn nested_push_front(self, item: Item) -> Self::Output;
}

/// A trait for pushing an element to the back of a nested tuple.
pub trait NestedTuplePushBack<Item> {
    /// The result type after pushing the item to the back.
    type Output;

    /// Push an element to the back.
    ///
    /// # Examples
    ///
    /// ```
    /// use tuplities::NestedTuplePushBack;
    ///
    /// let pushed = ().nested_push_back(1);
    /// assert_eq!(pushed, (1,));
    ///
    /// let pushed = (1, (2, (3,))).nested_push_back(4);
    /// assert_eq!(pushed, (1, (2, (3, (4,)))));
    /// ```
    fn nested_push_back(self, item: Item) -> Self::Output;
}

// Implementations
impl<Head> NestedTuplePopFront for (Head,) {
    type Front = Head;
    type Tail = ();

    #[inline]
    fn nested_pop_front(self) -> (Self::Front, Self::Tail) {
        let (head,) = self;
        (head, ())
    }
}

// Implementations
impl<'a, Head> NestedTuplePopFront for &'a (Head,) {
    type Front = &'a Head;
    type Tail = ();

    #[inline]
    fn nested_pop_front(self) -> (Self::Front, Self::Tail) {
        let (head,) = self;
        (head, ())
    }
}

impl<'a, Head> NestedTuplePopFront for &'a mut (Head,) {
    type Front = &'a mut Head;
    type Tail = ();

    #[inline]
    fn nested_pop_front(self) -> (Self::Front, Self::Tail) {
        let (head,) = self;
        (head, ())
    }
}

impl<Head, Tail> NestedTuplePopFront for (Head, Tail) {
    type Front = Head;
    type Tail = Tail;

    #[inline]
    fn nested_pop_front(self) -> (Self::Front, Self::Tail) {
        let (head, tail) = self;
        (head, tail)
    }
}

impl<'a, Head, Tail> NestedTuplePopFront for &'a (Head, Tail) {
    type Front = &'a Head;
    type Tail = &'a Tail;

    #[inline]
    fn nested_pop_front(self) -> (Self::Front, Self::Tail) {
        let (head, tail) = self;
        (head, tail)
    }
}

impl<'a, Head, Tail> NestedTuplePopFront for &'a mut (Head, Tail) {
    type Front = &'a mut Head;
    type Tail = &'a mut Tail;

    #[inline]
    fn nested_pop_front(self) -> (Self::Front, Self::Tail) {
        let (head, tail) = self;
        (head, tail)
    }
}

impl<Head> NestedTuplePopBack for (Head,) {
    type Init = ();
    type Back = Head;

    #[inline]
    fn nested_pop_back(self) -> (Self::Init, Self::Back) {
        let (head,) = self;
        ((), head)
    }
}

impl<Head, Tail> NestedTuplePopBack for (Head, Tail)
where
    Tail: NestedTuplePopBack,
    <Tail as NestedTuplePopBack>::Init: NestedTuplePushFront<Head>,
{
    type Init = <<Tail as NestedTuplePopBack>::Init as NestedTuplePushFront<Head>>::Output;
    type Back = <Tail as NestedTuplePopBack>::Back;

    #[inline]
    fn nested_pop_back(self) -> (Self::Init, Self::Back) {
        let (head, tail) = self;
        let (init, back) = tail.nested_pop_back();
        (init.nested_push_front(head), back)
    }
}

impl<Item> NestedTuplePushFront<Item> for () {
    type Output = (Item,);

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item,)
    }
}

impl<Item, Head> NestedTuplePushFront<Item> for (Head,) {
    type Output = (Item, (Head,));

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item, self)
    }
}

impl<Item, Head, Tail> NestedTuplePushFront<Item> for (Head, Tail) {
    type Output = (Item, (Head, Tail));

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item, self)
    }
}

impl<Item> NestedTuplePushFront<Item> for &() {
    type Output = (Item,);

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item,)
    }
}

impl<Item> NestedTuplePushFront<Item> for &mut () {
    type Output = (Item,);

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item,)
    }
}

impl<'a, Item, Head> NestedTuplePushFront<Item> for &'a (Head,) {
    type Output = (Item, &'a (Head,));

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item, self)
    }
}

impl<'a, Item, Head> NestedTuplePushFront<Item> for &'a mut (Head,) {
    type Output = (Item, &'a mut (Head,));

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item, self)
    }
}

impl<'a, Item, Head, Tail: NestedTuple> NestedTuplePushFront<Item> for &'a (Head, Tail) {
    type Output = (Item, &'a (Head, Tail));

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item, self)
    }
}

impl<'a, Item, Head, Tail: NestedTuple> NestedTuplePushFront<Item> for &'a mut (Head, Tail) {
    type Output = (Item, &'a mut (Head, Tail));

    #[inline]
    fn nested_push_front(self, item: Item) -> Self::Output {
        (item, self)
    }
}

impl<Item> NestedTuplePushBack<Item> for () {
    type Output = (Item,);

    #[inline]
    fn nested_push_back(self, item: Item) -> Self::Output {
        (item,)
    }
}

impl<Item, Head> NestedTuplePushBack<Item> for (Head,) {
    type Output = (Head, (Item,));

    #[inline]
    fn nested_push_back(self, item: Item) -> Self::Output {
        let (head,) = self;
        (head, (item,))
    }
}

impl<Item, Head, Tail> NestedTuplePushBack<Item> for (Head, Tail)
where
    Tail: NestedTuplePushBack<Item>,
{
    type Output = (Head, <Tail as NestedTuplePushBack<Item>>::Output);

    #[inline]
    fn nested_push_back(self, item: Item) -> Self::Output {
        let (head, tail) = self;
        (head, tail.nested_push_back(item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_pop_front_single() {
        let nested = (42,);
        let (front, tail) = nested.nested_pop_front();
        assert_eq!(front, 42);
        assert_eq!(tail, ());
    }

    #[test]
    fn test_nested_pop_front_nested() {
        let nested = (1, (2, (3,)));
        let (front, tail) = nested.nested_pop_front();
        assert_eq!(front, 1);
        assert_eq!(tail, (2, (3,)));
    }

    #[test]
    fn test_nested_pop_back_single() {
        let nested = (42,);
        let (init, back) = nested.nested_pop_back();
        assert_eq!(init, ());
        assert_eq!(back, 42);
    }

    #[test]
    fn test_nested_pop_back_nested() {
        let nested = (1, (2, (3,)));
        let (init, back) = nested.nested_pop_back();
        assert_eq!(init, (1, (2,)));
        assert_eq!(back, 3);
    }

    #[test]
    fn test_nested_push_front_empty() {
        let tail: () = ();
        let pushed: (i32,) = tail.nested_push_front(1i32);
        assert_eq!(pushed, (1,));
    }

    #[test]
    fn test_nested_push_front_nested() {
        let tail = (2, (3,));
        let pushed = tail.nested_push_front(1);
        assert_eq!(pushed, (1, (2, (3,))));
    }

    #[test]
    fn test_nested_push_back_empty() {
        let tail: () = ();
        let pushed: (i32,) = tail.nested_push_back(1i32);
        assert_eq!(pushed, (1,));
    }

    #[test]
    fn test_nested_push_back_nested() {
        let nested = (1, (2, (3,)));
        let pushed = nested.nested_push_back(4);
        assert_eq!(pushed, (1, (2, (3, (4,)))));
    }
}

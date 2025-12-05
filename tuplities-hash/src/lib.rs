#![no_std]

//! [tuplities](https://github.com/lucacappelletti94/tuplities) suite crate providing the `TupleHash` and `TupleSipHasher24` traits.

#[tuplities_derive::impl_tuple_hash]
/// A trait for hashing tuples with a generic hasher.
pub trait TupleHash {
    /// Hashes the tuple into the given hasher.
    fn tuple_hash<H: core::hash::Hasher>(&self, state: &mut H);
}

/// A trait for hashing tuples with SipHasher24.
pub trait TupleSipHasher24: TupleHash {
    /// Returns the hash value of the tuple using SipHasher24.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tuplities_hash::TupleSipHasher24;
    ///
    /// let tuple = (1, "hello", 42);
    /// let hash_value = tuple.tuple_sip_hash();
    ///
    /// // The hash value is deterministic for the same input
    /// assert_eq!(tuple.tuple_sip_hash(), hash_value);
    /// ```
    fn tuple_sip_hash(&self) -> u64 {
        use core::hash::Hasher;
        use siphasher::sip::SipHasher24;

        let mut hasher = SipHasher24::new();
        self.tuple_hash(&mut hasher);
        hasher.finish()
    }
}

impl<T: TupleHash + ?Sized> TupleSipHasher24 for T {}

//! [`Serialize`] and [`Deserialize`] implementations for [`ChunkedVec`].
//!
//! `ChunkedVec<T, N>` is serialized as a plain sequence of its elements,
//! exactly like `Vec<T>`, so the two are interchangeable on the wire. The
//! chunk size `N` is a memory-layout detail and is not part of the format.

use core::fmt;
use core::marker::PhantomData;

use serde::de::{Deserialize, Deserializer, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeSeq, Serializer};

use crate::{ChunkedVec, ChunkedVecSized};

/// Serializes the `ChunkedVec` as a sequence of its elements.
///
/// # Examples
/// ```
/// use chunked_vec::chunked_vec;
///
/// let vec = chunked_vec![1, 2, 3];
/// assert_eq!(serde_json::to_string(&vec).unwrap(), "[1,2,3]");
/// ```
impl<T, const N: usize> Serialize for ChunkedVec<T, N>
where
    T: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for elem in self.iter() {
            seq.serialize_element(elem)?;
        }
        seq.end()
    }
}

/// Deserializes a sequence of elements into a `ChunkedVec`.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
///
/// let vec: ChunkedVec<i32> = serde_json::from_str("[1,2,3]").unwrap();
/// assert_eq!(vec, [1, 2, 3]);
/// ```
impl<'de, T, const N: usize> Deserialize<'de> for ChunkedVec<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ChunkedVecVisitor<T, const N: usize>(PhantomData<T>);

        impl<'de, T, const N: usize> Visitor<'de> for ChunkedVecVisitor<T, N>
        where
            T: Deserialize<'de>,
        {
            type Value = ChunkedVec<T, N>;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a sequence")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                // size_hint comes from the input data (e.g. bincode's length
                // prefix), so corrupt or malicious input can claim an absurdly
                // large length. Cap what we preallocate from it; a genuinely
                // longer sequence still grows normally via push.
                const PREALLOC_CAP: usize = 4096;
                let cap = seq.size_hint().unwrap_or(0).min(PREALLOC_CAP);
                let mut vec = ChunkedVecSized::<T, N>::with_capacity(cap);
                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }
                Ok(vec)
            }
        }

        deserializer.deserialize_seq(ChunkedVecVisitor(PhantomData))
    }
}

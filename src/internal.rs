use std::array::from_fn;
use std::mem::MaybeUninit;
use crate::{Chunk, ChunkedVec};

impl<T, const N: usize> ChunkedVec<T, N> {
    pub(crate) fn create_new_chunk(value: T) -> Chunk<T, N> {
        let arr: [MaybeUninit<T>; N] = from_fn(|_| MaybeUninit::uninit());
        let mut chunk: Chunk<T, N> = Box::new(arr);
        chunk[0].write(value);
        chunk
    }
}

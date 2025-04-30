use crate::{Chunk, ChunkedVec};

impl<T, const N: usize> ChunkedVec<T, N> {
    pub(crate) fn create_new_chunk(value: T) -> Chunk<T, N> {
        let mut chunk = Box::new_uninit_slice(N);
        chunk[0].write(value);
        unsafe {
            let ptr = Box::into_raw(chunk) as *mut [T; N];
            Box::from_raw(ptr)
        }
    }
}

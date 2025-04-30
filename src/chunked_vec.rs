/// A vector-like container that stores elements in fixed-size chunks.
///
/// Type Parameters:
/// - `T`: The type of elements to store
/// - `N`: The size of each chunk (default: 64)
pub struct ChunkedVec<T, const N: usize = { crate::DEFAULT_CHUNK_SIZE }> {
    pub(crate) data: Vec<Chunk<T, N>>,
    pub(crate) len: usize,
}

pub struct ChunkedVecSized<T, const N: usize>(std::marker::PhantomData<T>);

pub type Chunk<T, const N: usize = { crate::DEFAULT_CHUNK_SIZE }> = Box<[T; N]>;


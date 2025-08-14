use std::mem::MaybeUninit;

/// A vector-like container that stores elements in fixed-size chunks, providing efficient
/// memory allocation and element access.
///
/// `ChunkedVec` is designed to balance memory usage and performance by storing elements
/// in fixed-size chunks rather than a single contiguous array. This approach can reduce
/// memory fragmentation and improve performance for certain operations.
///
/// # Type Parameters
/// - `T`: The type of elements to store. Can be any type that satisfies the required trait bounds.
/// - `N`: The size of each chunk (default: 64). This constant determines how many elements
///        are stored in each internal chunk. Larger chunks may improve cache locality but
///        increase memory overhead for partially filled chunks.
///
/// # Internal Structure
/// - Elements are stored in a series of fixed-size chunks, each containing exactly `N` elements
/// - The chunks are managed by a `Vec<Chunk<T, N>>`, where each `Chunk` is a boxed array
/// - The total number of elements is tracked separately from the chunk storage
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
///
/// // Create a new ChunkedVec with default chunk size
/// let mut vec = ChunkedVec::new();
///
/// // Add elements
/// vec.push(1);
/// vec.push(2);
///
/// // Access elements
/// assert_eq!(vec[0], 1);
/// assert_eq!(vec[1], 2);
/// assert_eq!(vec.len(), 2);
/// ```
#[derive(Debug)]
pub struct ChunkedVec<T, const N: usize = { crate::DEFAULT_CHUNK_SIZE }> {
    pub(crate) data: Vec<Chunk<T, N>>,
    pub(crate) len: usize,
}

/// A marker type used for compile-time chunk size validation.
///
/// This type is used internally to ensure that chunk sizes are valid at compile time.
pub struct ChunkedVecSized<T, const N: usize>(std::marker::PhantomData<T>);

/// A fixed-size chunk type used for storing elements in `ChunkedVec`.
///
/// Each chunk is a boxed array of exactly `N` elements, where `N` is the chunk size.
/// Using `Box` helps reduce stack pressure when chunk sizes are large.
pub type Chunk<T, const N: usize = { crate::DEFAULT_CHUNK_SIZE }> = Box<[MaybeUninit<T>; N]>;

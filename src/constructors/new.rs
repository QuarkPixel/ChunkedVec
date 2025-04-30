use crate::{ChunkedVec, ChunkedVecSized};

/// Implementation of creation methods for ChunkedVec with fixed chunk size.
///
/// This implementation provides methods to create ChunkedVec instances with a compile-time fixed chunk size.
/// The chunk size is specified through the type parameter `N` and cannot be changed after creation.
impl<T, const N: usize> ChunkedVecSized<T, N> {
    /// Creates a new empty `ChunkedVec` with a fixed chunk size of `N`.
    ///
    /// The chunk size `N` determines how many elements are stored in each internal chunk.
    /// This size is fixed at compile-time and provides optimal performance for scenarios
    /// where the chunk size is known in advance.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::{ChunkedVecSized, ChunkedVec};
    /// let vec: ChunkedVec<i32, 8> = ChunkedVecSized::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> ChunkedVec<T, N> {
        ChunkedVec {
            data: Vec::new(),
            len: 0,
        }
    }

    /// Creates an empty `ChunkedVec` with a fixed chunk size of `N` and the specified capacity.
    ///
    /// The actual number of chunks allocated will be calculated as ceiling(capacity / N),
    /// where N is the fixed chunk size. This method is useful when you know the approximate
    /// number of elements you'll be storing and want to avoid reallocations.
    ///
    /// # Arguments
    /// * `capacity` - The minimum number of elements the ChunkedVec should be able to hold
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::{ChunkedVecSized, ChunkedVec};
    /// let vec: ChunkedVec<i32, 8> = ChunkedVecSized::with_capacity(10);
    /// // This will allocate 2 chunks (ceiling(10/8) = 2) with total capacity of 16
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> ChunkedVec<T, N> {
        let chunk_size = capacity.div_ceil(N);
        ChunkedVec {
            data: Vec::with_capacity(chunk_size),
            len: 0,
        }
    }

    /// Creates an empty `ChunkedVec` with a fixed chunk size of `N` and pre-allocates
    /// the specified number of chunks.
    ///
    /// This method provides direct control over the number of chunks to allocate, which
    /// can be more intuitive than specifying capacity when working with chunked storage.
    ///
    /// # Arguments
    /// * `chunk_count` - The number of chunks to pre-allocate
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::{ChunkedVecSized, ChunkedVec};
    /// let vec: ChunkedVec<i32, 8> = ChunkedVecSized::with_chunk_count(2);
    /// // This will allocate 2 chunks with total capacity of 16
    /// ```
    #[inline]
    #[must_use]
    pub fn with_chunk_count(chunk_count: usize) -> ChunkedVec<T, N> {
        ChunkedVec {
            data: Vec::with_capacity(chunk_count),
            len: 0,
        }
    }
}

/// Implementation of basic creation methods for ChunkedVec with default chunk size.
///
/// This implementation provides convenient methods to create ChunkedVec instances using the default
/// chunk size (64). These methods are more flexible as they don't require specifying the chunk size
/// at compile-time, making them suitable for general use cases.
impl<T> ChunkedVec<T> {
    /// Creates a new empty `ChunkedVec` with the default chunk size.
    ///
    /// This is the most straightforward way to create a ChunkedVec when you don't need
    /// a specific chunk size. The default chunk size is optimized for general use cases.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let vec: ChunkedVec<i32> = ChunkedVec::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        ChunkedVecSized::new()
    }

    /// Creates an empty `ChunkedVec` with the specified capacity.
    ///
    /// The actual number of chunks allocated will be calculated as ceiling(capacity / N),
    /// where N is the default chunk size.
    ///
    /// # Arguments
    /// * `capacity` - The minimum number of elements the ChunkedVec should be able to hold
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let vec: ChunkedVec<i32> = ChunkedVec::with_capacity(10);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        ChunkedVecSized::with_capacity(capacity)
    }

    /// Creates an empty `ChunkedVec` with a fixed chunk size of `N` and the specified number of chunks.
    ///
    /// The total capacity will be N * chunk_count.
    ///
    /// # Arguments
    /// * `chunk_count` - The number of chunks to pre-allocate
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<i32>::with_chunk_count(2);
    /// // This will allocate 2 chunks with total capacity of 16
    /// ```
    #[inline]
    #[must_use]
    pub fn with_chunk_count(chunk_count: usize) -> Self {
        ChunkedVecSized::with_chunk_count(chunk_count)
    }
}
#[cfg(test)]
mod tests {
    use crate::{ChunkedVec, ChunkedVecSized};

    #[test]
    fn test_new() {
        let vec: ChunkedVec<i32> = ChunkedVec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let mut vec: ChunkedVec<i32> = ChunkedVec::with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 64);

        for i in 0..10 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 64);

        for i in 0..200 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 210);
        assert_eq!(vec.capacity(), 256)
    }

    #[test]
    fn test_with_chunks() {
        let mut vec = ChunkedVec::with_chunk_count(2);
        assert_eq!(vec.len(), 0);
        for i in 0..16 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 16);
    }

    #[test]
    fn test_with_chunk_size() {
        let vec: ChunkedVec<i32, 8> = ChunkedVecSized::new();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_with_chunk_size_and_capacity() {
        let mut vec: ChunkedVec<i32, 8> = ChunkedVecSized::with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 16);
        for i in 0..20 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 20);
        assert_eq!(vec.capacity(), 32);
    }
}

use crate::ChunkedVec;

/// Implementation of basic creation methods for ChunkedVec without fixed chunk size
impl<T> ChunkedVec<T> {
    /// Creates a new empty `ChunkedVec`.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let vec: ChunkedVec<i32> = ChunkedVec::new();
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
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
        use crate::DEFAULT_CHUNK_SIZE as N;

        let chunk_size = (capacity + N - 1) / N;
        dbg!(chunk_size);
        Self {
            data: Vec::with_capacity(chunk_size),
            len: 0,
        }
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
        Self {
            data: Vec::with_capacity(chunk_count),
            len: 0,
        }
    }
}

/// Implementation of creation methods for ChunkedVec with fixed chunk size
impl<T, const N: usize> ChunkedVec<T, N> {
    /// Creates a new empty `ChunkedVec` with a fixed chunk size of `N`.
    ///
    /// The chunk size `N` determines how many elements are stored in each internal chunk.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let vec: ChunkedVec<i32, 8> = ChunkedVec::with_chunk_size();
    /// ```
    #[inline]
    #[must_use]
    pub fn with_chunk_size() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    /// Creates an empty `ChunkedVec` with a fixed chunk size of `N` and the specified capacity.
    ///
    /// The actual number of chunks allocated will be calculated as ceiling(capacity / N),
    /// where N is the fixed chunk size.
    ///
    /// # Arguments
    /// * `capacity` - The minimum number of elements the ChunkedVec should be able to hold
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let vec: ChunkedVec<i32, 8> = ChunkedVec::with_chunk_size_and_capacity(10);
    /// // This will allocate 2 chunks (ceiling(10/8) = 2) with total capacity of 16
    /// ```
    #[inline]
    #[must_use]
    pub fn with_chunk_size_and_capacity(capacity: usize) -> Self {
        let chunk_size = (capacity + N - 1) / N;
        Self {
            data: Vec::with_capacity(chunk_size),
            len: 0,
        }
    }

    /// Creates an empty `ChunkedVec` with a fixed chunk size of `N` and pre-allocates
    /// the specified number of chunks.
    ///
    /// This is an alias for `with_chunks` with a more explicit name.
    ///
    /// # Arguments
    /// * `num_chunks` - The number of chunks to pre-allocate
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let vec: ChunkedVec<i32, 8> = ChunkedVec::with_chunk_size_and_count(2);
    /// // This will allocate 2 chunks with total capacity of 16
    /// ```
    #[inline]
    #[must_use]
    pub fn with_chunk_size_and_count(chunk_count: usize) -> Self {
        Self {
            data: Vec::with_capacity(chunk_count),
            len: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ChunkedVec;

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
        let vec: ChunkedVec<i32, 8> = ChunkedVec::with_chunk_size();
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_with_chunk_size_and_capacity() {
        let mut vec: ChunkedVec<i32, 8> = ChunkedVec::with_chunk_size_and_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 16);
        for i in 0..20 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 20);
        assert_eq!(vec.capacity(), 32);
    }
}

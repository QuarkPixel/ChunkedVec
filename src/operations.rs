use crate::ChunkedVec;

/// Implementation of basic operations for ChunkedVec.
///
/// This implementation provides core vector operations such as pushing elements,
/// querying length and capacity, and managing the internal chunk structure.
impl<T, const N: usize> ChunkedVec<T, N> {
    /// Appends an element to the back of the vector.
    ///
    /// If the current chunk is full, a new chunk will be allocated to store the element.
    /// The element is always added to the end of the vector.
    ///
    /// # Arguments
    /// * `value` - The value to push onto the vector
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<i32>::new();
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    pub fn push(&mut self, value: T) {
        let chunk_idx = self.len / N;
        let offset = self.len % N;

        if chunk_idx >= self.data.len() {
            assert_eq!(offset, 0);
            let chunk = Self::create_new_chunk(value);
            self.data.push(chunk);
        } else {
            self.data[chunk_idx][offset] = value;
        }
        self.len += 1;
    }

    /// Returns the number of elements in the vector.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<i32>::new();
    /// assert_eq!(vec.len(), 0);
    /// vec.push(1);
    /// assert_eq!(vec.len(), 1);
    /// ```
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector contains no elements.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<i32>::new();
    /// assert!(vec.is_empty());
    /// vec.push(1);
    /// assert!(!vec.is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the total number of elements the vector can hold without reallocating.
    ///
    /// The capacity is always a multiple of the chunk size N.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::{ChunkedVecSized, ChunkedVec};
    /// let vec: ChunkedVec<i32, 4> = ChunkedVecSized::with_capacity(10);
    /// assert!(vec.capacity() >= 12); // Rounds up to multiple of chunk size
    /// ```
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity() * N
    }

    /// Returns the number of elements that can be held in currently allocated chunks.
    ///
    /// This differs from capacity() in that it only counts space in chunks that have
    /// already been allocated, not potential space in the underlying Vec's capacity.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::{ChunkedVecSized, ChunkedVec};
    /// let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
    /// vec.push(1);
    /// assert_eq!(vec.allocated_capacity(), 4); // One chunk allocated
    /// ```
    #[inline]
    #[must_use]
    pub fn allocated_capacity(&self) -> usize {
        self.data.len() * N
    }
}

#[cfg(test)]
mod tests {
    use crate::ChunkedVecSized;

    use super::*;

    #[test]
    fn test_new_chunked_vec() {
        let vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();

        // Test adding the first element
        vec.push(1);
        assert_eq!(vec.len(), 1);
        assert!(!vec.is_empty());

        // Test adding more elements within the same chunk
        vec.push(2);
        vec.push(3);
        vec.push(4);
        assert_eq!(vec.len(), 4);

        // Test adding element that causes creation of a new chunk
        vec.push(5);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn test_capacity() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();

        // Add enough elements to create a new chunk
        for i in 0..5 {
            vec.push(i);
        }

        // Capacity should be able to hold at least two chunks
        assert!(vec.capacity() >= 8);
    }
}

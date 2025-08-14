use crate::ChunkedVec;
use std::array::from_fn;
use std::mem::MaybeUninit;
use std::ptr;

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
            self.data[chunk_idx][offset].write(value);
        }
        self.len += 1;
    }

    /// Resizes the `ChunkedVec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    ///
    /// This method requires `T` to implement [`Clone`],
    /// in order to be able to clone the passed value.
    /// If you need more flexibility (or want to rely on [`Default`] instead of
    /// [`Clone`]), use [`ChunkedVec::resize_with`].
    /// If you only need to resize to a smaller size, use [`Vec::truncate`].
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` _bytes_.
    ///
    /// # Examples
    ///
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<&str>::new();
    /// vec.resize(3, "example");
    /// let len = vec.len();
    /// assert_eq!(len, 3);
    /// ```
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        let old_len = self.len;

        if new_len > old_len {
            let required_chunks = (new_len + N - 1) / N;
            if required_chunks > self.data.len() {
                self.data.resize_with(required_chunks, || {
                    let arr: [MaybeUninit<T>; N] = from_fn(|_| MaybeUninit::uninit());
                    Box::new(arr)
                });
            }

            for i in old_len..new_len {
                let chunk_idx = i / N;
                let offset = i % N;
                self.data[chunk_idx][offset].write(value.clone());
            }
        } else if new_len < old_len {
            // 1. Dropar os elementos entre o novo e o antigo tamanho.
            for i in new_len..old_len {
                let chunk_idx = i / N;
                let offset = i % N;
                unsafe {
                    let elem_ptr = self.data[chunk_idx][offset].as_mut_ptr();
                    ptr::drop_in_place(elem_ptr);
                }
            }
            let required_chunks = if new_len == 0 {
                0
            } else {
                (new_len + N - 1) / N
            };
            self.data.truncate(required_chunks);
        }

        self.len = new_len;
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!(
                "removal index (is {index}) should be < len (is {})",
                self.len
            );
        }

        let (current_chunk_idx, offset) = self.chunk_and_offset(index);

        unsafe {
            // Read the element to be removed
            let ret = ptr::read(self.get_elem_ptr(current_chunk_idx, offset));

            // Shift elements within the current chunk
            let first_chunk_ptr = self.get_chunk_mut_ptr(current_chunk_idx);
            let count = N - 1 - offset;
            if count > 0 {
                ptr::copy(
                    first_chunk_ptr.add(offset + 1),
                    first_chunk_ptr.add(offset),
                    count,
                );
            }

            // Shift elements between chunks
            let until_chunk_idx = (self.len - 1) / N;
            for i in current_chunk_idx..until_chunk_idx {
                let current_chunk_ptr = self.get_chunk_mut_ptr(i);
                let next_chunk_ptr = self.get_chunk_mut_ptr(i + 1);

                let val_from_next = ptr::read(next_chunk_ptr);
                ptr::write(current_chunk_ptr.add(N - 1), val_from_next);
                ptr::copy(next_chunk_ptr.add(1), next_chunk_ptr, N - 1);
            }

            self.len -= 1;
            let required_chunks = if self.len == 0 {
                0
            } else {
                (self.len + N - 1) / N
            };
            self.data.truncate(required_chunks);

            ret
        }
    }

    /// Removes an element from the `ChunkedVec` and returns it.
    ///
    /// The removed element is replaced by the last element of the ChunkedVec.
    ///
    /// This does not preserve ordering of the remaining elements, but is *O*(1).
    /// If you need to preserve the element order, use [`remove`] instead.
    ///
    /// [`remove`]: ChunkedVec::remove
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use chunked_vec::chunked_vec;
    /// let mut v = chunked_vec!["foo", "bar", "baz", "qux"];
    ///
    /// assert_eq!(v.swap_remove(1), "bar");
    /// assert_eq!(v, ["foo", "qux", "baz"]);
    ///
    /// assert_eq!(v.swap_remove(0), "foo");
    /// assert_eq!(v, ["baz", "qux"]);
    /// ```
    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();
        if index >= len {
            panic!("swap_remove index (is {index}) should be < len (is {len})");
        }

        let current_pos = self.chunk_and_offset(index);
        unsafe {
            // We replace self[index] with the last element. Note that if the
            // bounds check above succeeds there must be a last element (which
            // can be self[index] itself).
            let current = self.get_elem_mut_ptr(current_pos.0, current_pos.1);
            let ret = ptr::read(current);

            let last_pos = self.chunk_and_offset(len - 1);
            let last = self.get_elem_ptr(last_pos.0, last_pos.1);
            ptr::copy(last, current, 1);

            self.len -= 1;
            ret
        }
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
    use super::*;
    use crate::ChunkedVecSized;

    #[test]
    fn test_new_chunked_vec() {
        let vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_push_single_chunk() {
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
        assert_eq!(vec.allocated_capacity(), 4);
    }

    #[test]
    fn test_push_multiple_chunks() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();

        // Test adding element that causes creation of a new chunk
        for i in 1..=5 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.allocated_capacity(), 8); // Two chunks allocated
    }

    #[test]
    fn test_capacity() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();

        // Add enough elements to create multiple chunks
        for i in 0..9 {
            vec.push(i);
        }

        // Capacity should be able to hold at least three chunks
        assert!(vec.capacity() >= 12);
        assert_eq!(vec.allocated_capacity(), 12); // Exactly three chunks
    }

    #[test]
    fn test_is_empty() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
        assert!(vec.is_empty());

        vec.push(1);
        assert!(!vec.is_empty());

        vec.push(2);
        assert!(!vec.is_empty());
    }

    #[test]
    fn test_resize_grow() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);

        vec.resize(5, 42);
        assert_eq!(vec.len(), 5);
        // Note: Can't directly test values without indexing implementation
    }

    #[test]
    fn test_resize_shrink() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        for i in 1..=7 {
            vec.push(i);
        }
        assert_eq!(vec.len(), 7);
        assert_eq!(vec.allocated_capacity(), 9); // 3 chunks

        vec.resize(4, 0);
        assert_eq!(vec.len(), 4);
        assert_eq!(vec.allocated_capacity(), 6); // 2 chunks after truncate
    }

    #[test]
    fn test_resize_to_zero() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        for i in 1..=5 {
            vec.push(i);
        }

        vec.resize(0, 0);
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
        assert_eq!(vec.allocated_capacity(), 0);
    }

    #[test]
    fn test_remove_first_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        let removed = vec.remove(0);
        assert_eq!(removed, 1);
        assert_eq!(vec.len(), 3);
        // Vector should now be [2, 3, 4]
    }

    #[test]
    fn test_remove_middle_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        for i in 1..=6 {
            vec.push(i);
        }

        let removed = vec.remove(2);
        assert_eq!(removed, 3);
        assert_eq!(vec.len(), 5);
        // Vector should now be [1, 2, 4, 5, 6]
    }

    #[test]
    fn test_remove_last_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let removed = vec.remove(2);
        assert_eq!(removed, 3);
        assert_eq!(vec.len(), 2);
        // Vector should now be [1, 2]
    }

    #[test]
    fn test_remove_single_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(42);

        let removed = vec.remove(0);
        assert_eq!(removed, 42);
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
        assert_eq!(vec.allocated_capacity(), 0);
    }

    #[test]
    fn test_remove_across_chunks() {
        let mut vec: ChunkedVec<i32, 2> = ChunkedVecSized::new();
        for i in 1..=7 {
            vec.push(i);
        }
        // Chunks: [1,2], [3,4], [5,6], [7]

        let removed = vec.remove(1); // Remove second element
        assert_eq!(removed, 2);
        assert_eq!(vec.len(), 6);
        // Should now be [1,3], [4,5], [6,7]
    }

    #[test]
    fn test_remove_causes_chunk_deallocation() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        for i in 1..=7 {
            vec.push(i);
        }
        assert_eq!(vec.allocated_capacity(), 9); // 3 chunks

        // Remove elements to cause chunk deallocation
        vec.remove(6); // Remove last element
        assert_eq!(vec.len(), 6);
        assert_eq!(vec.allocated_capacity(), 6); // Should still be 2 chunks

        vec.remove(5); // Remove what's now the last element
        assert_eq!(vec.len(), 5);
        vec.remove(4);
        assert_eq!(vec.len(), 4);
        vec.remove(3);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.allocated_capacity(), 3); // Should be 1 chunk now
    }

    #[test]
    #[should_panic(expected = "removal index (is 5) should be < len (is 3)")]
    fn test_remove_out_of_bounds() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        vec.remove(5); // This should panic
    }

    #[test]
    #[should_panic(expected = "removal index (is 0) should be < len (is 0)")]
    fn test_remove_empty_vec() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.remove(0); // This should panic
    }

    #[test]
    fn test_remove_with_drop_types() {
        use std::rc::Rc;

        let mut vec: ChunkedVec<Rc<i32>, 3> = ChunkedVecSized::new();
        let val1 = Rc::new(1);
        let val2 = Rc::new(2);
        let val3 = Rc::new(3);

        vec.push(val1.clone());
        vec.push(val2.clone());
        vec.push(val3.clone());

        assert_eq!(Rc::strong_count(&val2), 2); // One in vec, one in our variable

        let removed = vec.remove(1);
        assert_eq!(*removed, 2);
        assert_eq!(Rc::strong_count(&val2), 2); // Now one in removed, one in our variable
        assert_eq!(vec.len(), 2);

        drop(removed);
        assert_eq!(Rc::strong_count(&val2), 1); // Now only our variable holds it
    }

    // Tests for swap_remove function
    #[test]
    fn test_swap_remove_first_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);

        let removed = vec.swap_remove(0);
        assert_eq!(removed, 1);
        assert_eq!(vec.len(), 3);
        // Last element (4) should now be at position 0
        // Vector should now be [4, 2, 3] (order changed)
    }

    #[test]
    fn test_swap_remove_middle_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        for i in 1..=6 {
            vec.push(i);
        }

        let removed = vec.swap_remove(2);
        assert_eq!(removed, 3);
        assert_eq!(vec.len(), 5);
        // Last element (6) should now be at position 2
        // Vector should now be [1, 2, 6, 4, 5]
    }

    #[test]
    fn test_swap_remove_last_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let removed = vec.swap_remove(2);
        assert_eq!(removed, 3);
        assert_eq!(vec.len(), 2);
        // Vector should now be [1, 2] (last element removed, no swap needed)
    }

    #[test]
    fn test_swap_remove_single_element() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(42);

        let removed = vec.swap_remove(0);
        assert_eq!(removed, 42);
        assert_eq!(vec.len(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_swap_remove_across_chunks() {
        let mut vec: ChunkedVec<i32, 2> = ChunkedVecSized::new();
        for i in 1..=7 {
            vec.push(i);
        }
        // Chunks: [1,2], [3,4], [5,6], [7]

        let removed = vec.swap_remove(1); // Remove second element
        assert_eq!(removed, 2);
        assert_eq!(vec.len(), 6);
        // Last element (7) should now be at position 1
        // Should now be [1,7], [3,4], [5,6]
    }

    #[test]
    fn test_swap_remove_performance_characteristic() {
        // Test that swap_remove doesn't shift elements like remove does
        let mut vec: ChunkedVec<i32, 100> = ChunkedVecSized::new();
        for i in 0..1000 {
            vec.push(i);
        }

        let removed = vec.swap_remove(500);
        assert_eq!(removed, 500);
        assert_eq!(vec.len(), 999);
        // Element 999 (the last element) should now be at position 500
    }

    #[test]
    fn test_swap_remove_with_drop_types() {
        use std::rc::Rc;

        let mut vec: ChunkedVec<Rc<i32>, 3> = ChunkedVecSized::new();
        let val1 = Rc::new(1);
        let val2 = Rc::new(2);
        let val3 = Rc::new(3);

        vec.push(val1.clone());
        vec.push(val2.clone());
        vec.push(val3.clone());

        assert_eq!(Rc::strong_count(&val2), 2); // One in vec, one in our variable

        let removed = vec.swap_remove(1);
        assert_eq!(*removed, 2);
        assert_eq!(Rc::strong_count(&val2), 2); // Now one in removed, one in our variable
        assert_eq!(vec.len(), 2);

        drop(removed);
        assert_eq!(Rc::strong_count(&val2), 1); // Now only our variable holds it
    }

    #[test]
    #[should_panic(expected = "swap_remove index (is 5) should be < len (is 3)")]
    fn test_swap_remove_out_of_bounds() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        vec.swap_remove(5); // This should panic
    }

    #[test]
    #[should_panic(expected = "swap_remove index (is 0) should be < len (is 0)")]
    fn test_swap_remove_empty_vec() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.swap_remove(0); // This should panic
    }
}

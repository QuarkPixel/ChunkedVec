use std::array::from_fn;
use std::mem::MaybeUninit;
use std::ptr;
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
    pub fn resize(&mut self, new_len: usize, value: T) where T: Clone {
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
            panic!("removal index (is {index}) should be < len (is {})", self.len);
        }

        unsafe {
            let current_chunk_idx = index / N;
            let offset = index % N;

            let ret = ptr::read(self.data[current_chunk_idx].get_unchecked(offset).as_ptr());

            let first_chunk_ptr = self.data.get_unchecked_mut(current_chunk_idx).as_mut_ptr();
            let count = N - 1 - offset;
            if count > 0 {
                ptr::copy(
                    first_chunk_ptr.add(offset + 1),
                    first_chunk_ptr.add(offset),
                    count,
                );
            }

            let until_chunk_idx = (self.len - 1) / N;
            for i in current_chunk_idx..until_chunk_idx {
                let current_chunk_ptr = self.data.get_unchecked_mut(i).as_mut_ptr();
                let next_chunk_ptr = self.data.get_unchecked_mut(i + 1).as_mut_ptr();

                let val_from_next = ptr::read(next_chunk_ptr);
                ptr::write(current_chunk_ptr.add(N - 1), val_from_next);
                ptr::copy(
                    next_chunk_ptr.add(1),
                    next_chunk_ptr,
                    N - 1,
                );
            }

            let last_chunk_idx = self.len / N;
            let offset = self.len % N;
            *self.data[last_chunk_idx].get_unchecked_mut(offset) = MaybeUninit::uninit();

            self.len -= 1;
            let required_chunks = if self.len == 0 { 0 } else { (self.len + N - 1) / N };
            self.data.truncate(required_chunks);

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

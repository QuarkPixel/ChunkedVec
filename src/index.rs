use crate::ChunkedVec;
use std::ops::{Index, IndexMut};

/// Implementation of indexing operations for ChunkedVec.
///
/// This implementation provides various methods for accessing elements in the ChunkedVec,
/// including safe and unsafe access methods, as well as implementations of the Index and
/// IndexMut traits for convenient array-style access.
impl<T, const N: usize> ChunkedVec<T, N> {
    /// Returns a reference to an element without performing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior.
    ///
    /// # Arguments
    /// * `index` - The index of the element to access
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        let chunk_idx = index / N;
        let offset = index % N;
        self.data.get_unchecked(chunk_idx)[offset].assume_init_ref()
    }

    /// Returns a mutable reference to an element without performing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior.
    ///
    /// # Arguments
    /// * `index` - The index of the element to access
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        let chunk_idx = index / N;
        let offset = index % N;
        self.data.get_unchecked_mut(chunk_idx)[offset].assume_init_mut()
    }

    /// Returns a reference to an element at the given index.
    ///
    /// Returns None if the index is out of bounds.
    ///
    /// # Arguments
    /// * `index` - The index of the element to access
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<i32>::new();
    /// vec.push(1);
    /// assert_eq!(vec.get(0), Some(&1));
    /// assert_eq!(vec.get(1), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    /// Returns a mutable reference to an element at the given index.
    ///
    /// Returns None if the index is out of bounds.
    ///
    /// # Arguments
    /// * `index` - The index of the element to access
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::<i32>::new();
    /// vec.push(1);
    /// if let Some(x) = vec.get_mut(0) {
    ///     *x = 10;
    /// }
    /// assert_eq!(vec[0], 10);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked_mut(index) })
        }
    }

    /// Gets the chunk index and offset for a given element index.
    ///
    /// # Returns
    /// A tuple of (chunk_index, offset_within_chunk)
    #[inline]
    #[must_use]
    pub(crate) fn chunk_and_offset(&self, index: usize) -> (usize, usize) {
        (index / N, index % N)
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn get_chunk_ptr(&self, index: usize) -> *const T {
        self.data.get_unchecked(index).as_ptr().cast()
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn get_chunk_mut_ptr(&mut self, index: usize) -> *mut T {
        self.data.get_unchecked_mut(index).as_mut_ptr().cast()
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn get_elem_ptr(&self, index: usize, offset: usize) -> *const T {
        self.get_chunk_ptr(index).add(offset).cast()
    }

    #[inline]
    #[must_use]
    pub(crate) unsafe fn get_elem_mut_ptr(&mut self, index: usize, offset: usize) -> *mut T {
        self.get_chunk_mut_ptr(index).add(offset).cast()
    }
}

impl<T, const N: usize> Index<usize> for ChunkedVec<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: index {} >= length {}",
                index, self.len
            );
        }
        // Safety: We have already checked the index bounds
        unsafe { self.get_unchecked(index) }
    }
}

impl<T, const N: usize> IndexMut<usize> for ChunkedVec<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: index {} >= length {}",
                index, self.len
            );
        }
        // Safety: We have already checked the index bounds
        unsafe { self.get_unchecked_mut(index) }
    }
}

#[cfg(test)]
mod test {
    use crate::ChunkedVecSized;

    #[test]
    fn test_indexing() {
        let mut vec = ChunkedVecSized::<u8, 4>::new();

        vec.push(10);
        vec.push(20);
        vec.push(30);
        vec.push(40);
        vec.push(50);

        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 30);
        assert_eq!(vec[3], 40);
        assert_eq!(vec[4], 50);

        vec[1] = 99;
        assert_eq!(vec[1], 99);

        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn test_get() {
        let mut vec = ChunkedVecSized::<i32, 4>::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut vec = ChunkedVecSized::<i32, 4>::new();
        vec.push(1);
        vec.push(2);

        if let Some(x) = vec.get_mut(0) {
            *x = 10;
        }
        assert_eq!(vec[0], 10);
        assert_eq!(vec.get_mut(2), None);
    }
}

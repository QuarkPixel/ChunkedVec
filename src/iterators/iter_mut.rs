use likely_stable::unlikely;

use crate::ChunkedVec;

/// A mutable iterator over the elements of a ChunkedVec.
///
/// This struct is created by the [`iter_mut`] method on [`ChunkedVec`].
/// See its documentation for more.
pub struct IterMut<'a, T, const N: usize> {
    pub(crate) vec: &'a mut ChunkedVec<T, N>,
    pub(crate) chunk_idx: usize,
    pub(crate) offset: usize,
    pub(crate) remaining: usize,
}

impl<T, const N: usize> ChunkedVec<T, N> {
    /// Returns an iterator that allows modifying each element in the vector.
    ///
    /// The iterator yields all items from start to end.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::ChunkedVec;
    /// let mut vec = ChunkedVec::new();
    /// vec.push(1);
    /// vec.push(2);
    ///
    /// for element in vec.iter_mut() {
    ///     *element *= 2;
    /// }
    ///
    /// assert_eq!(vec[0], 2);
    /// assert_eq!(vec[1], 4);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut {
            remaining: self.len(),
            vec: self,
            chunk_idx: 0,
            offset: 0,
        }
    }
}

impl<'a, T, const N: usize> IterMut<'a, T, N> {
    /// Advances to the next position.
    #[inline]
    fn advance_position(&mut self) {
        self.offset += 1;
        if unlikely(self.offset == N) {
            self.chunk_idx += 1;
            self.offset = 0;
        }
        self.remaining -= 1;
    }

    /// Returns a pointer to the current element.
    #[inline]
    fn current_ptr(&mut self) -> *mut T {
        self.vec.data[self.chunk_idx][self.offset].as_mut_ptr()
    }
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if unlikely(self.remaining == 0) {
            return None;
        }

        unsafe {
            // 使用原始指针避免借用冲突
            let ptr = self.current_ptr();
            self.advance_position();

            // 将原始指针转换为正确生命周期的引用
            Some(&mut *ptr)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining;
        (remaining, Some(remaining))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_mut() {
        let mut vec = ChunkedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let mut iter = vec.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        let elem = iter.next();
        assert_eq!(elem, Some(&mut 3));
        *elem.unwrap() = 4;
        assert_eq!(iter.next(), None);
        assert_eq!(vec[2], 4);
    }
}

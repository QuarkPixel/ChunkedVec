use std::mem::MaybeUninit;

use likely_stable::unlikely;

use crate::ChunkedVec;

/// An iterator over the elements of a ChunkedVec.
///
/// This struct is created by the [`iter`] method on [`ChunkedVec`].
/// See its documentation for more.
pub struct Iter<'a, T, const N: usize> {
    pub(crate) vec: &'a ChunkedVec<T, N>,
    pub(crate) chunk_idx: usize,
    pub(crate) offset: usize,
    pub(crate) remaining: usize,
}

impl<T, const N: usize> ChunkedVec<T, N> {
    /// Returns an iterator over the elements of the vector.
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
    /// let mut sum = 0;
    /// for element in vec.iter() {
    ///     sum += *element;
    /// }
    /// assert_eq!(sum, 3);
    /// ```
    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter {
            vec: self,
            chunk_idx: 0,
            offset: 0,
            remaining: self.len(),
        }
    }
}

impl<'a, T, const N: usize> Iter<'a, T, N> {
    /// Advances to the next position.
    #[inline]
    unsafe fn advance_position(&mut self) {
        self.offset += 1;
        if unlikely(self.offset == N) {
            self.chunk_idx += 1;
            self.offset = 0;
        }
        self.remaining -= 1;
    }

    /// Returns a pointer to the current element.
    #[inline]
    fn current_ptr(&mut self) -> &'a MaybeUninit<T> {
        &self.vec.data[self.chunk_idx][self.offset]
    }
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if unlikely(self.remaining == 0) {
            return None;
        }

        unsafe {
            let value = self.current_ptr().assume_init_ref();
            self.advance_position();
            Some(value)
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
    fn test_iter() {
        let mut vec = ChunkedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let mut iter = vec.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }
}

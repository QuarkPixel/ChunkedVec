use likely_stable::unlikely;
use std::{mem::MaybeUninit, ptr};

use crate::ChunkedVec;

/// An owning iterator over the elements of a ChunkedVec.
///
/// This struct is created by the `into_iter` method on [`ChunkedVec`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
/// let mut vec = ChunkedVec::new();
/// vec.push(1);
/// vec.push(2);
///
/// let mut sum = 0;
/// for element in vec {
///     sum += element;
/// }
/// assert_eq!(sum, 3);
/// ```
pub struct IntoIter<T, const N: usize> {
    pub(crate) vec: ChunkedVec<T, N>,
    pub(crate) chunk_idx: usize,
    pub(crate) offset: usize,
    pub(crate) remaining: usize,
}

/// Implementation of IntoIterator for ChunkedVec, enabling use in for loops.
///
/// This implementation consumes the ChunkedVec, taking ownership of its elements.
impl<T, const N: usize> IntoIterator for ChunkedVec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            remaining: self.len(),
            vec: self,
            chunk_idx: 0,
            offset: 0,
        }
    }
}

impl<T, const N: usize> IntoIter<T, N> {
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
    fn current_ptr(&mut self) -> &mut MaybeUninit<T> {
        &mut self.vec.data[self.chunk_idx][self.offset]
    }

    /// Drops all remaining elements without returning them.
    /// More efficient than calling next() repeatedly.
    fn drop_remaining(&mut self) {
        while self.remaining > 0 {
            unsafe {
                self.current_ptr().assume_init_drop();
                *self.current_ptr() = MaybeUninit::uninit();
                self.advance_position();
            }
        }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if unlikely(self.remaining == 0) {
            return None;
        }

        unsafe {
            let value = ptr::read(self.current_ptr().as_ptr());
            *self.current_ptr() = MaybeUninit::uninit();
            self.advance_position();
            Some(value)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining;
        (remaining, Some(remaining))
    }
}

/// Implementation of Drop for IntoIter to handle partial consumption correctly.
impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        // Drop all remaining elements
        self.drop_remaining();

        // Prevent ChunkedVec's Drop from trying to drop elements again
        self.vec.len = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_iter() {
        let mut vec = ChunkedVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);

        let mut iter = vec.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
}

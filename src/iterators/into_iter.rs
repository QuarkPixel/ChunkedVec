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
    pub(crate) index: usize,
}

/// Implementation of IntoIterator for ChunkedVec, enabling use in for loops.
///
/// This implementation consumes the ChunkedVec, taking ownership of its elements.
impl<T, const N: usize> IntoIterator for ChunkedVec<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            vec: self,
            index: 0,
        }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let index = self.index;
            self.index += 1;

            // Calculate chunk and offset
            let chunk_idx = index / N;
            let offset = index % N;

            // Safety: We've already checked bounds and we know this element was initialized
            unsafe {
                let elem_ptr = self.vec.data[chunk_idx][offset].as_ptr();
                let value = ptr::read(elem_ptr);

                // Mark this slot as uninitialized to prevent double-drop
                self.vec.data[chunk_idx][offset] = MaybeUninit::uninit();

                Some(value)
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.len() - self.index;
        (remaining, Some(remaining))
    }
}

/// Implementation of Drop for IntoIter to handle partial consumption correctly.
///
/// When an IntoIter is dropped, we need to ensure that the ChunkedVec doesn't
/// try to drop elements that have already been moved out during iteration.
impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        // 手动释放未消费的元素以防止内存泄漏
        while self.index < self.vec.len {
            let chunk_idx = self.index / N;
            let offset = self.index % N;

            unsafe {
                // 释放仍然有效的元素
                self.vec.data[chunk_idx][offset].assume_init_drop();
            }
            self.index += 1;
        }

        // 现在可以安全地设置len为0，防止ChunkedVec的Drop再次尝试释放
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

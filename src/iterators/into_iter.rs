use std::mem;

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
            // Note: This implementation could be optimized in the future by:
            // 1. Directly accessing chunk data
            // 2. Batch processing elements within chunks
            // 3. Avoiding individual element replacement
            let index = self.index;
            self.index += 1;
            Some(mem::replace(&mut self.vec[index], unsafe { mem::zeroed() }))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.vec.len() - self.index;
        (remaining, Some(remaining))
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

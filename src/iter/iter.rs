use crate::ChunkedVec;

/// An iterator over the elements of a ChunkedVec.
///
/// This struct is created by the [`iter`] method on [`ChunkedVec`].
/// See its documentation for more.
pub struct Iter<'a, T, const N: usize> {
    pub(crate) vec: &'a ChunkedVec<T, N>,
    pub(crate) index: usize,
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
            index: 0,
        }
    }
}

impl<'a, T, const N: usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let index = self.index;
            self.index += 1;
            Some(&self.vec[index])
        } else {
            None
        }
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

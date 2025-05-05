use crate::ChunkedVec;

/// A mutable iterator over the elements of a ChunkedVec.
///
/// This struct is created by the [`iter_mut`] method on [`ChunkedVec`].
/// See its documentation for more.
pub struct IterMut<'a, T, const N: usize> {
    pub(crate) vec: &'a mut ChunkedVec<T, N>,
    pub(crate) index: usize,
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
            vec: self,
            index: 0,
        }
    }
}

impl<'a, T, const N: usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let index = self.index;
            self.index += 1;
            unsafe {
                // Safety: We use raw pointer to avoid multiple mutable references.
                // This is safe because we increment the index before yielding the next element,
                // ensuring we never yield multiple references to the same element.
                let ptr = self.vec.get_unchecked_mut(index) as *mut T;
                Some(&mut *ptr)
            }
        } else {
            None
        }
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

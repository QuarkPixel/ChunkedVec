use crate::ChunkedVec;

/// Implements the `FromIterator` trait for `ChunkedVec`, allowing it to be created from any iterator.
///
/// This implementation provides an efficient way to collect elements from an iterator into a `ChunkedVec`.
/// It pre-allocates space based on the iterator's size hint when available, which can improve performance
/// by reducing the number of reallocations.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
/// 
/// let vec = vec![1, 2, 3, 4, 5];
/// let chunked_vec: ChunkedVec<_> = vec.into_iter().collect();
/// assert_eq!(chunked_vec.len(), 5);
/// ```
impl<T> FromIterator<T> for ChunkedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();
        let mut chunked_vec = ChunkedVec::with_capacity(upper.unwrap_or(lower));
        for item in iter {
            chunked_vec.push(item);
        }
        chunked_vec
    }
}

/// Implements conversion from `Vec<T>` to `ChunkedVec<T>`.
///
/// This implementation efficiently converts a standard vector into a `ChunkedVec` by
/// consuming the original vector and reusing its memory allocation when possible.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
/// 
/// let vec = vec![1, 2, 3];
/// let chunked_vec = ChunkedVec::from(vec);
/// assert_eq!(chunked_vec[0], 1);
/// ```
impl<T> From<Vec<T>> for ChunkedVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from_iter(vec.into_iter())
    }
}

/// Implements conversion from fixed-size arrays to `ChunkedVec<T>`.
///
/// This allows creating a `ChunkedVec` from any array of known size `M`.
/// The conversion preserves the order of elements.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
/// 
/// let array = [1, 2, 3];
/// let chunked_vec = ChunkedVec::from(array);
/// assert_eq!(chunked_vec[0], 1);
/// ```
impl<T, const M: usize> From<[T; M]> for ChunkedVec<T> {
    fn from(array: [T; M]) -> Self {
        Self::from_iter(array)
    }
}

/// Implements conversion from slices to `ChunkedVec<T>`.
///
/// This implementation creates a new `ChunkedVec` by cloning elements from the slice.
/// The original slice remains unchanged and available for further use.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
/// 
/// let slice: &[i32] = &[1, 2, 3];
/// let chunked_vec = ChunkedVec::from(slice);
/// assert_eq!(chunked_vec[0], 1);
/// ```
impl<T: Clone> From<&[T]> for ChunkedVec<T> {
    fn from(slice: &[T]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_iterator() {
        let vec = vec![1, 2, 3, 4, 5];
        let chunked_vec: ChunkedVec<_> = vec.into_iter().collect();
        assert_eq!(chunked_vec.len(), 5);
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![1, 2, 3, 4, 5];
        let chunked_vec: ChunkedVec<_> = vec.into();
        assert_eq!(chunked_vec.len(), 5);

        let chunked_vec = ChunkedVec::from(vec![2, 3, 1]);
        assert_eq!(chunked_vec[0], 2);
        assert_eq!(chunked_vec[1], 3);
        assert_eq!(chunked_vec[2], 1);
    }

    #[test]
    fn test_from_array() {
        let chunked_vec = ChunkedVec::from([2, 3, 1]);
        assert_eq!(chunked_vec[0], 2);
        assert_eq!(chunked_vec[1], 3);
        assert_eq!(chunked_vec[2], 1);
    }

    #[test]
    fn test_from_slice() {
        let slice: &[i32] = &[2, 3, 1];
        let chunked_vec = ChunkedVec::from(slice);
        assert_eq!(chunked_vec[0], 2);
        assert_eq!(chunked_vec[1], 3);
        assert_eq!(chunked_vec[2], 1);
    }
}

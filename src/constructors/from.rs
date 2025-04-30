use crate::ChunkedVec;

impl<T> FromIterator<T> for ChunkedVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        // [TODO]: Higher performance implementation
        let mut chunked_vec = ChunkedVec::new();
        for item in iter {
            chunked_vec.push(item);
        }
        chunked_vec
    }
}

impl<T> From<Vec<T>> for ChunkedVec<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from_iter(vec.into_iter())
    }
}

impl<T, const M: usize> From<[T; M]> for ChunkedVec<T> {
    fn from(array: [T; M]) -> Self {
        Self::from_iter(array)
    }
}

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

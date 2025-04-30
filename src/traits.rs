use crate::ChunkedVec;

/// Implementation of the Default trait for ChunkedVec.
///
/// This implementation provides a way to create an empty ChunkedVec using the default() method.
/// The created vector will use the default chunk size (64) and have no pre-allocated chunks.
///
/// # Examples
/// ```
/// use chunked_vec::ChunkedVec;
/// let vec: ChunkedVec<i32> = ChunkedVec::default();
/// assert!(vec.is_empty());
/// ```
impl<T> Default for ChunkedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        let vec = ChunkedVec::<()>::default();
        assert_eq!(vec.len(), 0);
    }
}

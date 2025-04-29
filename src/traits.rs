use crate::ChunkedVec;

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

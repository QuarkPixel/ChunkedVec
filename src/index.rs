use crate::ChunkedVec;
use std::ops::{Index, IndexMut};

impl<T, const N: usize> ChunkedVec<T, N> {
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        let chunk_idx = index / N;
        let offset = index % N;
        &self.data.get_unchecked(chunk_idx).get_unchecked(offset)
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        let chunk_idx = index / N;
        let offset = index % N;
        &mut (*self.data.get_unchecked_mut(chunk_idx))[offset]
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked(index) })
        }
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            Some(unsafe { self.get_unchecked_mut(index) })
        }
    }
}

impl<T, const N: usize> Index<usize> for ChunkedVec<T, N> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: index {} >= length {}",
                index, self.len
            );
        }
        // Safety: 我们已经检查了索引边界
        unsafe { self.get_unchecked(index) }
    }
}

impl<T, const N: usize> IndexMut<usize> for ChunkedVec<T, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: index {} >= length {}",
                index, self.len
            );
        }
        // Safety: 我们已经检查了索引边界
        unsafe { self.get_unchecked_mut(index) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_indexing() {
        let mut vec = ChunkedVec::<u8, 4>::with_chunk_size();

        vec.push(10);
        vec.push(20);
        vec.push(30);
        vec.push(40);
        vec.push(50);

        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 30);
        assert_eq!(vec[3], 40);
        assert_eq!(vec[4], 50);

        vec[1] = 99;
        assert_eq!(vec[1], 99);

        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn test_get() {
        let mut vec = ChunkedVec::<i32, 4>::with_chunk_size();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut vec = ChunkedVec::<i32, 4>::with_chunk_size();
        vec.push(1);
        vec.push(2);

        if let Some(x) = vec.get_mut(0) {
            *x = 10;
        }
        assert_eq!(vec[0], 10);
        assert_eq!(vec.get_mut(2), None);
    }
}

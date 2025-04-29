use crate::{Chunk, ChunkedVec};

impl<T, const N: usize> ChunkedVec<T, N> {
    pub fn push(&mut self, value: T) {
        let chunk_idx = self.len / N;
        let offset = self.len % N;

        if chunk_idx >= self.data.len() {
            assert_eq!(offset, 0);
            let chunk = Self::create_new_chunk(value);
            self.data.push(chunk);
        } else {
            self.data[chunk_idx][offset] = value;
        }
        self.len += 1;
    }

    fn create_new_chunk(value: T) -> Chunk<T, N> {
        let mut chunk = Box::new_uninit_slice(N);
        // Initialize the first element
        chunk[0].write(value);
        // Convert to Box<Chunk<T, N>>
        unsafe {
            let ptr = Box::into_raw(chunk) as *mut [T; N];
            Box::from_raw(ptr)
        }
    }

    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        // [TODO]
        self.data.capacity() * N
    }

    pub fn get_chunk(&self, _index: usize) -> Option<Chunk<T, N>> {
        unimplemented!();
        // self.data.get(index).map(|chunk| &**chunk)
    }
}

impl<T> Default for ChunkedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_chunked_vec() {
        let vec: ChunkedVec<i32, 4> = ChunkedVec::with_chunk_size();
        assert!(vec.is_empty());
        assert_eq!(vec.len(), 0);
    }

    #[test]
    fn test_push() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVec::with_chunk_size();

        // 测试添加第一个元素
        vec.push(1);
        assert_eq!(vec.len(), 1);
        assert!(!vec.is_empty());

        // 测试在同一个chunk中添加更多元素
        vec.push(2);
        vec.push(3);
        vec.push(4);
        assert_eq!(vec.len(), 4);

        // 测试添加元素导致创建新的chunk
        vec.push(5);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn test_capacity() {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVec::with_chunk_size();

        // 添加足够多的元素以创建新的chunk
        for i in 0..5 {
            vec.push(i);
        }

        // 容量应该至少能容纳两个chunk
        assert!(vec.capacity() >= 8);
    }
}

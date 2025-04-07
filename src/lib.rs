//! ChunkedVec is a vector-like data structure that stores elements in fixed-size chunks.
//! 
//! # Features
//! - Fixed-size chunk-based storage for better memory management
//! - Standard vector-like interface
//! - Index-based access with bounds checking
//! 
//! # Example
//! ```
//! use chunked_vec::ChunkedVec;
//! 
//! let mut vec = ChunkedVec::<i32>::new();
//! vec.push(1);
//! vec.push(2);
//! assert_eq!(vec[0], 1);
//! ```

use std::ops::{Index, IndexMut};

/// A vector-like container that stores elements in fixed-size chunks.
/// 
/// Type Parameters:
/// - `T`: The type of elements to store
/// - `N`: The size of each chunk (default: 64)
pub struct ChunkedVec<T, const N: usize = 64> {
    data: Vec<Box<[T; N]>>,
    len: usize,
}

impl<T, const N: usize> ChunkedVec<T, N> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    pub fn push(&mut self, value: T)
    where
        T: Default + Copy, // 临时约束，实际实现可以更灵活
    {
        let chunk_idx = self.len / N;
        let offset = self.len % N;

        if chunk_idx >= self.data.len() {
            // 如果需要新块，分配并填充默认值
            let mut new_chunk = Box::new([T::default(); N]);
            new_chunk[offset] = value;
            self.data.push(new_chunk);
        } else {
            // 直接写入已有块
            self.data[chunk_idx][offset] = value;
        }
        self.len += 1;
    }

    // 添加len方法，便于检查长度
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T, const N: usize> Index<usize> for ChunkedVec<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: index {} >= length {}",
                index, self.len
            );
        }
        let chunk_idx = index / N;
        let offset = index % N;
        &self.data[chunk_idx][offset]
    }
}

impl<T, const N: usize> IndexMut<usize> for ChunkedVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!(
                "Index out of bounds: index {} >= length {}",
                index, self.len
            );
        }
        let chunk_idx = index / N;
        let offset = index % N;
        &mut self.data[chunk_idx][offset]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexing() {
        let mut vec = ChunkedVec::<u8, 4>::new(); // 使用小块大小便于测试

        // 添加一些元素
        vec.push(10);
        vec.push(20);
        vec.push(30);
        vec.push(40);
        vec.push(50);

        // 测试读取
        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 30);
        assert_eq!(vec[3], 40);
        assert_eq!(vec[4], 50);

        // 测试写入
        vec[1] = 99;
        assert_eq!(vec[1], 99);

        // 检查长度
        assert_eq!(vec.len(), 5);
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        let mut vec = ChunkedVec::<u8, 4>::new();
        vec.push(10);
        let _ = vec[1]; // 应该触发panic
    }
}

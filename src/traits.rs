use crate::ChunkedVec;

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

    fn create_new_chunk(value: T) -> Box<[T; N]> {
        let mut chunk = Box::new_uninit_slice(N);
        // Initialize the first element
        chunk[0].write(value);
        // Convert to Box<[T; N]>
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

    pub fn get_chunk(&self, _index: usize) -> Option<&[T; N]> {
        unimplemented!();
        // self.data.get(index).map(|chunk| &**chunk)
    }
}

impl<T> Default for ChunkedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

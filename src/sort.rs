use crate::ChunkedVec;
use std::cmp::Ordering;

/// Implementation of in-place sorting for ChunkedVec.
///
/// Sorting is performed without allocating any memory, matching the guarantee
/// of [`slice::sort_unstable`]. The algorithm is an introsort over logical
/// indices: quicksort partitioning for ranges that span multiple chunks, the
/// standard library's slice sort for any range that lies entirely within one
/// chunk, and a heapsort fallback that bounds the worst case at *O*(*n* log *n*).
///
/// Elements are only ever swapped in place, so every slot stays initialized at
/// all times: if a caller-supplied comparator panics, the vector is left in a
/// valid (permuted) state with no leaks or double drops.
impl<T, const N: usize> ChunkedVec<T, N> {
    /// Sorts the vector, but might not preserve the order of equal elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements) and in-place
    /// (i.e., does not allocate).
    ///
    /// # Complexity
    /// Worst case *O*(*n* log *n*). Best case *O*(*n*) when the vector fits
    /// in a single chunk; *O*(*n* log(*n*/*N*)) otherwise, even for
    /// already-sorted input.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::chunked_vec;
    /// let mut v = chunked_vec![-5, 4, 1, -3, 2];
    ///
    /// v.sort_unstable();
    /// assert_eq!(v, [-5, -3, 1, 2, 4]);
    /// ```
    pub fn sort_unstable(&mut self)
    where
        T: Ord,
    {
        self.sort_unstable_by(T::cmp);
    }

    /// Sorts the vector with a comparison function, but might not preserve
    /// the order of equal elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements) and in-place
    /// (i.e., does not allocate).
    ///
    /// If `compare` panics, the vector is left as a valid permutation of its
    /// former contents.
    ///
    /// # Complexity
    /// Worst case *O*(*n* log *n*). Best case *O*(*n*) when the vector fits
    /// in a single chunk; *O*(*n* log(*n*/*N*)) otherwise, even for
    /// already-sorted input.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::chunked_vec;
    /// let mut v = chunked_vec![5, 4, 1, 3, 2];
    ///
    /// v.sort_unstable_by(|a, b| b.cmp(a));
    /// assert_eq!(v, [5, 4, 3, 2, 1]);
    /// ```
    pub fn sort_unstable_by<F>(&mut self, mut compare: F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        if self.len < 2 {
            return;
        }
        let depth_limit = 2 * (usize::BITS - self.len.leading_zeros()) as usize;
        self.introsort(0, self.len, depth_limit, &mut compare);
    }

    /// Sorts the vector with a key extraction function, but might not
    /// preserve the order of equal elements.
    ///
    /// This sort is unstable (i.e., may reorder equal elements) and in-place
    /// (i.e., does not allocate).
    ///
    /// # Complexity
    /// Worst case *O*(*n* log *n*). Best case *O*(*n*) when the vector fits
    /// in a single chunk; *O*(*n* log(*n*/*N*)) otherwise, even for
    /// already-sorted input.
    ///
    /// # Examples
    /// ```
    /// use chunked_vec::chunked_vec;
    /// let mut v = chunked_vec![-5i32, 4, 1, -3, 2];
    ///
    /// v.sort_unstable_by_key(|k| k.abs());
    /// assert_eq!(v, [1, 2, -3, 4, -5]);
    /// ```
    pub fn sort_unstable_by_key<K, F>(&mut self, mut f: F)
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.sort_unstable_by(|a, b| f(a).cmp(&f(b)));
    }

    /// Returns the logical range `[lo, hi)` as a contiguous mutable slice.
    ///
    /// # Safety
    /// `[lo, hi)` must be non-empty, within `self.len`, and lie entirely
    /// within a single chunk.
    unsafe fn range_slice_mut(&mut self, lo: usize, hi: usize) -> &mut [T] {
        let (chunk_idx, offset) = self.chunk_and_offset(lo);
        std::slice::from_raw_parts_mut(self.get_elem_mut_ptr(chunk_idx, offset), hi - lo)
    }

    /// Swaps the elements at indices `a` and `b`.
    ///
    /// # Safety
    /// Both `a` and `b` must be below `self.len`.
    #[inline]
    unsafe fn swap_unchecked(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.len && b < self.len);
        if a == b {
            return;
        }

        let (a_chunk, a_off) = self.chunk_and_offset(a);
        let (b_chunk, b_off) = self.chunk_and_offset(b);
        // Safety: both indices are in bounds, so both slots are initialized;
        // distinct indices never overlap. Same-chunk pointers must be derived
        // from one borrow of the chunk to keep both provenances valid.
        if a_chunk == b_chunk {
            let base = self.get_chunk_mut_ptr(a_chunk);
            std::ptr::swap_nonoverlapping(base.add(a_off), base.add(b_off), 1);
        } else {
            let a_ptr = self.get_elem_mut_ptr(a_chunk, a_off);
            let b_ptr = self.get_elem_mut_ptr(b_chunk, b_off);
            std::ptr::swap_nonoverlapping(a_ptr, b_ptr, 1);
        }
    }

    /// Compares the elements at two in-bounds indices.
    #[inline]
    fn cmp_at<F>(&self, a: usize, b: usize, compare: &mut F) -> Ordering
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        // Safety: callers only pass indices below self.len.
        compare( unsafe { self.get_unchecked(a) }, unsafe { self.get_unchecked(b) } )
    }

    /// Sorts the logical range `[lo, hi)`.
    ///
    /// Recurses only into the smaller partition and loops on the larger one,
    /// bounding stack depth at *O*(log *n*).
    fn introsort<F>(&mut self, mut lo: usize, mut hi: usize, mut depth: usize, compare: &mut F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        loop {
            if hi - lo <= 1 {
                return;
            }
            if lo / N == (hi - 1) / N {
                // The whole range lies within one chunk: sort it as a
                // contiguous slice.
                // Safety: [lo, hi) is within self.len and in a single chunk.
                unsafe { self.range_slice_mut(lo, hi) }.sort_unstable_by(|a, b| compare(a, b));
                return;
            }
            if depth == 0 {
                self.heapsort(lo, hi, compare);
                return;
            }
            depth -= 1;

            let p = self.partition(lo, hi, compare);
            if p - lo < hi - (p + 1) {
                self.introsort(lo, p, depth, compare);
                lo = p + 1;
            } else {
                self.introsort(p + 1, hi, depth, compare);
                hi = p;
            }
        }
    }

    /// Partitions `[lo, hi)` around a median-of-3 pivot and returns the
    /// pivot's final position.
    fn partition<F>(&mut self, lo: usize, hi: usize, compare: &mut F) -> usize
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let mid = lo + (hi - lo) / 2;
        let last = hi - 1;

        // Safety (all swaps below): lo <= mid <= i <= j <= last < hi <= len.
        unsafe {
            // Median-of-3: order the elements at lo, mid, last, then park
            // the median at `last` as the pivot.
            if self.cmp_at(mid, lo, compare) == Ordering::Less {
                self.swap_unchecked(lo, mid);
            }
            if self.cmp_at(last, lo, compare) == Ordering::Less {
                self.swap_unchecked(lo, last);
            }
            if self.cmp_at(last, mid, compare) == Ordering::Less {
                self.swap_unchecked(mid, last);
            }
            self.swap_unchecked(mid, last);

            // Lomuto partition against the pivot, which stays at `last`
            // until the final swap places it at its sorted position.
            let mut i = lo;
            for j in lo..last {
                if self.cmp_at(j, last, compare) != Ordering::Greater {
                    self.swap_unchecked(i, j);
                    i += 1;
                }
            }
            self.swap_unchecked(i, last);
            i
        }
    }

    /// Heapsorts the logical range `[lo, hi)`.
    fn heapsort<F>(&mut self, lo: usize, hi: usize, compare: &mut F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        let n = hi - lo;
        for root in (0..n / 2).rev() {
            self.sift_down(lo, root, n, compare);
        }
        for end in (1..n).rev() {
            // Safety: lo + end < hi <= len.
            unsafe { self.swap_unchecked(lo, lo + end) };
            self.sift_down(lo, 0, end, compare);
        }
    }

    /// Restores the max-heap property for the heap of size `n` rooted at
    /// `lo`, sifting down from heap index `root`.
    fn sift_down<F>(&mut self, lo: usize, mut root: usize, n: usize, compare: &mut F)
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        loop {
            let mut child = 2 * root + 1;
            if child >= n {
                return;
            }
            if child + 1 < n && self.cmp_at(lo + child, lo + child + 1, compare) == Ordering::Less
            {
                child += 1;
            }
            if self.cmp_at(lo + root, lo + child, compare) != Ordering::Less {
                return;
            }
            // Safety: root < child < n, so both stay below lo + n <= len.
            unsafe { self.swap_unchecked(lo + root, lo + child) };
            root = child;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ChunkedVec, ChunkedVecSized};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn check_sort<const N: usize>(data: &[i32]) {
        let mut vec: ChunkedVec<i32, N> = ChunkedVecSized::new();
        for &x in data {
            vec.push(x);
        }

        vec.sort_unstable();

        let mut expected = data.to_vec();
        expected.sort_unstable();
        let actual: Vec<i32> = vec.iter().copied().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn swap_unchecked() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        for i in 0..5 {
            vec.push(i);
        }

        unsafe {
            // Swap within the same chunk
            vec.swap_unchecked(0, 1);
            assert_eq!(vec, [1, 0, 2, 3, 4]);

            // Swap across chunks
            vec.swap_unchecked(0, 4);
            assert_eq!(vec, [4, 0, 2, 3, 1]);

            // Swapping an index with itself is a no-op
            vec.swap_unchecked(2, 2);
            assert_eq!(vec, [4, 0, 2, 3, 1]);
        }
    }

    #[test]
    fn sort_empty_and_single() {
        check_sort::<3>(&[]);
        check_sort::<3>(&[42]);
    }

    #[test]
    fn sort_already_sorted() {
        let data: Vec<i32> = (0..20).collect();
        check_sort::<3>(&data);
    }

    #[test]
    fn sort_reverse_sorted() {
        let data: Vec<i32> = (0..20).rev().collect();
        check_sort::<3>(&data);
    }

    #[test]
    fn sort_all_duplicates() {
        // The Lomuto worst case: forces the heapsort fallback.
        check_sort::<3>(&[7; 40]);
    }

    #[test]
    fn sort_across_chunks() {
        let data = [5, -1, 3, 9, 0, -7, 2, 8, 1, 4, -3, 6, 7, -2];
        check_sort::<3>(&data); // len % N != 0
        check_sort::<4>(&data);
        check_sort::<7>(&data); // len % N == 0
    }

    #[test]
    fn sort_exactly_one_chunk() {
        // len == N: the pure slice-leaf path.
        check_sort::<8>(&[4, 7, 1, 8, 2, 6, 3, 5]);
    }

    #[test]
    fn sort_by_descending() {
        let mut vec: ChunkedVec<i32, 3> = ChunkedVecSized::new();
        vec.extend([5, 1, 4, 2, 3, 7, 6]);

        vec.sort_unstable_by(|a, b| b.cmp(a));

        assert_eq!(vec, [7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn sort_by_key() {
        let mut vec: ChunkedVec<(i32, &str), 3> = ChunkedVecSized::new();
        vec.extend([(3, "c"), (1, "a"), (4, "d"), (2, "b"), (5, "e")]);

        vec.sort_unstable_by_key(|&(num, _)| num);

        let keys: Vec<i32> = vec.iter().map(|&(num, _)| num).collect();
        assert_eq!(keys, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_large() {
        // Deterministic pseudo-random data across many default-size chunks.
        let data: Vec<i32> = (0..10_000u64)
            .map(|i| (i.wrapping_mul(2654435761) % 10_000) as i32)
            .collect();

        let mut vec = ChunkedVec::<i32>::new();
        vec.extend(data.iter().copied());

        vec.sort_unstable();

        let mut expected = data;
        expected.sort_unstable();
        let actual: Vec<i32> = vec.iter().copied().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn panicking_comparator_leaves_valid_state() {
        // Own the counter locally; the shared DROP_COUNT in drop.rs is
        // reserved for its tests (tests run in parallel).
        static SORT_PANIC_DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

        struct Counted(i32);
        impl Drop for Counted {
            fn drop(&mut self) {
                SORT_PANIC_DROP_COUNT.fetch_add(1, Ordering::SeqCst);
            }
        }

        let len = 20;
        let mut vec: ChunkedVec<Counted, 3> = ChunkedVecSized::new();
        for i in 0..len {
            vec.push(Counted((len - i) as i32));
        }

        let mut calls = 0;
        let result = catch_unwind(AssertUnwindSafe(|| {
            vec.sort_unstable_by(|a, b| {
                calls += 1;
                if calls == 10 {
                    panic!("comparator panic");
                }
                a.0.cmp(&b.0)
            })
        }));
        assert!(result.is_err());

        // The vector must still hold a permutation of the original elements,
        // with nothing dropped during unwinding.
        assert_eq!(SORT_PANIC_DROP_COUNT.load(Ordering::SeqCst), 0);
        assert_eq!(vec.len(), len);
        let mut values: Vec<i32> = vec.iter().map(|c| c.0).collect();
        values.sort_unstable();
        assert_eq!(values, (1..=len as i32).collect::<Vec<_>>());

        // Dropping the vector drops each element exactly once.
        drop(vec);
        assert_eq!(SORT_PANIC_DROP_COUNT.load(Ordering::SeqCst), len);
    }
}

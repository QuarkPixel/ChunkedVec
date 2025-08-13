use crate::ChunkedVec;
use std::ptr;

impl<T, const N: usize> Drop for ChunkedVec<T, N> {
    fn drop(&mut self) {
        if !std::mem::needs_drop::<T>() {
            return;
        }

        let mut remaining = self.len;
        for chunk in std::mem::take(&mut self.data).iter_mut() {
            let to_drop = remaining.min(N);
            if to_drop == 0 {
                break;
            }

            let chunk_ptr = chunk.as_mut_ptr();
            unsafe {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(
                    chunk_ptr.cast::<T>(),
                    to_drop,
                ));
            }
            remaining -= to_drop;
        }
    }
}

#[cfg(test)]
mod memory_safety_tests {
    use crate::ChunkedVecSized;

    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug, Clone)]
    struct Droper {
        id: usize,
    }

    impl Droper {
        fn new(id: usize) -> Self {
            Self { id }
        }
    }

    impl Drop for Droper {
        fn drop(&mut self) {
            println!("{} is dropped!", self.id);
            DROP_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn test_no_spurious_drops_on_extend() {
        DROP_COUNT.store(0, Ordering::SeqCst);

        {
            let mut vec: ChunkedVec<Option<Droper>, 2> = ChunkedVecSized::new();
            // extended None should not trigger drop
            vec.extend(std::iter::repeat(None).take(4));

            // add actual values
            vec.push(Some(Droper::new(1)));
            vec.push(Some(Droper::new(2)));

            assert_eq!(
                DROP_COUNT.load(Ordering::SeqCst),
                0,
                "No drops should occur during extend with None"
            );
        }

        assert_eq!(
            DROP_COUNT.load(Ordering::SeqCst),
            2,
            "Exactly 2 drops expected"
        );
    }

    #[test]
    fn test_proper_cleanup_on_drop() {
        DROP_COUNT.store(0, Ordering::SeqCst);

        {
            let mut vec: ChunkedVec<Droper, 3> = ChunkedVecSized::new();
            for i in 0..7 {
                vec.push(Droper::new(i));
            }
            assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 0);
        }

        // all 7 objects should be dropped correctly.
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 7);
    }

    #[test]
    fn test_resize_drop_behavior() {
        DROP_COUNT.store(0, Ordering::SeqCst);

        let mut vec: ChunkedVec<Droper, 2> = ChunkedVecSized::new();
        for i in 0..5 {
            vec.push(Droper::new(i));
        }

        // reduce to 3 elements, should drop 2 and value `Droper::new(999)`
        vec.resize(3, Droper::new(999));
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 2 + 1);

        // expand to 6 elements, there should not be any additional drop except the parameter itself
        vec.resize(6, Droper::new(888));
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 3 + 1);
    }
}

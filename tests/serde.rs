#![cfg(feature = "serde")]

use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

use chunked_vec::{chunked_vec, ChunkedVec, ChunkedVecSized};
use serde::de::{Deserialize, Deserializer};

fn round_trip<const N: usize>(vec: &ChunkedVec<i32, N>) -> ChunkedVec<i32, N> {
    let json = serde_json::to_string(vec).unwrap();
    serde_json::from_str(&json).unwrap()
}

#[test]
fn round_trip_empty() {
    let vec = ChunkedVec::<i32>::new();
    assert_eq!(serde_json::to_string(&vec).unwrap(), "[]");
    let back = round_trip(&vec);
    assert!(back.is_empty());
}

#[test]
fn round_trip_default_n() {
    let vec: ChunkedVec<i32> = (0..10).collect();
    let back = round_trip(&vec);
    assert_eq!(back.len(), vec.len());
    assert!(back.iter().eq(vec.iter()));
}

#[test]
fn round_trip_exact_chunk_boundary() {
    for len in [4, 8] {
        let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
        for i in 0..len {
            vec.push(i);
        }
        let back = round_trip(&vec);
        assert_eq!(back.len(), len as usize);
        assert!(back.iter().eq(vec.iter()));
    }
}

#[test]
fn round_trip_multiple_chunks_non_default_n() {
    let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
    for i in 0..10 {
        vec.push(i);
    }
    let back = round_trip(&vec);
    assert!(back.iter().eq(vec.iter()));
}

#[test]
fn round_trip_strings() {
    let mut vec: ChunkedVec<String, 2> = ChunkedVecSized::new();
    for s in ["alpha", "beta", "gamma", "delta", "epsilon"] {
        vec.push(s.to_string());
    }
    let json = serde_json::to_string(&vec).unwrap();
    let back: ChunkedVec<String, 2> = serde_json::from_str(&json).unwrap();
    assert!(back.iter().eq(vec.iter()));
}

#[test]
fn interop_with_vec() {
    let chunked = chunked_vec![1, 2, 3, 4, 5];
    let std_vec = vec![1, 2, 3, 4, 5];

    let chunked_json = serde_json::to_string(&chunked).unwrap();
    let std_json = serde_json::to_string(&std_vec).unwrap();
    assert_eq!(chunked_json, std_json);

    let from_std: ChunkedVec<i32, 3> = serde_json::from_str(&std_json).unwrap();
    assert!(from_std.iter().eq(std_vec.iter()));

    let from_chunked: Vec<i32> = serde_json::from_str(&chunked_json).unwrap();
    assert_eq!(from_chunked, std_vec);
}

#[test]
fn chunk_size_is_not_part_of_format() {
    let mut vec: ChunkedVec<i32, 4> = ChunkedVecSized::new();
    for i in 0..10 {
        vec.push(i);
    }
    let json = serde_json::to_string(&vec).unwrap();
    let back: ChunkedVec<i32, 16> = serde_json::from_str(&json).unwrap();
    assert!(back.iter().eq(vec.iter()));
}

static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct DropCounter(#[allow(dead_code)] i32);

impl Drop for DropCounter {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, SeqCst);
    }
}

impl<'de> Deserialize<'de> for DropCounter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        i32::deserialize(deserializer).map(DropCounter)
    }
}

#[test]
fn drop_safety_on_mid_sequence_error() {
    // Keep this the only test that touches DROP_COUNT: tests run in parallel.
    DROP_COUNT.store(0, SeqCst);
    let result: Result<ChunkedVec<DropCounter, 2>, _> = serde_json::from_str("[1, 2, 3, \"boom\"]");
    assert!(result.is_err());
    // The 3 elements pushed before the error must be dropped exactly once each.
    assert_eq!(DROP_COUNT.load(SeqCst), 3);
}

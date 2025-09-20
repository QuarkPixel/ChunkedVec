use chunked_vec::ChunkedVec;
use std::path::PathBuf;

#[test]
fn test_pathbuf_tuple_into_iter() {
    let mut data: ChunkedVec<(PathBuf, u64)> = ChunkedVec::new();

    // Push a large number of elements to trigger the original bug
    for i in 0..100 {
        data.push((PathBuf::from("/test"), i));
    }

    // This should work without panic now
    let mut count = 0;
    for (path, i) in data {
        assert_eq!(path, PathBuf::from("/test"));
        assert_eq!(i, count);
        count += 1;
    }

    assert_eq!(count, 100);
}

#[test]
fn test_pathbuf_tuple_into_iter_not_completely_consumed() {
    let mut data: ChunkedVec<(PathBuf, u64)> = ChunkedVec::new();

    // Push a large number of elements to trigger the original bug
    for i in 0..100 {
        data.push((PathBuf::from("/test"), i));
    }

    // This should work without panic now
    // let mut count = 0;
    let mut iter = data.into_iter();
    for i in 0..50 {
        let (path, count) = iter.next().unwrap();
        assert_eq!(path, PathBuf::from("/test"));
        assert_eq!(i, count);
    }

    drop(iter); // 可以正常运行，但是是否内存泄漏了呢？
}

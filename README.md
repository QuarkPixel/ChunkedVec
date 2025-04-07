# ChunkedVec

[![Crates.io](https://img.shields.io/crates/v/chunked_vec.svg)](https://crates.io/crates/chunked_vec)
[![Documentation](https://docs.rs/chunked_vec/badge.svg)](https://docs.rs/chunked_vec)

> **Note**: This is my first Rust library, created primarily as a learning exercise. While functional, it may not be optimized for production use.

ChunkedVec is a vector-like data structure that stores elements in fixed-size chunks. It provides a Vec-like interface while offering unique advantages in memory management and growth characteristics.

## Features

- Chunk-based storage with configurable chunk size (default: 64)
- Standard vector-like interface with index-based access
- Efficient memory allocation during growth
- O(1) random access time complexity

## Use Cases

ChunkedVec is particularly well-suited for:
- Scenarios with frequent append operations where traditional Vec's reallocation costs are significant
- Applications requiring dynamic buffers or logging systems
- Cases where memory fragmentation is acceptable in exchange for reduced reallocation overhead

## Performance Characteristics

### Advantages
- **Low Growth Cost**: Unlike Vec<T> which needs to reallocate and copy all elements when growing, ChunkedVec only needs to allocate a new chunk
- **Efficient Random Access**: O(1) access time through simple chunk index and offset calculations
- **Memory Efficiency**: Reduced reallocation overhead compared to Vec<T>

### Trade-offs
- **Memory Layout**: Elements are stored in separate chunks, which may lead to some memory fragmentation
- **Access Overhead**: Slightly higher access cost compared to Vec<T> due to additional calculations and indirection
- **Implementation Complexity**: More complex internal structure compared to continuous storage

## Usage Example

```rust
use chunked_vec::ChunkedVec;

fn main() {
    let mut vec = ChunkedVec::<i32>::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
}
```
# ChunkedVec

[![Crates.io](https://img.shields.io/crates/v/chunked_vec.svg)](https://crates.io/crates/chunked_vec)
[![Documentation](https://docs.rs/chunked_vec/badge.svg)](https://docs.rs/chunked_vec)

> **Note**: This is a learning project implementing a vector-like data structure with chunked storage.

ChunkedVec is a vector-like data structure that stores elements in fixed-size chunks. It provides a Vec-like interface while offering unique advantages in memory management and memory locality.

## Features

- Flexible chunk-based storage with compile-time or runtime configurable chunk size
- Standard vector-like interface with efficient operations
- Support for both fixed-size and default-size chunk construction
- O(1) random access time complexity
- Efficient memory allocation during growth
- Support for constructing from various types (Vec, Array, Slice, Iterator)

## Usage Examples

### Basic Usage with Default Chunk Size

```rust
use chunked_vec::ChunkedVec;

fn main() {
    let mut vec = ChunkedVec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
}
```

### Using Macro for Quick Initialization
```rust
use chunked_vec::{chunked_vec, ChunkedVec};

fn main() {
    // Create an empty ChunkedVec
    let vec: ChunkedVec<i32> = chunked_vec![];
    assert_eq!(vec.len(), 0);

    // Create a ChunkedVec with initial values
    let vec = chunked_vec![1, 2, 3, 4, 5];
    assert_eq!(vec[0], 1);

    // Create a ChunkedVec with repeated values
    let vec = chunked_vec![42; 5]; // [42, 42, 42, 42, 42]
    assert_eq!(vec.len(), 5);
}```

### Using Fixed Chunk Size

```rust
use chunked_vec::{ChunkedVec, ChunkedVecSized};

fn main() {
    // Create a ChunkedVec with chunk size of 8
    let mut vec: ChunkedVec<i32, 8> = ChunkedVecSized::new();
    vec.push(1);

    // Pre-allocate space for elements
    let vec = ChunkedVecSized::<i32, 8>::with_capacity(100);
    assert_eq!(vec.capacity(), 104);// Actual capacity of ChunkedVec rounds up to chunk size
}
```

### Converting from Other Collections

```rust
use chunked_vec::ChunkedVec;

fn main() {
    // From Vec
    let vec = vec![1, 2, 3];
    let chunked: ChunkedVec<i32> = vec.into();

    // From array
    let arr = [1, 2, 3];
    let chunked = ChunkedVec::from(arr);

    // From slice
    let slice: &[i32] = &[1, 2, 3];
    let chunked = ChunkedVec::from(slice);

    // From iterator
    let chunked: ChunkedVec<i32> = (0..10).collect();
}
```

## Current Implementation Status

### Implemented Features

- Comprehensive constructors (`new`, `with_capacity`, `with_chunk_count`)
- Push operation (supports all types)
- Index-based access (`get`, `get_mut`, `Index`/`IndexMut` traits)
- Length and capacity queries (`len`, `capacity`, `allocated_capacity`)
- Fixed-size chunk support via `ChunkedVecSized`
- From/FromIterator implementations for various types
- Safe and unsafe getter methods
- Efficient memory management

### Planned Features

- Advanced chunk-level operations
- Custom allocator support
- More collection traits implementation
- Performance optimizations for specific use cases

## Contributing

This is an open learning project. Contributions and suggestions are welcome! Feel free to:

- Report bugs and suggest features
- Submit pull requests
- Share your use cases and feedback
- Help improve documentation
```

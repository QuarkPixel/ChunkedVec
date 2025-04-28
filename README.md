# ChunkedVec

[![Crates.io](https://img.shields.io/crates/v/chunked_vec.svg)](https://crates.io/crates/chunked_vec)
[![Documentation](https://docs.rs/chunked_vec/badge.svg)](https://docs.rs/chunked_vec)

> **Note**: This is a learning project implementing a vector-like data structure with chunked storage.

ChunkedVec is a vector-like data structure that stores elements in fixed-size chunks. It provides a Vec-like interface while offering unique advantages in memory management.

## Current Implementation Status

### Implemented Features

- Basic constructors (`new`, `with_capacity`, `with_chunk_size`)
- Push operation (for `Default + Copy` types only)
- Index-based access (`get`, `Index`/`IndexMut` traits)
- Length and capacity queries
- Configurable chunk size (default: 64)

### Planned Features (Not Yet Implemented)

- Full iterator support
- Non-Copy type support
- Chunk-level operations
- More advanced memory management

## Features

- Chunk-based storage with configurable chunk size
- Standard vector-like interface for basic operations
- Efficient memory allocation during growth
- O(1) random access time complexity

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

## Contributing

This is an open learning project. Contributions and suggestions are welcome!

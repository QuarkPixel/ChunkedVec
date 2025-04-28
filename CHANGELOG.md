# Changelog

## [0.2.0] - 2025-04-28
### Added
- `IndexMut` trait implementation for mutable indexing
- Advanced constructors (`with_capacity`, `with_chunk_size`, `with_chunk_size_and_capacity`, `with_chunks`)
- Comprehensive test coverage for all core functionality
- Safe and unsafe getter methods (`get`, `get_mut`, `get_unchecked`, `get_unchecked_mut`)
- Capacity management methods (`capacity`, `with_capacity`)

### Changed
- Improved documentation with detailed usage examples
- Enhanced bounds checking in indexing operations
- Better memory management with flexible chunk size options
- More efficient index calculations

### Enhanced
- More robust index bounds checking
- Optimized chunk allocation strategy
- Improved type safety with const generics

## [0.1.0] - 2025-04-07
### Added
- Core `ChunkedVec` data structure
- Basic constructors (`new`)
- `push` operation (with `Default + Copy` constraint)
- `Index` trait implementation for read access
- `len` method for size query
- Initial test cases

### Limitations
- Only supports `Default + Copy` types
- No iterator support
- No chunk-level operations
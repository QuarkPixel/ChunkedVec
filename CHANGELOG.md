# Changelog

## [0.3.4] - 2025-09-21

### Fixed

- Fixed critical zero-initialization panic in `IntoIterator` implementation for heap-allocated types ([#6](https://github.com/QuarkPixel/ChunkedVec/issues/6))
  - Replaced unsafe `mem::zeroed()` with proper `ptr::read()` from `MaybeUninit` storage
  - Added proper memory management in `Drop` implementation for `IntoIter` to prevent memory leaks
  - Fixed double-drop issues when iterator is partially consumed

### Enhanced

- Significantly improved iterator performance across all iterator types:
  - Cached chunk index and offset calculations to avoid repeated division/modulo operations
  - Added `unlikely` branch prediction hints for better optimization
  - Unified iterator implementation patterns with proper abstraction
- Added `size_hint` implementation for `Iter` and `IterMut` iterators for better collection compatibility
- Enhanced memory safety with more robust cleanup in partially consumed iterators

### Internal

- Refactored iterator implementations with better separation of concerns:
  - Extracted `advance_position()` method for cleaner position management
  - Improved code reuse and maintainability across iterator types
- Added comprehensive test coverage for heap-allocated types and partial iterator consumption

## [0.3.3] - 2025-08-21

- Implemented `remove` method to remove an element at a specific index
- Implemented `swap_remove` method for efficient element removal by swapping with the last element
- Derived `Debug` trait and implemented `PartialEq` temporarily for ChunkedVec to make doctests happy :)

## [0.3.2] - 2025-08-13

### Fixed

- Fixed `assume_init` bug by changing `ChunkedVec` element type from `T` to `MaybeUninit<T>` and implementing `Drop` to ensure proper memory safety. ([#1](https://github.com/QuarkPixel/ChunkedVec/issues/1))

## [0.3.1] - 2025-05-05

### Added

- Added `chunked_vec!` macro for simplified `ChunkedVec` creation
- Implemented `Extend` trait for `ChunkedVec`
- Implemented `From<&[T; M]>` trait for array references

### Changed

- Renamed `src/iter` directory to `src/iterators`

## [0.3.0] - 2025-04-30

### Added

- Added `ChunkedVecSized` type for compile-time fixed chunk size construction
- Added `from` module to support constructing ChunkedVec from multiple types
  - Implemented `FromIterator` trait
  - Implemented `From<Vec<T>>` trait
  - Implemented `From<[T; M]>` trait
  - Implemented `From<&[T]>` trait
- Added `allocated_capacity` method to query actual allocated capacity

### Changed

- Restructured project into modular components:
  - Moved constructor-related code to `constructors` module
  - Extracted `Chunk` type to separate implementation
  - Optimized code organization
- Improved `push()` method by removing `T: Copy + Default` constraint
- Renamed `with_chunks` to `with_chunk_count` for better clarity
- Applied Clippy suggestions to optimize code quality
- Fixed repository link to point to the correct address

### Enhanced

- Significantly improved documentation:
  - Added detailed usage examples
  - Enhanced API documentation
  - Optimized code comments

## [0.2.1] - 2025-04-28

- Fixed repository link points to the correct address

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
